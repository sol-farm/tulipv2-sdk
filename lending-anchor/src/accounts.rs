use anchor_lang::prelude::*;

#[derive(Accounts, Clone)]
pub struct Reserve<'info> {
    #[account(mut)]
    pub liquidity_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub collateral_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    #[account(signer)]
    pub user_transfer_authority: AccountInfo<'info>,
    pub clock: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts, Clone)]
pub struct RefreshReserve<'info> {
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    pub oracle: AccountInfo<'info>,
    pub clock: AccountInfo<'info>,
}
