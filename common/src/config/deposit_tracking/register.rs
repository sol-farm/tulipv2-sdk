//! the `register` submodule provides helper objects to facilitate registration of
//! a deposit tracking account, used for reward tracking token lockup, etc...

use super::traits::RegisterDepositTracking;
use crate::config::deposit_tracking::derivations::{
    derive_tracking_address, derive_tracking_pda_address, derive_tracking_queue_address,
};
use crate::config::ID;
use anchor_lang::prelude::Pubkey;
use sighashdb::GlobalSighashDB;
use solana_program::{
    instruction::AccountMeta, instruction::Instruction, msg, system_program, sysvar,
};

#[derive(Clone, Debug, Default)]
pub struct RegisterDepositTrackingAddresses {
    pub authority: Pubkey,
    pub vault: Pubkey,
    pub deposit_tracking_account: Pubkey,
    pub deposit_tracking_queue_account: Pubkey,
    pub deposit_tracking_hold_account: Pubkey,
    pub shares_mint: Pubkey,
    pub underlying_mint: Pubkey,
    pub deposit_tracking_pda: Pubkey,
}

impl RegisterDepositTrackingAddresses {
    pub fn new(
        user: Pubkey,
        vault: Pubkey,
        shares_mint: Pubkey,
        underlying_mint: Pubkey,
    ) -> RegisterDepositTrackingAddresses {
        let deposit_tracking_account = derive_tracking_address(&vault, &user, &ID).0;

        let deposit_tracking_pda = derive_tracking_pda_address(&deposit_tracking_account, &ID).0;

        let deposit_tracking_queue_account =
            derive_tracking_queue_address(&deposit_tracking_pda, &ID).0;

        let deposit_tracking_hold_account =
            spl_associated_token_account::get_associated_token_address(
                &deposit_tracking_pda,
                &shares_mint,
            );

        RegisterDepositTrackingAddresses {
            authority: user,
            vault,
            deposit_tracking_account,
            deposit_tracking_queue_account,
            deposit_tracking_pda,
            shares_mint,
            deposit_tracking_hold_account,
            underlying_mint,
        }
    }
}

impl RegisterDepositTracking for RegisterDepositTrackingAddresses {
    fn authority(&self) -> Pubkey {
        self.authority
    }
    fn vault(&self) -> Pubkey {
        self.vault
    }
    fn deposit_tracking_account(&self) -> Pubkey {
        self.deposit_tracking_account
    }
    fn deposit_tracking_pda(&self) -> Pubkey {
        self.deposit_tracking_pda
    }
    fn deposit_tracking_queue_account(&self) -> Pubkey {
        self.deposit_tracking_queue_account
    }
    fn deposit_tracking_hold_account(&self) -> Pubkey {
        self.deposit_tracking_hold_account
    }
    fn shares_mint(&self) -> Pubkey {
        self.shares_mint
    }
    fn underlying_mint(&self) -> Pubkey {
        self.underlying_mint
    }
    fn instruction(&self, farm_type: tulipv2_sdk_farms::Farm) -> Option<Instruction> {
        let ix_sighash = self.ix_data()?;
        // 8 bytes for sighash, 16 for farm_type
        let mut ix_data = Vec::with_capacity(24);
        ix_data.extend_from_slice(&ix_sighash[..]);
        match farm_type.serialize() {
            Ok(farm_type_data) => ix_data.extend_from_slice(&farm_type_data[..]),
            Err(err) => {
                #[cfg(feature = "logs")]
                msg!("failed to serialize farm_type {:#?}", err);
                return None;
            }
        }
        let accounts = self.to_account_meta(None);
        Some(Instruction {
            program_id: ID,
            accounts: self.to_account_meta(None),
            data: ix_data,
        })
    }
    fn ix_data(&self) -> Option<[u8; 8]> {
        GlobalSighashDB.get("register_deposit_tracking_account")
    }
    fn to_account_meta(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority(), true),
            AccountMeta::new_readonly(self.vault(), false),
            AccountMeta::new(self.deposit_tracking_account(), false),
            AccountMeta::new(self.deposit_tracking_queue_account(), false),
            AccountMeta::new(self.deposit_tracking_hold_account(), false),
            AccountMeta::new(self.shares_mint(), false),
            AccountMeta::new(self.deposit_tracking_pda(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ]
    }
}
