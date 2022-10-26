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

#[cfg(test)]
mod test {
    use anchor_lang::{AccountDeserialize, solana_program::program_pack::Pack};
    use tulipv2_sdk_common::traits::vault::TokenizedShares;
    use super::*;
    use anchor_lang::solana_program::{self, pubkey::Pubkey, account_info::IntoAccountInfo};
    use solana_client::rpc_client::RpcClient;
    use static_pubkey::static_pubkey;
    #[test]
    fn test_into_strategy_vault_usdcv1() {
        let vault_key = static_pubkey!("3wPiV9inTGexMZjp6x5Amqwp2sRNtpSheG8Hbv2rgq8W");
        let rpc = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let account = rpc.get_account(&vault_key).unwrap();

        let mut acct_tup = (vault_key, account);
        let mut acct = acct_tup.into_account_info();

        let strat_vault = into_strategy_vault(&acct);
        assert!(strat_vault.eq(&StrategyVaults::USDCv1));
    }
    #[test]
    fn test_into_strategy_vault_usdtv1() {
        let vault_key = static_pubkey!("BBRkN5paHbHLku4KrZMN8Mc5U3Ygasd4v2FtxdwG7F8F");
        let rpc = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let account = rpc.get_account(&vault_key).unwrap();

        let mut acct_tup = (vault_key, account);
        let mut acct = acct_tup.into_account_info();

        let strat_vault = into_strategy_vault(&acct);
        assert!(strat_vault.eq(&StrategyVaults::USDTv1));
    }
    #[test]
    fn test_into_strategy_vault_rayv1() {
        let vault_key = static_pubkey!("EH1iQnhDqQpHsVJWLw8oC1ehDqVaPGh7JH6ctG4dAQ2d");
        let rpc = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let account = rpc.get_account(&vault_key).unwrap();

        let mut acct_tup = (vault_key, account);
        let mut acct = acct_tup.into_account_info();

        let strat_vault = into_strategy_vault(&acct);
        assert!(strat_vault.eq(&StrategyVaults::RAYv1));
    }
    #[test]
    fn test_into_strategy_vault_solv1() {
        let vault_key = static_pubkey!("2WNw7tW2G54UCXN726S5tR9XutSEDeMf7xamidQtWszK");
        let rpc = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let account = rpc.get_account(&vault_key).unwrap();

        let mut acct_tup = (vault_key, account);
        let mut acct = acct_tup.into_account_info();

        let strat_vault = into_strategy_vault(&acct);
        assert!(strat_vault.eq(&StrategyVaults::SOLv1));
    }
    #[test]
    fn test_rayv1_exchange_rate() {
        let vault_key = static_pubkey!("EH1iQnhDqQpHsVJWLw8oC1ehDqVaPGh7JH6ctG4dAQ2d");
        let rpc = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let mut strat_vault = MultiDepositOptimizerV1::try_deserialize_unchecked(&mut &rpc.get_account(&vault_key).unwrap().data[..]).unwrap();
        let share_mint = spl_token::state::Mint::unpack_unchecked(&mut &rpc.get_account(&strat_vault.base.shares_mint).unwrap().data[..]).unwrap();

        // this will update the vault state synchronizing shares issued tracked 
        // by the vault with the actual supply of the mint itself.
        //
        // mainly intended for on-chain usage
        //
        // however if you fetch the multi deposit vault state and then invoke the 
        // exchange rate function after some period of time without refetching
        // the multi deposit vault state, this may be useful off-chain
        let exch_rate = strat_vault.base.exchange_rate(&share_mint);
        println!("exchange rate {}", exch_rate);
        // doesn't synchronize the vault state intended for off-chain usage
        let exch_rate = strat_vault.base.cached_exchange_rate(&share_mint);
        println!("exchange rate {}", exch_rate);
    }
}