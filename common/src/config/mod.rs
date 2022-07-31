//! provides configuration information for all vaults
//! as public constant variables. configurations are broken down
//! into the following folder structure:
//!         * <vault_type>
//!             * <vault_name>
//! for example, the following file stores usdc lending vault information
//!         * lending/usdc.rs

//! The config module provides configuration variables, helper objects, and functions for all vaults
//! and related accounts. It is broken down into submodules organized by "account type". For example
//! the `lending` submodule is for v2 lending optimizer vault, while the `deposit_tracking` submodule
//! is for the deposit tracking account

pub mod deposit_tracking;
pub mod lending;

#[cfg(feature = "levfarm")]
pub mod levfarm;

use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use static_pubkey::static_pubkey;

/// tulip v2 vaults program id
pub const ID: Pubkey = static_pubkey!("TLPv2tuSVvn3fSk8RgW3yPddkp5oFivzZV3rA9hQxtX");
pub const SUNNY_QUARRY_PROGRAM: Pubkey =
    static_pubkey!("SPQR4kT3q2oUKEJes2L6NNSBCiPW9SfuhkuqC9bp6Sx");
pub const ORCA_AQUAFARM_PROGRAM: Pubkey =
    static_pubkey!("82yxjeMsvaURa4MbZZ7WZZHfobirZYkH1zF8fmeGtyaQ");
pub const ORCA_SWAP_PROGRAM: Pubkey =
    static_pubkey!("9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP");

/// address of raydium liquidity program
pub const RAYDIUM_LIQUIDITY_V4: Pubkey = static_pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
/// address of first raydium staking program
pub const RAYDIUM_STAKE: Pubkey = static_pubkey!("EhhTKczWMGQt46ynNeRX1WfeagwwJd7ufHvCDjRxjo5Q");
/// address of the latest raydium staking program
pub const RAYDIUM_STAKE_V5: Pubkey = static_pubkey!("9KEPoZmtHUrBbhWN1v1KWLMkkvwY6WLtAVUCPRtRjP4z");