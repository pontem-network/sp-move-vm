use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use core::cell::RefCell;

use anyhow::Error;
use anyhow::{anyhow, ensure};
use hashbrown::HashMap;

#[cfg(feature = "move_stdlib")]
use {
    diem_types::account_config,
    diem_types::chain_id::ChainId,
    move_core_types::value::{serialize_values, MoveValue},
};

use diem_types::on_chain_config::VMConfig;
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::CostTable;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, TypeTag, CORE_CODE_ADDRESS};
use move_core_types::vm_status::StatusCode;

use crate::gas_schedule::cost_table;
use crate::io::balance::CurrencyInfo;
use crate::io::traits::{Balance, BalanceAccess, CurrencyAccessPath, EventHandler, Storage};
use crate::mvm::Mvm;
use crate::types::{Gas, ModulePackage, PublishPackageTx};
use crate::Vm;

pub fn init_storage<S>(storage: S, config: GenesisConfig) -> Result<(), Error>
    where
        S: Storage,
{
    let fork = StorageFork::new(storage);
    let vm_config = VMConfig {
        gas_schedule: config.cost_table.clone(),
    };
    let vm = Mvm::new_with_config(&fork, NopeEventHandler, NopeBalance, vm_config)?;
    let result = vm.publish_module_package(Gas::infinite(), config.stdlib, false);
    ensure!(
        result.status_code == StatusCode::EXECUTED,
        "Failed to publish stdlib:{:?}",
        result
    );

    if let Some(init_func_config) = config.init_func_config {
        let res = vm.execute_function(
            CORE_CODE_ADDRESS,
            Gas::infinite(),
            &ModuleId::new(
                CORE_CODE_ADDRESS,
                Identifier::from_utf8(init_func_config.module)?,
            ),
            Identifier::from_utf8(init_func_config.func)?.as_ident_str(),
            vec![],
            init_func_config.args,
            None,
        );

        if res.status_code != StatusCode::EXECUTED {
            return Err(anyhow!("Failed to execution genesis function:{:?}", res));
        }
    }

    fork.merge();
    Ok(())
}

// Genesis configuration.
pub struct GenesisConfig {
    pub stdlib: PublishPackageTx,
    // Standard library.
    pub init_func_config: Option<InitFuncConfig>,
    // Initialize function config.
    cost_table: CostTable,                        // Cost table.
}

impl Default for GenesisConfig {
    #[cfg(feature = "move_stdlib")]
    fn default() -> Self {
        use core::convert::TryFrom;
        let mut diem_stdlib = ModulePackage::try_from(stdlib::diem_stdlib_package()).expect("Expected valid stdlib");
        let move_stdlib = ModulePackage::try_from(stdlib::stdlib_package()).expect("Expected valid stdlib");
        diem_stdlib.join(move_stdlib);
        let stdlib = diem_stdlib.into_tx(CORE_CODE_ADDRESS);

        let cost_table = cost_table();
        let instr_gas_costs = bcs::to_bytes(&cost_table.instruction_table)
            .expect("Expected to convert instr gas costs to bsc bytes");
        let native_gas_costs = bcs::to_bytes(&cost_table.native_table)
            .expect("Expected to convert genesis native gas costs to bsc bytes");
        let chain_id: ChainId = Default::default();

        GenesisConfig {
            stdlib,
            cost_table,
            init_func_config: Some(InitFuncConfig {
                module: "Genesis".as_bytes().to_vec(),
                func: "initialize".as_bytes().to_vec(),
                args: serialize_values(&vec![
                    MoveValue::Signer(account_config::diem_root_address()), // dr_signer
                    MoveValue::Signer(account_config::treasury_compliance_account_address()), // tr_signer
                    MoveValue::vector_u8(account_config::diem_root_address().to_u8()[20..].to_vec()), // dr_address
                    MoveValue::vector_u8(
                        account_config::treasury_compliance_account_address().to_u8()[20..].to_vec(),
                    ), // tr_address
                    MoveValue::Vector(vec![]), // Initial allow list.
                    MoveValue::Bool(true),
                    MoveValue::vector_u8(instr_gas_costs),
                    MoveValue::vector_u8(native_gas_costs),
                    MoveValue::U8(chain_id.id()),
                    MoveValue::U64(0),
                    MoveValue::vector_u8(vec![])
                ]),
            }),
        }
    }

    #[cfg(not(feature = "move_stdlib"))]
    fn default() -> Self {
        let stdlib = ModulePackage::default().into_tx(CORE_CODE_ADDRESS);

        GenesisConfig {
            stdlib,
            cost_table: cost_table(),
            init_func_config: Default::default(),
        }
    }
}

/// Passing arguments to init function, so it configurable.
#[derive(Default)]
pub struct InitFuncConfig {
    pub module: Vec<u8>,
    pub func: Vec<u8>,
    pub args: Vec<Vec<u8>>,
}

pub fn build_genesis_config(
    stdlib: PublishPackageTx,
    init_func_config: Option<InitFuncConfig>,
) -> GenesisConfig {
    GenesisConfig {
        stdlib,
        cost_table: cost_table(),
        init_func_config,
    }
}

struct NopeEventHandler;

impl EventHandler for NopeEventHandler {
    fn on_event(&self, _: Vec<u8>, _: u64, _: TypeTag, _: Vec<u8>) {
        //no-op
    }
}

struct NopeBalance;

impl BalanceAccess for NopeBalance {
    fn get_currency_info(&self, _: &CurrencyAccessPath) -> Option<CurrencyInfo> {
        None
    }

    fn get_balance(&self, _: &AccountAddress, _: &CurrencyAccessPath) -> Option<Balance> {
        None
    }

    fn add(&self, _: &AccountAddress, _: &CurrencyAccessPath, _: Balance) {
        //no-op
    }

    fn sub(&self, _: &AccountAddress, _: &CurrencyAccessPath, _: Balance) {
        //no-op
    }
}

pub struct StorageFork<S: Storage> {
    inner: S,
    diff: RefCell<HashMap<Cow<'static, [u8]>, Option<Vec<u8>>>>,
}

impl<S: Storage> StorageFork<S> {
    pub fn new(storage: S) -> StorageFork<S> {
        StorageFork {
            inner: storage,
            diff: RefCell::new(Default::default()),
        }
    }

    pub fn merge(self) {
        for (key, val) in self.diff.take() {
            match val {
                None => {
                    self.inner.remove(key.as_ref());
                }
                Some(val) => {
                    self.inner.insert(key.as_ref(), val.as_ref());
                }
            }
        }
    }
}

impl<S: Storage> Storage for &StorageFork<S> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let diff = self.diff.borrow();
        if let Some(val) = diff.get(key) {
            val.to_owned()
        } else {
            self.inner.get(key)
        }
    }

    fn insert(&self, key: &[u8], value: &[u8]) {
        let mut diff = self.diff.borrow_mut();

        diff.insert(Cow::Owned(key.to_vec()), Some(value.to_vec()));
    }

    fn remove(&self, key: &[u8]) {
        let mut diff = self.diff.borrow_mut();
        diff.insert(Cow::Owned(key.to_vec()), None);
    }
}
