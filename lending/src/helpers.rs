use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn refresh_reserve<'info>(
    lending_program_id: &AccountInfo<'info>,
    clock: &AccountInfo<'info>,
    reserve: &AccountInfo<'info>,
    oracle: &AccountInfo<'info>,
) -> Result<(), ProgramError> {
    let ix =     super::instruction::refresh_reserve(
        *lending_program_id.key,
        *reserve.key,
        *oracle.key,
    );
    solana_program::program::invoke(
        &ix,
        &[reserve.clone(), oracle.clone(), clock.clone()],
    )?;
    Ok(())
}


pub fn deposit_reserve_liquidity<'info>(
    lending_program_id: &AccountInfo<'info>,
    source_liquidity: &AccountInfo<'info>,
    destination_collateral: &AccountInfo<'info>,
    reserve: &AccountInfo<'info>,
    reserve_liquidity: &AccountInfo<'info>,
    reserve_collateral_mint: &AccountInfo<'info>,
    lending_market: &AccountInfo<'info>,
    lending_market_authority: &AccountInfo<'info>,
    user_transfer_authority: &AccountInfo<'info>,
    clock: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<(), ProgramError> {
    let ix = super::instruction::deposit_reserve_liquidity(
        *lending_program_id.key,
        amount,
        *source_liquidity.key,
        *destination_collateral.key,
        *reserve.key,
        *reserve_liquidity.key,
        *reserve_collateral_mint.key,
        *lending_market.key,
        *user_transfer_authority.key,
    );
   solana_program::program::invoke_signed(
        &ix,
        &[
            source_liquidity.clone(),
            destination_collateral.clone(),
            reserve.clone(),
            reserve_liquidity.clone(),
            reserve_collateral_mint.clone(),
            lending_market.clone(),
            lending_market_authority.clone(),
            user_transfer_authority.clone(),
            clock.clone(),
            token_program.clone(),
        ],
        signer_seeds,
    )?;
    Ok(())
}


pub fn redeem_reserve_collateral<'info>(
    lending_program_id: &AccountInfo<'info>,
    source_collateral: &AccountInfo<'info>,
    destination_liquidity: &AccountInfo<'info>,
    reserve: &AccountInfo<'info>,
    reserve_collateral_mint: &AccountInfo<'info>,
    reserve_liquidity: &AccountInfo<'info>,
    lending_market: &AccountInfo<'info>,
    lending_market_authority: &AccountInfo<'info>,
    user_transfer_authority: &AccountInfo<'info>,
    clock: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<(), ProgramError> {
    let ix = super::instruction::redeem_reserve_collateral(
        *lending_program_id.key,
        amount,
        *source_collateral.key,
        *destination_liquidity.key,
        *reserve.key,
        *reserve_collateral_mint.key,
        *reserve_liquidity.key,
        *lending_market.key,
        *user_transfer_authority.key,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            source_collateral.clone(),
            destination_liquidity.clone(),
            reserve.clone(),
            reserve_liquidity.clone(),
            reserve_collateral_mint.clone(),
            lending_market.clone(),
            lending_market_authority.clone(),
            user_transfer_authority.clone(),
            clock.clone(),
            token_program.clone(),
        ],
        signer_seeds,
    )?;
    Ok(())
}
