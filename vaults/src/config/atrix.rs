use crate::accounts::orca_vault::{
    derive_dd_compound_queue_address, derive_dd_withdraw_queue_address, derive_user_farm_address,
};
use crate::accounts::raydium_vault::derive_associated_stake_info_address;
use crate::accounts::{
    derive_compound_queue_address, derive_pda_address, derive_shares_mint_address,
    derive_withdraw_queue_address, raydium_vault::derive_user_stake_info_address,
};
use anchor_lang::solana_program::pubkey::Pubkey;
use so_defi_atrix::addresses as atrix_addresses;

use super::VaultBaseConfig;

pub struct AtrixVaultConfig {
    pub vault: Pubkey,
    pub pda: Pubkey,
    pub withdraw_queue: Pubkey,
    pub compound_queue: Pubkey,
    pub deposit_queue: Pubkey,
    pub underlying_mint: Pubkey,
    pub shares_mint: Pubkey,
    pub staker_account: Pubkey,
    pub harvester_account: Pubkey,
}

impl AtrixVaultConfig {
    pub fn new(vault: Pubkey, underlying_mint: Pubkey, farm_key: Pubkey, crop_key: Pubkey) -> Self {
        let pda = derive_pda_address(&vault).0;
        let shares_mint = derive_shares_mint_address(&vault, &underlying_mint).0;
        let withdraw_queue = derive_withdraw_queue_address(&vault, &underlying_mint).0;
        let compound_queue = derive_compound_queue_address(&vault, &underlying_mint).0;
        let staker_account = atrix_addresses::find_staker_address(farm_key, pda).0;
        let harvester_account = atrix_addresses::find_harvester_address(crop_key, pda).0;
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
            harvester_account,
            staker_account,
        }
    }
}

impl VaultBaseConfig for AtrixVaultConfig {
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
