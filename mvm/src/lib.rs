#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;
extern crate sp_core;
use crate::data::ExecutionContext;
use crate::types::{Gas, ModuleTx, PublishPackageTx, ScriptTx, VmResult};

pub mod data;
pub mod gas_schedule;
pub mod genesis;
pub mod io;
pub mod mvm;
pub mod types;

pub trait Vm {
    /// Publishes module to the chain.
    fn publish_module(&self, gas: Gas, module: ModuleTx, dry_run: bool) -> VmResult;

    /// Publishes package of modules to the chain.
    fn publish_module_package(
        &self,
        gas: Gas,
        package: PublishPackageTx,
        dry_run: bool,
    ) -> VmResult;

    /// Execute script.
    fn execute_script(
        &self,
        gas: Gas,
        context: ExecutionContext,
        tx: ScriptTx,
        dry_run: bool,
    ) -> VmResult;

    /// Clear vm cache.
    fn clear(&self);
}
