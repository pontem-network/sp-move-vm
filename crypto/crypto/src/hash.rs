// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! This module defines traits and implementations of
//! [cryptographic hash functions](https://en.wikipedia.org/wiki/Cryptographic_hash_function)
//! for the Libra project.
//!
//! It is designed to help authors protect against two types of real world attacks:
//!
//! 1. **Semantic Ambiguity**: imagine that Alice has a private key and is using
//!    two different applications, X and Y. X asks Alice to sign a message saying
//!    "I am Alice". Alice accepts to sign this message in the context of X. However,
//!    unbeknownst to Alice, in application Y, messages beginning with the letter "I"
//!    represent transfers. " am " represents a transfer of 500 coins and "Alice"
//!    can be interpreted as a destination address. When Alice signed the message she
//!    needed to be aware of how other applications might interpret that message.
//!
//! 2. **Format Ambiguity**: imagine a program that hashes a pair of strings.
//!    To hash the strings `a` and `b` it hashes `a + "||" + b`. The pair of
//!    strings `a="foo||", b = "bar"` and `a="foo", b = "||bar"` result in the
//!    same input to the hash function and therefore the same hash. This
//!    creates a collision.
//!
//! Regarding (1), this library makes it easy for Libra developers to create as
//! many new "hashable" Rust types as needed so that each Rust type hashed and signed
//! in Libra has a unique meaning, that is, unambiguously captures the intent of a signer.
//!
//! Regarding (2), this library provides the `CryptoHasher` abstraction to easily manage
//! cryptographic seeds for hashing. Hashing seeds aim to ensure that
//! the hashes of values of a given type `MyNewStruct` never collide with hashes of values
//! from another type.
//!
//! Finally, to prevent format ambiguity within a same type `MyNewStruct` and facilitate protocol
//! specifications, we use [Libra Canonical Serialization (LCS)](../../libra_canonical_serialization/index.html)
//! as the recommended solution to write Rust values into a hasher.
//!
//! # Quick Start
//!
//! To obtain a `hash()` method for any new type `MyNewStruct`, it is (strongly) recommended to
//! use the derive macros of `serde` and `libra_crypto_derive` as follows:
//! ```
//! use libra_crypto::hash::CryptoHash;
//! use libra_crypto_derive::{CryptoHasher, LCSCryptoHash};
//! use serde::{Deserialize, Serialize};
//! #[derive(Serialize, Deserialize, CryptoHasher, LCSCryptoHash)]
//! struct MyNewStruct { /*...*/ }
//!
//! let value = MyNewStruct { /*...*/ };
//! value.hash();
//! ```
//!
//! Under the hood, this will generate a new implementation `MyNewStructHasher` for the trait
//! `CryptoHasher` and implement the trait `CryptoHash` for `MyNewStruct` using LCS.
//!
//! # Implementing New Hashers
//!
//! The trait `CryptoHasher` captures the notion of a pre-seeded hash function, aka a "hasher".
//! New implementations can be defined in two ways.
//!
//! ## Derive macro (recommended)
//!
//! For any new structure `MyNewStruct` that needs to be hashed, it is recommended to simply
//! use the derive macro [`CryptoHasher`](https://doc.rust-lang.org/reference/procedural-macros.html).
//!
//! ```
//! use libra_crypto_derive::CryptoHasher;
//! use serde::Deserialize;
//! #[derive(Deserialize, CryptoHasher)]
//! #[serde(rename = "OptionalCustomSerdeName")]
//! struct MyNewStruct { /*...*/ }
//! ```
//!
//! The macro `CryptoHasher` will define a hasher automatically called `MyNewStructHasher`, and derive a salt
//! using the name of the type as seen by the Serde library. In the example above, this name
//! was changed using the Serde parameter `rename`: the salt will be based on the value `OptionalCustomSerdeName`
//! instead of the default name `MyNewStruct`.
//!
//! ## Customized hashers
//!
//! **IMPORTANT:** Do NOT use this for new code unless you know what you are doing.
//!
//! This library also provides a few customized hashers defined in the code as follows:
//!
//! ```
//! # // To get around that there's no way to doc-test a non-exported macro:
//! # macro_rules! define_hasher { ($e:expr) => () }
//! define_hasher! { (MyNewDataHasher, MY_NEW_DATA_HASHER, MY_NEW_DATA_SEED, b"MyUniqueSaltString") }
//! ```
//!
//! # Using a hasher directly
//!
//! **IMPORTANT:** Do NOT use this for new code unless you know what you are doing.
//!
//! ```
//! use libra_crypto::hash::{CryptoHasher, TestOnlyHasher};
//!
//! let mut hasher = TestOnlyHasher::default();
//! hasher.update("Test message".as_bytes());
//! let hash_value = hasher.finish();
//! ```

