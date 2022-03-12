//! provides a vault implementation targetting the Quarry protocol
//! this vault type may or may not need to be wrapped in additional
//! accounts depending on how Quarry is being used
use super::vault_base::VaultBaseV1;
use super::InitVaultArgsV1;
use anchor_lang::prelude::*;
#[cfg(not(target_arch = "bpf"))]
use tulip_derivative::*;
use tulipv2_sdk_common::msg_panic;
use tulipv2_sdk_common::traits::pausable::Pausable;
use tulipv2_sdk_common::{traits::vault::TokenizedShares, DEFAULT_KEY};
use tulipv2_sdk_farms::Farm;

#[cfg(not(target_arch = "bpf"))]
use type_layout::TypeLayout;

pub const QUARRY_VAULT_ACCOUNT_SIZE: usize = 1256;
pub const SABER_CONFIG_DATA_ACCOUNT_SIZE: usize = 458;
pub const SUNNY_CONFIG_DATA_ACCOUNT_SIZE: usize = 490;

/// defines the platform for which we are farming
/// that uses quarry. Some platforms may have platform
/// specific features on top of quarry, such as Saber's
/// iou token, and this enum allows us to conditionally
/// active platform specific functionality
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum QuarryVariant {
    /// indicates this is a standard/vanilla variant which uses
    /// Quarry as is
    Vanilla = 0,
    /// indicates this follows the saber variant which uses an IOU
    /// token instead of a typical reward token
    Saber = 1,
    /// indicates this follows the sunny variant which uses a quarry fork
    /// for their own vaults, ontop of saber/quarry. essentially it's quarry ontop of quarry
    Sunny = 2,
    UNKNOWN = u64::MAX,
}

#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
pub struct QuarryVaultV1 {
    pub base: VaultBaseV1,
    /// address of the vault's miner account
    pub miner: Pubkey,
    /// the token account owned by the miner account
    /// which is responsible for holding the tokens that are being farmed with
    ///
    /// also known as the miner_vault
    pub miner_token_account: Pubkey,
    /// reward mint wrapper account
    pub mint_wrapper: Pubkey,
    /// the quarry minter account
    pub minter: Pubkey,
    // address of the quarry account
    pub quarry: Pubkey,
    /// address of the rewarder account that is responsible
    /// for distributing the reward tokens emitted by this quarry
    pub rewarder: Pubkey,
    /// the address of the token mint account for the reward token
    /// emitted by the quarry
    pub reward_token_mint: Pubkey,
    /// the address of the vault's reward token account
    pub reward_token_account: Pubkey,
    pub swap_markets: [Pubkey; 3],
    /// indicates the quarry variant this vault supports
    pub variant: QuarryVariant,
    /// some quarry variants require extra configuration data
    /// that isn't shared by all quarry variants, therefore
    /// this allows us to store extra configuration information
    /// if this is equal to `DEFAULT_KEY` than there is no config
    /// data account needed
    pub config_data: Pubkey,
    pub config_data_initialized: u8,
    /// an account to use when buffer space runs out
    pub extra_data_account: Pubkey,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 263],
}

#[account]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
/// bundles configuration information needed for Saber quarry vault variants
pub struct SaberConfigurationDataV1 {
    /// the vault this account belongs to
    pub vault: Pubkey,
    /// indicates if the "tokenA" is decimal wrapped
    pub is_decimal_wrapped_a: u8,
    /// indicates if the "tokenB" is decimal wrapped"
    pub is_decimal_wrapped_b: u8,
    /// indicates the wrapped token mint for tokenA
    pub wrapper_mint_a: Pubkey,
    /// indicates the address of the wrapper account for tokenA
    /// this is NOT the token account address, but the address of the decimal wrap
    /// program wrapper account
    pub wrapper_account_a: Pubkey,
    /// see wrapper_account_a comment above
    pub wrapper_account_b: Pubkey,
    pub wrapper_mint_b: Pubkey,
    /// optional extra account to store additional information
    /// should the buffer run out of room
    pub extra_data_account: Pubkey,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 256],
}

