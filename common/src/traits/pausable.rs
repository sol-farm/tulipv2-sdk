//! Pausable provides the ability to define one or more vault actions
//! as "pausable", allowing them to enable enabled/disabled.
//!
//! If you are familiar with Solidity development, you can consider this the
//! a rudimentary implemetnation of a solana version of OpenZeppelin's ownable library.

use std::str::FromStr;

/// defines an action type that is used when
/// interacting with the pausable trait
#[derive(Clone, Copy, Debug)]
pub enum PausableAction {
    Deposit,
    Withdrawal,
    DepositAndWithdrawal,
    Compound,
    Rebase,
    Rebalance,
    All,
    Unknown,
}

/// defines traits required for a vault to be "pausable"
pub trait Pausable {
    /// checks to see if the given action can be performed
    fn can_do(&self, action: PausableAction) -> bool;
    /// pauses a vault, has no action if it is already paused
    fn pause(&mut self, action: PausableAction);
    /// unpauses a vault, has no action if it is already unpaused
    fn unpause(&mut self, action: PausableAction);
}

impl From<u8> for PausableAction {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Deposit,
            1 => Self::Withdrawal,
            2 => Self::DepositAndWithdrawal,
            3 => Self::Compound,
            4 => Self::Rebase,
            5 => Self::Rebalance,
            6 => Self::All,
            _ => Self::Unknown,
        }
    }
}

impl From<PausableAction> for u8 {
    fn from(action: PausableAction) -> Self {
        match action {
            PausableAction::Deposit => 0,
            PausableAction::Withdrawal => 1,
            PausableAction::DepositAndWithdrawal => 2,
            PausableAction::Compound => 3,
            PausableAction::Rebase => 4,
            PausableAction::Rebalance => 5,
            PausableAction::All => 6,
            PausableAction::Unknown => 255,
        }
    }
}

impl FromStr for PausableAction {
    type Err = std::str::Utf8Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "deposit" => Ok(Self::Deposit),
            "withdrawal" => Ok(Self::Withdrawal),
            "depositandwithdrawal" => Ok(Self::DepositAndWithdrawal),
            "compound" => Ok(Self::Compound),
            "rebase" => Ok(Self::Rebase),
            "rebalance" => Ok(Self::Rebalance),
            "all" => Ok(Self::All),
            _ => Ok(Self::Unknown),
        }
    }
}

impl ToString for PausableAction {
    fn to_string(&self) -> String {
        match self {
            PausableAction::Deposit => String::from("Deposit"),
            PausableAction::Withdrawal => String::from("Withdrawal"),
            PausableAction::DepositAndWithdrawal => String::from("DepositAndWithdrawal"),
            PausableAction::Compound => String::from("Compound"),
            PausableAction::Rebase => String::from("Rebase"),
            PausableAction::Rebalance => String::from("Rebalance"),
            PausableAction::All => String::from("All"),
            _ => String::from("Unknown"),
        }
    }
}
