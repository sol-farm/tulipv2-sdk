use tulipv2_sdk_common::msg_panic;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use tulipv2_sdk_common::{
    traits::vault::{TokenizedShares, TokenizedSharesHolder},
    DEFAULT_KEY,
};
#[cfg(not(target_arch = "bpf"))]
use derivative::*;

pub const DEPOSIT_TRACKING_ACCOUNT_SIZE: usize = 440;
pub const EPHEMERAL_TRACKING_ACCOUNT_SIZE: usize = 440;
#[cfg(not(target_arch = "bpf"))]
use type_layout::TypeLayout;



#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
/// provides an account type that is used to track user balances
/// and rewards while they are actively deposited into the vault
/// as well as providing a means to securely withdraw from a vault,
/// and enforcing a minimum lockup time
///
/// note when someone takes their deposit our of the tracking account
/// they are no longer able to track their rewards until depositing back into
/// the tracking ccount
///
/// each DepositTracking account has a "hold" account which is used
/// to hold onto tokens while the user isn't actively withdrawing from
/// the tracking account.
pub struct DepositTrackingV1 {
    /// the authority for which this tracking account is owned by
    pub owner: Pubkey,
    /// the vault this account applies to
    pub vault: Pubkey,
    /// nonce of the tracking pda account
    pub pda_nonce: u8,
    /// nonce of the tracking queue account
    pub queue_nonce: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub _alignment: [u8; 6],
    /// amount of shares issued
    pub shares: u64,
    /// amount of underlying tokens that have been deposited
    pub deposited_balance: u64,
    /// the last time someone deposited into the tracking account
    pub last_deposit_time: i64,
    /// the amount of underlying that is set to be withdrawn from the vault withdraw queue
    pub pending_withdraw_amount: u64,
    /// the total amount of underlying asset deposited into this account
    pub total_deposited_underlying: u64,
    /// the total amount of underlying asset withdrawing from this account
    pub total_withdrawn_underlying: u64,
    pub last_pending_reward: u64,
    pub reward_per_share_paid: u128,
    /// an optional account used for storing extra data, should only be used
    /// when there is no buffer left.
    pub extra_data_account: Pubkey,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 256],
}

#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
/// provides an ephemeral tracking account used for temporarily tracking balances
/// in between actions such as orca withdraws. the account is always closed after
/// it is finished being used
pub struct EphemeralTrackingV1 {
    /// the authority for who this tracking account belongs to
    pub authority: Pubkey,
    /// the amount of funds this tracking account has
    /// available for withdraw
    pub available_for_withdraw: u64,
    /// the amount of lp tokens to remove liq for, after having reverted all farm tokens
    pub liq_to_remove: u64,
    pub configured: u8,
    pub can_withdraw: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    /// an arbitrary byte slice of metadata
    pub metadata_one: [u8; 64],
    /// an arbitrary byte slice of metadata
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub metadata_two: [u8; 64],
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 254],
}

impl TokenizedSharesHolder for DepositTrackingV1 {
    fn shares_to_give(&self, vault: &impl TokenizedShares, amount: u64) -> u64 {
        if vault.total_deposited_tokens() == 0 {
            return amount;
        }
        tulipv2_sdk_common::math::calculate_shares_to_give(
            amount,
            vault.total_shares(),
            vault.total_deposited_tokens(),
        )
    }
    fn underlying_to_redeem(&self, vault: &impl TokenizedShares, amount: u64) -> Option<u64> {
        if vault.is_locked(self.last_deposit_time) {
            return None;
        }
        Some(tulipv2_sdk_common::math::calculate_underlying_to_withdraw(
            amount,
            vault.total_shares(),
            vault.total_deposited_tokens(),
        ))
    }
    fn record_deposit(&mut self, amount: u64, shares: u64) {
        msg_panic!("noop");
    }
    /// used to record the effect of withdrawing and burning the shares for their
    /// underlying assets. note you will need to calculate the shares to burn
    /// and the corresponding balance to remove
    fn record_withdraw(&mut self, vault: &impl TokenizedShares, amount: u64) -> u64 {
        msg_panic!("noop");
    }
    fn deposited_balance(&self) -> u64 {
        self.deposited_balance
    }
    fn issued_shares(&self) -> u64 {
        self.shares
    }
    fn total_deposited_underlying(&self) -> u64 {
        self.total_deposited_underlying
    }
    fn total_withdrawn_underlying(&self) -> u64 {
        self.total_withdrawn_underlying
    }
    #[cfg(not(tarpaulin_include))]
    fn withdraw_shares<'a, 'b, 'c, 'info>(
        &mut self,
        deposit_account: &AccountInfo<'info>,
        hold_account: &AccountInfo<'info>,
        pda_account: &AccountInfo<'info>,
        receiving_shares_account: &AccountInfo<'info>,
        token_program: &AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        msg_panic!("noop");
    }
}