use alloc::string::String;
use anyhow::{ensure, Error, Result};
use bytes::Bytes;
use mirai_annotations::*;
#[cfg(not(feature = "std"))]
use once_cell::race::OnceBox;
#[cfg(feature = "std")]
use once_cell::sync::OnceCell;
use short_hex_str::ShortHexStr;
use sp_std::prelude::Vec;
use sp_std::{self, convert::AsRef, fmt, str::FromStr};
use static_assertions::const_assert;
use tiny_keccak::{Hasher, Sha3};

/// A prefix used to begin the salt of every libra hashable structure. The salt
/// consists in this global prefix, concatenated with the specified
/// serialization name of the struct.
pub(crate) const LIBRA_HASH_PREFIX: &[u8] = b"LIBRA::";

/// Output value of our hash function. Intentionally opaque for safety and modularity.
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct HashValue {
    hash: [u8; HashValue::LENGTH],
}

impl HashValue {
    /// The length of the hash in bytes.
    pub const LENGTH: usize = 32;
    /// The length of the hash in bits.
    pub const LENGTH_IN_BITS: usize = Self::LENGTH * 8;
    /// The length of the hash in nibbles.
    pub const LENGTH_IN_NIBBLES: usize = Self::LENGTH * 2;

    /// Create a new [`HashValue`] from a byte array.
    pub fn new(hash: [u8; HashValue::LENGTH]) -> Self {
        HashValue { hash }
    }

    /// Create from a slice (e.g. retrieved from storage).
    pub fn from_slice(src: &[u8]) -> Result<Self> {
        ensure!(
            src.len() == HashValue::LENGTH,
            "HashValue decoding failed due to length mismatch. HashValue \
             length: {}, src length: {}",
            HashValue::LENGTH,
            src.len()
        );
        let mut value = Self::zero();
        value.hash.copy_from_slice(src);
        Ok(value)
    }

    /// Dumps into a vector.
    pub fn to_vec(&self) -> Vec<u8> {
        self.hash.to_vec()
    }

    /// Creates a zero-initialized instance.
    pub const fn zero() -> Self {
        HashValue {
            hash: [0; HashValue::LENGTH],
        }
    }

    /// Convenience function that computes a `HashValue` internally equal to
    /// the sha3_256 of a byte buffer. It will handle hasher creation, data
    /// feeding and finalization.
    ///
    /// Note this will not result in the `<T as CryptoHash>::hash()` for any
    /// reasonable struct T, as this computes a sha3 without any ornaments.
    pub fn sha3_256_of(buffer: &[u8]) -> Self {
        let mut sha3 = Sha3::v256();
        sha3.update(buffer);
        HashValue::from_keccak(sha3)
    }

    #[cfg(feature = "std")]
    #[cfg(test)]
    pub fn from_iter_sha3<'a, I>(buffers: I) -> Self
    where
        I: IntoIterator<Item = &'a [u8]>,
    {
        let mut sha3 = Sha3::v256();
        for buffer in buffers {
            sha3.update(buffer);
        }
        HashValue::from_keccak(sha3)
    }

    fn as_ref_mut(&mut self) -> &mut [u8] {
        &mut self.hash[..]
    }

    fn from_keccak(state: Sha3) -> Self {
        let mut hash = Self::zero();
        state.finalize(hash.as_ref_mut());
        hash
    }

