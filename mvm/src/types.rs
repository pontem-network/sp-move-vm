use alloc::vec::Vec;
use anyhow::*;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::StatusCode;
use move_vm_types::values::Value;
use parity_scale_codec::{Decode, Encode};
use sp_std::fmt;

const GAS_AMOUNT_MAX_VALUE: u64 = u64::MAX / 1000;

/// Stores gas metadata for vm execution.
#[derive(Debug)]
pub struct Gas {
    /// Max gas units to be used in transaction execution.
    pub(crate) max_gas_amount: u64,
    /// Price in `XFI` coins per unit of gas.
    pub(crate) gas_unit_price: u64,
}

impl Gas {
    /// Constructor.
    pub fn new(max_gas_amount: u64, gas_unit_price: u64) -> Result<Gas> {
        ensure!(
            max_gas_amount < GAS_AMOUNT_MAX_VALUE,
            "max_gas_amount value must be in the range from 0 to {}",
            GAS_AMOUNT_MAX_VALUE
        );

        Ok(Gas {
            max_gas_amount,
            gas_unit_price,
        })
    }

    /// Returns max gas units to be used in transaction execution.
    pub fn max_gas_amount(&self) -> u64 {
        self.max_gas_amount
    }

    /// Returns price in `DFI` coins per unit of gas.
    pub fn gas_unit_price(&self) -> u64 {
        self.gas_unit_price
    }
}

/// Module transaction.
#[derive(Clone, Encode, Decode)]
pub struct ModuleTx {
    code: Vec<u8>,
    sender: AccountAddress,
}

impl ModuleTx {
    /// Constructor.
    pub fn new(code: Vec<u8>, sender: AccountAddress) -> ModuleTx {
        ModuleTx { code, sender }
    }

    /// Returns module bytecode.
    pub fn code(&self) -> &[u8] {
        &self.code
    }

    /// Convert into internal data.
    pub fn into_inner(self) -> (Vec<u8>, AccountAddress) {
        (self.code, self.sender)
    }
}

impl fmt::Debug for ModuleTx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Module")
            .field("code", &hex::encode(&self.code))
            .field("sender", &self.sender)
            .finish()
    }
}

/// Script bytecode + passed arguments and type parameters.
pub struct ScriptTx {
    code: Vec<u8>,
    args: Vec<Value>,
    type_args: Vec<TypeTag>,
    senders: Vec<AccountAddress>,
}

/// Script transaction.
impl ScriptTx {
    /// Constructor.
    pub fn new(
        code: Vec<u8>,
        args: Vec<ScriptArg>,
        type_args: Vec<TypeTag>,
        senders: Vec<AccountAddress>,
    ) -> Result<Self> {
        ensure!(
            !senders.is_empty(),
            "senders value must be in the range from 0 to ",
        );
        Ok(ScriptTx {
            code,
            args: args.into_iter().map(ScriptArg::into).collect(),
            type_args,
            senders,
        })
    }

    /// Script bytecode.
    pub fn code(&self) -> &[u8] {
        &self.code
    }

    /// Parameters passed to main() function.
    pub fn args(&self) -> &[Value] {
        &self.args
    }

    /// Convert into internal data.
    pub fn into_inner(self) -> (Vec<u8>, Vec<Value>, Vec<TypeTag>, Vec<AccountAddress>) {
        (self.code, self.args, self.type_args, self.senders)
    }
}

impl fmt::Debug for ScriptTx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Script")
            .field("code", &hex::encode(&self.code))
            .field("args", &self.args)
            .field("type_args", &self.type_args)
            .field("senders", &self.senders)
            .finish()
    }
}

/// Move VM result.
#[derive(Debug)]
pub struct VmResult {
    /// Execution status code.
    pub status_code: StatusCode,
    /// Gas used.
    pub gas_used: u64,
}

impl VmResult {
    /// Create new Vm result
    pub(crate) fn new(status_code: StatusCode, gas_used: u64) -> VmResult {
        VmResult {
            status_code,
            gas_used,
        }
    }
}

#[derive(Clone, Debug, Encode, Decode, PartialEq, PartialOrd)]
pub enum ScriptArg {
    U8(u8),
    U64(u64),
    U128(u128),
    Bool(bool),
    Address(AccountAddress),
    VectorU8(Vec<u8>),
    VectorU64(Vec<u64>),
    VectorU128(Vec<u128>),
    VectorBool(Vec<bool>),
    VectorAddress(Vec<AccountAddress>),
}

impl From<ScriptArg> for Value {
    fn from(arg: ScriptArg) -> Self {
        match arg {
            ScriptArg::U8(val) => Value::u8(val),
            ScriptArg::U64(val) => Value::u64(val),
            ScriptArg::U128(val) => Value::u128(val),
            ScriptArg::Bool(val) => Value::bool(val),
            ScriptArg::Address(val) => Value::address(val),
            ScriptArg::VectorU8(val) => Value::vector_u8(val),
            ScriptArg::VectorU64(val) => Value::vector_u64(val),
            ScriptArg::VectorU128(val) => Value::vector_u128(val),
            ScriptArg::VectorBool(val) => Value::vector_bool(val),
            ScriptArg::VectorAddress(val) => Value::vector_address(val),
        }
    }
}
