//! usdc lending optimizer configuration variables

use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use static_pubkey::static_pubkey;
use tulipv2_sdk_farms::{lending::Lending, Farm};
use crate::config::deposit_tracking::issue_shares::DepositAddresses;
use crate::config::deposit_tracking::register::RegisterDepositTrackingAddresses;
use crate::config::deposit_tracking::traits::{
    IssueShares, RegisterDepositTracking, WithdrawDepositTracking,
};
use crate::config::deposit_tracking::withdraw::WithdrawDepositTrackingAddresses;
use crate::config::lending::traits::WithdrawMultiOptimizerVault;
use crate::config::lending::withdraw::{WithdrawAddresses, PlatformConfigAddresses};
use crate::config::lending::Platform;

/// bundles configuration information for the usdc lending optimizer multi deposit vault

pub mod multi_deposit {
    use super::*;

    /// empty struct used to implement the various traits used
    /// to interact with the ray lending optimizer vault
    pub struct ProgramConfig;

    pub const TAG_STRING: &str = "rayv1";
    pub const FARM_KEY: Farm = Farm::Lending {
        name: Lending::MULTI_DEPOSIT,
    };

    /// address of the multi deposit vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("EH1iQnhDqQpHsVJWLw8oC1ehDqVaPGh7JH6ctG4dAQ2d");
    /// address of the multi deposit vault pda
    pub const PDA: Pubkey = static_pubkey!("2fuStNc5WdN2fyqU4At35mbzFeuuhAz5XUs3k5DEJ4L9");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("6JKtKEFiyp67VPN61nTCyUpY1cLrAZ7qpXiBsKqWZqWh");
    /// address of the multi depsoit vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("5GdPPY5avs9VdYJPgCE7GuYdAbmUnsdkxHZ9UYATtmx2");
    /// address of the multi deposit vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("CZMaihH88aKCkYEtUJXYxKCPizmxifc5wtFTgH4eWjcR");
    /// address of the multi deposit vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("BYySDEiFHyqWohWuvLiDQyZnex39vREtdRckn9MkqY35");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");
    pub const REBALANCE_STATE_TRANSITION: Pubkey =
        static_pubkey!("3r7Ft3TawHenBiiTMCDpsRpaQmaTk26Hd67iCBycS6Bv");
    pub const REBALANCE_STATE_TRANSITION_UNDERLYING: Pubkey =
        static_pubkey!("6zFk7gwbEysqeoedRadpVm9dMUX3vjuam5YBaK1guF3V");

    /// the address of the multi deposit vault's shares token account for the solend standalone vault
    pub const SOLEND_OPTIMIZER_SHARES_ACCOUNT: Pubkey =
        static_pubkey!("3o1T6wQ33RChKuhvnPpd1MdnNfvXK64Hv3w7Y1RDEgWd");
    /// the address of the multi deposit vault's shares token account for the tulip standalone vault
    pub const TULIP_OPTIMIZER_SHARES_ACCOUNT: Pubkey =
        static_pubkey!("EU7UfDMra9S1E6XAkWnNCdfNdE9L8Cngn3Db6uwidDNT");
    /// the address of the multi deposit vault's shares token account for the mango standalone vault
    pub const MANGO_OPTIMIZER_SHARES_ACCOUNT: Pubkey =
        static_pubkey!("Cq8zdnesy1soVFVYiyuMH7rRYvzb4K42YHMRcDvN8Pv7");

    impl ProgramConfig {
        #[inline(always)]
        pub fn issue_shares_ix(user: Pubkey) -> impl IssueShares {
            DepositAddresses::new(user, ACCOUNT, PDA, SHARES_MINT, UNDERLYING_MINT)
        }
        #[inline(always)]
        pub fn register_deposit_tracking_ix(user: Pubkey) -> impl RegisterDepositTracking {
            RegisterDepositTrackingAddresses::new(user, ACCOUNT, SHARES_MINT, UNDERLYING_MINT)
        }
        #[inline(always)]
        pub fn withdraw_deposit_tracking(user: Pubkey) -> impl WithdrawDepositTracking {
            WithdrawDepositTrackingAddresses::new(user, ACCOUNT, SHARES_MINT)
        }
        #[inline(always)]
        pub fn withdraw_multi_deposit_optimizer_vault(
            user: Pubkey,
            platform: Platform,
        ) -> std::result::Result<impl WithdrawMultiOptimizerVault, std::io::Error> {
            let (standalone_config, platform_config) = if platform.eq(&Platform::MangoV3) {
                (
                    (
                        ProgramConfig::get_mango_remaining_accounts().to_vec(),
                        platform,
                    ),
                    super::mango::platform_config(),
                )
            } else if platform.eq(&Platform::Solend) {
                (
                    (
                        ProgramConfig::get_solend_remaining_accounts().to_vec(),
                        platform,
                    ),
                    super::solend::platform_config(),
                )
            } else {
                (
                    (
                        ProgramConfig::get_tulip_remaining_accounts().to_vec(),
                        platform,
                    ),
                    super::tulip::platform_config(),
                )
            };
            WithdrawAddresses::new(
                user,
                ACCOUNT,
                PDA,
                SHARES_MINT,
                UNDERLYING_MINT,
                UNDERLYING_WITHDRAW_QUEUE,
                platform_config,
                (&standalone_config.0, standalone_config.1),
            )
        }
        #[inline(always)]
        pub fn get_tulip_remaining_accounts() -> [Pubkey; 7] {
            [
                super::tulip::COLLATERAL_TOKEN_ACCOUNT,
                super::tulip::RESERVE_ACCOUNT,
                super::tulip::RESERVE_LIQUIDITY_ACCOUNT,
                super::tulip::COLLATERAL_MINT,
                super::tulip::LENDING_MARKET_ACCOUNT,
                super::tulip::LENDING_MARKET_AUTHORITY,
                super::tulip::PYTH_PRICE_ACCOUNT,
            ]
        }

