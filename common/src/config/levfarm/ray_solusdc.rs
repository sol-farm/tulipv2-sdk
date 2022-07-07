use anchor_lang::solana_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;

use super::{LevFarmConfig, GLOBAL, LENDING_PROGRAM, RAYDIUM_VAULT_PROGRAM};

/// configuration variables for the leverage farm program
pub mod farm_config {
    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("");
    pub const VAULT_ACCOUNT: Pubkey = static_pubkey!("");
    pub const BASE_TOKEN_ACCOUNT: Pubkey = static_pubkey!("");
    pub const QUOTE_TOKEN_ACCOUNT: Pubkey = static_pubkey!("");
}

pub mod vault_config {
    use crate::DEFAULT_KEY;

    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("");
    pub const PDA: Pubkey = static_pubkey!("");
    pub const REWARD_A_TOKEN_ACCOUNT: Pubkey = static_pubkey!("");
    pub const POOL_REWARD_A_TOKEN_ACCOUNT: Pubkey = static_pubkey!("");
    pub const POOL_REWARD_B_TOKEN_ACCOUNT: Pubkey = static_pubkey!("");
    pub const REWARD_B_TOKEN_ACCOUNT: Pubkey = static_pubkey!("");
    pub const POOL_AUTHORITY: Pubkey = static_pubkey!("");
    pub const LP_TOKEN_ACCOUNT: Pubkey = static_pubkey!("");
    // set to this if not used
    // pub const OLD_VAULT_INFO: Pubkey = DEFAULT_KEY;
    pub const OLD_VAULT_INFO_ACCOUNT: Pubkey = static_pubkey!("");
    pub const VAULT_INFO_ACCOUNT: Pubkey = static_pubkey!("");
}


pub mod market_config {
    use super::*;
    pub const SERUM_MARKET: Pubkey = static_pubkey!("");
    pub const SERUM_REQUEST_QUEUE: Pubkey = static_pubkey!("");
    pub const SERUM_EVENT_QUEUE: Pubkey = static_pubkey!("");
    pub const SERUM_BIDS: Pubkey = static_pubkey!("");
    pub const SERUM_ASKS: Pubkey = static_pubkey!("");
    pub const SERUM_COIN_VAULT: Pubkey = static_pubkey!("");
    pub const SERUM_PC_VAULT: Pubkey = static_pubkey!("");
    pub const SERUM_FEE_RECEIVER: Pubkey = static_pubkey!("");
    pub const SERUM_OPEN_ORDERS: Pubkey = static_pubkey!("");
    pub const SERUM_VAULT_SIGNER: Pubkey = static_pubkey!("");
    pub const LP_MINT: Pubkey = static_pubkey!("");
    pub const COIN_TOKEN_MINT: Pubkey = static_pubkey!("");
    pub const PC_TOKEN_MINT: Pubkey = static_pubkey!("");
    pub const AMM_ID: Pubkey = static_pubkey!("");
    pub const AMM_OPEN_ORDERS: Pubkey = static_pubkey!("");
    pub const AMM_QUANTITIES_OR_TARGET_ORDERS: Pubkey = static_pubkey!("");
    pub const AMM_COIN_ACCOUNT: Pubkey = static_pubkey!("");
    pub const AMM_PC_ACCOUNT: Pubkey = static_pubkey!("");
    pub const AMM_TEMP_ACCOUNT: Pubkey = static_pubkey!("");
    pub const AMM_WITHDRAW_QUEUE: Pubkey = static_pubkey!("");
    pub const AMM_AUTHORITY: Pubkey = static_pubkey!("");
}

