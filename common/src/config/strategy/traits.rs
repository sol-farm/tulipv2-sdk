//! traits which are exclusive to the lending optimizer vaults

use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;

use crate::config::deposit_tracking::traits::{IssueShares, RegisterDepositTracking};
use crate::config::deposit_tracking::traits::WithdrawDepositTracking;
use super::Platform;

/// The `WithdrawMultiOptimizerVault` trait is used to
/// burn a lending optimizer's tokenized shares, in exchange
/// for the underlying asset backing the burned amount.
///
/// To accomplish this, the caller provides the configuration
/// information for one of the protocols the lending optimizer
/// is deposited into, allowing the user to withdraw up to
/// the amount deposited by the vault in that protocol, providing
/// the caller has the appropriate amount of share tokens to burn.
pub trait WithdrawMultiOptimizerVault {
    fn authority(&self) -> Pubkey;
    /// the main vault that users interact with, which is called a MultiDepositOptimizerVaultV1
    fn multi_deposit_vault(&self) -> Pubkey;
    fn multi_deposit_vault_pda(&self) -> Pubkey;
    /// to enable maximal defi legos, multiple "standalone vaults" may exist, where
    /// a single standalone vault is deposits one asset (ie USDC) into one protocol (ie Tulip),
    /// this allows the standalone vaults to be reused by any of tulip's v2 vaults for
    /// maximal composability
    fn withdraw_vault(&self) -> Pubkey;
    fn withdraw_vault_pda(&self) -> Pubkey;
    /// an account which stores information related to the platform/protocol
    /// a standalone vault is deposited into
    fn platform_information(&self) -> Pubkey;
    /// an account which stores configuration information related to the standalone
    /// vault for the platform/protocol being farmed
    fn platform_config_data(&self) -> Pubkey;
    fn lending_program(&self) -> Pubkey;
    /// the token account owned by the caller/authority which holds onto the tokenized shares
    /// issued by the multi deposit vault. these would be the tokens that you remove
    /// from a deposit tracking account with the `withdraw_deposit_tracking` instruction
    fn multi_burning_shares_token_account(&self) -> Pubkey;
    /// just like other v2 vaults, the standalone vaults issue shares, however
    /// these shares are held, and managed by the multi deposit vault
    fn withdraw_burning_shares_token_account(&self) -> Pubkey;
    /// the token account which will receive the underlying assets backing
    /// the shares once the shares have been burned
    fn receiving_underlying_token_account(&self) -> Pubkey;
    /// a token account that holds onto assets in between various steps
    /// of the withdraw flow
    fn multi_underlying_withdraw_queue(&self) -> Pubkey;
    /// the mint of the tokenized shares issued by the multi deposit vault
    fn multi_shares_mint(&self) -> Pubkey;
    /// the mint of the tokenized shares issued by the standalone vault
    /// which we are withdrawing from
    fn withdraw_shares_mint(&self) -> Pubkey;
    /// the deposit queue account used by the standalone vault to
    /// hold funds while they are being deposited. even though this is
    /// a withdraw instruction, the deposit queue account is needed
    /// so that when the vault rebases before the withdraw happens,
    /// all assets are accounted for
    fn withdraw_vault_underlying_deposit_queue(&self) -> Pubkey;
    /// returns accounts specific to the standalone vault being used
    ///
    /// when the caller is withdrawing from a Tulip standalone vault
    /// the following accounts are used
    ///
    ///         0 [writable]        -> source_collateral_token_account
    ///
    ///         1 [writable]        -> reserve_account
    ///
    ///         2 [writable]        -> reserve_liquidity_supply
    ///
    ///         3 [writable]        -> reserve_collateral_mint
    ///
    ///         4 []                -> lending_market_account
    ///    
    ///         5 []                -> derived_lending_market_authority
    ///
    ///         6 []                -> reserve_oracle
    ///
    /// when the caller is withdrawing from a Solend standalone vault
    /// the following accounts are used:
    ///
    ///         0 [writable]       -> source_collateral_token_account
    ///
    ///         1 [writable]        -> reserve_account
    ///
    ///         2 [writable]        -> reserve_liquidity_supply
    ///
    ///         3 [writable]        -> reserve_collateral_mint
    ///
    ///         4 []                -> lending_market_account
    ///    
    ///         5 []                -> derived_lending_market_authority
    ///
    ///         6 []                -> reserve_pyth_price_account
    ///
    ///         7 []                -> reserve_switchboard_price_account
    ///
    /// when the caller is withdrawing from a Mango standalone vaults
    /// the following accounts are used:
    ///
    ///         0 []                -> mango_group_account
    ///
    ///         1 [writable]        -> optimizer_mango_account
    /// .
    ///         2 [writable]        -> mango_cache
    ///
    ///         3 [writable]        -> root_bank
    ///
    ///         4 [writable]        -> node_bank
    ///
    ///         5 [writable]        -> mango_token_account
    ///
    ///         6 []                -> mango_group_signer
    ///
    ///         7 []                -> system_program
    ///
    fn standalone_vault_accounts(&self) -> Option<Vec<AccountMeta>>;
    /// returns the Instruction object which can be used to invoke
    /// the withdraw_multi_optimizer_vault instruction via CPI, or off-chain clients
    ///
    /// `farm_type` is the farm key used by a particular vault, while `amount`
    /// is the amount of underlying asset the caller wants to deposit
    fn instruction(&self, amount: u64) -> Option<Instruction>;
    fn ix_data(&self) -> Option<[u8; 8]>;
    /// please note the _is_signer variable is ignored, it's simply here to provide
    /// compatibility
    fn to_account_meta(&self, _is_signer: Option<bool>) -> Vec<AccountMeta>;
}

