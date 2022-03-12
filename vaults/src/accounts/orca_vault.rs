use super::{vault_base::VaultBaseV1, InitVaultArgsV1};
use anchor_lang::prelude::*;
#[cfg(not(target_arch = "bpf"))]
use tulip_derivative::*;
use tulipv2_sdk_common::msg_panic;
use tulipv2_sdk_common::{traits::vault::TokenizedShares, DEFAULT_KEY};
use tulipv2_sdk_farms::Farm;

pub const ORCA_VAULT_ACCOUNT_SIZE: usize = 1376;
pub const ORCA_DOUBLE_DIP_VAULT_ACCOUNT_SIZE: usize = 2016;
#[cfg(not(target_arch = "bpf"))]
use type_layout::TypeLayout;

#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
/// wraps the VaultBaseV1 type in an Orca vault
/// that is capable of farming non-double dip vaults
pub struct OrcaVaultV1 {
    pub base: VaultBaseV1,
    pub farm_data: OrcaVaultDataV1,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 135],
}

#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
/// wraps the VaultBaseV1 type in an Orca vault
/// that is capable of farming non-double dip vaults
pub struct OrcaDoubleDipVaultV1 {
    pub base: VaultBaseV1,
    /// configuration information for the non-double dip farm
    pub farm_data: OrcaVaultDataV1,
    /// configuration information for the double dip farm
    pub dd_farm_data: OrcaVaultDataV1,
    pub dd_compound_queue: Pubkey,
    pub dd_compound_queue_nonce: u8,
    /// indicates if the double dip configuration stage has been applied
    pub dd_configured: u8,
    pub dd_withdraw_queue: Pubkey,
    pub dd_withdraw_queue_nonce: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 35],
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
#[repr(C)]
/// main data container for both double dip and regular orca farms
pub struct OrcaVaultDataV1 {
    /// address of the vault's user farm account
    /// for orca farms
    pub user_farm_addr: Pubkey,
    pub user_farm_nonce: u8,
    /// the vault's token account for the lp pair's tokenA
    pub vault_swap_token_a: Pubkey,
    /// the vault's token account for the lp pairs tokenB
    pub vault_swap_token_b: Pubkey,
    /// the pool's token account for tokenA
    pub pool_swap_token_a: Pubkey,
    /// the pool's token account for tokenB
    pub pool_swap_token_b: Pubkey,
    /// the pool's account that's used to store the amm information
    /// orca equivalent of raydium amm id, or serum market
    pub pool_swap_account: Pubkey,
    /// the vault's token account for the reward otken
    /// given out by a farm
    pub vault_reward_token_account: Pubkey,
    /// the vault's farm token account, which is the token
    /// given out in exchange for depositing into a farm
    pub vault_farm_token_account: Pubkey,
    // the vault's token account for the pool's lp token account
    pub vault_swap_token_account: Pubkey,
    pub global_base_token_vault: Pubkey,
    pub global_reward_token_vault: Pubkey,
    pub global_farm: Pubkey,
    pub farm_token_mint: Pubkey,
    pub reward_token_mint: Pubkey,
    pub swap_pool_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    /// provide up to three different orca swap markets / pools / amms
    /// for which this vault can swap against
    pub swap_markets: [Pubkey; 3],
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct ConfigureOrcaVaultArgs {
    pub user_farm_addr: Pubkey,
    pub user_farm_nonce: u8,
    pub vault_swap_token_a: Pubkey,
    pub vault_swap_token_b: Pubkey,
    pub pool_swap_token_a: Pubkey,
    pub pool_swap_token_b: Pubkey,
    pub pool_swap_account: Pubkey,
    pub vault_reward_token_account: Pubkey,
    pub vault_farm_token_account: Pubkey,
    pub vault_swap_token_account: Pubkey,
    pub global_farm: Pubkey,
    pub global_base_token_vault: Pubkey,
    pub global_reward_token_vault: Pubkey,
    pub farm_token_mint: Pubkey,
    pub reward_token_mint: Pubkey,
    pub swap_pool_mint: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub swap_markets: [Pubkey; 3],
}

impl super::Base for OrcaVaultV1 {
    fn base(&self) -> VaultBaseV1 {
        self.base
    }
    fn shares_mut(&mut self) -> &mut dyn TokenizedShares {
        &mut self.base
    }
    fn shares(&self) -> &dyn TokenizedShares {
        &self.base
    }
    /// unlike the majority of other vault implementations
    /// the lending optimizer initializes deposits, withdraws
    /// and compoudning to disabled
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

impl super::Base for OrcaDoubleDipVaultV1 {
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

/// used to derive the address of the double dip compound queue acocount
/// it holds the aquafarm tokens which will be compounded into double dips
pub fn derive_dd_compound_queue_address(vault: &Pubkey, farm_token_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"ddcompoundqueue", vault.as_ref(), farm_token_mint.as_ref()],
        &crate::ID,
    )
}

pub fn derive_dd_withdraw_queue_address(vault: &Pubkey, farm_token_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"ddwithdrawqueue", vault.as_ref(), farm_token_mint.as_ref()],
        &crate::ID,
    )
}