pub mod reserve_config {
    use super::*;
    pub const LENDING_MARKET: Pubkey = static_pubkey!("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym");
    pub const LENDING_MARKET_AUTHORITY: Pubkey = static_pubkey!("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s");
    pub const PYTH_LP_PRICE_ACCOUNT: Pubkey = static_pubkey!("jnNMpLbMMLDCYJmxJa2RuArEsLrtLKN1c1yjMuvjFsE");
    pub const PYTH_COIN_PRICE_ACCOUNT: Pubkey = static_pubkey!("DQAcms41gjYzidRooXRE9GQM1jAauPXDcEpMbVh4FEc7");
    pub const PYTH_PC_PRICE_ACCOUNT: Pubkey = static_pubkey!("ExzpbWgczTgd8J58BrnESndmzBkRVfc6PhFjSGiQXgAB");
    pub const COIN_RESERVE_FEE_RECEIVER: Pubkey = static_pubkey!("9GfaYar1r2HrGANq5qNiQnx88HJ7GULQqxXxt2B4U3Zy");
    pub const PC_RESERVE_FEE_RECEIVER: Pubkey = static_pubkey!("GPf4tD3q71BzPU79YCadYB2NnLciXAVmYuxfgbKKzUdU");
    pub const BASE_RESERVE: Pubkey = static_pubkey!("FzbfXR7sopQL29Ubu312tkqWMxSre4dYSrFyYAjUYiC4,");
    pub const QUOTE_RESERVE: Pubkey = static_pubkey!("FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt");
    pub const BASE_RESERVE_LIQUIDITY_SUPPLY: Pubkey = static_pubkey!("");
    pub const QUOTE_RESERVE_LIQUIDITY_SUPPLY: Pubkey = static_pubkey!("64QJd6MYXUjCBvCaZKaqxiKmaMkPUdNonE1KuY1YoGGb");
    pub const BASE_RESERVE_COLLATERAL_MINT: Pubkey = static_pubkey!("");
    pub const QUOTE_RESERVE_COLLATERAL_MINT: Pubkey = static_pubkey!("Amig8TisuLpzun8XyGfC5HJHHGUQEscjLgoTWsCCKihg");
}


use farm_config::*;
use market_config::*;
use reserve_config::*;

#[inline(always)]
pub fn get_lev_farm_config() -> LevFarmConfig {
    LevFarmConfig { 
        lending_program: LENDING_PROGRAM,
        account: ACCOUNT,
        global: GLOBAL,
        vault_account: VAULT_ACCOUNT, 
        base_token_account: BASE_TOKEN_ACCOUNT, 
        quote_token_account: QUOTE_TOKEN_ACCOUNT, 
        serum_market: SERUM_MARKET, 
        serum_request_queue: SERUM_REQUEST_QUEUE, 
        serum_event_queue: SERUM_EVENT_QUEUE,
        serum_bids: SERUM_BIDS, 
        serum_asks: SERUM_ASKS, 
        serum_coin_vault: SERUM_COIN_VAULT, 
        serum_pc_vault: SERUM_PC_VAULT, 
        serum_fee_receiver: SERUM_FEE_RECEIVER, 
        serum_open_orders: SERUM_OPEN_ORDERS, 
        lp_mint: LP_MINT, 
        amm_id: AMM_ID, 
        amm_open_orders: AMM_OPEN_ORDERS, 
        amm_quantities_or_target_orders: AMM_QUANTITIES_OR_TARGET_ORDERS, 
        amm_coin_account: AMM_COIN_ACCOUNT, 
        amm_pc_account: AMM_PC_ACCOUNT, 
        amm_temp_account: AMM_TEMP_ACCOUNT, 
        amm_withdraw_queue: AMM_WITHDRAW_QUEUE,
        lending_market: LENDING_MARKET,
        lending_market_authority: LENDING_MARKET_AUTHORITY,
        lp_price_account: PYTH_LP_PRICE_ACCOUNT, 
        coin_price_account: PYTH_COIN_PRICE_ACCOUNT, 
        pc_price_account: PYTH_PC_PRICE_ACCOUNT, 
        coin_reserve_fee_receiver: COIN_RESERVE_FEE_RECEIVER, 
        pc_reserve_fee_receiver: PC_RESERVE_FEE_RECEIVER, 
        base_reserve: BASE_RESERVE, 
        quote_reserve: QUOTE_RESERVE,
        solfarm_vault_program: RAYDIUM_VAULT_PROGRAM,
        base_token_mint: COIN_TOKEN_MINT,
        quote_token_mint: PC_TOKEN_MINT,
    }
}