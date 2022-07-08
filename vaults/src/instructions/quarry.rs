use sighashdb::GlobalSighashDB;

use super::*;

pub fn new_withdraw_quarry_vault_ix(
    authority: Pubkey,
    vault: Pubkey,
    vault_pda: Pubkey,
    rewarder: Pubkey,
    quarry: Pubkey,
    miner: Pubkey,
    miner_vault: Pubkey,
    shares_mint: Pubkey,
    burning_shares_token_account: Pubkey,
    vault_withdraw_queue: Pubkey,
    receiving_underlying_token_account: Pubkey,
    fee_destination: Pubkey,
    mine_program: Pubkey,
    amount: u64,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("withdraw_quarry_vault")?;
    let mut ix_data = Vec::with_capacity(16);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);
    Some(Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(vault, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(rewarder, false),
            AccountMeta::new(quarry, false),
            AccountMeta::new(miner, false),
            AccountMeta::new(miner_vault, false),
            AccountMeta::new(shares_mint, false),
            AccountMeta::new(burning_shares_token_account, false),
            AccountMeta::new(vault_withdraw_queue, false),
            AccountMeta::new(receiving_underlying_token_account, false),
            AccountMeta::new(fee_destination, false),
            AccountMeta::new_readonly(mine_program, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(anchor_lang::solana_program::sysvar::clock::id(), false),
        ],
        data: ix_data,
    })
}

pub fn new_withdraw_sunny_vault_ix(
    authority: Pubkey,
    vault: Pubkey,
    vault_pda: Pubkey,
    config_data: Pubkey,
    sunny_internal_mint: Pubkey,
    sunny_tvault_vendor_token_account: Pubkey,
    sunny_tvault_internal_token_account: Pubkey,
    sunny_pool: Pubkey,
    sunny_tvault: Pubkey,
    sunny_quarry: Pubkey,
    sunny_miner: Pubkey,
    sunny_miner_vault: Pubkey,
    sunny_rewarder: Pubkey,
    rewarder: Pubkey,
    quarry: Pubkey,
    miner: Pubkey,
    miner_vault: Pubkey,
    shares_mint: Pubkey,
    burning_shares_token_account: Pubkey,
    vault_withdraw_queue: Pubkey,
    receiving_underlying_token_account: Pubkey,
    fee_destination: Pubkey,
    mine_program: Pubkey,
    sunny_quarry_program: Pubkey,
    amount: u64
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("withdraw_sunny_vault");
    let mut ix_data = Vec::with_capacity(16);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);
    Some(Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(vault, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new(config_data, false),
            AccountMeta::new(sunny_internal_mint, false),
            AccountMeta::new(sunny_tvault_vendor_token_account, false),
            AccountMeta::new(sunny_tvault_internal_token_account, false),
            AccountMeta::new(sunny_pool, false),
            AccountMeta::new(sunny_tvault, false),
            AccountMeta::new(sunny_quarry, false),
            AccountMeta::new(sunny_miner, false),
            AccountMeta::new(sunny_miner_vault, false),
            AccountMeta::new_readonly(sunny_rewarder, false),
            AccountMeta::new_readonly(rewarder, false),
            AccountMeta::new(quarry, false),
            AccountMeta::new(miner, false),
            AccountMeta::new(miner_vault, false),
            AccountMeta::new(shares_mint, false),
            AccountMeta::new(burning_shares_token_account, false),
            AccountMeta::new(vault_withdraw_queue, false),
            AccountMeta::new(receiving_underlying_token_account, false),
            AccountMeta::new(fee_destination, false),
            AccountMeta::new_readonly(mine_program, false),
            AccountMeta::new_readonly(sunny_quarry_program, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(anchor_lang::solana_program::sysvar::clock::id(), false),
        ],
        data: ix_data,
    })
}