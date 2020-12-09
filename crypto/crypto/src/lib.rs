// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! This feature gets turned on only if libra-crypto is compiled via MIRAI in a nightly build.
#![cfg_attr(mirai, allow(incomplete_features), feature(const_generics))]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod ed25519;
pub mod hash;
pub mod test_utils;
pub mod traits;

pub use self::traits::*;
pub use hash::HashValue;

///Reexport once_cell and serde_name for use in CryptoHasher Derive implementation.
#[doc(hidden)]
pub use once_cell as _once_cell;
#[doc(hidden)]
pub use serde_name as _serde_name;
