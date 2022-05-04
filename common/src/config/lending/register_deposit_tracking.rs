use anchor_lang::prelude::Pubkey;
use crate::config::lending::usdc;
use crate::config::lending::utils::{derive_tracking_address, derive_tracking_pda_address, derive_tracking_queue_address};
use crate::lending::ID;

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
    pub fn new(user: Pubkey) -> RegisterDepositTrackingAddresses {
        let vault = usdc::multi_deposit::ACCOUNT;
        let shares_mint = usdc::multi_deposit::SHARES_MINT;
        let underlying_mint = usdc::multi_deposit::UNDERLYING_MINT;
        let (deposit_tracking_account, _) = derive_tracking_address(
            &vault,
            &user,
            &ID
        );

        let (deposit_tracking_pda, _) = derive_tracking_pda_address(
            &deposit_tracking_account,
            &ID
        );

        let (deposit_tracking_queue_account, _) = derive_tracking_queue_address(
            &deposit_tracking_account,
            &ID
        );

        let deposit_tracking_hold_account = spl_associated_token_account::get_associated_token_address(
            &deposit_tracking_pda,
            &shares_mint
        );

        RegisterDepositTrackingAddresses {
            authority: user,
            vault,
            deposit_tracking_account,
            deposit_tracking_queue_account,
            deposit_tracking_pda,
            shares_mint,
            deposit_tracking_hold_account,
            underlying_mint
        }
    }
}