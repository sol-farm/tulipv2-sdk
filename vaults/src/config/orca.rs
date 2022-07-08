use crate::accounts::orca_vault::{
    derive_dd_compound_queue_address, derive_dd_withdraw_queue_address, derive_user_farm_address,
};
use crate::accounts::raydium_vault::derive_associated_stake_info_address;
use crate::accounts::{
    derive_compound_queue_address, derive_pda_address, derive_shares_mint_address,
    derive_withdraw_queue_address, raydium_vault::derive_user_stake_info_address,
};

use anchor_lang::solana_program::pubkey::Pubkey;

use super::VaultBaseConfig;

pub struct OrcaVaultConfig {
    pub vault: Pubkey,
    pub pda: Pubkey,
    pub withdraw_queue: Pubkey,
    pub compound_queue: Pubkey,
    pub deposit_queue: Pubkey,
    pub underlying_mint: Pubkey,
    pub shares_mint: Pubkey,
    pub user_farm: Pubkey,
    pub dd_compound_queue: Option<Pubkey>,
    pub dd_withdraw_queue: Option<Pubkey>,
    pub dd_user_farm: Option<Pubkey>,
}

impl OrcaVaultConfig {
    pub fn new(
        vault: Pubkey,
        underlying_mint: Pubkey,
        global_farm: Pubkey,
        aquafarm_program: Pubkey,
        // if Some, double dip orca vault
        dd_global_farm: Option<Pubkey>,
        dd_compound_queue_mint: Option<Pubkey>,
    ) -> Self {
        let pda = derive_pda_address(&vault).0;
        let shares_mint = derive_shares_mint_address(&vault, &underlying_mint).0;
        let withdraw_queue = derive_withdraw_queue_address(&vault, &underlying_mint).0;
        let compound_queue = derive_compound_queue_address(&vault, &underlying_mint).0;
        let user_farm =
            derive_user_farm_address(&global_farm, &pda, &spl_token::id(), &aquafarm_program).0;
        let (dd_compound_queue, dd_withdraw_queue, dd_user_farm) =
            if let (Some(dd_global_farm), Some(dd_compound_queue_mint)) =
                (dd_global_farm, dd_compound_queue_mint)
            {
                let dd_compound_queue =
                    derive_dd_compound_queue_address(&vault, &dd_compound_queue_mint).0;
                let dd_withdraw_queue =
                    derive_dd_withdraw_queue_address(&vault, &dd_compound_queue_mint).0;
                let dd_user_farm = derive_user_farm_address(
                    &dd_global_farm,
                    &pda,
                    &spl_token::id(),
                    &aquafarm_program,
                )
                .0;
                (
                    Some(dd_compound_queue),
                    Some(dd_withdraw_queue),
                    Some(dd_user_farm),
                )
            } else {
                (None, None, None)
            };
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
            user_farm,
            dd_compound_queue,
            dd_user_farm,
            dd_withdraw_queue,
        }
    }
}

impl VaultBaseConfig for OrcaVaultConfig {
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
