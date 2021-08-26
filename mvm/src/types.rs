use alloc::vec::Vec;
use core::convert::TryFrom;
use core::fmt;

use anyhow::*;
use parity_scale_codec_derive::{Decode, Encode};
use serde::{Deserialize, Serialize};

use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::value::MoveValue;
use move_core_types::vm_status::StatusCode;

use crate::error::SubStatus;
use diem_types::account_config::{diem_root_address, treasury_compliance_account_address};
use move_binary_format::errors::Location;
use move_core_types::identifier::Identifier;

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

    /// Create infinite gas.
    pub fn infinite() -> Gas {
        Gas {
            max_gas_amount: GAS_AMOUNT_MAX_VALUE,
            gas_unit_price: 1,
        }
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
    call: Call,
    args: Vec<Vec<u8>>,
    type_args: Vec<TypeTag>,
    signers: Vec<AccountAddress>,
}

/// Script transaction.
impl ScriptTx {
    /// Constructor.
    pub fn with_script(
        code: Vec<u8>,
        args: Vec<ScriptArg>,
        type_args: Vec<TypeTag>,
        signers: Vec<AccountAddress>,
    ) -> Result<ScriptTx> {
        let args = args
            .into_iter()
            .map(ScriptArg::into)
            .map(|val: MoveValue| bcs::to_bytes(&val))
            .collect::<Result<_, _>>()
            .map_err(Error::msg)?;
        Ok(ScriptTx {
            call: Call::Script { code },
            args,
            type_args,
            signers,
        })
    }

    /// Script bytecode.
    pub fn call(&self) -> &Call {
        &self.call
    }

    /// Parameters passed to the main function.
    pub fn args(&self) -> &[Vec<u8>] {
        &self.args
    }

    /// Type parameters passed to the main function.
    pub fn type_parameters(&self) -> &[TypeTag] {
        &self.type_args
    }

    /// Returns script signers.
    pub fn signers(&self) -> &[AccountAddress] {
        &self.signers
    }

    /// Convert into internal data.
    pub fn into_inner(self) -> (Call, Vec<Vec<u8>>, Vec<TypeTag>, Vec<AccountAddress>) {
        (self.call, self.args, self.type_args, self.signers)
    }
}

/// Move VM result.
#[derive(Debug)]
pub struct VmResult {
    /// Execution status code.
    pub status_code: StatusCode,
    /// Execution sub status code.
    pub sub_status: Option<SubStatus>,
    /// Gas used.
    pub gas_used: u64,
    /// Error location
    pub location: Option<Location>,
}

impl VmResult {
    /// Create new Vm result
    pub(crate) fn new(
        status_code: StatusCode,
        sub_status: Option<u64>,
        location: Option<Location>,
        gas_used: u64,
    ) -> VmResult {
        VmResult {
            status_code,
            sub_status: sub_status.map(SubStatus::new),
            gas_used,
            location,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
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

impl From<ScriptArg> for MoveValue {
    fn from(arg: ScriptArg) -> Self {
        match arg {
            ScriptArg::U8(val) => MoveValue::U8(val),
            ScriptArg::U64(val) => MoveValue::U64(val),
            ScriptArg::U128(val) => MoveValue::U128(val),
            ScriptArg::Bool(val) => MoveValue::Bool(val),
            ScriptArg::Address(val) => MoveValue::Address(val),
            ScriptArg::VectorU8(val) => MoveValue::vector_u8(val),
            ScriptArg::VectorU64(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::U64).collect())
            }
            ScriptArg::VectorU128(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::U128).collect())
            }
            ScriptArg::VectorBool(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::Bool).collect())
            }
            ScriptArg::VectorAddress(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::Address).collect())
            }
        }
    }
}

/// Signer type.
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Signer {
    /// Root signer.
    Root,
    /// Treasury signer.
    Treasury,
    /// Template to replace.
    Placeholder,
}

/// Transaction model.
#[derive(Serialize, Deserialize, Debug)]
pub enum Transaction {
    /// Version 1.
    V1(TxV1),
}

/// Transaction model.
#[derive(Serialize, Deserialize, Debug)]
pub struct TxV1 {
    /// Signers.
    pub signers: Vec<Signer>,
    /// Call declaration.
    pub call: Call,
    /// Script args.
    pub args: Vec<Vec<u8>>,
    /// Script type arguments.
    pub type_args: Vec<TypeTag>,
}

/// Call declaration.
#[derive(Serialize, Deserialize, Debug)]
pub enum Call {
    /// Script
    Script {
        /// Script bytecode.
        code: Vec<u8>,
    },
    /// Function in module with script viability.
    ScriptFunction {
        /// Module address.
        mod_address: AccountAddress,
        /// Module name.
        mod_name: Identifier,
        /// Function name.
        func_name: Identifier,
    },
}

impl Transaction {
    pub fn into_script(self, mut signers: Vec<AccountAddress>) -> Result<ScriptTx> {
        let tx = self.inner();
        let signers = tx
            .signers
            .iter()
            .map(|s| match *s {
                Signer::Root => Ok(diem_root_address()),
                Signer::Treasury => Ok(treasury_compliance_account_address()),
                Signer::Placeholder => {
                    if let Some(signer) = signers.pop() {
                        Ok(signer)
                    } else {
                        Err(anyhow!("Invalid signers count."))
                    }
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ScriptTx {
            call: tx.call,
            args: tx.args,
            type_args: tx.type_args,
            signers,
        })
    }

    fn inner(self) -> TxV1 {
        match self {
            Transaction::V1(val) => val,
        }
    }

    pub fn has_root_signer(&self) -> bool {
        self.as_ref().signers.iter().any(|s| *s == Signer::Root)
    }

    pub fn has_treasury_signer(&self) -> bool {
        self.as_ref().signers.iter().any(|s| *s == Signer::Treasury)
    }

    pub fn signers_count(&self) -> usize {
        self.as_ref()
            .signers
            .iter()
            .filter(|s| **s == Signer::Placeholder)
            .count()
    }
}

impl AsRef<TxV1> for Transaction {
    fn as_ref(&self) -> &TxV1 {
        match self {
            Transaction::V1(val) => val,
        }
    }
}

impl TryFrom<&[u8]> for Transaction {
    type Error = Error;

    fn try_from(blob: &[u8]) -> Result<Self, Self::Error> {
        bcs::from_bytes(blob).map_err(Error::msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ModulePackage {
    modules: Vec<Vec<u8>>,
}

impl ModulePackage {
    pub fn into_tx(self, address: AccountAddress) -> PublishPackageTx {
        PublishPackageTx {
            modules: self.modules,
            address,
        }
    }
}

impl TryFrom<&[u8]> for ModulePackage {
    type Error = Error;

    fn try_from(blob: &[u8]) -> Result<Self, Self::Error> {
        bcs::from_bytes(blob).map_err(Error::msg)
    }
}

#[derive(Debug)]
pub struct PublishPackageTx {
    modules: Vec<Vec<u8>>,
    address: AccountAddress,
}

impl PublishPackageTx {
    pub fn into_inner(self) -> (Vec<Vec<u8>>, AccountAddress) {
        (self.modules, self.address)
    }
}