        #[inline(always)]
        pub fn get_solend_remaining_accounts() -> [Pubkey; 8] {
            [
                super::solend::COLLATERAL_TOKEN_ACCOUNT,
                super::solend::RESERVE_ACCOUNT,
                super::solend::RESERVE_LIQUIDITY_ACCOUNT,
                super::solend::COLLATERAL_MINT,
                super::solend::LENDING_MARKET_ACCOUNT,
                super::solend::LENDING_MARKET_AUTHORITY,
                super::solend::PYTH_PRICE_ACCOUNT,
                super::solend::SWITCHBOARD_PRICE_ACCOUNT,
            ]
        }

        #[inline(always)]
        pub fn get_mango_remaining_accounts() -> [Pubkey; 7] {
            [
                super::mango::GROUP,
                super::mango::OPTIMIZER_MANGO_ACCOUNT,
                super::mango::CACHE,
                super::mango::ROOT_BANK,
                super::mango::NODE_BANK,
                super::mango::GROUP_TOKEN_ACCOUNT,
                super::mango::GROUP_SIGNER,
            ]
        }
    }
}

/// bundles configuration information for the solend usdc standalone vault
pub mod solend {
    use super::*;
    pub const TAG_STRING: &str = "solend";
    pub const FARM_KEY: Farm = Farm::Lending { name: Lending::RAY };

    /// address of the standalone vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("8eAWyCa27MCukBBswRWBMc6RB4pLQQ2oyXr7sTKFvGXH");
    /// address of the standalone vault pda
    pub const PDA: Pubkey = static_pubkey!("Bauzn5i1iWNJrFFBG4f8R1M8ozmiYdiB8Ldz8vWehLpn");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("D99R1P1t3N6x8Sh43fBmU22Lk8e64QY2EoUmRMwxRDKm");
    /// address of the standalone vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("EDnKnorFduAJj1hsy8roWxYxMTTzcqkUooS82Kj9v3fg");
    /// address of the standalone vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("HV54izUjLNThr6GCuxKtZVxd97R5jW9h3KeERzparGWA");
    /// address of the standalone vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("BV3XEjiiSqYSTbYvHaMP12x1wfUf6E4NbZ2JwK5hdYDA");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");
    /// address of the configuration data account used the v2 vaults program
    pub const CONFIG_DATA_ACCOUNT: Pubkey =
        static_pubkey!("FXXz3MhPpL9qxEkDDXtJvpfXXWBRgmcbcGuckEbmr7L2");
    /// address of the information account which stores configuration information for the config data account
    pub const INFORMATION_ACCOUNT: Pubkey =
        static_pubkey!("HQqw2qS8FbDBV6uHZv2m8dmavgkDrBxWBevCRYj8fr6g");
    /// address of the program id this standalone vault deposits into
    pub const PROGRAM_ID: Pubkey = static_pubkey!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");
    /// address of the reserves collateral token mint
    pub const COLLATERAL_MINT: Pubkey =
        static_pubkey!("2d95ZC8L5XP6xCnaKx8D5U5eX6rKbboBBAwuBLxaFmmJ");
    /// address of the lending market which owns the reserve
    pub const LENDING_MARKET_ACCOUNT: Pubkey =
        static_pubkey!("4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY");
    /// address of the derived lending market authority
    pub const LENDING_MARKET_AUTHORITY: Pubkey =
        static_pubkey!("DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby");
    /// address of the oracle that goes into the first element
    /// of the oracle keys array
    pub const PYTH_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("AnLf8tVYCM816gmBjiy8n53eXKKEDydT5piYjjQDPgTB");
    /// address of the oracle that goes into the second element
    /// of the oracle keys array
    pub const SWITCHBOARD_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("CppyF6264uKZkGua1brTUa2fSVdMFSCszwzDs76HCuzU");
    /// address of the program which owns the first oracle
    pub const PYTH_PROGRAM_ID: Pubkey =
        static_pubkey!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
    /// address of the program which owns the second oracle
    pub const SWITCHBOARD_PROGRAM_ID: Pubkey =
        static_pubkey!("DtmE9D2CSB4L5D6A15mraeEjrGMm6auWVzgaD8hK2tZM");
    /// address of the lending market reserve account
    pub const RESERVE_ACCOUNT: Pubkey =
        static_pubkey!("9n2exoMQwMTzfw6NFoFFujxYPndWVLtKREJePssrKb36");
    /// address of the reserve's token account which holders
    /// the underlying tokens
    pub const RESERVE_LIQUIDITY_ACCOUNT: Pubkey =
        static_pubkey!("5JT6EK5wLEYGpAXMY2BXvhoeuQCp93eo4os2EtXwnPG1");
    /// the address of the standalone vault's token account for the
    /// reserve's collateral token mint
    pub const COLLATERAL_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("4pNagyM2WWUzimpfRvATKUvoZXmHs3WYFuWXnX6N3gWr");