    /// Returns a `HashValueBitIterator` over all the bits that represent this `HashValue`.
    pub fn iter_bits(&self) -> HashValueBitIterator<'_> {
        HashValueBitIterator::new(self)
    }

    /// Constructs a `HashValue` from an iterator of bits.
    pub fn from_bit_iter(iter: impl ExactSizeIterator<Item = bool>) -> Result<Self> {
        ensure!(
            iter.len() == Self::LENGTH_IN_BITS,
            "The iterator should yield exactly {} bits. Actual number of bits: {}.",
            Self::LENGTH_IN_BITS,
            iter.len(),
        );

        let mut buf = [0; Self::LENGTH];
        for (i, bit) in iter.enumerate() {
            if bit {
                buf[i / 8] |= 1 << (7 - i % 8);
            }
        }
        Ok(Self::new(buf))
    }

    /// Returns the length of common prefix of `self` and `other` in bits.
    pub fn common_prefix_bits_len(&self, other: HashValue) -> usize {
        self.iter_bits()
            .zip(other.iter_bits())
            .take_while(|(x, y)| x == y)
            .count()
    }

    /// Returns the length of common prefix of `self` and `other` in nibbles.
    pub fn common_prefix_nibbles_len(&self, other: HashValue) -> usize {
        self.common_prefix_bits_len(other) / 4
    }

    /// Returns first 4 bytes as hex-formatted string
    pub fn short_str(&self) -> ShortHexStr {
        const_assert!(HashValue::LENGTH >= ShortHexStr::SOURCE_LENGTH);
        ShortHexStr::try_from_bytes(&self.hash)
            .expect("This can never fail since HashValue::LENGTH >= ShortHexStr::SOURCE_LENGTH")
    }

    /// Full hex representation of a given hash value.
    pub fn to_hex(&self) -> String {
        hex::encode(self.hash)
    }

    /// Parse a given hex string to a hash value.
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        //   Self::from_slice(hex::decode(hex_str)?.as_slice())
        Self::from_slice(hex::decode(hex_str).map_err(anyhow::Error::msg)?.as_slice())
    }
}

impl Default for HashValue {
    fn default() -> Self {
        HashValue::zero()
    }
}

impl AsRef<[u8; HashValue::LENGTH]> for HashValue {
    fn as_ref(&self) -> &[u8; HashValue::LENGTH] {
        &self.hash
    }
}

impl sp_std::ops::Index<usize> for HashValue {
    type Output = u8;

    fn index(&self, s: usize) -> &u8 {
        self.hash.index(s)
    }
}

impl fmt::Binary for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.hash {
            write!(f, "{:08b}", byte)?;
        }
        Ok(())
    }
}

impl fmt::LowerHex for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.hash {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl fmt::Debug for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HashValue(")?;
        <Self as fmt::LowerHex>::fmt(self, f)?;
        write!(f, ")")?;
        Ok(())
    }
}

/// Will print shortened (4 bytes) hash
impl fmt::Display for HashValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.hash.iter().take(4) {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl From<HashValue> for Bytes {
    fn from(value: HashValue) -> Bytes {
        Bytes::copy_from_slice(value.hash.as_ref())
    }
}

impl FromStr for HashValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        HashValue::from_hex(s)
    }
}

/// An iterator over `HashValue` that generates one bit for each iteration.
pub struct HashValueBitIterator<'a> {
    /// The reference to the bytes that represent the `HashValue`.
    hash_bytes: &'a [u8],
    pos: sp_std::ops::Range<usize>,
    // invariant hash_bytes.len() == HashValue::LENGTH;
    // invariant pos.end == hash_bytes.len() * 8;
}

impl<'a> HashValueBitIterator<'a> {
    /// Constructs a new `HashValueBitIterator` using given `HashValue`.
    fn new(hash_value: &'a HashValue) -> Self {
        HashValueBitIterator {
            hash_bytes: hash_value.as_ref(),
            pos: (0..HashValue::LENGTH_IN_BITS),
        }
    }

    /// Returns the `index`-th bit in the bytes.
    fn get_bit(&self, index: usize) -> bool {
        assume!(index < self.pos.end); // assumed precondition
        assume!(self.hash_bytes.len() == HashValue::LENGTH); // invariant
        assume!(self.pos.end == self.hash_bytes.len() * 8); // invariant
        let pos = index / 8;
        let bit = 7 - index % 8;
        (self.hash_bytes[pos] >> bit) & 1 != 0
    }
}

