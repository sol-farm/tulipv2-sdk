use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct CreateUserFarm<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    pub global: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
}


pub fn create_user_farm<'info>(
    accounts: CreateUserFarm<'info>,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("create_user_farm")?;
    let mut ix_data = Vec::with_capacity(40);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&crate::ID).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}