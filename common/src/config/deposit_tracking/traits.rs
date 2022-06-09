



use anchor_lang::prelude::*;
use solana_sdk::instruction::Instruction;
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