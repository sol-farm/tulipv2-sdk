use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct WithdrawOrcaVaultDoubleDip<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault_user_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
    #[account(mut)]
    pub vault_pda: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    // #[account(mut)]
    // pub user_farm_owner: AccountInfo<'info>,
    // pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    // this is the address of the vault's "converted" pool/lp token account
    pub user_farm_token_account: AccountInfo<'info>,
    // this is the address of vault's "double converted" pool/lp token account
    #[account(mut)]
    pub user_farm_dd_token_account: AccountInfo<'info>,
    #[account(mut)]
    // this is the address of the vault's reward token account
    pub user_reward_dd_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub global_base_dd_token_vault: AccountInfo<'info>,
    #[account(mut)]
    pub farm_dd_token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub global_farm_dd: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_dd: AccountInfo<'info>,
    #[account(mut)]
    pub global_reward_dd_token_vault: AccountInfo<'info>,
    pub convert_authority_dd: AccountInfo<'info>,
    pub aqua_farm_program: AccountInfo<'info>,
    #[account(mut)]
    pub leveraged_user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    pub solfarm_vault_program: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
}

pub fn withdraw_orca_vault_dd_close<'info>(
    accounts: WithdrawOrcaVaultDoubleDip<'info>,
    lending_market_account: &AccountInfo<'info>,
    user_farm_obligation: &AccountInfo<'info>,
    lending_market_authority: &AccountInfo<'info>,
    lending_program: &AccountInfo<'info>,
    position_info_account: &AccountInfo<'info>,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("withdraw_orca_vault_dd_close")?;
    let mut ix_data = Vec::with_capacity(11);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&withdraw_percent).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&close_method).unwrap());

    let mut accounts = accounts.to_account_metas(None);
    accounts.push(AccountMeta::new_readonly(
        lending_market_account.key(),
        false,
    ));
    accounts.push(AccountMeta::new(user_farm_obligation.key(), false));
    accounts.push(AccountMeta::new_readonly(
        lending_market_authority.key(),
        false,
    ));
    accounts.push(AccountMeta::new_readonly(lending_program.key(), false));
    accounts.push(AccountMeta::new(position_info_account.key(), false));

    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts,
        data: ix_data,
    })
}
