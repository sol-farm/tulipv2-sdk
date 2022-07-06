//! a base vault component providing tokenized shares, as well as pausable deposits / withdraws

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use tulip_arrform::{arrform, ArrForm};
#[cfg(not(target_arch = "bpf"))]
use tulip_derivative::*;
use tulipv2_sdk_common::math::decimal::Decimal;
use tulipv2_sdk_common::math::uint::U192;
use tulipv2_sdk_common::msg_panic;
use tulipv2_sdk_common::{
    math,
    traits::{
        pausable::{Pausable, PausableAction},
        vault::TokenizedShares,
    },
    vaults::tag_to_str,
};
use tulipv2_sdk_farms::Farm;

/// size of the VaultBase struct in bytes
pub const VAULT_BASE_SIZE: usize = 560;
#[cfg(not(feature = "staging"))]
/// hard coded lockup time of 25 minutes
pub const REQUIRED_LOCK_DURATION_SECONDS: i64 = 900;
#[cfg(feature = "localnet")]
/// hard coded lockup time of 14 seconds for localnet
pub const REQUIRED_LOCK_DURATION_SECONDS: i64 = 14;
#[cfg(all(feature = "staging", not(feature = "localnet")))]
/// hard coded lockup time of 60 seconds
pub const REQUIRED_LOCK_DURATION_SECONDS: i64 = 60;
/// seconds per year defined as a constant decimal
pub const SECONDS_PER_YEAR: Decimal = Decimal(U192([14_679_384_701_502_750_720, 1_710_705, 0]));
/// the number `100` defined as a constant decimal
pub const ONE_HUNDRED: Decimal = Decimal(U192([7_766_279_631_452_241_920, 5, 0]));

#[cfg(not(target_arch = "bpf"))]
use type_layout::TypeLayout;

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Default)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
#[repr(C)]
pub struct FeesV1 {
    /// multiplier used to divide fee results
    /// to ensure they are within bounds as
    /// in the case of multi deposit optimizers
    /// this divides the sum of the fees. For example
    /// if fees add up to 300, and the multiplier is 100,
    /// the resulting fee percent would be 3
    pub fee_multiplier: u64,
    pub controller_fee: u64,
    pub platform_fee: u64,
    pub withdraw_fee: u64,
    pub deposit_fee: u64,
    /// the address of the fee wallet for which we derive ATA's
    /// for fee recipient addresses
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub fee_wallet: Pubkey,
    /// the total amount of fees that have been collected
    /// for "rewardA" assets in raydium dual reward vaults, or the non-double dip
    /// rewards for double dip orca vaults (or non double dip vaults)
    ///
    /// in the case of lending optimizers it's the only reward value set
    pub total_collected_a: u64,
    /// the total amount of fees that have been collected for rewardB assets in raydium
    /// dual reward vaults, or the double dip reward for double dip orca vaults.
    ///
    /// in the case of lending optimizers, and non-double dip orca vaults, this value is unused
    pub total_collected_b: u64,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u64; 6],
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Default)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
#[repr(C)]
pub struct TestData {
    pub a: u128,
    pub b: u128,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u64; 4],
}

/// base layer of a vault providing protocol-independent functionality
#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, Default)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
#[repr(C)]
pub struct VaultBaseV1 {
    /// the nonce used to generate the vault account
    pub nonce: u8,
    /// vault metadata tag used when generating the vault address
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub tag: [u8; 32],
    /// this is the address that is responsible
    /// for managing accounts associated with the vault
    pub pda: Pubkey,
    /// the nonce used to generate the pda signer address
    pub pda_nonce: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub _pda_alignment: [u8; 6],
    //pub _pda_alignment: [u8; 7],
    /// the total amount of the underlying assets
    /// backing the shares that have been deposited into the vault
    pub total_deposited_balance: u64,
    /// the total amount of shares issued by the vault
    /// essentially caches the shares min total supply
    pub total_shares: u64,
    /// the token mint account of the underlying asset
    pub underlying_mint: Pubkey,
    /// the account that holds tokens from the underlying mint which are being withdrawn
    pub underlying_withdraw_queue: Pubkey,
    /// the account that holds tokens from the underlying mint
    /// which have been receiving during share issuance
    pub underlying_deposit_queue: Pubkey,
    /// the queue account which holds lp tokens acquired during compounding
    pub underlying_compound_queue: Pubkey,
    /// the token mint account of the vault shares
    pub shares_mint: Pubkey,
    /// if true prevents withdrawing from the vault
    pub withdraws_paused: u8,
    /// if true prevents depositing into the vault
    pub deposits_paused: u8,
    /// if true prevents vault compound operations from taking place
    pub compound_paused: u8,
    /// if true indicates the vault does not support compounding operations
    pub supports_compound: u8,
    pub rebase_paused: u8,
    pub rebalance_paused: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub _state_alignment: [u8; 2],
    /// no longer used
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub _precision_factor: u64,
    /// time at which a compound last took place at
    pub last_compound_time: i64,
    /// the frequency at which compounding operations take place at
    /// compound operations may take place faster than this interval
    /// however this interval is used as a metric to assist initial token
    /// lockups to prevent reward gamification
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub compound_interval: i64,
    /// amount of acceptable slippage when trading
    pub slippage_tolerance: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub _slip_alignment: [u8; 7],
    /// fees charged on various vault interactions
    pub fees: FeesV1,

