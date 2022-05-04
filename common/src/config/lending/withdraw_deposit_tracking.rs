use anchor_lang::prelude::Pubkey;
use crate::config::lending::usdc;
use crate::config::lending::utils::{derive_tracking_address, derive_tracking_pda_address};
use crate::lending::ID;

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
    pub fn new(user: Pubkey) -> WithdrawDepositTrackingAddresses {
        let vault = usdc::multi_deposit::ACCOUNT;
        let shares_mint = usdc::multi_deposit::SHARES_MINT;
        let (deposit_tracking_account, _) = derive_tracking_address(
            &vault,
            &user,
            &ID
        );

        let (deposit_tracking_pda, _) = derive_tracking_pda_address(
            &deposit_tracking_account,
            &ID
        );

        let deposit_tracking_hold_account = spl_associated_token_account::get_associated_token_address(
            &deposit_tracking_pda,
            &shares_mint
        );

        let receiving_shares_account = spl_associated_token_account::get_associated_token_address(
            &user,
            &shares_mint
        );

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