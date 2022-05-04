//! tulip v1 lending program account types, and helper functions

#![deny(unused_must_use)]
#![deny(clippy::all)]
#![allow(clippy::bool_assert_comparison)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]

pub mod error;
pub mod last_update;
pub mod lending_obligation;
pub mod leverage_farm;
pub mod obligation;
pub mod reserve;

use anchor_lang::solana_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;

pub const ID: Pubkey = static_pubkey!("TLPv2tuSVvn3fSk8RgW3yPddkp5oFivzZV3rA9hQxtX");

use crate::math::{common::WAD, decimal::Decimal};

use anchor_lang::solana_program::{
    clock::{DEFAULT_TICKS_PER_SECOND, DEFAULT_TICKS_PER_SLOT, SECONDS_PER_DAY},
    msg,
    program_error::ProgramError,
};

/// Current version of the program and all new accounts created
pub const PROGRAM_VERSION: u8 = 1;
/// Percentage of an obligation that can be repaid during each liquidation call
pub const LIQUIDATION_CLOSE_FACTOR: u8 = 50;
/// Accounts are created with data zeroed out, so uninitialized state instances
/// will have the version set to 0.
pub const UNINITIALIZED_VERSION: u8 = 0;
/// Collateral tokens are initially valued at a ratio of 5:1 (collateral:liquidity)
// @FIXME: restore to 5
pub const INITIAL_COLLATERAL_RATIO: u64 = 1;
const INITIAL_COLLATERAL_RATE: u64 = INITIAL_COLLATERAL_RATIO * WAD;

/// Number of slots per year
pub const SLOTS_PER_YEAR: u64 =
    DEFAULT_TICKS_PER_SECOND / DEFAULT_TICKS_PER_SLOT * SECONDS_PER_DAY * 365;

// Helpers
fn pack_decimal(decimal: Decimal, dst: &mut [u8; 16]) {
    *dst = decimal
        .to_scaled_val()
        .expect("Decimal cannot be packed")
        .to_le_bytes();
}

fn unpack_decimal(src: &[u8; 16]) -> Decimal {
    Decimal::from_scaled_val(u128::from_le_bytes(*src))
}

fn pack_bool(boolean: bool, dst: &mut [u8; 1]) {
    *dst = (boolean as u8).to_le_bytes()
}

fn unpack_bool(src: &[u8; 1]) -> Result<bool, ProgramError> {
    match u8::from_le_bytes(*src) {
        0 => Ok(false),
        1 => Ok(true),
        _ => {
            msg!("Boolean cannot be unpacked");
            Err(ProgramError::InvalidAccountData)
        }
    }
}
