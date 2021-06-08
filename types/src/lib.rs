#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod chain_id;
pub mod account_config;
pub mod on_chain_config;
pub mod access_path;