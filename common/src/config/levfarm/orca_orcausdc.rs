use anchor_lang::solana_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;

use super::{LevFarmConfig, GLOBAL, LENDING_PROGRAM, ORCA_VAULT_PROGRAM};

/// configuration variables for the leverage farm program
pub mod farm_config {
    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("5o3EsLS1NTciKHXVsGNYqQQ8iBBK3dBfSPwCH7wsdtRT");
    pub const VAULT_ACCOUNT: Pubkey =
        static_pubkey!("2Dts63SfTz2yivx57izMcVfDMAdpxuBgqM99ChWeJXun");
    pub const BASE_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("HGkRMx91bZzzcbvJVyzWgDAswHA7Rhn11tuox9GYTB5c");
    pub const QUOTE_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("E68tCFxPPFZa6VWb6d6Txe7ugzF977TZvoVF59W951gv");
}

pub mod vault_config {

    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("2Dts63SfTz2yivx57izMcVfDMAdpxuBgqM99ChWeJXun");
    pub const PDA: Pubkey = static_pubkey!("2d2xTss5cCzemByg55CUVB5dZsxrpaLk8BdiZe9WVuXe");
    pub const AQUA_FARM_BASE_TOKEN_MINT: Pubkey =
        static_pubkey!("n8Mpu28RjeYD7oUX3LG1tPxzhRZh3YYLRSHcHRdS3Zx");
    pub const AQUA_FARM_BASE_TOKEN_VAULT: Pubkey =
        static_pubkey!("45BAAQCZYd2kP3Z3WvRwdtfUhvuW4FvpqVK4m8qrR5x1");
    pub const AQUA_FARM_REWARD_TOKEN_MINT: Pubkey =
        static_pubkey!("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE");
    pub const AQUA_FARM_REWARD_TOKEN_VAULT: Pubkey =
        static_pubkey!("DEiqe2Ta9TRMRtWdBqiFV13dhVrqCeG8MMmVwywvXvJo");
    pub const FARM_TOKEN_MINT: Pubkey =
        static_pubkey!("Gc7W5U66iuHQcC1cQyeX9hxkPF2QUVJPTf1NWbW8fNrt");

    pub const VAULT_REWARD_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("82EjQKzz74AndA144SA1RC57puyjsDHggwmPs9VXbH5s");
    pub const VAULT_SWAP_POOL_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("JCGNW8e3V4ADCiCWU42BTF3FHJLidfgcxHYo2m7kDYvN");
    pub const VAULT_SWAP_POOL_TOKEN_A_ACCOUNT: Pubkey =
        static_pubkey!("82EjQKzz74AndA144SA1RC57puyjsDHggwmPs9VXbH5s");
    pub const VAULT_SWAP_POOL_TOKEN_B_ACCOUNT: Pubkey =
        static_pubkey!("8cNdAEu4y6KRWZwekBavQRpQbYBYNAmNCB8xTJgWibz1");
    pub const VAULT_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("GEogPAbFyMyUuxjjgMDBh1of9qaz5BGEtTyYX9LVDfSA");
}

