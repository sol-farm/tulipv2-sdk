use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct ClosePositionInfoAccount<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub position_info_account: AccountInfo<'info>,
}


pub fn close_position_info_account<'info>(
    accounts: ClosePositionInfoAccount<'info>,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("close_position_info_account")?;
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_sighash.to_vec(),
    })
}