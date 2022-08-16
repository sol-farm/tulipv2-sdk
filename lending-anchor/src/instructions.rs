use crate::*;
use anchor_lang::prelude::*;

pub fn deposit_reserve_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Reserve<'info>>,
    amount: u64,
) -> Result<()> {
    tulipv2_sdk_lending::helpers::deposit_reserve_liquidity(
        &ctx.program,
        &ctx.accounts.liquidity_token_account,
        &ctx.accounts.collateral_token_account,
        &ctx.accounts.reserve,
        &ctx.accounts.reserve_liquidity,
        &ctx.accounts.reserve_collateral_mint,
        &ctx.accounts.lending_market,
        &ctx.accounts.lending_market_authority,
        &ctx.accounts.user_transfer_authority,
        &ctx.accounts.clock,
        &ctx.accounts.token_program,
        ctx.signer_seeds,
        amount,
    )
    .map_err(Into::into)
}

pub fn redeem_reserve_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, Reserve<'info>>,
    amount: u64,
) -> Result<()> {
    tulipv2_sdk_lending::helpers::redeem_reserve_collateral(
        &ctx.program,
        &ctx.accounts.collateral_token_account,
        &ctx.accounts.liquidity_token_account,
        &ctx.accounts.reserve,
        &ctx.accounts.reserve_collateral_mint,
        &ctx.accounts.reserve_liquidity,
        &ctx.accounts.lending_market,
        &ctx.accounts.lending_market_authority,
        &ctx.accounts.user_transfer_authority,
        &ctx.accounts.clock,
        &ctx.accounts.token_program,
        ctx.signer_seeds,
        amount,
    )
    .map_err(Into::into)
}

pub fn refresh_reserve<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RefreshReserve<'info>>,
) -> Result<()> {
    tulipv2_sdk_lending::helpers::refresh_reserve(
        &ctx.program,
        &ctx.accounts.clock,
        &ctx.accounts.reserve,
        &ctx.accounts.oracle,
    )
    .map_err(Into::into)
}
