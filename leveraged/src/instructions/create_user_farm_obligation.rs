use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct CreateUserFarmObligation<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub lending_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

pub fn create_user_farm_obligation<'info>(
    accounts: CreateUserFarmObligation<'info>,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("create_user_farm_obligation")?;
    Some(Instruction {
        data: ix_sighash.to_vec(),
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None)
    })
}