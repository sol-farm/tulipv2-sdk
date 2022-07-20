use crate::accounts::derive_quarry_vault_config_data_address;
use crate::accounts::orca_vault::{
    derive_dd_compound_queue_address, derive_dd_withdraw_queue_address, derive_user_farm_address,
};
use crate::accounts::quarry_vault::{derive_miner_address, derive_sunny_vault_address};
use crate::accounts::raydium_vault::derive_associated_stake_info_address;
use crate::accounts::{
    derive_compound_queue_address, derive_pda_address, derive_shares_mint_address,
    derive_withdraw_queue_address, raydium_vault::derive_user_stake_info_address,
};

use anchor_lang::solana_program::pubkey::Pubkey;

use super::VaultBaseConfig;

/// configuration for plain old quarry vaults
pub struct QuarryVaultConfig {
    pub vault: Pubkey,
    pub pda: Pubkey,
    pub withdraw_queue: Pubkey,
    pub compound_queue: Pubkey,
    pub deposit_queue: Pubkey,
    pub underlying_mint: Pubkey,
    pub shares_mint: Pubkey,
    pub miner: Pubkey,
    pub miner_token_account: Pubkey,
}

/// configuration for saber variant quarry vaults which consist of an iou token accoun
pub struct QuarrySaberVaultConfig {
    pub quarry_config: QuarryVaultConfig,
    pub vault_saber_config_account: Pubkey,
}

/// configuration for sunny variant quarry vaults, which consist of the saber vault setup
/// with an additional quarry implementation ontop of it
pub struct QuarrySunnyVaultConfig {
    pub saber_config: QuarrySaberVaultConfig,
    pub vault_sunny_config_account: Pubkey,
    pub sunny_tvault: Pubkey,
    pub sunny_tvault_miner: Pubkey,
    pub sunny_tvault_miner_token_account: Pubkey,
    pub sunny_tvault_internal_token_account: Pubkey,
    pub sunny_tvault_vendor_token_account: Pubkey,
}

impl QuarrySunnyVaultConfig {
    pub fn new(
        vault: Pubkey,
        underlying_mint: Pubkey,
        quarry: Pubkey,
        mine_program: Pubkey,
        sunny_pool: Pubkey,
        sunny_quarry: Pubkey,
        sunny_internal_token_mint: Pubkey,
    ) -> Self {
        // sunny has a slight variant  to the way the miner address is derived, and instead of the authority
        // being the vault pda, it is the sunny tvault.
        let mut saber_config =
            QuarrySaberVaultConfig::new(vault, underlying_mint, quarry, mine_program);
        let sunny_config_data = derive_quarry_vault_config_data_address(&vault).0;
        let sunny_tvault =
            derive_sunny_vault_address(&sunny_pool, &saber_config.quarry_config.pda).0;
        saber_config.quarry_config.miner =
            derive_miner_address(&quarry, &sunny_tvault, &mine_program).0;
        saber_config.quarry_config.miner_token_account =
            spl_associated_token_account::get_associated_token_address(
                &saber_config.quarry_config.miner,
                &underlying_mint,
            );

        let sunny_miner = derive_miner_address(&sunny_quarry, &sunny_tvault, &mine_program).0;
        let sunny_miner_token_account = spl_associated_token_account::get_associated_token_address(
            &sunny_miner,
            &sunny_internal_token_mint,
        );
        let sunny_tvault_internal_token_account =
            spl_associated_token_account::get_associated_token_address(
                &sunny_tvault,
                &sunny_internal_token_mint,
            );
        let sunny_tvault_vendor_token_account =
            spl_associated_token_account::get_associated_token_address(
                &sunny_tvault,
                &underlying_mint,
            );
        Self {
            saber_config,
            vault_sunny_config_account: sunny_config_data,
            sunny_tvault,
            sunny_tvault_miner: sunny_miner,
            sunny_tvault_miner_token_account: sunny_miner_token_account,
            sunny_tvault_vendor_token_account,
            sunny_tvault_internal_token_account,
        }
    }
}

impl QuarrySaberVaultConfig {
    pub fn new(
        vault: Pubkey,
        underlying_mint: Pubkey,
        quarry: Pubkey,
        mine_program: Pubkey,
    ) -> Self {
        let quarry_config = QuarryVaultConfig::new(vault, underlying_mint, quarry, mine_program);
        let config_data = derive_quarry_vault_config_data_address(&quarry_config.vault).0;
        Self {
            quarry_config,
            vault_saber_config_account: config_data,
        }
    }
}

impl QuarryVaultConfig {
    pub fn new(
        vault: Pubkey,
        underlying_mint: Pubkey,
        quarry: Pubkey,
        mine_program: Pubkey,
    ) -> Self {
        let pda = derive_pda_address(&vault).0;
        let shares_mint = derive_shares_mint_address(&vault, &underlying_mint).0;
        let withdraw_queue = derive_withdraw_queue_address(&vault, &underlying_mint).0;
        let compound_queue = derive_compound_queue_address(&vault, &underlying_mint).0;
        let quarry_miner = derive_miner_address(&quarry, &pda, &mine_program).0;
        let quarry_miner_token_account = spl_associated_token_account::get_associated_token_address(
            &quarry_miner,
            &underlying_mint,
        );
        Self {
            vault,
            pda,
            shares_mint,
            withdraw_queue,
            compound_queue,
            underlying_mint,
            deposit_queue: spl_associated_token_account::get_associated_token_address(
                &pda,
                &underlying_mint,
            ),
            miner: quarry_miner,
            miner_token_account: quarry_miner_token_account,
        }
    }
}

impl VaultBaseConfig for QuarryVaultConfig {
    fn compound_queue(&self) -> Pubkey {
        self.compound_queue
    }
    fn deposit_queue(&self) -> Pubkey {
        self.deposit_queue
    }
    fn vault(&self) -> Pubkey {
        self.vault
    }
    fn vault_pda(&self) -> Pubkey {
        self.pda
    }
    fn withdraw_queue(&self) -> Pubkey {
        self.withdraw_queue
    }
    fn shares_mint(&self) -> Pubkey {
        self.shares_mint
    }
}
