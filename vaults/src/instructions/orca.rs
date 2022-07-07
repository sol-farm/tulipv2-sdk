use sighashdb::GlobalSighashDB;

use super::*;

pub fn new_withdraw_orca_vault_stage_one_ix(
    authority: Pubkey,
    vault: Pubkey,
    vault_pda: Pubkey,
    burning_shares_token_account: Pubkey,
    receiving_underlying_token_account: Pubkey,
    vault_farm_token_account: Pubkey,
    vault_reward_token_account: Pubkey,
    vault_swap_token_account: Pubkey,
    global_reward_token_vault: Pubkey,
    pool_token_a: Pubkey,
    pool_token_b: Pubkey,
    global_farm: Pubkey,
    user_farm: Pubkey,
    convert_authority: Pubkey,
    swap_account: Pubkey,
    swap_authority: Pubkey,
    swap_pool_token_mint: Pubkey,
    farm_token_mint: Pubkey,
    shares_mint: Pubkey,
    swap_pool_fee: Pubkey,
    swap_program: Pubkey,
    aquafarm_program: Pubkey,
    ephemeral_tracking_account: Pubkey,
    fee_collector_token_account: Pubkey,
    amount: u64,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("withdraw_orca_vault")?;
    let mut ix_data = Vec::with_capacity(18);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&false).ok()?[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);

    Some(Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(vault, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new(burning_shares_token_account, false),
            AccountMeta::new(receiving_underlying_token_account, false),
            AccountMeta::new(vault_farm_token_account, false),
            AccountMeta::new(vault_reward_token_account, false),
            AccountMeta::new(vault_swap_token_account, false),
            AccountMeta::new(global_reward_token_vault, false),
            AccountMeta::new(pool_token_a, false),
            AccountMeta::new(pool_token_b, false),
            AccountMeta::new(global_farm, false),
            AccountMeta::new(user_farm, false),
            AccountMeta::new_readonly(convert_authority, false),
            AccountMeta::new(swap_account, false),
            AccountMeta::new_readonly(swap_authority, false),
            AccountMeta::new(swap_pool_token_mint, false),
            AccountMeta::new(farm_token_mint, false),
            AccountMeta::new(shares_mint, false),
            AccountMeta::new(swap_pool_fee, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(swap_program, false),
            AccountMeta::new_readonly(aquafarm_program, false),
            AccountMeta::new(ephemeral_tracking_account, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new(fee_collector_token_account, false),
        ],
        data: ix_data,
    })
}

