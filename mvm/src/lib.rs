#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;
extern crate sp_io;

pub mod access_path;
pub mod data;
pub mod gas_schedule;
pub mod mvm;
pub mod types;

use crate::types::{Gas, ModuleTx, ScriptTx};
use move_core_types::vm_status::StatusCode;

pub trait Vm {
    /// Publishes module to the chain.
    fn publish_module(&self, gas: Gas, module: ModuleTx) -> StatusCode;
    /// Execute script.
    fn execute_script(&self, gas: Gas, tx: ScriptTx) -> StatusCode;
    /// Clear vm cache.
    fn clear(&mut self);
}
