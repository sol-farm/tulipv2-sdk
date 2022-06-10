//! traits which are exclusive to the lending optimizer vaults

use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;

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
    /// compatability
    fn to_account_meta(&self, _is_signer: Option<bool>) -> Vec<AccountMeta>;
}
