pub mod accounts;
pub mod config;
pub mod instructions;

use accounts::{multi_optimizer::MultiDepositOptimizerV1, Base};
use anchor_lang::{
    prelude::{AccountInfo, AccountLoader},
    solana_program::{self, pubkey::Pubkey},
};
use static_pubkey::static_pubkey;
use tulipv2_sdk_common::{config::strategy::StrategyVaults, tag::tag_to_str};
use tulipv2_sdk_farms::{lending::Lending, Farm};

pub const ID: Pubkey = static_pubkey!("TLPv2tuSVvn3fSk8RgW3yPddkp5oFivzZV3rA9hQxtX");
pub const MANAGEMENT: Pubkey = tulipv2_sdk_common::config::V2_MANAGEMENT;

/// attempts to parse an account into a StrategyVault type, requires
/// that the account is a v2 vault account, and that it's of type MultiDepositOptimizerV1
#[inline(always)]
pub fn into_strategy_vault<'info>(account: &AccountInfo<'info>) -> StrategyVaults {
    let loader: AccountLoader<MultiDepositOptimizerV1> = AccountLoader::try_from(account).unwrap();
    {
        let vault = loader.load().unwrap();
        let farm = vault.farm();
        match farm {
            Farm::Lending { name } => match name {
                Lending::MULTI_DEPOSIT => {
                    let mut tag = tag_to_str(&vault.base.tag);
                    tag.make_ascii_lowercase();
                    match tag.as_str() {
                        "usdcv1" => StrategyVaults::USDCv1,
                        "usdtv1" => StrategyVaults::USDTv1,
                        "solv1" => StrategyVaults::SOLv1,
                        "rayv1" => StrategyVaults::RAYv1,
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}
