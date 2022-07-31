use anchor_lang::solana_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;

use super::{LevFarmConfig, GLOBAL, LENDING_PROGRAM, RAYDIUM_VAULT_PROGRAM};

/// configuration variables for the leverage farm program
pub mod farm_config {
    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("EpAAqa5Q8uYkP6jpREPvwGPsdPh8rmKvZyJKNq7YiMYA");
    pub const VAULT_ACCOUNT: Pubkey =
        static_pubkey!("91M42pKURwf4VQHACzx1VFZ8PGZgW2RDwPkwbBk8peGU");
    pub const BASE_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("3HHbo1CWtnrLVjcnEhGPjhguG7D2p91pxKAf12WeTj8t");
    pub const QUOTE_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("52AXPLa2k2eDL9i4mwhPFdEC765HsJFKDGJKhYTCRxWj");
}

pub mod vault_config {
    use crate::DEFAULT_KEY;

    use super::*;
    pub const ACCOUNT: Pubkey = static_pubkey!("91M42pKURwf4VQHACzx1VFZ8PGZgW2RDwPkwbBk8peGU");
    pub const PDA: Pubkey = static_pubkey!("AP3vn3Uenkw4v8fwDwkffcpQQAsidqSugz2MKYKfrAPp");
    pub const REWARD_A_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("EBG35vxzKwWs58WJgnUdcMboSX5GfK5YmJPj1jZLpx2g");
    pub const POOL_REWARD_A_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("38YS2N7VUb856QDsXHS1h8zv5556YgEy9zKbbL2mefjf");
    pub const POOL_REWARD_B_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("ANDJUfDryy3jY6DngwGRXVyxCJBT5JfojLDXwZYSpnEL");
    pub const REWARD_B_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("BHsQPgxquqT2pBsYaNmsbUNuAg8QMkgWS51E1z2miLbd");
    pub const POOL_AUTHORITY: Pubkey =
        static_pubkey!("DgbCWnbXg43nmeiAveMCkUUPEpAr3rZo3iop3TyP6S63");
    pub const LP_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("J6ECnRDZEXcxuruvErXDWsPZn9czowKynUr9eDSQ4QeN");
    // set to this if not used
    // pub const OLD_VAULT_INFO: Pubkey = DEFAULT_KEY;
    pub const VAULT_INFO_ACCOUNT: Pubkey =
        static_pubkey!("HB1FUY2CkPC9W4xrc8FLEbzTwqo23ErBRGuLwoaoAdYW");
}

