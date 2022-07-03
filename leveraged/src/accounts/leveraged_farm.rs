use anchor_lang::prelude::*;
use super::Farms;

#[account(zero_copy)]
pub struct LeveragedFarm {
    /// denotes the account key of the global state account used for access control
    pub global: Pubkey,
    pub solfarm_vault_program: Pubkey,
    pub solfarm_vault_address: Pubkey,
    // in the case of a farm such as Orca
    // or anything other form which doesn't use serum
    // this is used as a general identifier for a given "market".
    // spl token swap uses pools so this would be the "swap account"
    pub serum_market: Pubkey,
    pub serum_request_queue: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_market_bids: Pubkey,
    pub serum_market_asks: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_fee_recipient: Pubkey,
    pub serum_dex_program: Pubkey,
    // allows receiving referral fees
    // RAY-USDC this would be RAY
    pub serum_base_referral_account: Pubkey,
    // RAY-USDC this would be USDC
    pub serum_quote_referral_account: Pubkey,
    pub serum_open_orders_account: Pubkey,
    // address of the lp token that is being farmed
    pub raydium_lp_mint_address: Pubkey,
    pub raydium_amm_id: Pubkey,
    pub raydium_amm_authority: Pubkey,
    pub raydium_amm_open_orders: Pubkey,
    pub raydium_amm_quantities_or_target_orders: Pubkey,
    pub raydium_liquidity_program: Pubkey,
    // aka pool_coin_token_account
    pub raydium_coin_account: Pubkey,
    // aka pool_pc_token_account
    pub raydium_pc_account: Pubkey,
    // temporary token account used for holding lp tokens
    // during withdrawal
    pub raydium_pool_temp_token_account: Pubkey,
    pub raydium_pool_withdraw_queue: Pubkey,
    // the lending market account
    pub lending_market: Pubkey,
    // the token lending program id
    pub lending_program: Pubkey,
    // for spl token swap and other
    // similar style markets 
    // base / coin == token_a 
    // quote / pc == token_b
    pub base_mint: Pubkey,
    // for spl token swap and other
    // similar style markets 
    // base / coin == token_a 
    // quote / pc == token_b
    pub quote_mint: Pubkey,
    // for spl token swap and other
    // similar style markets 
    // base / coin == token_a 
    // quote / pc == token_b
    pub base_reserve: Pubkey,
    // for spl token swap and other
    // similar style markets 
    // base / coin == token_a 
    // quote / pc == token_b
    pub quote_reserve: Pubkey,
    // for spl token swap and other
    // similar style markets 
    // base / coin == token_a 
    // quote / pc == token_b
    // token account to hold base tokens
    pub base_token_account: Pubkey,
    // for spl token swap and other
    // similar style markets 
    // base / coin == token_a 
    // quote / pc == token_b
    // token account to hold quotce tokens
    pub quote_token_account: Pubkey,
    pub lp_decimals: u8,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub farm_type: Farms,
    pub initialized: bool,
    pub supports_fee: bool,
    pub fee_receiver: Pubkey,

    // pyth price account for the lp token being farmed by this account
    pub lp_token_price_account: Pubkey,
    // pyth price account for the lp token being farmed by this account
    pub coin_price_account: Pubkey,
    // pyth price account for the lp token being farmed by this account
    pub pc_price_account: Pubkey,

    // the account which will receive reserve liquidity fees for the base reserve
    pub coin_reserve_liquidity_fee_receiver: Pubkey,
    // the account which will receive reserve liquidity fees for the quote reserve
    pub pc_reserve_liquidity_fee_receiver: Pubkey,

    pub borrow_authorizer: Pubkey,
    pub borrow_authorizer_nonce: u8,
    pub nonce: u8,
    pub buy_slip: u64,
    pub sell_slip: u64,
    pub buffer: [u8; 304],
}