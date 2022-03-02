//! provides a tokenized shares vault targetting the Raydium protocol
//! capable of supporting any v4 and onwards Raydium AMM farm
use super::{vault_base::VaultBaseV1, InitVaultArgsV1};
use tulipv2_sdk_farms::Farm;
use tulipv2_sdk_common::msg_panic;
use anchor_lang::prelude::*;
use tulipv2_sdk_common::{traits::vault::TokenizedShares, DEFAULT_KEY};
pub const VAULT_ACCOUNT_SIZE: usize = 1712;
#[cfg(not(target_arch = "bpf"))]
use derivative::*;

#[cfg(not(target_arch = "bpf"))]
use type_layout::TypeLayout;



#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
/// wraps the VaultBaseV1 type in a Raydium vault
pub struct RaydiumVaultV1 {
    pub base: VaultBaseV1,
    pub raydium_lp_mint_address: Pubkey,
    pub raydium_amm_id: Pubkey,
    pub raydium_amm_authority: Pubkey,
    pub raydium_amm_open_orders: Pubkey,
    pub raydium_amm_quantities_or_target_orders: Pubkey,
    pub raydium_stake_program: Pubkey,
    pub raydium_liquidity_program: Pubkey,
    pub raydium_coin_token_account: Pubkey,
    pub raydium_pc_token_account: Pubkey,
    pub raydium_pool_temp_token_account: Pubkey,
    pub raydium_pool_lp_token_account: Pubkey,
    pub raydium_pool_withdraw_queue: Pubkey,
    pub raydium_pool_id: Pubkey,
    pub raydium_pool_authority: Pubkey,
    pub raydium_pool_reward_a_token_account: Pubkey,
    pub raydium_pool_reward_b_token_account: Pubkey,
    /// indicates if this particular raydium vault emits dual rewards
    pub dual_rewards: u8,
    /// the vault token acount used to
    /// store rewardA tokens, in pairs like
    /// RAY-USDC, this would be the RAY reward account
    /// in pairs like TULIP-USDC this would be RAY account
    ///
    /// in general raydium seems to have the convention where
    /// rewardA is almost always the raydium token account
    /// even if its not actually given as a reward
    /// having rewardB be the actual reward token account
    pub vault_reward_a_token_account: Pubkey,
    /// like vault_reward_a_account but for the rewardB token
    pub vault_reward_b_token_account: Pubkey,

    /// this is the address of the vault's
    /// raydium user stake info account
    pub vault_stake_info_account: Pubkey,

    /// addresss of the new associated stake info account
    pub associated_stake_info_address: Pubkey,

    /// address of the coin mint account, for RAY-USDC
    /// this would be the RAY token mint
    pub coin_mint: Pubkey,
    /// address of the pc mint account, for RAY-USDC
    /// this would be the USDC token mint
    pub pc_mint: Pubkey,

    /// address of the serum market associated with this raydium farm
    pub serum_market: Pubkey,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 407],
}


impl super::Base for RaydiumVaultV1 {
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

/// returns the address used to derive the user stake info account
pub fn derive_user_stake_info_address(vault_pda: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"info", vault_pda.as_ref()], &crate::ID)
}