pub mod market_config {
    use super::*;
    pub const SERUM_MARKET: Pubkey = static_pubkey!("9wFFyRfZBsuAha4YcuxcXLKwMxJR43S7fPfQLusDBzvT");
    pub const SERUM_REQUEST_QUEUE: Pubkey =
        static_pubkey!("AZG3tFCFtiCqEwyardENBQNpHqxgzbMw8uKeZEw2nRG5");
    pub const SERUM_EVENT_QUEUE: Pubkey =
        static_pubkey!("5KKsLVU6TcbVDK4BS6K1DGDxnh4Q9xjYJ8XaDCG5t8ht");
    pub const SERUM_BIDS: Pubkey = static_pubkey!("14ivtgssEBoBjuZJtSAPKYgpUK7DmnSwuPMqJoVTSgKJ");
    pub const SERUM_ASKS: Pubkey = static_pubkey!("CEQdAFKdycHugujQg9k2wbmxjcpdYZyVLfV9WerTnafJ");
    pub const SERUM_COIN_VAULT: Pubkey =
        static_pubkey!("36c6YqAwyGKQG66XEp2dJc5JqjaBNv7sVghEtJv4c7u6");
    pub const SERUM_PC_VAULT: Pubkey =
        static_pubkey!("8CFo8bL8mZQK8abbFyypFMwEDd8tVJjHTTojMLgQTUSZ");
    pub const SERUM_FEE_RECEIVER: Pubkey =
        static_pubkey!("PgjTxGVTmFGQjGX8DrETmT68hRt6Jyps4SFTnxJcf3S");
    pub const SERUM_OPEN_ORDERS: Pubkey =
        static_pubkey!("5XAxLutApKRQ9WTtRNPpRq2btQkqF94Ezsh2mAQ7uqhR");
    pub const SERUM_VAULT_SIGNER: Pubkey =
        static_pubkey!("F8Vyqk3unwxkXukZFQeYyGmFfTG3CAX4v24iyrjEYBJV");
    pub const LP_MINT: Pubkey = static_pubkey!("8HoQnePLqPj4M7PUDzfw8e3Ymdwgc7NLGnaTUapubyvu");
    pub const COIN_TOKEN_MINT: Pubkey =
        static_pubkey!("So11111111111111111111111111111111111111112");
    pub const PC_TOKEN_MINT: Pubkey =
        static_pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    pub const AMM_ID: Pubkey = static_pubkey!("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");
    pub const AMM_OPEN_ORDERS: Pubkey =
        static_pubkey!("HRk9CMrpq7Jn9sh7mzxE8CChHG8dneX9p475QKz4Fsfc");
    pub const AMM_QUANTITIES_OR_TARGET_ORDERS: Pubkey =
        static_pubkey!("CZza3Ej4Mc58MnxWA385itCC9jCo3L1D7zc3LKy1bZMR");
    pub const AMM_COIN_ACCOUNT: Pubkey =
        static_pubkey!("DQyrAcCrDXQ7NeoqGgDCZwBvWDcYmFCjSb9JtteuvPpz");
    pub const AMM_PC_ACCOUNT: Pubkey =
        static_pubkey!("HLmqeL62xR1QoZ1HKKbXRrdN1p3phKpxRMb2VVopvBBz");
    pub const AMM_TEMP_ACCOUNT: Pubkey =
        static_pubkey!("Awpt6N7ZYPBa4vG4BQNFhFxDj4sxExAA9rpBAoBw2uok");
    pub const AMM_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("G7xeGGLevkRwB5f44QNgQtrPKBdMfkT6ZZwpS9xcC97n");
    pub const AMM_AUTHORITY: Pubkey =
        static_pubkey!("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1");
}

pub mod reserve_config {
    use super::*;
    use crate::config::levfarm::reserves::sol as sol_reserve;
    use crate::config::levfarm::reserves::usdc as usdc_reserve;
    pub const LENDING_MARKET: Pubkey =
        static_pubkey!("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym");
    pub const LENDING_MARKET_AUTHORITY: Pubkey =
        static_pubkey!("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s");
    pub const PYTH_LP_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("jnNMpLbMMLDCYJmxJa2RuArEsLrtLKN1c1yjMuvjFsE");
    pub const PYTH_COIN_PRICE_ACCOUNT: Pubkey = sol_reserve::PYTH_PRICE_ACCOUNT;
    pub const PYTH_PC_PRICE_ACCOUNT: Pubkey = usdc_reserve::PYTH_PRICE_ACCOUNT;
    pub const COIN_RESERVE_FEE_RECEIVER: Pubkey = sol_reserve::LIQUIDITY_FEE_RECEIVER;
    pub const PC_RESERVE_FEE_RECEIVER: Pubkey = usdc_reserve::LIQUIDITY_FEE_RECEIVER;
    pub const BASE_RESERVE: Pubkey = static_pubkey!("FzbfXR7sopQL29Ubu312tkqWMxSre4dYSrFyYAjUYiC4");
    pub const QUOTE_RESERVE: Pubkey =
        static_pubkey!("FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt");
    pub const BASE_RESERVE_LIQUIDITY_SUPPLY: Pubkey = sol_reserve::LIQUIDITY_SUPPLY_TOKEN_ACCOUNT;
    pub const QUOTE_RESERVE_LIQUIDITY_SUPPLY: Pubkey = usdc_reserve::LIQUIDITY_SUPPLY_TOKEN_ACCOUNT;
    pub const BASE_RESERVE_COLLATERAL_MINT: Pubkey = sol_reserve::LIQUIDITY_MINT;
    pub const QUOTE_RESERVE_COLLATERAL_MINT: Pubkey = usdc_reserve::LIQUIDITY_MINT;
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
