pub mod atrix;
pub mod deposit_tracking;
pub mod orca;
pub mod quarry;
pub mod raydium;
use anchor_lang::solana_program::pubkey::Pubkey;

/// base configuration accounts used across all vault types
pub trait VaultBaseConfig {
    fn vault(&self) -> Pubkey;
    fn vault_pda(&self) -> Pubkey;
    fn withdraw_queue(&self) -> Pubkey;
    fn compound_queue(&self) -> Pubkey;
    fn deposit_queue(&self) -> Pubkey;
    fn shares_mint(&self) -> Pubkey;
}
