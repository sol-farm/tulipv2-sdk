use crate::accounts::{
    derive_tracking_address, derive_tracking_orca_dd_queue_address, derive_tracking_pda_address,
    derive_tracking_queue_address,
};

use super::*;

pub struct DepositTrackingConfig {
    pub tracking_account: Pubkey,
    pub tracking_pda: Pubkey,
    pub tracking_queue: Pubkey,
    pub tracking_hold: Pubkey,
    pub orca_dd_queue: Option<Pubkey>,
}

impl DepositTrackingConfig {
    pub fn new(
        authority: Pubkey,
        vault: Pubkey,
        shares_mint: Pubkey,
        is_orca_double_dip: bool,
    ) -> Self {
        let tracking_account = derive_tracking_address(&vault, &authority, &crate::ID).0;
        let tracking_pda = derive_tracking_pda_address(&tracking_account, &crate::ID).0;
        let tracking_queue = derive_tracking_queue_address(&tracking_pda, &crate::ID).0;
        let tracking_hold =
            spl_associated_token_account::get_associated_token_address(&tracking_pda, &shares_mint);
        let orca_dd_queue = if !is_orca_double_dip {
            None
        } else {
            Some(derive_tracking_orca_dd_queue_address(&tracking_pda, &vault).0)
        };
        Self {
            tracking_account,
            tracking_pda,
            tracking_queue,
            tracking_hold,
            orca_dd_queue,
        }
    }
}
