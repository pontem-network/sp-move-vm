// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! Internal module containing convenience utility functions mainly for testing

/// A keypair consisting of a private and public key
#[derive(PartialEq, Eq)]
pub struct KeyPair<S, P>
where
    for<'a> P: From<&'a S>,
{
    /// the private key component
    pub private_key: S,
    /// the public key component
    pub public_key: P,
}