    #[inline(always)]
    pub fn platform_config() -> PlatformConfigAddresses {
        PlatformConfigAddresses {
            vault: ACCOUNT,
            vault_pda: PDA,
            information_account: INFORMATION_ACCOUNT,
            config_data_account: CONFIG_DATA_ACCOUNT,
            shares_mint: SHARES_MINT,
            underlying_deposit_queue: UNDERLYING_DEPOSIT_QUEUE,
            lending_program: PROGRAM_ID,
        }
    }
}

/// bundles configuration information for the tulip usdc standalone vault
pub mod tulip {
    use super::*;

    pub const TAG_STRING: &str = "tulip";
    pub const FARM_KEY: Farm = Farm::Lending { name: Lending::RAY };

    /// address of the standalone vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("7RqGWFDK8J7AUB498ged2ZrmK5otqPE2nFM5RZQ3sCZ3");
    /// address of the standalone vault pda
    pub const PDA: Pubkey = static_pubkey!("CdN3RmtGr6qF3Jbwvbac3m1JG8RRBK9hLwEf3hohcD5Q");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("nEG5LAQCeHRs5TMcneJShmn36XCBNBaSYet9LP9CVtn");
    /// address of the standalone vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("4CPrFtixPnQnRmB6PeAase7nuMTodYLEaBsnEE7TycGJ");
    /// address of the standalone vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("HvmxJubQcvhL3vuZBnUhoQ2AB1otYZXuvG3LDeGRpzrS");
    /// address of the standalone vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("2iEp3QGNUgzaQL5h3Cng7v6PxQCskDiccf7M3jR1YUzb");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");
    /// address of the configuration data account used the v2 vaults program
    pub const CONFIG_DATA_ACCOUNT: Pubkey =
        static_pubkey!("r5Qo4jPp9J5Qn83RhL4LAAzWyfRWBwjup5dsQcx58G3");
    /// address of the information account which stores configuration information for the config data account
    pub const INFORMATION_ACCOUNT: Pubkey =
        static_pubkey!("J4kVv8c7JWb8vnTYb3pjySYCovt22pFGpfesDsF4mxFS");
    /// address of the program id this standalone vault deposits into
    pub const PROGRAM_ID: Pubkey = static_pubkey!("4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM");
    /// address of the reserves collateral token mint
    pub const COLLATERAL_MINT: Pubkey =
        static_pubkey!("8Lg7TowFuMQoGiTsLE6qV9x3czRgDmVy8f8Vv8KS4uW");
    /// address of the lending market which owns the reserve
    pub const LENDING_MARKET_ACCOUNT: Pubkey =
        static_pubkey!("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym");
    /// address of the derived lending market authority
    pub const LENDING_MARKET_AUTHORITY: Pubkey =
        static_pubkey!("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s");
    /// address of the oracle that goes into the first element
    /// of the oracle keys array
    pub const PYTH_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("83fYH17UZaTCpr6GNcPcS5pZkfxwR1CaEVhYKfkqE8YF");
    /// address of the program which owns the first oracle
    pub const PYTH_PROGRAM_ID: Pubkey =
        static_pubkey!("5JQ8Mhdp2wv3HWcfjq9Ts8kwzCAeBADFBDAgBznzRsE4");
    /// address of the lending market reserve account
    pub const RESERVE_ACCOUNT: Pubkey =
        static_pubkey!("9Bm8d2izGsf9eT6Wr79DTnXBkW2LHYVQa57QzeoTbsAF");
    /// address of the reserve's token account which holders
    /// the underlying tokens
    pub const RESERVE_LIQUIDITY_ACCOUNT: Pubkey =
        static_pubkey!("9SG6E3jBTTHLNgpV6ueUYypMYMkm4K5zyS9tk9Rsjm8Y");
    /// the address of the standalone vault's token account for the
    /// reserve's collateral token mint
    pub const COLLATERAL_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("A3YnPZ79vShgpKmM6VdKfGdgjNYPZZRE73N5qRheqFgi");