    /// the farm this vault is farming
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub farm: [u64; 2],

    // indicates if the vault has been configured and ready for usage
    pub configured: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub _configured_alignment: [u8; 7],
    /// pending vault fees not yet claimed by the protocol
    /// currently unused
    pub pending_fees: u64,
    /// if greater than 0, it defines the upper limit of deposited underlying
    /// before further share issuance is disabled.
    pub total_deposited_balance_cap: u64,
    pub test_data: TestData,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    /// buffer with 32 bytes of space
    pub buffer: [u64; 4],
}

pub struct NewVaultArgsV1;

impl VaultBaseV1 {
    /// returns a formatted name of farm_name-tag({})
    /// for LENDING-USDC with a tag of solend it would be
    /// LENDING-USDC-tag(solend)
    pub fn formatted_name(&self) -> String {
        let farm = Farm::from(self.farm);
        arrform!(512, "{}-tag({})", farm.name(), tag_to_str(&self.tag))
            .as_str()
            .to_owned()
    }
}

impl TokenizedShares for VaultBaseV1 {
    fn is_locked(&self, last_deposit_time: i64) -> bool {
        let unix_timestamp = Clock::get().unwrap().unix_timestamp;

        let min_time = last_deposit_time
            .checked_add(REQUIRED_LOCK_DURATION_SECONDS)
            .unwrap();
        // locked if min time is greater or equal to current time
        min_time >= unix_timestamp
    }
    fn compound(&mut self, _balance_to_add: u64) -> bool {
        msg_panic!("noop");
    }
    fn supports_compound(&self) -> bool {
        self.supports_compound == 1
    }
    fn shares_to_give(&self, amount: u64) -> u64 {
        // if the current vault balance is 0 return amount
        if self.total_deposited_balance == 0 {
            return amount;
        }
        math::calculate_shares_to_give(amount, self.total_shares, self.total_deposited_balance)
    }
    fn underlying_to_redeem(&self, amount: u64) -> u64 {
        math::calculate_underlying_to_withdraw(
            amount,
            self.total_shares,
            self.total_deposited_balance,
        )
    }
    fn record_deposit(&mut self, _amount: u64) -> u64 {
        msg_panic!("noop");
    }
    fn record_withdraw(&mut self, _shares_to_burn: u64, _balance_to_remove: u64) {
        msg_panic!("noop");
    }
    fn total_deposited_tokens(&self) -> u64 {
        self.total_deposited_balance
    }
    fn total_shares(&self) -> u64 {
        self.total_shares
    }
    fn shares_mint(&self) -> Pubkey {
        self.shares_mint
    }
    fn underlying_mint(&self) -> Pubkey {
        self.underlying_mint
    }
    #[cfg(not(tarpaulin_include))]
    fn issue_shares<'a, 'b, 'c, 'info>(
        &mut self,
        _mint: &AccountInfo<'info>,
        // token account which will receive the shares
        _receiver: &AccountInfo<'info>,
        _pda: &AccountInfo<'info>,
        _token_program: &AccountInfo<'info>,
        _signer_seeds: &[&[&[u8]]],
        // number of shares to mint
        _amount: u64,
    ) -> Result<()> {
        msg_panic!("noop");
    }
    #[cfg(not(tarpaulin_include))]
    fn burn_shares<'a, 'b, 'c, 'info>(
        &mut self,
        // the shares token account from which we are burning funds
        _shares_account: &AccountInfo<'info>,
        // the mint account of the shares we are burning
        _shares_mint: &mut Box<Account<'info, Mint>>,
        _authority: &AccountInfo<'info>,
        // the underlying token account for which we are transferring
        // the redeemed underlying assets to
        _token_program: &AccountInfo<'info>,
        _signer_seeds: &[&[&[u8]]],
        // amount of shares we are burning
        _shares_to_burn: u64,
        _additional_signers: Option<Vec<AccountInfo<'info>>>,
    ) -> Result<()> {
        msg_panic!("noop");
    }
    #[cfg(not(tarpaulin_include))]
    fn transfer_underlying<'a, 'b, 'c, 'info>(
        &mut self,
        // account from which the underlying tokens will be sent from
        _underlying_account: &AccountInfo<'info>,
        // token account which will receive the underlying asset
        _receiver: &AccountInfo<'info>,
        _pda: &AccountInfo<'info>,
        _token_program: &AccountInfo<'info>,
        _signer_seeds: &[&[&[u8]]],
        // number of shares to mint
        _amount: u64,
    ) -> Result<()> {
        msg_panic!("noop");
    }
    fn sync_shares(&mut self, mint: &spl_token::state::Mint) {
        self.total_shares = mint.supply;
    }
    fn deposits_capped(&self, incoming_deposit_amount: u64) -> bool {
        self.total_deposited_balance_cap > 0
            && self
                .total_deposited_balance
                .checked_add(incoming_deposit_amount)
                .unwrap()
                > self.total_deposited_balance_cap
    }
    fn exchange_rate(&mut self, mint: &spl_token::state::Mint) -> f64 {
        // sync shares first
        self.sync_shares(mint);
        let total_deposited_balance =
            spl_token::amount_to_ui_amount(self.total_deposited_balance, mint.decimals);
        let total_shares = spl_token::amount_to_ui_amount(self.total_shares, mint.decimals);
        total_deposited_balance / total_shares
    }
}