/// trait type that is used to return configuration information, instruction helpers, etc...
/// for any given multi deposit optimizer vault, aka strategy vault. 
pub trait MultiVaultProgramConfig {
    fn account(&self) -> Pubkey;
    fn pda(&self) -> Pubkey;
    fn shares_mint(&self) -> Pubkey;
    fn underlying_compound_queue(&self) -> Pubkey;
    fn underlying_deposit_queue(&self) -> Pubkey;
    fn underlying_withdraw_queue(&self) -> Pubkey;
    fn underlying_mint(&self) -> Pubkey;
    fn rebalance_state_transition(&self) -> Pubkey;
    fn rebalance_state_transition_underlying(&self) -> Pubkey;
    fn optimizer_shares_account(&self, platform: Platform) -> Pubkey;
    fn issue_shares(&self, user: Pubkey) -> Box<dyn IssueShares>;
    fn permissioned_issue_shares(&self, user: Pubkey) -> Box<dyn IssueShares>;
    fn register_deposit_tracking(&self, user: Pubkey) -> Box<dyn RegisterDepositTracking>;
    fn withdraw_deposit_tracking(&self, user: Pubkey) -> Box<dyn WithdrawDepositTracking>;
    fn withdraw_multi_deposit_optimizer_vault(&self, user: Pubkey, platform: Platform) -> std::result::Result<Box<dyn WithdrawMultiOptimizerVault>, std::io::Error>;
    /// returns the remaining accounts needed for withdrawal instructions to the specific platform
    fn remaining_accounts(&self, platform: Platform) -> Vec<Pubkey>;
}


