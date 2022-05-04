use anchor_lang::solana_program::pubkey::Pubkey;
use crate::config::lending::usdc;
use tulipv2_sdk_vaults;
use crate::config::lending::utils::{derive_tracking_address, derive_tracking_pda_address};
use crate::lending::ID;

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
    pub fn new(user: Pubkey) -> DepositAddresses {
        let vault = usdc::multi_deposit::ACCOUNT;
        let vault_pda = usdc::multi_deposit::PDA;
        let shares_mint = usdc::multi_deposit::SHARES_MINT;
        let underlying_mint = usdc::multi_deposit::UNDERLYING_MINT;
        let (deposit_tracking_account, _) = derive_tracking_address(
            &vault,
            &user,
            &ID
        );

        let (deposit_tracking_pda, _) = derive_tracking_pda_address(&deposit_tracking_account, &ID);
        let deposit_tracking_hold_account = spl_associated_token_account::get_associated_token_address(
            &deposit_tracking_pda,
            &shares_mint
        );

        // deposit ata for the user
        let depositing_underlying_account = spl_associated_token_account::get_associated_token_address(
            &user,
            &underlying_mint
        );

        let vault_underlying_account = spl_associated_token_account::get_associated_token_address(
            &vault_pda,
            &underlying_mint
        );

        DepositAddresses{
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