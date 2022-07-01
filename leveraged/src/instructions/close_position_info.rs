use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClosePositionInfoAccount<'info> {
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub position_info_account: AccountInfo<'info>,
}
