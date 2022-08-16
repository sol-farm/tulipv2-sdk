mod accounts;
mod instructions;

use anchor_lang::prelude::*;
use tulipv2_sdk_common::config::levfarm::LENDING_PROGRAM as ID;

pub use {accounts::*, instructions::*};

#[derive(Clone)]
pub struct TulipLending;

impl anchor_lang::AccountDeserialize for TulipLending {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        TulipLending::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self> {
        Ok(TulipLending)
    }
}

impl anchor_lang::Id for TulipLending {
    fn id() -> Pubkey {
        ID
    }
}
