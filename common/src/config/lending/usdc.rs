//! usdc lending optimizer configuration variables

use anchor_lang::solana_program;
use static_pubkey::static_pubkey;
use anchor_lang::prelude::*;

/// bundles configuration information for the usdc lending optimizer multi deposit vault
pub mod multi_deposit {
    use super::*;

    /// address of the multi deposit vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("3wPiV9inTGexMZjp6x5Amqwp2sRNtpSheG8Hbv2rgq8W");
    /// address of the multi deposit vault pda
    pub const PDA: Pubkey = static_pubkey!("14fdy6YXbhDgnVQz4VcgSGgUcZ35eE48SKDrfqF87NUP");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("Cvvh8nsKZet59nsDDo3orMa3rZnPWQhpgrMCVcRDRgip");
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey = static_pubkey!("FAr7Xouceyn9Ms7Egx4JUQryy3RQXuM27RVvCqH6X1o3");
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey = static_pubkey!("36KtHLHxcGnrfEb2GLwPcbN9nHUkeoi3gd6rMQj8wwVj");
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey = static_pubkey!("HLVcpKPkBJJJGTHTSaZcAixDppy4R65x1is3k8Q7qZpj");
    pub const REBALANCE_STATE_TRANSITION: Pubkey = static_pubkey!("3Vjgd77xSaAeBX9DmtZ2Rw7EwVCqvb8aoPvDD7Z75HXP");
    pub const REBALANCE_STATE_TRANSITION_UNDERLYING: Pubkey = static_pubkey!("BBAcBhNSvGpHd4FHh1XF1VGpGjfeUDeNxxww9TRKra7r");
    
    /// the address of the multi deposit vault's shares token account for the solend standalone vault
    pub const SOLEND_OPTIMIZER_SHARES_ACCOUNT: Pubkey = static_pubkey!("UxVNZzzx3xRxKFAuGjRQgbDaa7mirSkFEAu7qiYQ1h4");
    /// the address of the multi deposit vault's shares token account for the tulip standalone vault
    pub const TULIP_OPTIMIZER_SHARES_ACCOUNT: Pubkey = static_pubkey!("M7akLS7xYVhp68LnMkBBCemvqkGcCycQ3qp7e3SsKR1");
    /// the address of the multi deposit vault's shares token account for the mango standalone vault
    pub const MANGO_OPTIMIZER_SHARES_ACCOUNT: Pubkey = static_pubkey!("A9kM8NKf3v29F3DgRQ5Rw7TJoadFZZDfBGLRBGNzASrr");
}