#[account]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
/// bundles configuration information needed for Sunny quarry vault variants
pub struct SunnyConfigurationDataV1 {
    /// the tulip vault this account belongs to
    pub vault: Pubkey,
    /// indicates if the "tokenA" is decimal wrapped
    pub is_decimal_wrapped_a: u8,
    /// indicates if the "tokenB" is decimal wrapped"
    pub is_decimal_wrapped_b: u8,
    /// indicates the wrapped token mint for tokenA
    pub wrapper_mint_a: Pubkey,
    /// indicates the address of the wrapper account for tokenA
    /// this is NOT the token account address, but the address of the decimal wrap
    /// program wrapper account
    pub wrapper_account_a: Pubkey,
    /// see wrapper_account_a comment above
    pub wrapper_account_b: Pubkey,
    pub wrapper_mint_b: Pubkey,
    /// the sunny equivalent of a quarry
    pub sunny_pool: Pubkey,
    /// this is the vault account we create within sunny
    /// that sunny uses internally, similar to the way we use
    /// deposit tracking accounts
    ///
    /// this is hte vault account created in stage one
    pub sunny_tvault: Pubkey,
    /// this is the quarry miner account that sunny creates
    /// within quarry on behalf of our tvault
    pub sunny_miner: Pubkey,
    /// the token account owned by the sunny miner
    pub sunny_miner_token_account: Pubkey,
    pub sunny_rewarder: Pubkey,
    /// optional extra account to store additional information
    /// should the buffer run out of room
    pub extra_data_account: Pubkey,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 128],
}

impl super::Base for QuarryVaultV1 {
    fn base(&self) -> VaultBaseV1 {
        self.base
    }
    fn shares_mut(&mut self) -> &mut dyn TokenizedShares {
        &mut self.base
    }
    fn shares(&self) -> &dyn TokenizedShares {
        &self.base
    }
    fn init(&mut self, args: &InitVaultArgsV1) {
        msg_panic!("noop");
    }
    fn farm(&self) -> Farm {
        Farm::from(self.base.farm)
    }
    fn sync_shares(&mut self, mint: &anchor_spl::token::Mint) {
        self.base.sync_shares(mint);
    }
}

impl From<QuarryVariant> for u64 {
    fn from(val: QuarryVariant) -> Self {
        match val {
            QuarryVariant::Vanilla => 0,
            QuarryVariant::Saber => 1,
            QuarryVariant::Sunny => 2,
            QuarryVariant::UNKNOWN => u64::MAX,
        }
    }
}

impl From<u64> for QuarryVariant {
    fn from(val: u64) -> Self {
        match val {
            0 => Self::Vanilla,
            1 => Self::Saber,
            2 => Self::Sunny,
            _ => Self::UNKNOWN,
        }
    }
}

impl From<tulipv2_sdk_farms::quarry::Quarry> for QuarryVariant {
    fn from(val: tulipv2_sdk_farms::quarry::Quarry) -> Self {
        match val {
            tulipv2_sdk_farms::quarry::Quarry::VANILLA => Self::Vanilla,
            tulipv2_sdk_farms::quarry::Quarry::SABER => Self::Saber,
            tulipv2_sdk_farms::quarry::Quarry::SUNNY => Self::Sunny,
            tulipv2_sdk_farms::quarry::Quarry::UNKNOWN => Self::UNKNOWN,
        }
    }
}

impl Default for QuarryVariant {
    fn default() -> Self {
        Self::Vanilla
    }
}

impl std::fmt::Display for QuarryVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuarryVariant::Vanilla => f.write_str("QuarryVariant::Vanilla"),
            QuarryVariant::Saber => f.write_str("QuarryVariant::Saber"),
            QuarryVariant::Sunny => f.write_str("QuarryVariant::Sunny"),
            QuarryVariant::UNKNOWN => f.write_str("QuarryVariant::UNKNOWN"),
        }
    }
}
