// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

/// A Rust representation of an Event Handle Resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventHandle {
    /// Total number of events emitted to this event stream.
    counter: u64,
    /// A globally unique ID for this event stream.
    guid: GUIDWrapper,
}

impl EventHandle {
    /// Constructs a new Event Handle
    pub fn new(guid: GUIDWrapper, counter: u64) -> Self {
        EventHandle { counter, guid }
    }

    /// Return the counter for the handle
    pub fn count(&self) -> u64 {
        self.counter
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GUIDWrapper {
    len_bytes: u8,
    guid: GUID,
}

/// A globally unique identifier derived from the sender's address and a counter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GUID {
    id: ID,
}

/// A non-privileged identifier that can be freely created by anyone. Useful for looking up GUID's.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ID {
    /// If creation_num is `i`, this is the `i+1`th GUID created by `addr`
    creation_num: u64,
    /// Address that created the GUID
    addr: AccountAddress,
}
