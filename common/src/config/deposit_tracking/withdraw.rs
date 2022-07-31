//! the `withdraw` submodule provides helper objects to facilitate withdrawing
//! tokenized shares from deposit tracking accounts

use super::traits::{RegisterDepositTracking, WithdrawDepositTracking};
use crate::config::deposit_tracking::derivations::{
    derive_tracking_address, derive_tracking_pda_address,
};
use crate::config::ID;
use anchor_lang::prelude::Pubkey;
use anchor_lang::AnchorSerialize;
use sighashdb::GlobalSighashDB;
use solana_program::{instruction::AccountMeta, instruction::Instruction, sysvar};

#[derive(Clone, Debug, Default)]
pub struct WithdrawDepositTrackingAddresses {
    pub authority: Pubkey,
    pub vault: Pubkey,
    pub deposit_tracking_account: Pubkey,
    pub deposit_tracking_pda: Pubkey,
    pub deposit_tracking_hold_account: Pubkey,
    pub receiving_shares_account: Pubkey,
    pub shares_mint: Pubkey,
}

impl WithdrawDepositTrackingAddresses {
    pub fn new(
        user: Pubkey,
        vault: Pubkey,
        shares_mint: Pubkey,
    ) -> WithdrawDepositTrackingAddresses {
        let deposit_tracking_account = derive_tracking_address(&vault, &user, &ID).0;

        let deposit_tracking_pda = derive_tracking_pda_address(&deposit_tracking_account, &ID).0;

        let deposit_tracking_hold_account =
            spl_associated_token_account::get_associated_token_address(
                &deposit_tracking_pda,
                &shares_mint,
            );

        let receiving_shares_account =
            spl_associated_token_account::get_associated_token_address(&user, &shares_mint);

        WithdrawDepositTrackingAddresses {
            authority: user,
            vault,
            deposit_tracking_account,
            deposit_tracking_pda,
            deposit_tracking_hold_account,
            receiving_shares_account,
            shares_mint,
        }
    }
}

impl WithdrawDepositTracking for WithdrawDepositTrackingAddresses {
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
    fn deposit_tracking_hold_account(&self) -> Pubkey {
        self.deposit_tracking_hold_account
    }
    fn shares_mint(&self) -> Pubkey {
        self.shares_mint
    }
    fn receiving_shares_account(&self) -> Pubkey {
        self.receiving_shares_account
    }
    fn instruction(&self, amount: u64, farm_type: tulipv2_sdk_farms::Farm) -> Option<Instruction> {
        let ix_sighash = self.ix_data()?;
        // 8 for the sighash, 16 for the farm_type, 8 for the amount
        let mut ix_data = Vec::with_capacity(32);
        ix_data.extend_from_slice(&ix_sighash[..]);
        match AnchorSerialize::try_to_vec(&amount) {
            Ok(amount_data) => ix_data.extend_from_slice(&amount_data[..]),
            Err(_err) => {
                #[cfg(feature = "logs")]
                msg!("failed to serialize amount {:#?}", err);
                return None;
            }
        }
        match farm_type.serialize() {
            Ok(farm_type_data) => ix_data.extend_from_slice(&farm_type_data[..]),
            Err(_err) => {
                #[cfg(feature = "logs")]
                msg!("failed to serialize farm_type {:#?}", err);
                return None;
            }
        }
        Some(Instruction {
            program_id: ID,
            accounts: self.to_account_meta(None),
            data: ix_data,
        })
    }
    fn ix_data(&self) -> Option<[u8; 8]> {
        GlobalSighashDB.get("withdraw_deposit_tracking")
    }
    fn to_account_meta(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.authority, true),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(self.deposit_tracking_account(), false),
            AccountMeta::new(self.deposit_tracking_pda(), false),
            AccountMeta::new(self.deposit_tracking_hold_account(), false),
            AccountMeta::new(self.receiving_shares_account(), false),
            AccountMeta::new_readonly(self.shares_mint(), false),
            AccountMeta::new(self.vault(), false),
        ]
    }
}
