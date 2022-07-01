use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct OrcaAddLiquidityQueue<'info> {
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
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault_user_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    #[account(mut)]
    pub vault_pda: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    // this seems to be a false positive
    #[account(mut)]
    //#[soteria(ignore)]
    pub lev_farm_coin_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub lev_farm_pc_token_account: AccountInfo<'info>,
    #[account(mut)]
    //#[soteria(ignore)]
    pub pool_coin_token_account: AccountInfo<'info>,
    // for RAY-USDC this would be the pool's
    // USDC token account
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    //#[soteria(ignore)]
    pub liquidity_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_id: AccountInfo<'info>,
    #[account(mut)]
    pub amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub vault_deposit_queue: AccountInfo<'info>,
    #[account(mut)]
    //#[soteria(ignore)]
    pub lp_mint_address: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market_account: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    #[account(mut)]
    pub derived_lending_market_authority: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    pub dex_program: AccountInfo<'info>,
    // protocol-orca
    pub solfarm_vault_program: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
}

pub fn orca_add_liquidity_queue<'info>(
    accounts: OrcaAddLiquidityQueue<'info>,
    account_nonce: u8,
    obligation_index: u8
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("orca_add_liquidity_queue")?;
    let mut ix_data = Vec::with_capacity(10);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&account_nonce).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}