impl<'a> sp_std::iter::Iterator for HashValueBitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos.next().map(|x| self.get_bit(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.pos.size_hint()
    }
}

impl<'a> sp_std::iter::DoubleEndedIterator for HashValueBitIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pos.next_back().map(|x| self.get_bit(x))
    }
}

impl<'a> sp_std::iter::ExactSizeIterator for HashValueBitIterator<'a> {}

/// A type that can be cryptographically hashed to produce a `HashValue`.
///
/// In most cases, this trait should not be implemented manually but rather derived using
/// the macros `serde::Serialize`, `CryptoHasher`, and `LCSCryptoHash`.
pub trait CryptoHash {
    /// The associated `Hasher` type which comes with a unique salt for this type.
    type Hasher: CryptoHasher;

    /// Hashes the object and produces a `HashValue`.
    fn hash(&self) -> HashValue;
}

/// A trait for representing the state of a cryptographic hasher.
pub trait CryptoHasher: Default {
    /// the seed used to initialize hashing `Self` before the serialization bytes of the actual value
    fn seed() -> &'static [u8; 32];

    /// Write bytes into the hasher.
    fn update(&mut self, bytes: &[u8]);

    /// Finish constructing the [`HashValue`].
    fn finish(self) -> HashValue;
}

/// The default hasher underlying generated implementations of `CryptoHasher`.
#[doc(hidden)]
#[derive(Clone)]
pub struct DefaultHasher {
    state: Sha3,
}

impl DefaultHasher {
    #[doc(hidden)]
    /// This function does not return a HashValue in the sense of our usual
    /// hashes, but a construction of initial bytes that are fed into any hash
    /// provided we're passed  a (lcs) serialization name as argument.
    pub fn prefixed_hash(buffer: &[u8]) -> [u8; HashValue::LENGTH] {
        // The salt is initial material we prefix to actual value bytes for
        // domain separation. Its length is variable.
        let salt: Vec<u8> = [LIBRA_HASH_PREFIX, buffer].concat();
        // The seed is a fixed-length hash of the salt, thereby preventing
        // suffix attacks on the domain separation bytes.
        HashValue::sha3_256_of(&salt[..]).hash
    }

    #[doc(hidden)]
    pub fn new(typename: &[u8]) -> Self {
        let mut state = Sha3::v256();
        if !typename.is_empty() {
            state.update(&Self::prefixed_hash(typename));
        }
        DefaultHasher { state }
    }

    #[doc(hidden)]
    pub fn update(&mut self, bytes: &[u8]) {
        self.state.update(bytes);
    }

    #[doc(hidden)]
    pub fn finish(self) -> HashValue {
        let mut hasher = HashValue::default();
        self.state.finalize(hasher.as_ref_mut());
        hasher
    }
}

impl fmt::Debug for DefaultHasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DefaultHasher: state = Sha3")
    }
}

macro_rules! define_hasher {
    (
        $(#[$attr:meta])*
        ($hasher_type: ident, $hasher_name: ident, $seed_name: ident, $salt: expr)
    ) => {

        #[derive(Clone, Debug)]
        $(#[$attr])*
        pub struct $hasher_type(DefaultHasher);

        impl $hasher_type {
            fn new() -> Self {
                $hasher_type(DefaultHasher::new($salt))
            }
        }

        #[cfg(feature = "std")]
        static $hasher_name: OnceCell<$hasher_type> = OnceCell::new();
        #[cfg(not(feature = "std"))]
        static $hasher_name: OnceBox<$hasher_type> = OnceBox::new();

        #[cfg(feature = "std")]
        static $seed_name: OnceCell<[u8; 32]> = OnceCell::new();
        #[cfg(not(feature = "std"))]
        static $seed_name: OnceBox<[u8; 32]> = OnceBox::new();

        #[cfg(feature = "std")]
        impl Default for $hasher_type {
            fn default() -> Self {
                $hasher_name.get_or_init(|| $hasher_type::new()).clone()
            }
        }

        #[cfg(not(feature = "std"))]
        impl Default for $hasher_type {
            fn default() -> Self {
                $hasher_name.get_or_init(|| alloc::boxed::Box::new($hasher_type::new())).clone()
            }
        }

        impl CryptoHasher for $hasher_type {
            #[cfg(feature = "std")]
            fn seed() -> &'static [u8;32] {
                $seed_name.get_or_init(|| {
                    DefaultHasher::prefixed_hash($salt)
                })
            }

            #[cfg(not(feature = "std"))]
            fn seed() -> &'static [u8;32] {
                $seed_name.get_or_init(|| {
                    alloc::boxed::Box::new(DefaultHasher::prefixed_hash($salt))
                })
            }

            fn update(&mut self, bytes: &[u8]) {
                self.0.update(bytes);
            }

            fn finish(self) -> HashValue {
                self.0.finish()
            }
        }
    };
}

