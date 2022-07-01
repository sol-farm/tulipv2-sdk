use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    // seems to be false positive
    //#[soteria(ignore)]
    pub user_farm: AccountInfo<'info>,
    // fairly certain false positive
    #[account(mut)]
    //#[soteria(ignore)]
    pub leveraged_farm: AccountInfo<'info>,
    // fairly certain false positive
    //#[soteria(ignore)]
    pub liquidity_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    #[account(mut)]
    //#[soteria(ignore)]
    pub amm_quantities_or_target_orders: AccountInfo<'info>,
    // fairly certain false positive
    #[account(mut)]
    //#[soteria(ignore)]
    pub lp_mint_address: AccountInfo<'info>,
    // for RAY-USDC this would be the pool's
    // RAY token account
    // this seems to be a false positive
    #[account(mut)]
    //#[soteria(ignore)]
    pub pool_coin_token_account: AccountInfo<'info>,
    // for RAY-USDC this would be the pool's
    // USDC token account
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    // seems to be a false positive
    #[account(mut)]
    //#[soteria(ignore)]
    pub serum_market: AccountInfo<'info>,
    // this seems to be a false positive
    //#[soteria(ignore)]
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    // this seems to be a false positive
    //#[soteria(ignore)]
    pub lev_farm_coin_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub lev_farm_pc_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_lp_token_account: AccountInfo<'info>,
    pub pyth_price_account: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    pub derived_lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub dex_program: AccountInfo<'info>,
}

pub fn add_liquidity_stats<'info>(
    accounts: AddLiquidity<'info>,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("add_liquidity_stats")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}