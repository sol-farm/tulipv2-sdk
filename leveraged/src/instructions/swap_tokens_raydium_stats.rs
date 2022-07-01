use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct RaydiumSwap<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    // ensure global account of leveraged farm matches the given global account
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    // #[account(mut)]
    // pub user_farm_manager: Loader<'info, UserFarmManager>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    // i believe this is the wallet that will receive funds??
    // or is this the wallet that will contain the price coin
    // #[account(mut)]
    // pub pc_wallet: CpiAccount<'info, TokenAccount>,
    // pub market: MarketAccounts<'info>,
    pub token_program: AccountInfo<'info>,
    // pub rent: Sysvar<'info, Rent>,
    // pub dex_program: AccountInfo<'info>,
    pub vault_signer: AccountInfo<'info>,
    pub swap_or_liquidity_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_authority: AccountInfo<'info>,
    // safe to ignore raydium validates
    #[account(mut)]
    //#[soteria(ignore)]
    pub amm_open_orders: AccountInfo<'info>,
    #[account(mut)]
    //#[soteria(ignore)]
    pub amm_quantities_or_target_orders: AccountInfo<'info>,
    #[account(mut)]
    pub pool_coin_tokenaccount: AccountInfo<'info>,
    #[account(mut)]
    pub pool_pc_tokenaccount: AccountInfo<'info>,
    pub serum_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub serum_vault_signer: AccountInfo<'info>,
    #[account(mut)]
    pub coin_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub pc_wallet: AccountInfo<'info>,
}

pub fn swap_tokens_raydium_stats<'info>(
    accounts: RaydiumSwap<'info>,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("swap_tokens_raydium_stats")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}
