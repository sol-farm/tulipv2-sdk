//! Provides configuration information for all vaults, leveraged yield farms,
//! lending reserves, etc.. This module is broken down into a few different folders
//! levfarm - configuration leveraged yield farms (also includes lending reserves)
//! strategy- configuration strategy vaults
//! deposit_tracking - configuration

pub mod deposit_tracking;
pub mod strategy;

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
pub const RAYDIUM_LIQUIDITY_V4: Pubkey =
    static_pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
/// address of first raydium staking program
pub const RAYDIUM_STAKE: Pubkey = static_pubkey!("EhhTKczWMGQt46ynNeRX1WfeagwwJd7ufHvCDjRxjo5Q");
/// address of the latest raydium staking program
pub const RAYDIUM_STAKE_V5: Pubkey = static_pubkey!("9KEPoZmtHUrBbhWN1v1KWLMkkvwY6WLtAVUCPRtRjP4z");
