use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;
use tulipv2_sdk_farms::Farm;

/// The `IssueShares` trait is used to deposit into all v2 vaults regardless of their type,
/// and takes assets from the user moving them to a vault's withdraw queue where they are swept
/// into whatever protocol the vault is farming. In exchange for these assets the user receives
/// tokenized vault shares which are locked into a deposit tracking account
pub trait IssueShares {
    fn authority(&self) -> Pubkey;
    fn vault(&self) -> Pubkey;
    fn deposit_tracking_account(&self) -> Pubkey;
    fn deposit_tracking_pda(&self) -> Pubkey;
    fn vault_pda(&self) -> Pubkey;
    fn shares_mint(&self) -> Pubkey;
    /// the "hold account", or rather the ATA of the deposit tracking pda
    /// which holds onto issued shares before they are withdrawn or unlocked
    fn receiving_shares_account(&self) -> Pubkey;
    /// the token account from the `authority` which is depositing the underlying asset
    fn depositing_underlying_account(&self) -> Pubkey;
    fn vault_underlying_account(&self) -> Pubkey;
    /// returns the Instruction object which can be used to invoke
    /// the issue_shares instruction via CPI, or off-chain clients
    ///
    /// `farm_type` is the farm key used by a particular vault, while `amount`
    /// is the amount of underlying asset the caller wants to deposit
    fn instruction(&self, farm_type: Farm, amount: u64) -> Option<Instruction>;
    fn ix_data(&self) -> Option<[u8; 8]>;
    /// please note the _is_signer variable is ignored, it's simply here to provide
    /// compatability
    fn to_account_meta(&self, _is_signer: Option<bool>) -> Vec<AccountMeta>;
}

/// The `RegisterDepositTracking` trait is used to initialize and create the
/// deposit tracking account which is used by all v2 vaults to hold onto funds
/// both for the temporary lockup and reward tracking.
pub trait RegisterDepositTracking {
    fn authority(&self) -> Pubkey;
    fn vault(&self) -> Pubkey;
    fn deposit_tracking_account(&self) -> Pubkey;
    fn deposit_tracking_pda(&self) -> Pubkey;
    /// the deposit tracking queue account is a token account
    /// used to temporarily hold onto funds during withdraw
    /// processes that take more than one instruction
    fn deposit_tracking_queue_account(&self) -> Pubkey;
    /// the deposit tracking hold account is an ATA account
    /// for the underlying token mint that is used to hold
    /// onto issued shares before they are removed from the
    /// deposit tracking account
    ///
    /// **YOU MUST CREATE THIS ASSOCIATED TOKEN ACCOUNT YOURSELF**
    /// **THE V2 VAULTS PROGRAM DOES NOT INVOKE THE ATA PROGRAM**
    /// **PLEASE INCLUDE THE INSTRUCTINO TO CREATE THE ATA BEFORE THE REGISTER INSTRUCTION**
    fn deposit_tracking_hold_account(&self) -> Pubkey;
    fn shares_mint(&self) -> Pubkey;
    /// the token mint for the underlying asset backing the tokenized shares
    fn underlying_mint(&self) -> Pubkey;
    /// returns the Instruction object which can be used to invoke
    /// the `register_deposit_tracking` instruction via CPI or off-chain cliemts
    ///
    /// `farm_type` is the farm key used by a particular vault
    fn instruction(&self, farm_type: Farm) -> Option<Instruction>;
    fn ix_data(&self) -> Option<[u8; 8]>;
    /// please note the _is_signer variable is ignored, it's simply here to provide
    /// compatability
    fn to_account_meta(&self, _is_signer: Option<bool>) -> Vec<AccountMeta>;
}

/// The `WithdrawDepositTracking` trait is used to withdraw tokenized vault shares
/// which are held by the deposit tracking account, allowing the shares to be held
/// directly by a users wallet, or used with any supported protocols.
///
/// Note that once the shares are removed from the deposit tracking account, the
/// deposit tracking account will no longer be tracking rewards from those share tokens.
/// The share tokens still earn yield just the same as if they were locked in the deposit
/// tracking account, however the only difference is that Tulip's UI will no longer
/// be displaying real-time reward gains.
pub trait WithdrawDepositTracking {
    fn authority(&self) -> Pubkey;
    fn vault(&self) -> Pubkey;
    fn deposit_tracking_account(&self) -> Pubkey;
    fn deposit_tracking_pda(&self) -> Pubkey;
    fn deposit_tracking_hold_account(&self) -> Pubkey;
    fn shares_mint(&self) -> Pubkey;
    /// the address which should receive the tokenized shares being removed from the vault
    fn receiving_shares_account(&self) -> Pubkey;
    /// returns the Instruction object which can be used to invoke
    /// the `withdraw_deposit_tracking` instruction via CPI or off-chain cliemts
    ///
    /// `amount` is the amount of tokenized shares the caller wishes to withdraw
    /// `farm_type` is the farm key used by a particular vault
    fn instruction(&self, amount: u64, farm_type: Farm) -> Option<Instruction>;
    fn ix_data(&self) -> Option<[u8; 8]>;
    /// please note the _is_signer variable is ignored, it's simply here to provide
    /// compatability
    fn to_account_meta(&self, _is_signer: Option<bool>) -> Vec<AccountMeta>;
}