impl Pausable for VaultBaseV1 {
    fn can_do(&self, action: PausableAction) -> bool {
        // if the vault isn't configured, always return false
        if self.configured == 0 {
            return false;
        }
        match action {
            PausableAction::DepositAndWithdrawal => {
                self.deposits_paused == 0 && self.withdraws_paused == 0
            }
            PausableAction::Deposit => self.deposits_paused == 0,
            PausableAction::Withdrawal => self.withdraws_paused == 0,
            PausableAction::Compound => self.compound_paused == 0,
            PausableAction::Rebalance => self.rebalance_paused == 0,
            PausableAction::Rebase => self.rebase_paused == 0,
            PausableAction::All => {
                self.deposits_paused == 0
                    && self.withdraws_paused == 0
                    && self.compound_paused == 0
                    && self.rebase_paused == 0
                    && self.rebalance_paused == 0
            }
            _ => msg_panic!("unsupported mode"),
        }
    }
    /// pauses a vault, has no action if it is already paused
    fn pause(&mut self, action: PausableAction) {
        match action {
            PausableAction::DepositAndWithdrawal => {
                self.deposits_paused = 1;
                self.withdraws_paused = 1;
            }
            PausableAction::Deposit => {
                self.deposits_paused = 1;
            }
            PausableAction::Withdrawal => {
                self.withdraws_paused = 1;
            }
            PausableAction::Compound => {
                self.compound_paused = 1;
            }
            PausableAction::Rebase => {
                self.rebase_paused = 1;
            }
            PausableAction::Rebalance => {
                self.rebalance_paused = 1;
            }
            PausableAction::All => {
                self.deposits_paused = 1;
                self.withdraws_paused = 1;
                self.compound_paused = 1;
                self.rebase_paused = 1;
                self.rebalance_paused = 1;
            }
            _ => msg_panic!("unsupported mode"),
        }
    }
    /// unpauses a vault, has no action if it is already unpaused
    fn unpause(&mut self, action: PausableAction) {
        match action {
            PausableAction::DepositAndWithdrawal => {
                self.deposits_paused = 0;
                self.withdraws_paused = 0;
            }
            PausableAction::Deposit => {
                self.deposits_paused = 0;
            }
            PausableAction::Withdrawal => {
                self.withdraws_paused = 0;
            }
            PausableAction::Compound => {
                self.compound_paused = 0;
            }
            PausableAction::Rebase => {
                self.rebase_paused = 0;
            }
            PausableAction::Rebalance => {
                self.rebalance_paused = 0;
            }
            PausableAction::All => {
                self.deposits_paused = 0;
                self.withdraws_paused = 0;
                self.compound_paused = 0;
                self.rebase_paused = 0;
                self.rebalance_paused = 0;
            }
            _ => msg_panic!("unsupported mode"),
        }
    }
}
