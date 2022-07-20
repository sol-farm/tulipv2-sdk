use super::{vault_base::VaultBaseV1, InitVaultArgsV1};
use tulipv2_sdk_common::msg_panic;
use tulipv2_sdk_farms::Farm;

use tulipv2_sdk_common::{traits::vault::TokenizedShares, DEFAULT_KEY};
use anchor_lang::prelude::*;

#[cfg(not(target_arch = "bpf"))]
use type_layout::TypeLayout;

pub const ATRIX_VAULT_ACCOUNT_SIZE: usize = 1184;

#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
/// wraps the VaultBaseV1 type in an Orca vault
/// that is capable of farming non-double dip vaults
pub struct AtrixVaultV1 {
    pub base: VaultBaseV1,
    /// the actual atrix farm account, with this information
    /// we can load the account, and source all other validation
    /// related information
    pub atrix_farm_account: Pubkey,
    /// the staker account which is owned by the vault pda
    pub vault_staker_account: Pubkey,
    pub vault_harvester_account: Pubkey,
    pub dual_crop: u8,
    pub buffer: [u8; 519],
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct ConfigureAtrixVaultArgs {
    pub atrix_farm_account: Pubkey,
    pub vault_staker_account: Pubkey,
    pub vault_harvester_account: Pubkey,
    pub dual_crop: u8,
}

impl super::Base for AtrixVaultV1 {
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

impl AtrixVaultV1 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn configure(&mut self, args: &ConfigureAtrixVaultArgs) {
        if self.base.configured == 1 {
            msg_panic!("vault is already configured")
        }
        self.atrix_farm_account = args.atrix_farm_account;
        self.vault_staker_account = args.vault_staker_account;
        self.vault_harvester_account = args.vault_harvester_account;
        self.dual_crop = args.dual_crop;
        self.base.configured = 1;
        self.base.deposits_paused = 0;
        self.base.withdraws_paused = 0;
        self.base.compound_paused = 0;
    }
}

impl Default for AtrixVaultV1 {
    fn default() -> Self {
        Self {
            base: VaultBaseV1::default(),
            atrix_farm_account: DEFAULT_KEY,
            vault_staker_account: DEFAULT_KEY,
            vault_harvester_account: DEFAULT_KEY,
            dual_crop: 0,
            buffer: [0_u8; 519],
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size_of() {
        println!("{}", AtrixVaultV1::type_layout());
        let size = std::mem::size_of::<AtrixVaultV1>();
        assert_eq!(size + 8, ATRIX_VAULT_ACCOUNT_SIZE);
    }
}