define_hasher! {
     /// The hasher used to compute the hash of an internal node in the transaction accumulator.
     (
          TransactionAccumulatorHasher,
          TRANSACTION_ACCUMULATOR_HASHER,
          TRANSACTION_ACCUMULATOR_SEED,
          b"TransactionAccumulator"
     )
}

define_hasher! {
     /// The hasher used to compute the hash of an internal node in the event accumulator.
     (
          EventAccumulatorHasher,
          EVENT_ACCUMULATOR_HASHER,
          EVENT_ACCUMULATOR_SEED,
          b"EventAccumulator"
     )
}

define_hasher! {
     /// The hasher used to compute the hash of an internal node in the Sparse Merkle Tree.
     (
          SparseMerkleInternalHasher,
          SPARSE_MERKLE_INTERNAL_HASHER,
          SPARSE_MERKLE_INTERNAL_SEED,
          b"SparseMerkleInternal"
     )
}

define_hasher! {
     /// The hasher used to compute the hash of an internal node in the transaction accumulator.
     (
          VoteProposalHasher,
          VOTE_PROPOSAL_HASHER,
          VOTE_PROPOSAL_SEED,
          b"VoteProposalHasher"
     )
}

define_hasher! {
     /// The hasher used only for testing. It doesn't have a salt.
     (TestOnlyHasher, TEST_ONLY_HASHER, TEST_ONLY_SEED, b"")
}

fn create_literal_hash(word: &str) -> HashValue {
    let mut s = word.as_bytes().to_vec();
    assert!(s.len() <= HashValue::LENGTH);
    s.resize(HashValue::LENGTH, 0);
    HashValue::from_slice(&s).expect("Cannot fail")
}

macro_rules! define_hash {
    (
        $(#[$attr:meta])*
        ($hash_name: ident, $fn_name: ident, $salt: expr)
    ) => {
        #[cfg(feature = "std")]
        static $hash_name: OnceCell<HashValue> = OnceCell::new();

        #[cfg(not(feature = "std"))]
        static $hash_name: OnceBox<HashValue> = OnceBox::new();

        #[cfg(feature = "std")]
        $(#[$attr])*
        pub fn $fn_name() -> &'static HashValue {
            $hash_name.get_or_init(|| create_literal_hash($salt))
        }

        #[cfg(not(feature = "std"))]
        $(#[$attr])*
        pub fn $fn_name() -> &'static HashValue {
            $hash_name.get_or_init(|| alloc::boxed::Box::new(create_literal_hash($salt)))
        }
    };
}

define_hash! {
    /// Placeholder hash of `Accumulator`.
    (
        ACCUMULATOR_PLACEHOLDER_HASH,
        accumulator_placeholder_hash,
        "ACCUMULATOR_PLACEHOLDER_HASH"
    )
}

define_hash! {
    /// Placeholder hash of `SparseMerkleTree`.
    (
        SPARSE_MERKLE_PLACEHOLDER_HASH,
        sparse_merkle_placeholder_hash,
        "SPARSE_MERKLE_PLACEHOLDER_HASH"
    )
}

define_hash! {
    /// Block id reserved as the id of parent block of the genesis block.
    (
        PRE_GENESIS_BLOCK_ID,
        pre_genesis_block_id,
        "PRE_GENESIS_BLOCK_ID"
    )
}
