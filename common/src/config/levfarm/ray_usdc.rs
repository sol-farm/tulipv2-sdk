use anchor_lang::solana_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;

use super::{LevFarmConfig, GLOBAL, LENDING_PROGRAM, RAYDIUM_VAULT_PROGRAM};

/// configuration variables for the leverage farm program
pub mod farm_config {
    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("84ayseJgpJavzfeESgRdyfMoDo2bs4J2YUBjMT4iTs66");
    pub const VAULT_ACCOUNT: Pubkey = static_pubkey!("HvNpbHuQUqGG748ZzgzcH5216wdQdTc283CEyFMc3RdG");
    pub const BASE_TOKEN_ACCOUNT: Pubkey = static_pubkey!("9k3gGV5WCWug8BNCemv3McAC12p8Tqhxb6GN5hBkmfDE");
    pub const QUOTE_TOKEN_ACCOUNT: Pubkey = static_pubkey!("2nHaWRW4PkutKbpDGvVwJ2JkcW1dKA6gMgLv4rPAmqLk");
}

pub mod market_config {
    use super::*;
    pub const SERUM_MARKET: Pubkey = static_pubkey!("2xiv8A5xrJ7RnGdxXB42uFEkYHJjszEhaJyKKt4WaLep");
    pub const SERUM_REQUEST_QUEUE: Pubkey = static_pubkey!("39mE6bYktM1XAKKmB6WN971X3Sa1yGkHxtCTWMkVrwN2");
    pub const SERUM_EVENT_QUEUE: Pubkey = static_pubkey!("H9dZt8kvz1Fe5FyRisb77KcYTaN8LEbuVAfJSnAaEABz");
    pub const SERUM_BIDS: Pubkey = static_pubkey!("Hf84mYadE1VqSvVWAvCWc9wqLXak4RwXiPb4A91EAUn5");
    pub const SERUM_ASKS: Pubkey = static_pubkey!("DC1HsWWRCXVg3wk2NndS5LTbce3axwUwUZH1RgnV4oDN");
    pub const SERUM_COIN_VAULT: Pubkey = static_pubkey!("GGcdamvNDYFhAXr93DWyJ8QmwawUHLCyRqWL3KngtLRa");
    pub const SERUM_PC_VAULT: Pubkey = static_pubkey!("GGcdamvNDYFhAXr93DWyJ8QmwawUHLCyRqWL3KngtLRa");
    pub const SERUM_FEE_RECEIVER: Pubkey = static_pubkey!("PgjTxGVTmFGQjGX8DrETmT68hRt6Jyps4SFTnxJcf3S");
    pub const SERUM_OPEN_ORDERS: Pubkey = static_pubkey!("CAjCs8wTVXg28ViAxsECxAa7WiwZU53rnWLiNikBeXG3");
    pub const LP_MINT: Pubkey = static_pubkey!("FbC6K13MzHvN42bXrtGaWsvZY9fxrackRSZcBGfjPc7m");
    pub const COIN_TOKEN_MINT: Pubkey = static_pubkey!("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");
    pub const PC_TOKEN_MINT: Pubkey = static_pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    pub const AMM_ID: Pubkey = static_pubkey!("6UmmUiYoBjSrhakAobJw8BvkmJtDVxaeBtbt7rxWo1mg");
    pub const AMM_OPEN_ORDERS: Pubkey = static_pubkey!("J8u8nTHYtvudyqwLrXZboziN95LpaHFHpd97Jm5vtbkW");
    pub const AMM_QUANTITIES_OR_TARGET_ORDERS: Pubkey = static_pubkey!("3cji8XW5uhtsA757vELVFAeJpskyHwbnTSceMFY5GjVT");
    pub const AMM_COIN_ACCOUNT: Pubkey = static_pubkey!("FdmKUE4UMiJYFK5ogCngHzShuVKrFXBamPWcewDr31th");
    pub const AMM_PC_ACCOUNT: Pubkey = static_pubkey!("Eqrhxd7bDUCH3MepKmdVkgwazXRzY6iHhEoBpY7yAohk");
    pub const AMM_TEMP_ACCOUNT: Pubkey = static_pubkey!("D1V5GMf3N26owUFcbz2qR5N4G81qPKQvS2Vc4SM73XGB");
    pub const AMM_WITHDRAW_QUEUE: Pubkey = static_pubkey!("ERiPLHrxvjsoMuaWDWSTLdCMzRkQSo8SkLBLYEmSokyr");
}

pub mod reserve_config {
    use super::*;
    pub const LENDING_MARKET: Pubkey = static_pubkey!("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym");
    pub const LENDING_MARKET_AUTHORITY: Pubkey = static_pubkey!("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s");
    pub const PYTH_LP_PRICE_ACCOUNT: Pubkey = static_pubkey!("AV5GeH126btrRE9uq36tZWjdgCuLc1DdzKEatdjmoNex");
    pub const PYTH_COIN_PRICE_ACCOUNT: Pubkey = static_pubkey!("83fYH17UZaTCpr6GNcPcS5pZkfxwR1CaEVhYKfkqE8YF");
    pub const PYTH_PC_PRICE_ACCOUNT: Pubkey = static_pubkey!("ExzpbWgczTgd8J58BrnESndmzBkRVfc6PhFjSGiQXgAB");
    pub const COIN_RESERVE_FEE_RECEIVER: Pubkey = static_pubkey!("4bRQL2hLqfinNJTsiQW6odhYtYjKXH7zsPc2tafadgoj");
    pub const PC_RESERVE_FEE_RECEIVER: Pubkey = static_pubkey!("GPf4tD3q71BzPU79YCadYB2NnLciXAVmYuxfgbKKzUdU");
    pub const BASE_RESERVE: Pubkey = static_pubkey!("9Bm8d2izGsf9eT6Wr79DTnXBkW2LHYVQa57QzeoTbsAF");
    pub const QUOTE_RESERVE: Pubkey = static_pubkey!("FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt");
    pub const BASE_RESERVE_LIQUIDITY_SUPPLY: Pubkey = static_pubkey!("9SG6E3jBTTHLNgpV6ueUYypMYMkm4K5zyS9tk9Rsjm8Y");
    pub const QUOTE_RESERVE_LIQUIDITY_SUPPLY: Pubkey = static_pubkey!("64QJd6MYXUjCBvCaZKaqxiKmaMkPUdNonE1KuY1YoGGb");
    pub const BASE_RESERVE_COLLATERAL_MINT: Pubkey = static_pubkey!("8Lg7TowFuMQoGiTsLE6qV9x3czRgDmVy8f8Vv8KS4uW");
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