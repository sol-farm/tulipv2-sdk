use anchor_lang::solana_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;

use super::{LevFarmConfig, GLOBAL, LENDING_PROGRAM, RAYDIUM_VAULT_PROGRAM};

/// configuration variables for the leverage farm program
pub mod farm_config {
    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("5NvAB6MzoVBPVC2BgzL7ZkyRZjvZjpy5kYax1pGtrfdi");
    pub const VAULT_ACCOUNT: Pubkey =
        static_pubkey!("EkePqacuxaubJJxCYW9RxqyXc1r4LzLJTRfF4bW64UQv");
    pub const BASE_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("kVLZnvKQS7951uDqoDn6eSsJ8aj4WTFB2hP3dq4GTqm");
    pub const QUOTE_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("CCE919uwcutmQZBxEzYRfTKYwmufaYWhzXeawEZyjPPD");
}

pub mod vault_config {

    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("EkePqacuxaubJJxCYW9RxqyXc1r4LzLJTRfF4bW64UQv");
    pub const PDA: Pubkey = static_pubkey!("DSzcCvDSQxhhvPfdu1M2aNi1jYBtdUmoFNMsnu6wK41L");
    pub const REWARD_A_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("4qXxKeyqFmNz3TEnWSofNCQQrGa5Jqqm9chYeozZH3Cg");
    pub const REWARD_B_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("5Zeq2DGLqrGngpdnNemydZhvzxDRHiB4uaNkvqtPtGYs");
    pub const POOL_REWARD_A_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("5ihtMmeTAx3kdf459Yt3bqos5zDe4WBBcSZSB6ooNxLt");
    pub const POOL_REWARD_B_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("5ihtMmeTAx3kdf459Yt3bqos5zDe4WBBcSZSB6ooNxLt");
    pub const POOL_AUTHORITY: Pubkey =
        static_pubkey!("DdFXxCbn5vpxPRaGmurmefCTTSUa5XZ9Kh6Noc4bvrU9");
    pub const LP_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("9jngyDQiEhYh7RWVS5SCJnyhN3kPjG8rJBJeKcFGs9eB");
    // set to this if not used
    // pub const OLD_VAULT_INFO: Pubkey = DEFAULT_KEY;
    pub const OLD_VAULT_INFO_ACCOUNT: Pubkey =
        static_pubkey!("36ckujxHmZLyqvzfT5wE94tHEgYfMkChPKAhkGUyGkYW");
    pub const VAULT_INFO_ACCOUNT: Pubkey =
        static_pubkey!("8RGsvBeGytCzpf8jfFSbLygvSSLsb4tPgqqQqvyrXfKx");
}

