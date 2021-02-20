use alloc::vec::Vec;
use anyhow::*;
use core::convert::TryFrom;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::vm_status::StatusCode;
use move_lang::parser::ast::{ModuleAccess_, ModuleIdent_, Type, Type_};
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::parse_type;
use move_vm_types::values::Value;
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
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

    /// Parameters passed to the main function.
    pub fn args(&self) -> &[Value] {
        &self.args
    }

    /// Type parameters passed to the main function.
    pub fn type_parameters(&self) -> &[TypeTag] {
        &self.type_args
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

pub fn parse_type_params(tkn: &str) -> Result<Vec<TypeTag>> {
    let map_err = |err| Error::msg(format!("{:?}", err));

    let mut lexer = Lexer::new(tkn, "query", Default::default());
    lexer.advance().map_err(map_err)?;

    let mut types = Vec::new();
    while lexer.peek() != Tok::EOF {
        let ty = parse_type(&mut lexer).map_err(map_err)?;
        types.push(unwrap_spanned_ty_(ty, None)?);
        let tkn = lexer.peek();

        if tkn != Tok::Semicolon && tkn != Tok::Comma && tkn != Tok::EOF {
            return Err(Error::msg("Invalid separator. Expected [;,]"));
        }
        lexer.advance().map_err(map_err)?;
    }
    Ok(types)
}

fn unwrap_spanned_ty_(ty: Type, this: Option<AccountAddress>) -> Result<TypeTag, Error> {
    let st = match ty.value {
        Type_::Apply(ma, mut ty_params) => {
            match (ma.value, this) {
                // N
                (ModuleAccess_::Name(name), this) => match name.value.as_ref() {
                    "bool" => TypeTag::Bool,
                    "u8" => TypeTag::U8,
                    "u64" => TypeTag::U64,
                    "u128" => TypeTag::U128,
                    "address" => TypeTag::Address,
                    "signer" => TypeTag::Signer,
                    "Vec" if ty_params.len() == 1 => TypeTag::Vector(
                        unwrap_spanned_ty_(ty_params.pop().unwrap(), this)
                            .unwrap()
                            .into(),
                    ),
                    _ => bail!("Could not parse input: type without struct name & module address"),
                },
                // M.S
                (ModuleAccess_::ModuleAccess(_module, _struct_name), None) => {
                    bail!("Could not parse input: type without module address");
                }
                // M.S + parent address
                (ModuleAccess_::ModuleAccess(name, struct_name), Some(this)) => {
                    TypeTag::Struct(StructTag {
                        address: this,
                        module: Identifier::new(name.0.value)?,
                        name: Identifier::new(struct_name.value)?,
                        type_params: ty_params
                            .into_iter()
                            .map(|ty| unwrap_spanned_ty_(ty, Some(this)))
                            .map(|res| match res {
                                Ok(st) => st,
                                Err(err) => panic!("{:?}", err),
                            })
                            .collect(),
                    })
                }

                // OxADDR.M.S
                (ModuleAccess_::QualifiedModuleAccess(module_id, struct_name), _) => {
                    let ModuleIdent_ { name, address } = module_id.0.value;
                    let address = AccountAddress::new(address.to_u8());
                    TypeTag::Struct(StructTag {
                        address,
                        module: Identifier::new(name.0.value)?,
                        name: Identifier::new(struct_name.value)?,
                        type_params: ty_params
                            .into_iter()
                            .map(|ty| unwrap_spanned_ty_(ty, Some(address)))
                            .map(|res| match res {
                                Ok(st) => st,
                                Err(err) => panic!("{:?}", err),
                            })
                            .collect(),
                    })
                }
            }
        }
        _ => {
            bail!("Could not parse input: unsupported type");
        }
    };

    Ok(st)
}

/// Transaction model.
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    signers_count: u8,
    code: Vec<u8>,
    args: Vec<ScriptArg>,
    type_args: Vec<TypeTag>,
}

impl Transaction {
    pub fn into_script(self, signers: Vec<AccountAddress>) -> Result<ScriptTx> {
        ensure!(
            signers.len() == self.signers_count as usize,
            "Invalid signers count."
        );
        ScriptTx::new(self.code, self.args, self.type_args, signers)
    }

    pub fn signers_count(&self) -> u8 {
        self.signers_count
    }
}

impl TryFrom<&[u8]> for Transaction {
    type Error = Error;

    fn try_from(blob: &[u8]) -> Result<Self, Self::Error> {
        bcs::from_bytes(&blob).map_err(|err| Error::msg(err))
    }
}
