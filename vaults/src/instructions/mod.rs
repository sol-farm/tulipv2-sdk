//! helper functions for invoking Tulip V2 vaults instructions, largely limited to lending vaults

#![allow(clippy::too_many_arguments)]
use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, system_program, sysvar},
};
use sighashdb::GlobalSighashDB;
use tulipv2_sdk_farms::Farm;
pub mod atrix;
pub mod deposit_tracking;
pub mod multi_deposit_optimizer;
pub mod orca;
pub mod quarry;
pub mod raydium;

/// returns an instruction that takes underlying assets from the caller
/// deposits into a vault in exchange for tokenized vault shares, and
/// locks the issued shares into the deposit tracking account
pub fn new_issue_shares_ix(
    authority: Pubkey,
    vault: Pubkey,
    deposit_tracking: Pubkey,
    deposit_tracking_pda: Pubkey,
    vault_pda: Pubkey,
    vault_underlying_account: Pubkey,
    shares_mint: Pubkey,
    receiving_shares_account: Pubkey,
    depositing_underlying_account: Pubkey,
    farm_type: Farm,
    amount: u64,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("issue_shares")?;
    let farm_type: [u64; 2] = farm_type.into();
    let mut ix_data = Vec::with_capacity(40);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&farm_type).ok()?[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(vault, false),
            AccountMeta::new(deposit_tracking, false),
            AccountMeta::new(deposit_tracking_pda, false),
            AccountMeta::new_readonly(vault_pda, false),
            AccountMeta::new(vault_underlying_account, false),
            AccountMeta::new(shares_mint, false),
            AccountMeta::new(receiving_shares_account, false),
            AccountMeta::new(depositing_underlying_account, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: ix_data,
    })
}
