use sighashdb::GlobalSighashDB;

use super::*;

pub fn new_withdraw_raydium_vault_ix(
    authority: Pubkey,
    vault: Pubkey,
    vault_pda: Pubkey,
    vault_stake_info_account: Pubkey,
    pool_id: Pubkey,
    pool_authority: Pubkey,
    underlying_withdraw_queue: Pubkey,
    pool_lp_token_account: Pubkey,
    vault_reward_a_token_account: Pubkey,
    pool_reward_a_token_account: Pubkey,
    vault_reward_b_token_account: Pubkey,
    pool_reward_b_token_account: Pubkey,
    burning_shares_token_account: Pubkey,
    receiving_underlying_token_account: Pubkey,
    shares_mint: Pubkey,
    raydium_stake_program: Pubkey,
    fee_collector_reward_token_a: Pubkey,
    fee_collector_reward_token_b: Option<Pubkey>,
    amount: u64,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("withdraw_raydium_vault")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);
    let mut accounts = vec![
        AccountMeta::new(authority, true),
        AccountMeta::new(vault, false),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new(vault_stake_info_account, false),
        AccountMeta::new(pool_id, false),
        AccountMeta::new(pool_authority, false),
        AccountMeta::new(underlying_withdraw_queue, false),
        AccountMeta::new(pool_lp_token_account, false),
        AccountMeta::new(vault_reward_a_token_account, false),
        AccountMeta::new(pool_reward_a_token_account, false),
        AccountMeta::new(vault_reward_b_token_account, false),
        AccountMeta::new(pool_reward_b_token_account, false),
        AccountMeta::new(burning_shares_token_account, false),
        AccountMeta::new(receiving_underlying_token_account, false),
        AccountMeta::new(shares_mint, false),
        AccountMeta::new_readonly(anchor_lang::solana_program::sysvar::clock::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(raydium_stake_program, false),
        AccountMeta::new(fee_collector_reward_token_a, false),
    ];
    if let Some(fee_collector_reward_token_b) = fee_collector_reward_token_b {
        accounts.push(AccountMeta::new(fee_collector_reward_token_b, false));
    }
    Some(Instruction {
        program_id: crate::ID,
        accounts,
        data: ix_data,
    })
}
