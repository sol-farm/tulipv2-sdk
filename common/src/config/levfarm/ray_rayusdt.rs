use anchor_lang::solana_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;

use super::{LevFarmConfig, GLOBAL, LENDING_PROGRAM, RAYDIUM_VAULT_PROGRAM};

/// configuration variables for the leverage farm program
pub mod farm_config {
    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("7cWB89JJst8covE6KgXWqWwzUM61NzKDr2arsAPXT654");
    pub const VAULT_ACCOUNT: Pubkey = static_pubkey!("1ZpdBUTiDLTUe3izSdYfRXSf93fpJPmoKtA5bFjGesS");
    pub const BASE_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("2BADq6FU8HpaLp3deEsrP4ErnUjiCyMpfszccNQUjT1v");
    pub const QUOTE_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("9JDCPWh1YrUKXJonTgdpagB9CHfs3kHdvec5nAKP2vU6");
}

pub mod vault_config {
    use crate::DEFAULT_KEY;

    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("1ZpdBUTiDLTUe3izSdYfRXSf93fpJPmoKtA5bFjGesS");
    pub const PDA: Pubkey = static_pubkey!("9jUJCfHEAxo1hU44g1QKfNfYBD5ctmwtdPFM7xLQKdPG");
    pub const REWARD_A_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("1ZpdBUTiDLTUe3izSdYfRXSf93fpJPmoKtA5bFjGesS");
    pub const POOL_REWARD_A_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("HCHNuGzkqSnw9TbwpPv1gTnoqnqYepcojHw9DAToBrUj");
    pub const POOL_REWARD_B_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("HCHNuGzkqSnw9TbwpPv1gTnoqnqYepcojHw9DAToBrUj");
    pub const REWARD_B_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("3GvyLReVXibBoHjfFrxa3ubBVdjp9uR5rJYzN5trjT5F");
    pub const POOL_AUTHORITY: Pubkey =
        static_pubkey!("8JYVFy3pYsPSpPRsqf43KSJFnJzn83nnRLQgG88XKB8q");
    pub const LP_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("GoW9fQtCrX1Bd4abWzETcpewbeCyQQWEWCyWmnQg3coK");
    // set to this if not used
    // pub const OLD_VAULT_INFO: Pubkey = DEFAULT_KEY;
    pub const OLD_VAULT_INFO_ACCOUNT: Pubkey = DEFAULT_KEY;
    pub const VAULT_INFO_ACCOUNT: Pubkey =
        static_pubkey!("Bo8xiDzWWJgGgjG1YMriLT6hNkQX7NzeeemjkmXiwJmp");
}