pub mod market_config {
    use super::*;
    pub const SERUM_MARKET: Pubkey = static_pubkey!("2p7nYbtPBgtmY69NsE8DAW6szpRJn7tQvDnqvoEWQvjY");
    pub const SERUM_REQUEST_QUEUE: Pubkey = static_pubkey!("11111111111111111111111111111111");
    pub const SERUM_EVENT_QUEUE: Pubkey = static_pubkey!("11111111111111111111111111111111");
    pub const SERUM_BIDS: Pubkey = static_pubkey!("11111111111111111111111111111111");
    pub const SERUM_ASKS: Pubkey = static_pubkey!("11111111111111111111111111111111");
    pub const SERUM_COIN_VAULT: Pubkey =
        static_pubkey!("9vYWHBPz817wJdQpE8u3h8UoY3sZ16ZXdCcvLB7jY4Dj");
    pub const SERUM_PC_VAULT: Pubkey =
        static_pubkey!("6UczejMUv1tzdvUzKpULKHxrK9sqLm8edR1v9jinVWm9");
    pub const SERUM_FEE_RECEIVER: Pubkey =
        static_pubkey!("7CXZED4jfRp3qdHB9Py3up6v1C4UhHofFvfT6RXbJLRN");
    pub const SERUM_OPEN_ORDERS: Pubkey = static_pubkey!("11111111111111111111111111111111");
    pub const SERUM_VAULT_SIGNER: Pubkey = static_pubkey!("11111111111111111111111111111111");
    pub const LP_MINT: Pubkey = static_pubkey!("n8Mpu28RjeYD7oUX3LG1tPxzhRZh3YYLRSHcHRdS3Zx");
    pub const COIN_TOKEN_MINT: Pubkey =
        static_pubkey!("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE");
    pub const PC_TOKEN_MINT: Pubkey =
        static_pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    pub const AMM_ID: Pubkey = static_pubkey!("2p7nYbtPBgtmY69NsE8DAW6szpRJn7tQvDnqvoEWQvjY");
    pub const AMM_OPEN_ORDERS: Pubkey = static_pubkey!("11111111111111111111111111111111");
    pub const AMM_QUANTITIES_OR_TARGET_ORDERS: Pubkey =
        static_pubkey!("11111111111111111111111111111111");
    pub const AMM_COIN_ACCOUNT: Pubkey =
        static_pubkey!("9vYWHBPz817wJdQpE8u3h8UoY3sZ16ZXdCcvLB7jY4Dj");
    pub const AMM_PC_ACCOUNT: Pubkey =
        static_pubkey!("6UczejMUv1tzdvUzKpULKHxrK9sqLm8edR1v9jinVWm9");
    pub const AMM_TEMP_ACCOUNT: Pubkey = static_pubkey!("11111111111111111111111111111111");
    pub const AMM_WITHDRAW_QUEUE: Pubkey = static_pubkey!("11111111111111111111111111111111");
    pub const AMM_AUTHORITY: Pubkey =
        static_pubkey!("3fr1AhdiAmWLeNrS24CMoAu9pPgbzVhwLtJ6QUPmw2ob");
}

pub mod reserve_config {
    use super::*;
    pub const LENDING_MARKET: Pubkey =
        static_pubkey!("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym");
    pub const LENDING_MARKET_AUTHORITY: Pubkey =
        static_pubkey!("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s");
    pub const PYTH_LP_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("8VAsQbfX5jPGTPWqAdK6qm42erK299oe86jHM3dnDVtp");
    pub const PYTH_COIN_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("ELHiqZR2cYkN2MerbfMSWGy3nwdPuHYiXbiKsQG7Uy9p");
    pub const PYTH_PC_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("ExzpbWgczTgd8J58BrnESndmzBkRVfc6PhFjSGiQXgAB");
    pub const COIN_RESERVE_FEE_RECEIVER: Pubkey =
        static_pubkey!("ADz1SJ28TH2TnLzA9FG7DqiW41CKeLfRvbvDckK1dvFv");
    pub const PC_RESERVE_FEE_RECEIVER: Pubkey =
        static_pubkey!("GPf4tD3q71BzPU79YCadYB2NnLciXAVmYuxfgbKKzUdU");
    pub const BASE_RESERVE: Pubkey = static_pubkey!("6sJg8f3zcAjrd38QhSA3C34n8MzLq1XVTiQr4msozAuv");
    pub const QUOTE_RESERVE: Pubkey =
        static_pubkey!("FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt");
    pub const BASE_RESERVE_LIQUIDITY_SUPPLY: Pubkey =
        static_pubkey!("4LqykQes5scGz4bZLNaNm1bmgMutCCGqQZVni4VJjWhZ");
    pub const QUOTE_RESERVE_LIQUIDITY_SUPPLY: Pubkey =
        static_pubkey!("64QJd6MYXUjCBvCaZKaqxiKmaMkPUdNonE1KuY1YoGGb");
    pub const BASE_RESERVE_COLLATERAL_MINT: Pubkey =
        static_pubkey!("HRYfSMc1gYEvxo9zsh14jYHKxAcfJfoVakKtRtJEomb8");
    pub const QUOTE_RESERVE_COLLATERAL_MINT: Pubkey =
        static_pubkey!("Amig8TisuLpzun8XyGfC5HJHHGUQEscjLgoTWsCCKihg");
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
        solfarm_vault_program: ORCA_VAULT_PROGRAM,
        base_token_mint: COIN_TOKEN_MINT,
        quote_token_mint: PC_TOKEN_MINT,
    }
}