pub fn new_withdraw_orca_vault_dd_stage_one_ix(
    authority: Pubkey,
    vault: Pubkey,
    vault_pda: Pubkey,
    burning_shares_token_account: Pubkey,
    receiving_underlying_token_account: Pubkey,
    vault_farm_token_account: Pubkey,
    vault_reward_token_account: Pubkey,
    vault_swap_token_account: Pubkey,
    global_reward_token_vault: Pubkey,
    pool_token_a: Pubkey,
    pool_token_b: Pubkey,
    global_farm: Pubkey,
    user_farm: Pubkey,
    convert_authority: Pubkey,
    swap_account: Pubkey,
    swap_authority: Pubkey,
    swap_pool_token_mint: Pubkey,
    farm_token_mint: Pubkey,
    shares_mint: Pubkey,
    swap_pool_fee: Pubkey,
    swap_program: Pubkey,
    aquafarm_program: Pubkey,
    ephemeral_tracking_account: Pubkey,
    fee_collector_token_account: Pubkey,
    vault_dd_withdraw_queue: Pubkey,
    amount: u64,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("withdraw_orca_vault_dd_stage_one")?;
    let mut ix_data = Vec::with_capacity(18);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&true).ok()?[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);

    Some(Instruction {
        program_id: crate::ID,
        accounts:  vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(vault, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new(burning_shares_token_account, false),
            AccountMeta::new(receiving_underlying_token_account, false),
            AccountMeta::new(vault_farm_token_account, false),
            AccountMeta::new(vault_reward_token_account, false),
            AccountMeta::new(vault_swap_token_account, false),
            AccountMeta::new(global_reward_token_vault, false),
            AccountMeta::new(pool_token_a, false),
            AccountMeta::new(pool_token_b, false),
            AccountMeta::new(global_farm, false),
            AccountMeta::new(user_farm, false),
            AccountMeta::new_readonly(convert_authority, false),
            AccountMeta::new(swap_account, false),
            AccountMeta::new_readonly(swap_authority, false),
            AccountMeta::new(swap_pool_token_mint, false),
            AccountMeta::new(farm_token_mint, false),
            AccountMeta::new(shares_mint, false),
            AccountMeta::new(swap_pool_fee, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(swap_program, false),
            AccountMeta::new_readonly(aquafarm_program, false),
            AccountMeta::new(ephemeral_tracking_account, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new(fee_collector_token_account, false),
            AccountMeta::new(vault_dd_withdraw_queue, false),
        ],
        data: ix_data,
    })
}

/// for orca double dip vaults, a two-stage or two instruction withdrawal
/// flow must be completed. the second stage reverts the non double dip aquafarm
/// tokens for their underlying lp tokens
pub fn new_withdraw_orca_vault_dd_stage_two_ix(
    authority: Pubkey,
    vault: Pubkey,
    vault_pda: Pubkey,
    burning_shares_token_account: Pubkey,
    receiving_underlying_token_account: Pubkey,
    vault_farm_token_account: Pubkey,
    vault_reward_token_account: Pubkey,
    vault_swap_token_account: Pubkey,
    global_reward_token_vault: Pubkey,
    pool_token_a: Pubkey,
    pool_token_b: Pubkey,
    global_farm: Pubkey,
    user_farm: Pubkey,
    convert_authority: Pubkey,
    swap_account: Pubkey,
    swap_authority: Pubkey,
    swap_pool_token_mint: Pubkey,
    farm_token_mint: Pubkey,
    shares_mint: Pubkey,
    swap_pool_fee: Pubkey,
    swap_program: Pubkey,
    aquafarm_program: Pubkey,
    ephemeral_tracking_account: Pubkey,
    fee_collector_token_account: Pubkey,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("withdraw_orca_vault_dd_stage_two")?;
    let mut ix_data = Vec::with_capacity(18);
    ix_data.extend_from_slice(&ix_sighash[..]);

    Some(Instruction {
        program_id: crate::ID,
        accounts:  vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(vault, false),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new(burning_shares_token_account, false),
            AccountMeta::new(receiving_underlying_token_account, false),
            AccountMeta::new(vault_farm_token_account, false),
            AccountMeta::new(vault_reward_token_account, false),
            AccountMeta::new(vault_swap_token_account, false),
            AccountMeta::new(global_reward_token_vault, false),
            AccountMeta::new(pool_token_a, false),
            AccountMeta::new(pool_token_b, false),
            AccountMeta::new(global_farm, false),
            AccountMeta::new(user_farm, false),
            AccountMeta::new_readonly(convert_authority, false),
            AccountMeta::new(swap_account, false),
            AccountMeta::new_readonly(swap_authority, false),
            AccountMeta::new(swap_pool_token_mint, false),
            AccountMeta::new(farm_token_mint, false),
            AccountMeta::new(shares_mint, false),
            AccountMeta::new(swap_pool_fee, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(swap_program, false),
            AccountMeta::new_readonly(aquafarm_program, false),
            AccountMeta::new(ephemeral_tracking_account, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new(fee_collector_token_account, false),
        ],
        data: ix_data,
    })
}

/// final stage of both double dip and non double dip vault withdrawals
pub fn new_withdraw_orca_vault_remove_liq_ix(
    authority: Pubkey,
    vault: Pubkey,
    vault_pda: Pubkey,
    burning_underlying_token_account: Pubkey,
    funding_token_a_account: Pubkey,
    funding_token_b_account: Pubkey,
    pool_token_a: Pubkey,
    pool_token_b: Pubkey,
    swap_program: Pubkey,
    swap_account: Pubkey,
    swap_authority: Pubkey,
    swap_pool_token_mint: Pubkey,
    swap_fee_account: Pubkey,
    ephemeral_tracking_account: Pubkey,
    shares_mint: Pubkey,
    double_dip: bool,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("withdraw_orca_vault_remove_liq")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&double_dip).ok()?[..]);
    Some(Instruction {
        program_id:  crate::ID,
        accounts: vec![
        AccountMeta::new(authority, true),
        AccountMeta::new(vault, false),
        AccountMeta::new_readonly(vault_pda, false),
        AccountMeta::new(burning_underlying_token_account, false),
        AccountMeta::new(funding_token_a_account, false),
        AccountMeta::new(funding_token_b_account, false),
        AccountMeta::new(pool_token_a, false),
        AccountMeta::new(pool_token_b, false),
        AccountMeta::new_readonly(swap_program, false),
        AccountMeta::new(swap_account, false),
        AccountMeta::new_readonly(swap_authority, false),
        AccountMeta::new(swap_pool_token_mint, false),
        AccountMeta::new(swap_fee_account, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new(ephemeral_tracking_account, false),
        AccountMeta::new_readonly(shares_mint, false),
        ],
        data: ix_data,
    })
}