pub mod accounts;
pub mod instructions;

use anchor_lang::{solana_program, solana_program::pubkey::Pubkey};

use static_pubkey::static_pubkey;

pub const ID: Pubkey = static_pubkey!("Bt2WPMmbwHPk36i4CRucNDyLcmoGdC7xEdrVuxgJaNE6");