/// Trait type that is used to return configuration information, instruction helpers, etc...
/// for a single standalone vault.
pub trait StandaloneVaultProgramConfig {
    /// returns the address of the standalone vault account
    fn account() -> Pubkey;
    /// returns the address of the standalone vault pda
    fn pda() -> Pubkey;
    /// returns the address of the standalone vault shares mint
    fn shares_mint() -> Pubkey;
    /// returns the address of the standalone vault underlying compound queue
    fn underlying_compound_queue() -> Pubkey;
    /// returns the address of the standalone vault underlying deposit queue
    fn underlying_deposit_queue() -> Pubkey;
    /// returns the address of the standalone vault underlying withdraw queue
    fn underlying_withdraw_queue() -> Pubkey;
    /// returns the address of the standalone vault underlying token mint
    /// which is the mint of the token the vault accepts for deposits
    fn underlying_mint() -> Pubkey;
    /// returns the address of configuration data account
    fn config_data_account() -> Pubkey;
    /// returns the address of the configuration information account
    fn information_account() -> Pubkey;
    /// returns the address of the program this standalone vault farms. for example
    /// solend standalone vaults will return the address of the solend lending program
    /// while mango standalone vaults will return the address of the mango program
    fn program_id() -> Pubkey;
    /// when the implementation of this trait is a solend standalone vault
    /// calling this method returns Some(...)
    fn solend_config() -> Option<Box<dyn SolendProgramConfig>>;
    /// when the implementation of this trait is a tulip standalone vault
    /// calling this method returns Some(...)
    fn tulip_config() -> Option<Box<dyn TulipProgramConfig>>;
    /// when the implementation of this trait is a mango standalone vault
    /// calling this method returns Some(...)
    fn mango_config() -> Option<Box<dyn MangoProgramConfig>>;
    /// returns true if the instance of the implementation of this trait is a platform
    /// matching the one specified in `platform`, otherwise returns false
    fn is_platform(platform: Platform) -> bool;
}

/// Trait type that is used to return configuration information, instruction helpers, etc..
/// for solend standalone vaults
pub trait SolendProgramConfig {
    /// returns the address of the collateral mint issued by the lending reserve
    fn collateral_mint(&self) -> Pubkey;
    /// returns the lending market that the reserve is a part of
    fn lending_market(&self) -> Pubkey;
    /// returns the authority of the lending market
    fn lending_market_authority(&self) -> Pubkey;
    /// returns the pyth price feed account 
    fn pyth_price_account(&self) -> Pubkey;
    /// returns the switchboard price account
    fn switchboard_price_account(&self) -> Pubkey;
    /// returns the address of the pyth oracle program
    fn pyth_program_id(&self) -> Pubkey;
    /// returns the address of the switchboard oracle program
    fn switchboard_program_id(&self) -> Pubkey;
    /// returns the address of the lending reserve deposits go into
    fn reserve(&self) -> Pubkey;
    /// returns the token account the reserve uses to hold deposited liquidity
    fn reserve_liquidity(&self) -> Pubkey;
    /// the solend standalone vault's collateral token account
    fn vault_collateral_account(&self) -> Pubkey;
}

/// Trait type that is used to return configuration information, instruction helpers, etc..
/// for solend tulip vaults
pub trait TulipProgramConfig {
    /// returns the address of the collateral mint issued by the lending reserve
    fn collateral_mint(&self) -> Pubkey;
    /// returns the lending market that the reserve is a part of
    fn lending_market(&self) -> Pubkey;
    /// returns the authority of the lending market
    fn lending_market_authority(&self) -> Pubkey;
    /// returns the pyth price feed account 
    fn pyth_price_account(&self) -> Pubkey;
    /// returns the address of the pyth oracle program
    fn pyth_program_id(&self) -> Pubkey;
    /// returns the address of the lending reserve deposits go into
    fn reserve(&self) -> Pubkey;
    /// returns the token account the reserve uses to hold deposited liquidity
    fn reserve_liquidity(&self) -> Pubkey;
    /// the tulip standalone vault's collateral token account
    fn vault_collateral_account(&self) -> Pubkey;
}

/// Trait type that is used to return configuration information, instruction helpers, etc..
/// for solend mango vaults
pub trait MangoProgramConfig {
    /// returns the address of the mango cache
    fn cache(&self) -> Pubkey;
    /// returns the address of the mango group
    fn group(&self) -> Pubkey;
    /// returns the address of the group authority
    fn group_signer(&self) -> Pubkey;
    /// returns the address of the group's token account which accepts the asset
    /// being accepted by the strategy. for example usdc strategy vaults, this will be the mango
    /// group's usdc token account
    fn group_token_account(&self) -> Pubkey;
    /// returns the address of the mango root bank
    fn root_bank(&self) -> Pubkey;
    /// returns the address of the mango node bank
    fn node_bank(&self) -> Pubkey;
    /// address of the standalone vaults mango account
    fn optimizer_mango_account(&self) -> Pubkey;
}