    #[inline(always)]
    pub fn platform_config() -> PlatformConfigAddresses {
        PlatformConfigAddresses {
            vault: ACCOUNT,
            vault_pda: PDA,
            information_account: INFORMATION_ACCOUNT,
            config_data_account: CONFIG_DATA_ACCOUNT,
            shares_mint: SHARES_MINT,
            underlying_deposit_queue: UNDERLYING_DEPOSIT_QUEUE,
            lending_program: PROGRAM_ID,
        }
    }
}

/// bundles configuration information for the mango usdc standalone vault
pub mod mango {
    use super::*;
    
    pub const TAG_STRING: &str = "mango";
    pub const FARM_KEY: Farm = Farm::Lending { name: Lending::RAY };

    /// address of the standalone vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("A1tu6mY4CrVynaNTVPMtWfecJmXT2nNqwkhmyAjRaSvc");
    /// address of the standalone vault pda
    pub const PDA: Pubkey = static_pubkey!("2aLJaJoMczpeFk3Fbi2cj4waxieS2VB5EwFsvUVevRMu");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("HcrgGozsr3b35yhpGz132dSFaXHkRzYJhHt6tsFKsQon");
    /// address of the standalone vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("EE1Abe9AT17PH6tM1m4wBPqiFBPRMskCyUPWAqxeNoA6");
    /// address of the standalone vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("Ee8ASkBb6G2d4V4aTqXMcVqs8JMEY82B1CsMgRqDoeG9");
    /// address of the standalone vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("B8S7mgLdLHgToqwFert2piFYvQKGSxrjnQb9XvTf1Nsq");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");
    /// address of the configuration data account used the v2 vaults program
    pub const CONFIG_DATA_ACCOUNT: Pubkey =
        static_pubkey!("Bzfz6u83YDZfJuiHcYQu6SqUoaShQWQRxQXzVCG3Sx63");
    /// address of the information account which stores configuration information for the config data account
    pub const INFORMATION_ACCOUNT: Pubkey =
        static_pubkey!("HmschYQ6kCgnvXV8FvVymnJJaLv8iZYK4jhDZQVq6EKv");
    pub const PROGRAM_ID: Pubkey = static_pubkey!("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68");
    /// address of the mango cache
    pub const CACHE: Pubkey = static_pubkey!("EBDRoayCDDUvDgCimta45ajQeXbexv7aKqJubruqpyvu");
    /// address of the mango group account
    pub const GROUP: Pubkey = static_pubkey!("98pjRuQjK3qA6gXts96PqZT4Ze5QmnCmt3QYjhbUSPue");
    /// address of the mango signer pda
    pub const GROUP_SIGNER: Pubkey = static_pubkey!("9BVcYqEQxyccuwznvxXqDkSJFavvTyheiTYk231T1A8S");
    /// address of the mango group usdc token account
    pub const GROUP_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("4XgHdo9zTMm42bYTPjNgq8FwR3yVwL67kvCFjWVurW44");
    /// address of the mango root bank
    pub const ROOT_BANK: Pubkey = static_pubkey!("7TNHrBUDH3FL9uy9hxjmRcKNNaCBG9sYPuDJSJuj3LGs");
    /// address of the mango usdc node bank
    pub const NODE_BANK: Pubkey = static_pubkey!("GDNCSCaVzhD2L164GwUv8JqTdaHCuYGg21JjXQDtuofk");
    /// address of the standalone vault's mango account
    pub const OPTIMIZER_MANGO_ACCOUNT: Pubkey =
        static_pubkey!("41U4mtXqjiBH2WLJ3E5Sr4f7VcxfTtjEvcAnREn3AGrL");

    #[inline(always)]
    pub fn platform_config() -> PlatformConfigAddresses {
        PlatformConfigAddresses {
            vault: ACCOUNT,
            vault_pda: PDA,
            information_account: INFORMATION_ACCOUNT,
            config_data_account: CONFIG_DATA_ACCOUNT,
            shares_mint: SHARES_MINT,
            underlying_deposit_queue: UNDERLYING_DEPOSIT_QUEUE,
            lending_program: PROGRAM_ID,
        }
    }
}
