#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

#[cfg(feature = "sp_check")]
extern crate sp_io;
#[cfg(feature = "sp_check")]
#[allow(unused_imports)]
use sp_io::EcdsaVerifyError;

use crate::io::context::ExecutionContext;
use crate::types::{Gas, ModuleTx, PublishPackageTx, ScriptTx, VmResult};
use alloc::vec::Vec;
use anyhow::Error;
use diem_types::account_address::AccountAddress;
use move_core_types::language_storage::{ModuleId, StructTag};

pub mod abi;
pub mod error;
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

pub trait StateAccess {
    /// Return module bytecode by its id. `module_id` is ModuleId encoded by bcs.
    fn get_module(&self, module_id: &[u8]) -> Result<Option<Vec<u8>>, Error>;
    /// Return module abi bytecode by its id. `module_id` is ModuleId encoded by bcs.
    fn get_module_abi(&self, module_id: &[u8]) -> Result<Option<Vec<u8>>, Error>;
    /// Return resource by its account address and  struct tag. `tag` is StructTag encoded by bcs.
    fn get_resource(&self, address: &AccountAddress, tag: &[u8]) -> Result<Option<Vec<u8>>, Error>;
}
