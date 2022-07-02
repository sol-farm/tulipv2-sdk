use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct WithdrawOrcaFarm<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault_user_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    #[account(mut)]
    pub vault_pda: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_owner: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    // this is the address of the vault's pool/lp token account
    pub user_base_token_account: AccountInfo<'info>,
    #[account(mut)]
    // this is the address of the vault's "converted" pool/lp token account
    pub user_farm_token_account: AccountInfo<'info>,
    #[account(mut)]
    // this is the address of the vault's reward token account
    pub user_reward_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub global_base_token_vault: AccountInfo<'info>,
    #[account(mut)]
    pub farm_token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub global_farm: AccountInfo<'info>,
    #[account(mut)]
    pub orca_user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub global_reward_token_vault: AccountInfo<'info>,
    pub convert_authority: AccountInfo<'info>,
    pub aqua_farm_program: AccountInfo<'info>,
    #[account(mut)]
    pub receiving_token_account: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    #[account(mut)]
    pub leveraged_user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    pub solfarm_vault_program: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
}

pub fn withdraw_orca_vault<'info>(
    accounts: WithdrawOrcaFarm<'info>,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("withdraw_orca_vault")?;
    let mut ix_data = Vec::with_capacity(11);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&withdraw_percent).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&close_method).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}

pub fn withdraw_orca_vault_close<'info>(
    accounts: WithdrawOrcaFarm<'info>,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("withdraw_orca_vault_close")?;
    let mut ix_data = Vec::with_capacity(11);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&withdraw_percent).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&close_method).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}

pub fn withdraw_orca_vault_without_shares<'info>(
    accounts: WithdrawOrcaFarm<'info>,
    obligation_index: u8
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("withdraw_orca_vault_without_shares")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data.to_vec(),
    })
}