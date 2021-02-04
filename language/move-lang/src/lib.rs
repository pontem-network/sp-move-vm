#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

use crate::codespan::Span;
use alloc::collections::BTreeMap;
use alloc::string::String;

pub mod codespan;
pub mod errors;
pub mod location;
pub mod parser;
pub mod shared;

/// Types to represent comments.
pub type CommentMap = BTreeMap<&'static str, MatchedFileCommentMap>;
pub type MatchedFileCommentMap = BTreeMap<u32, String>;
pub type FileCommentMap = BTreeMap<Span, String>;
