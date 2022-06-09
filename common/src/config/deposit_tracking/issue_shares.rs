//! the `issue_shares` submodule provides helper objects to facilitate issueing `issue_shares` instructions
//! which are a vault-agnostic method of depositing into the Tulip V2 vaults program

use super::{
    derivations::{derive_tracking_address, derive_tracking_pda_address},
    traits::IssueShares,
};
use crate::config::ID;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::AnchorSerialize;
use sighashdb::GlobalSighashDB;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::msg;
use tulipv2_sdk_farms::Farm;

/// object used to bundle together information required by the
/// `IssueShares` trait
#[derive(Clone, Debug, Default)]
pub struct DepositAddresses {
    pub authority: Pubkey,
    pub vault: Pubkey,
    pub deposit_tracking_account: Pubkey,
    pub deposit_tracking_pda: Pubkey,
    pub vault_pda: Pubkey,
    pub shares_mint: Pubkey,
    pub receiving_shares_account: Pubkey,
    pub depositing_underlying_account: Pubkey,
    pub vault_underlying_account: Pubkey,
}

impl DepositAddresses {
    pub fn new(
        user: Pubkey,
        vault: Pubkey,
        vault_pda: Pubkey,
        shares_mint: Pubkey,
        underlying_mint: Pubkey,
    ) -> DepositAddresses {
        let deposit_tracking_account = derive_tracking_address(&vault, &user, &ID).0;

        let deposit_tracking_pda = derive_tracking_pda_address(&deposit_tracking_account, &ID).0;
        let deposit_tracking_hold_account =
            spl_associated_token_account::get_associated_token_address(
                &deposit_tracking_pda,
                &shares_mint,
            );

        // deposit ata for the user
        let depositing_underlying_account =
            spl_associated_token_account::get_associated_token_address(&user, &underlying_mint);

        let vault_underlying_account = spl_associated_token_account::get_associated_token_address(
            &vault_pda,
            &underlying_mint,
        );

        DepositAddresses {
            authority: user,
            vault,
            deposit_tracking_account,
            deposit_tracking_pda,
            vault_pda,
            shares_mint,
            receiving_shares_account: deposit_tracking_hold_account,
            depositing_underlying_account,
            vault_underlying_account,
        }
    }
}

impl IssueShares for DepositAddresses {
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
    fn vault_pda(&self) -> Pubkey {
        self.vault_pda
    }
    fn shares_mint(&self) -> Pubkey {
        self.shares_mint
    }
    fn receiving_shares_account(&self) -> Pubkey {
        self.receiving_shares_account
    }
    fn depositing_underlying_account(&self) -> Pubkey {
        self.depositing_underlying_account
    }
    fn vault_underlying_account(&self) -> Pubkey {
        self.vault_underlying_account
    }
    fn instruction(&self, farm_type: Farm, amount: u64) -> Option<Instruction> {
        let ix_sighash = self.ix_data()?;
        // 8 bytes for the sighash, 8 bytes for thet amount
        // 16 bytes for the serialized farm_type
        let mut ix_data = Vec::with_capacity(32);
        ix_data.extend_from_slice(&ix_sighash[..]);
        match farm_type.serialize() {
            Ok(farm_type_data) => ix_data.extend_from_slice(&farm_type_data[..]),
            Err(err) => {
                #[cfg(feature = "logs")]
                msg!("failed to serialize farm_type {:#?}", err);
                return None;
            }
        }
        match AnchorSerialize::try_to_vec(&amount) {
            Ok(amount_data) => ix_data.extend_from_slice(&amount_data[..]),
            Err(err) => {
                #[cfg(feature = "logs")]
                msg!("failed to serialize amount {:#?}", err);
                return None;
            }
        }
        Some(Instruction {
            program_id: crate::config::ID,
            accounts: self.to_account_meta(None),
            data: ix_data,
        })
    }
    fn ix_data(&self) -> Option<[u8; 8]> {
        GlobalSighashDB.get("issue_shares")
    }
    fn to_account_meta(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.authority(), true),
            AccountMeta::new(self.vault(), false),
            AccountMeta::new(self.deposit_tracking_account(), false),
            AccountMeta::new(self.deposit_tracking_pda(), false),
            AccountMeta::new_readonly(self.vault_pda(), false),
            AccountMeta::new(self.vault_underlying_account(), false),
            AccountMeta::new(self.shares_mint(), false),
            AccountMeta::new(self.receiving_shares_account(), false),
            AccountMeta::new(self.deposit_tracking_account(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ]
    }
}
