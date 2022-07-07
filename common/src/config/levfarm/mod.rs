use anchor_lang::{prelude::Pubkey, solana_program};
use static_pubkey::static_pubkey;

#[cfg(feature = "ray-rayusdc-levfarm")]
pub mod ray_rayusdc;
#[cfg(feature = "orca-orcausdc-levfarm")]
pub mod orca_orcausdc;
#[cfg(feature = "orca-atlasusdc-levfarm")]
pub mod orca_atlasusdc;
#[cfg(feature = "orca-basisusdc-levfarm")]
pub mod orca_basisusdc;
#[cfg(feature = "orca-gmtusdc-levfarm")]
pub mod orca_gmtusdc;
#[cfg(feature = "orca-gstusdc-levfarm")]
pub mod orca_gstusdc;
#[cfg(feature = "orca-orcasol-levfarm")]
pub mod orca_orcasol;
#[cfg(feature = "orca-polisusdc-levfarm")]
pub mod orca_polisusdc;
#[cfg(feature = "orca-samousdc-levfarm")]
pub mod orca_samousdc;
#[cfg(feature = "orca-shdwsol-levfarm")]
pub mod orca_shdwsol;
#[cfg(feature = "orca-shdwusdc-levfarm")]
pub mod orca_shdwusdc;
#[cfg(feature = "orca-solusdc-levfarm")]
pub mod orca_solusdc;
#[cfg(feature = "orca-solusdt-levfarm")]
pub mod orca_solusdt;
#[cfg(feature = "orca-stsolusdc-levfarm")]
pub mod orca_stsolusdc;
#[cfg(feature = "orca-whethsol-levfarm")]
pub mod orca_whethsol;
#[cfg(feature = "orca-whethusdc-levfarm")]
pub mod orca_whethusdc;


#[cfg(feature = "ray-atlasray-levfarm")]
pub mod ray_atlasray;
#[cfg(feature = "ray-btcstsol-levfarm")]
pub mod ray_btcstsol;
#[cfg(feature = "ray-dflusdc-levfarm")]
pub mod ray_dflusdc;
#[cfg(feature = "ray-ethstsol-levfarm")]
pub mod ray_ethstsol;
#[cfg(feature = "ray-generay-levfarm")]
pub mod ray_generay;
#[cfg(feature = "ray-geneusdc-levfarm")]
pub mod ray_geneusdc;
#[cfg(feature = "ray-likeusdc-levfarm")]
pub mod ray_likeusdc;
#[cfg(feature = "ray-mbsusdc-levfarm")]
pub mod ray_mbsusdc;
#[cfg(feature = "ray-msolusdc-levfarm")]
pub mod ray_msolusdc;
#[cfg(feature = "ray-msolusdt-levfarm")]
pub mod ray_msolusdt;
#[cfg(feature = "ray-polisray-levfarm")]
pub mod ray_polisray;
#[cfg(feature = "ray-prismusdc-levfarm")]
pub mod ray_prismusdc;
#[cfg(feature = "ray-rayeth-levfarm")]
pub mod ray_rayeth;
#[cfg(feature = "ray-raysol-levfarm")]
pub mod ray_raysol;
#[cfg(feature = "ray-raysrm-levfarm")]
pub mod ray_raysrm;
#[cfg(feature = "ray-rayusdt-levfarm")]
pub mod ray_rayusdt;
#[cfg(feature = "ray-raywheth-levfarm")]
pub mod ray_raywheth;
#[cfg(feature = "ray-realusdc-levfarm")]
pub mod ray_realusdc;
#[cfg(feature = "ray-samoray-levfarm")]
pub mod ray_samoray;
#[cfg(feature = "ray-slclusdc-levfarm")]
pub mod ray_slclusdc;
#[cfg(feature = "ray-solusdcray-levfarm")]
pub mod ray_solusdcray;
#[cfg(feature = "ray-solusdc-levfarm")]
pub mod ray_solusdc;
#[cfg(feature = "ray-solusdt-levfarm")]
pub mod ray_solusdt;
#[cfg(feature = "ray-starsusdc-levfarm")]
pub mod ray_starsusdc;
#[cfg(feature = "ray-stsolusdc-levfarm")]
pub mod ray_stsolusdc;
#[cfg(feature = "ray-stsolusdt-levfarm")]
pub mod ray_stsolusdt;
#[cfg(feature = "ray-walephusdc-levfarm")]
pub mod ray_walephusdc;
#[cfg(feature = "ray-whethsol-levfarm")]
pub mod ray_whethsol;
#[cfg(feature = "ray-whethusdc-levfarm")]
pub mod ray_whethusdc;


pub const GLOBAL: Pubkey = static_pubkey!("HLuVf6p3SqgEKy8poYA6g26CDGuQddcbETmf8VdJKqjF");
pub const LENDING_PROGRAM: Pubkey = static_pubkey!("4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM");
pub const ORCA_VAULT_PROGRAM: Pubkey = static_pubkey!("FoNqK2xudK7TfKjPFxpzAcTaU2Wwyt81znT4RjJBLFQp");
pub const RAYDIUM_VAULT_PROGRAM: Pubkey = static_pubkey!("7vxeyaXGLqcp66fFShqUdHxdacp4k4kwUpRSSeoZLCZ4");
pub const BORROW_AUTHORIZER: Pubkey = static_pubkey!("Gp1oj71gwapSBjSQoPkWxEyjXxDxrtBVe1ijsVThknXT");

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
    pub solfarm_vault_program: Pubkey,
    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,
}