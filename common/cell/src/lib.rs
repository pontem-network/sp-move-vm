#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
mod lazy;
mod once_cell;

pub use crate::once_cell::OnceCell;
pub use lazy::Lazy;
