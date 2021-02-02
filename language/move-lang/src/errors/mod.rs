// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::location::Loc;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;

//**************************************************************************************************
// Types
//**************************************************************************************************

pub type Errors = Vec<Error>;
pub type Error = Vec<(Loc, String)>;
pub type ErrorSlice = [(Loc, String)];
pub type HashableError = Vec<(&'static str, usize, usize, String)>;

pub type FilesSourceText = HashMap<&'static str, String>;
