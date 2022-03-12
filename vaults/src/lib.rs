pub mod accounts;
pub mod instructions;

use anchor_lang::solana_program::{self, pubkey::Pubkey};
use static_pubkey::static_pubkey;

pub const ID: Pubkey = static_pubkey!("TLPv2tuSVvn3fSk8RgW3yPddkp5oFivzZV3rA9hQxtX");
