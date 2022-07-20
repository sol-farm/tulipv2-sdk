use crate::accounts::orca_vault::{
    derive_dd_compound_queue_address, derive_dd_withdraw_queue_address, derive_user_farm_address,
};

use crate::accounts::{
    derive_compound_queue_address, derive_pda_address, derive_shares_mint_address,
    derive_withdraw_queue_address,
};
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::pubkey::Pubkey;
use tulipv2_sdk_common::config::deposit_tracking::issue_shares::DepositAddresses;
use tulipv2_sdk_common::config::deposit_tracking::register::RegisterDepositTrackingAddresses;
use tulipv2_sdk_common::config::deposit_tracking::traits::{
    IssueShares, RegisterDepositTracking, WithdrawDepositTracking,
};
use tulipv2_sdk_common::config::deposit_tracking::withdraw::WithdrawDepositTrackingAddresses;

use super::VaultBaseConfig;

#[derive(Debug)]
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
    pub fn register_deposit_tracking(&self, authority: Pubkey) -> impl RegisterDepositTracking {
        RegisterDepositTrackingAddresses::new(
            authority,
            self.vault,
            self.shares_mint,
            self.underlying_mint,
        )
    }
    pub fn issue_shares(&self, authority: Pubkey) -> impl IssueShares {
        DepositAddresses::new(
            authority,
            self.vault,
            self.pda,
            self.shares_mint,
            self.underlying_mint,
        )
    }
    pub fn withdraw_deposit_tracking(&self, authority: Pubkey) -> impl WithdrawDepositTracking {
        WithdrawDepositTrackingAddresses::new(authority, self.vault, self.shares_mint)
    }
    pub fn add_liq_issue_shares(
        &self,
        authority: Pubkey,
        deposit_tracking_account: Pubkey,
        deposit_tracking_pda: Pubkey,
        receiving_shares_account: Pubkey,
        depositing_underlying_account: Pubkey,
        aqua_farm_program: Pubkey,
        funding_token_a_account: Pubkey,
        funding_token_b_account: Pubkey,
        pool_token_a: Pubkey,
        pool_token_b: Pubkey,
        swap_program: Pubkey,
        swap_account: Pubkey,
        swap_authority: Pubkey,
        token_amount_a: u64,
        token_amount_b: u64,
        farm_type: [u64; 2],
    ) -> Option<Instruction> {
        crate::instructions::orca::new_orca_add_liq_issue_shares_ix(
            authority,
            self.vault,
            deposit_tracking_account,
            deposit_tracking_pda,
            self.pda,
            self.shares_mint,
            receiving_shares_account,
            depositing_underlying_account,
            self.deposit_queue,
            aqua_farm_program,
            funding_token_a_account,
            funding_token_b_account,
            pool_token_a,
            pool_token_b,
            swap_program,
            swap_account,
            swap_authority,
            self.underlying_mint,
            token_amount_a,
            token_amount_b,
            farm_type,
        )
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
