use anchor_lang::{prelude::Pubkey, solana_program};
use static_pubkey::static_pubkey;

#[cfg(feature = "rayusdc-levfarm")]
pub mod ray_usdc;

pub const GLOBAL: Pubkey = static_pubkey!("HLuVf6p3SqgEKy8poYA6g26CDGuQddcbETmf8VdJKqjF");
pub const LENDING_PROGRAM: Pubkey = static_pubkey!("4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM");

pub struct LevFarmConfig {
    pub lending_program: Pubkey,
    pub account: Pubkey,
    pub global: Pubkey,
    pub vault_account: Pubkey,
    pub base_token_account: Pubkey,
    pub quote_token_account: Pubkey,
    pub serum_market: Pubkey,
    pub serum_request_queue: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_coin_vault: Pubkey,
    pub serum_pc_vault: Pubkey,
    pub serum_fee_receiver: Pubkey,
    pub serum_open_orders: Pubkey,
    pub lp_mint: Pubkey,
    pub amm_id: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_quantities_or_target_orders: Pubkey,
    pub amm_coin_account: Pubkey,
    pub amm_pc_account: Pubkey,
    pub amm_temp_account: Pubkey,
    pub amm_withdraw_queue: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub lp_price_account: Pubkey,
    pub coin_price_account: Pubkey,
    pub pc_price_account: Pubkey,
    pub coin_reserve_fee_receiver: Pubkey,
    pub pc_reserve_fee_receiver: Pubkey,
    pub base_reserve: Pubkey,
    pub quote_reserve: Pubkey,
}