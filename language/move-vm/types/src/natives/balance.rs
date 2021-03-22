use core::fmt;
use core::fmt::{Display, Formatter};

use hashbrown::HashMap;

use move_core_types::account_address::AccountAddress;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WalletId {
    pub address: AccountAddress,
    pub module: String,
    pub name: String,
}

impl WalletId {
    pub fn new(address: AccountAddress, module: String, name: String) -> WalletId {
        WalletId {
            address,
            module,
            name,
        }
    }
}

impl Display for WalletId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}::{}", self.address, self.module, self.name)
    }
}

pub type Balance = u128;

pub trait NativeBalance {
    fn get_balance(&self, address: &WalletId) -> Option<Balance>;
}

#[derive(Debug)]
pub enum BalanceOperation {
    Deposit(Balance),
    Withdraw(Balance),
}

impl BalanceOperation {
    pub fn empty() -> BalanceOperation {
        BalanceOperation::Deposit(0)
    }

    pub fn merge(&mut self, op: BalanceOperation) {
        let op = match (&self, op) {
            (BalanceOperation::Deposit(current), BalanceOperation::Deposit(change)) => {
                BalanceOperation::Deposit(*current + change)
            }
            (BalanceOperation::Withdraw(current), BalanceOperation::Withdraw(change)) => {
                BalanceOperation::Withdraw(*current + change)
            }
            (BalanceOperation::Deposit(current), BalanceOperation::Withdraw(change)) => {
                if *current >= change {
                    BalanceOperation::Deposit(*current - change)
                } else {
                    BalanceOperation::Withdraw(change - *current)
                }
            }
            (BalanceOperation::Withdraw(current), BalanceOperation::Deposit(change)) => {
                if *current >= change {
                    BalanceOperation::Withdraw(*current - change)
                } else {
                    BalanceOperation::Deposit(change - *current)
                }
            }
        };

        *self = op;
    }
}

pub struct MasterOfCoin<B: NativeBalance> {
    native_balances: B,
    bank: HashMap<WalletId, BalanceOperation>,
}

impl<B> MasterOfCoin<B>
where
    B: NativeBalance,
{
    pub fn new(native_balances: B) -> MasterOfCoin<B> {
        MasterOfCoin {
            native_balances,
            bank: Default::default(),
        }
    }

    pub fn get_balance(&self, wallet_id: &WalletId) -> Option<Balance> {
        self.native_balances
            .get_balance(wallet_id)
            .map(|mut balance| {
                if let Some(op) = self.bank.get(wallet_id) {
                    match op {
                        BalanceOperation::Deposit(val) => {
                            balance -= *val;
                        }
                        BalanceOperation::Withdraw(val) => {
                            balance += *val;
                        }
                    }
                }
                balance
            })
            .or_else(|| {
                self.bank.get(wallet_id).and_then(|op| {
                    if let BalanceOperation::Withdraw(val) = op {
                        Some(*val)
                    } else {
                        None
                    }
                })
            })
    }

    pub fn save_balance_operation(&mut self, wallet_id: WalletId, op: BalanceOperation) {
        let entry = self.bank.entry(wallet_id);
        let current_op = entry.or_insert_with(BalanceOperation::empty);
        current_op.merge(op);
    }
}

impl<B: NativeBalance> From<MasterOfCoin<B>> for HashMap<WalletId, BalanceOperation> {
    fn from(moc: MasterOfCoin<B>) -> Self {
        moc.bank
    }
}