pub mod market_config {
    use super::*;
    pub const SERUM_MARKET: Pubkey = static_pubkey!("Cm4MmknScg7qbKqytb1mM92xgDxv3TNXos4tKbBqTDy7");
    pub const SERUM_REQUEST_QUEUE: Pubkey =
        static_pubkey!("8nd4pSGRFepzj4KRCVpwxdMGNUJVUoa8NcHYEnt99NEn");
    pub const SERUM_EVENT_QUEUE: Pubkey =
        static_pubkey!("4afBYfMNsNpLQxFFt72atZsSF4erfU28XvugpX6ugvr1");
    pub const SERUM_BIDS: Pubkey = static_pubkey!("G65a5G6xHpc9zV8tGhVSKJtz7AcAJ8Q3hbMqnDJQgMkz");
    pub const SERUM_ASKS: Pubkey = static_pubkey!("7bKEjcZEqVAWsiRGDnxXvTnNwhZLt2SH6cHi5hpcg5de");
    pub const SERUM_COIN_VAULT: Pubkey =
        static_pubkey!("5QDTh4Bpz4wruWMfayMSjUxRgDvMzvS2ifkarhYtjS1B");
    pub const SERUM_PC_VAULT: Pubkey =
        static_pubkey!("76CofnHCvo5wEKtxNWfLa2jLDz4quwwSHFMne6BWWqx");
    pub const SERUM_FEE_RECEIVER: Pubkey =
        static_pubkey!("F5u9eKddWdRyRbK9fDKB1AMCQJKsx8GGAoXSL6LEvA6n");
    pub const SERUM_OPEN_ORDERS: Pubkey =
        static_pubkey!("J1HDghAEkThP8Xn9KhkRPCABHCdPPJT9YF4hpHm8skYX");
    pub const SERUM_VAULT_SIGNER: Pubkey =
        static_pubkey!("AorjCaSV1L6NGcaFZXEyUrmbSqY3GdB3YXbQnrh85v6F");
    pub const LP_MINT: Pubkey = static_pubkey!("7P5Thr9Egi2rvMmEuQkLn8x8e8Qro7u2U7yLD2tU2Hbe");
    pub const COIN_TOKEN_MINT: Pubkey =
        static_pubkey!("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");
    pub const PC_TOKEN_MINT: Pubkey = static_pubkey!("SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt");
    pub const AMM_ID: Pubkey = static_pubkey!("GaqgfieVmnmY4ZsZHHA6L5RSVzCGL3sKx4UgHBaYNy8m");
    pub const AMM_OPEN_ORDERS: Pubkey =
        static_pubkey!("7XWbMpdyGM5Aesaedh6V653wPYpEswA864sBvodGgWDp");
    pub const AMM_QUANTITIES_OR_TARGET_ORDERS: Pubkey =
        static_pubkey!("9u8bbHv7DnEbVRXmptz3LxrJsryY1xHqGvXLpgm9s5Ng");
    pub const AMM_COIN_ACCOUNT: Pubkey =
        static_pubkey!("3FqQ8p72N85USJStyttaohu1EBsTsEZQ9tVqwcPWcuSz");
    pub const AMM_PC_ACCOUNT: Pubkey =
        static_pubkey!("384kWWf2Km56EReGvmtCKVo1BBmmt2SwiEizjhwpCmrN");
    pub const AMM_TEMP_ACCOUNT: Pubkey =
        static_pubkey!("8jqpuijsM2ne5dkwLyjQxa9oCbYEjM6bE1uBaFXmC3TE");
    pub const AMM_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("58z15NsT3JJyfywFbdYzn2GVeDDC444WHyUrssZ5tCm7");
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
        static_pubkey!("EksgiG3ZB1JaCeWuYmAueZHLKnxaAfYgsJzJvUy8g3aM");
    pub const PYTH_COIN_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("83fYH17UZaTCpr6GNcPcS5pZkfxwR1CaEVhYKfkqE8YF");
    pub const PYTH_PC_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("9wrKnvGNaSB3ppbpEYcvWfvtTZq7AAW5J4HYfEEciUVT");
    pub const COIN_RESERVE_FEE_RECEIVER: Pubkey =
        static_pubkey!("4bRQL2hLqfinNJTsiQW6odhYtYjKXH7zsPc2tafadgoj");
    pub const PC_RESERVE_FEE_RECEIVER: Pubkey =
        static_pubkey!("EMwowECdn8hdGf89RuyGL8cbygwa14BN9tLbYDXgMckr");
    pub const BASE_RESERVE: Pubkey = static_pubkey!("9Bm8d2izGsf9eT6Wr79DTnXBkW2LHYVQa57QzeoTbsAF");
    pub const QUOTE_RESERVE: Pubkey =
        static_pubkey!("9AiGVt7Qtap2ijvim4JSudDYgTrSWhwaZmKv8BWGFms9");
    pub const BASE_RESERVE_LIQUIDITY_SUPPLY: Pubkey =
        static_pubkey!("9SG6E3jBTTHLNgpV6ueUYypMYMkm4K5zyS9tk9Rsjm8Y");
    pub const QUOTE_RESERVE_LIQUIDITY_SUPPLY: Pubkey =
        static_pubkey!("93JKmnXMYHQ9KPkKRRJ5Mb7bqSv4newhDFvz9QE2suRG");
    pub const BASE_RESERVE_COLLATERAL_MINT: Pubkey =
        static_pubkey!("8Lg7TowFuMQoGiTsLE6qV9x3czRgDmVy8f8Vv8KS4uW");
    pub const QUOTE_RESERVE_COLLATERAL_MINT: Pubkey =
        static_pubkey!("4QSK13NTKxTBExbMjHFsj3QfHBn4Hfp3DGLSba8GvFvh");
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
