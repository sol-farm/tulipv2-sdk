use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct WithdrawFarm<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    #[account(mut)]
    pub authority_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub vault_program: AccountInfo<'info>,

    #[account(mut)]
    pub user_balance_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_info_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_lp_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_reward_a_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pool_reward_a_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_reward_b_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pool_reward_b_token_account: AccountInfo<'info>,
    pub token_program_id: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    // the account of the vault pda
    #[account(mut)]
    pub vault_pda_account: AccountInfo<'info>,
    #[account(mut)]
    // the token account associated with the lp token that the pool holds
    pub pool_lp_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pool_authority: AccountInfo<'info>,
    #[account(mut)]
    pub pool_id: AccountInfo<'info>,
    pub stake_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub user_balance_meta: AccountInfo<'info>,
}

pub fn withdraw_raydium_vault_close<'info>(
    accounts: WithdrawFarm<'info>,
    lending_market_account: &AccountInfo<'info>,
    user_farm_obligation: &AccountInfo<'info>,
    lending_market_authority: &AccountInfo<'info>,
    lending_program: &AccountInfo<'info>,
    position_info_account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    rent: &AccountInfo<'info>,
    meta_nonce: u8,
    nonce: u8,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("withdraw_raydium_vault_close")?;
    let mut ix_data = Vec::with_capacity(12);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&meta_nonce).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&nonce).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&withdraw_percent).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&close_method).unwrap());

    let mut accounts = accounts.to_account_metas(None);
    accounts.push(AccountMeta::new_readonly(lending_market_account.key(), false));
    accounts.push(AccountMeta::new(user_farm_obligation.key(), false));
    accounts.push(AccountMeta::new_readonly(lending_market_authority.key(), false));
    accounts.push(AccountMeta::new_readonly(lending_program.key(), false));
    accounts.push(AccountMeta::new(position_info_account.key(), false));
    accounts.push(AccountMeta::new_readonly(system_program.key(), false));
    accounts.push(AccountMeta::new_readonly(rent.key(), false));

    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts,
        data: ix_data,
    })
}