#![no_std]

#[macro_use]
extern crate alloc;

pub mod gas_schedule;
pub mod types;
pub mod dvm;

use anyhow::{Result, ensure};
use crate::types::{ModuleTx, VmResult, ScriptTx, Gas};

pub trait Vm {
    fn publish_module(&self, gas: Gas, module: ModuleTx) -> VmResult;
    fn execute_script(&self, gas: Gas, tx: ScriptTx) -> VmResult;
    fn clear(&mut self);
}
