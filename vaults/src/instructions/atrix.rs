use sighashdb::GlobalSighashDB;

use super::*;


pub fn new_withdraw_atrix_vault_ix(
    authority: Pubkey,
    vault: Pubkey,
    vault_pda: Pubkey,
    vault_staker_account: Pubkey,
    farm_account: Pubkey,
    farm_stake_token_account: Pubkey,
    crop_account: Pubkey,
    crop_reward_token_account: Pubkey,
    vault_harvester_account: Pubkey,
    vault_reward_token_account: Pubkey,
    underlying_withdraw_queue: Pubkey,
    burning_shares_token_account: Pubkey,
    shares_mint: Pubkey,
    receiving_underlying_token_account: Pubkey,
    atrix_farm_program: Pubkey,
    amount: u64,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("withdraw_atrix_vault")?;
    let mut ix_data = Vec::with_capacity(16);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?);
    Some(Instruction {
        program_id: crate::ID,
        accounts: vec![
        AccountMeta::new(authority, true),
        AccountMeta::new(vault, false),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new(vault_staker_account, false),
        AccountMeta::new_readonly(farm_account, false),
        AccountMeta::new(farm_stake_token_account, false),
        AccountMeta::new(crop_account, false),
        AccountMeta::new(crop_reward_token_account, false),
        AccountMeta::new(vault_harvester_account, false),
        AccountMeta::new(vault_reward_token_account, false),
        AccountMeta::new(underlying_withdraw_queue, false),
        AccountMeta::new(burning_shares_token_account, false),
        AccountMeta::new(shares_mint, false),
        AccountMeta::new(receiving_underlying_token_account, false),
        AccountMeta::new(atrix_farm_program, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(anchor_lang::solana_program::sysvar::clock::id(), false),
        ],
        data: vec![],
    })
}