pub mod market_config {
    use super::*;
    pub const SERUM_MARKET: Pubkey = static_pubkey!("teE55QrL4a4QSfydR9dnHF97jgCfptpuigbb53Lo95g");
    pub const SERUM_REQUEST_QUEUE: Pubkey =
        static_pubkey!("8RrrYN9WtASWGv9KfC5maF6AKxT6GSCikZ8SkSaimGZa");
    pub const SERUM_EVENT_QUEUE: Pubkey =
        static_pubkey!("58KcficuUqPDcMittSddhT8LzsPJoH46YP4uURoMo5EB");
    pub const SERUM_BIDS: Pubkey = static_pubkey!("AvKStCiY8LTp3oDFrMkiHHxxhxk4sQUWnGVcetm4kRpy");
    pub const SERUM_ASKS: Pubkey = static_pubkey!("Hj9kckvMX96mQokfMBzNCYEYMLEBYKQ9WwSc1GxasW11");
    pub const SERUM_COIN_VAULT: Pubkey =
        static_pubkey!("2kVNVEgHicvfwiyhT2T51YiQGMPFWLMSp8qXc1hHzkpU");
    pub const SERUM_PC_VAULT: Pubkey =
        static_pubkey!("5AXZV7XfR7Ctr6yjQ9m9dbgycKeUXWnWqHwBTZT6mqC7");
    pub const SERUM_FEE_RECEIVER: Pubkey =
        static_pubkey!("F2szJdBzeiYNb45Ysd8ufsjdh8csEbXv1HaTrG4Tyv2R");
    pub const SERUM_OPEN_ORDERS: Pubkey =
        static_pubkey!("DhCYkD7AhiH6zUmmFc4PVXTw6eKxh2bbTWs1t7BgAA2P");
    pub const SERUM_VAULT_SIGNER: Pubkey =
        static_pubkey!("HzWpBN6ucpsA9wcfmhLAFYqEUmHjE9n2cGHwunG5avpL");
    pub const LP_MINT: Pubkey = static_pubkey!("C3sT1R3nsw4AVdepvLTLKr5Gvszr7jufyBWUCvy4TUvT");
    pub const COIN_TOKEN_MINT: Pubkey =
        static_pubkey!("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");
    pub const PC_TOKEN_MINT: Pubkey =
        static_pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
    pub const AMM_ID: Pubkey = static_pubkey!("DVa7Qmb5ct9RCpaU7UTpSaf3GVMYz17vNVU67XpdCRut");
    pub const AMM_OPEN_ORDERS: Pubkey =
        static_pubkey!("7UF3m8hDGZ6bNnHzaT2YHrhp7A7n9qFfBj6QEpHPv5S8");
    pub const AMM_QUANTITIES_OR_TARGET_ORDERS: Pubkey =
        static_pubkey!("3K2uLkKwVVPvZuMhcQAPLF8hw95somMeNwJS7vgWYrsJ");
    pub const AMM_COIN_ACCOUNT: Pubkey =
        static_pubkey!("3wqhzSB9avepM9xMteiZnbJw75zmTBDVmPFLTQAGcSMN");
    pub const AMM_PC_ACCOUNT: Pubkey =
        static_pubkey!("5GtSbKJEPaoumrDzNj4kGkgZtfDyUceKaHrPziazALC1");
    pub const AMM_TEMP_ACCOUNT: Pubkey =
        static_pubkey!("FBzqDD1cBgkZ1h6tiZNFpkh4sZyg6AG8K5P9DSuJoS5F");
    pub const AMM_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("8VuvrSWfQP8vdbuMAP9AkfgLxU9hbRR6BmTJ8Gfas9aK");
    pub const AMM_AUTHORITY: Pubkey =
        static_pubkey!("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1");
}

pub mod reserve_config {
    use super::*;
    pub const LENDING_MARKET: Pubkey =
        static_pubkey!("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym");
    pub const LENDING_MARKET_AUTHORITY: Pubkey =
        static_pubkey!("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s");
    pub const PYTH_LP_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("8m4XxNbrpFLRahQrbkDcdudPQsmUk3rotgkY9yNR4uA6");
    pub const PYTH_COIN_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("83fYH17UZaTCpr6GNcPcS5pZkfxwR1CaEVhYKfkqE8YF");
    pub const PYTH_PC_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("uo3MK2mD9KogjNLxTWVaB5XqA9Hg4mx4QuRm9SRtKdE");
    pub const COIN_RESERVE_FEE_RECEIVER: Pubkey =
        static_pubkey!("4bRQL2hLqfinNJTsiQW6odhYtYjKXH7zsPc2tafadgoj");
    pub const PC_RESERVE_FEE_RECEIVER: Pubkey =
        static_pubkey!("3VQV6yjMUXaTcR6KCvZSAgEKkvZUVEQnNdyEREPbjj31");
    pub const BASE_RESERVE: Pubkey = static_pubkey!("9Bm8d2izGsf9eT6Wr79DTnXBkW2LHYVQa57QzeoTbsAF");
    pub const QUOTE_RESERVE: Pubkey =
        static_pubkey!("Csn3exasdhDzxYApmnci3d8Khb629VmgK4NQqdeyZBNt");
    pub const BASE_RESERVE_LIQUIDITY_SUPPLY: Pubkey =
        static_pubkey!("9SG6E3jBTTHLNgpV6ueUYypMYMkm4K5zyS9tk9Rsjm8Y");
    pub const QUOTE_RESERVE_LIQUIDITY_SUPPLY: Pubkey =
        static_pubkey!("124J21csiR1FdDywteXa8LhAmeqBXZRvozhoE7zq9znc");
    pub const BASE_RESERVE_COLLATERAL_MINT: Pubkey =
        static_pubkey!("8Lg7TowFuMQoGiTsLE6qV9x3czRgDmVy8f8Vv8KS4uW");
    pub const QUOTE_RESERVE_COLLATERAL_MINT: Pubkey =
        static_pubkey!("gLhY2arqFpmVGkpbBbTi3TeWbsWevA8dqrwbKacK3vJ");
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
