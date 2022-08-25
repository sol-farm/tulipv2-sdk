//! usdc lending optimizer configuration variables

use crate::config::deposit_tracking::issue_shares::DepositAddresses;
use crate::config::deposit_tracking::register::RegisterDepositTrackingAddresses;
use crate::config::deposit_tracking::traits::{
    IssueShares, RegisterDepositTracking, WithdrawDepositTracking,
};
use crate::config::deposit_tracking::withdraw::WithdrawDepositTrackingAddresses;
use crate::config::strategy::traits::WithdrawMultiOptimizerVault;
use crate::config::strategy::withdraw::{PlatformConfigAddresses, WithdrawAddresses};
use crate::config::strategy::Platform;
use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use static_pubkey::static_pubkey;
use tulipv2_sdk_farms::{lending::Lending, Farm};
use crate::config::deposit_tracking::issue_shares::DepositAddressesPermissioned;
/// bundles configuration information for the usdc lending optimizer multi deposit vault
pub mod multi_deposit {
    use crate::config::strategy::traits::MultiVaultProgramConfig;

    use super::*;
    /// empty struct used to implement the various traits used
    /// to interact with the sol lending optimizer vault
    pub struct ProgramConfig;

    pub const TAG_STRING: &str = "solv1";
    pub const FARM_KEY: Farm = Farm::Lending {
        name: Lending::MULTI_DEPOSIT,
    };

    /// address of the multi deposit vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("2WNw7tW2G54UCXN726S5tR9XutSEDeMf7xamidQtWszK");
    /// address of the multi deposit vault pda
    pub const PDA: Pubkey = static_pubkey!("FzSwmSA7ZsKsfCRZGnfxb13HYX4nZpMQfBjqpg5bjT5V");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("DgnaQdr5VkNbH7n8qN471Hgsw19hUbMBhUs1N3MQKgtu");
    /// address of the multi deposit vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("637PczfBDWSRX5d55M8punyQEzdx3G3htMwhrD6cVcAk");
    /// address of the multi deposit vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("8HjBSJvQQwaC81gY1sND1ekNwjEvewYvrJeEh4Be6nQQ");
    /// address of the multi deposit vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("9k8MUkfjNDH1XPRNhSia57aAb7HGneP1jREQRPc3XZh7");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("So11111111111111111111111111111111111111112");
    pub const REBALANCE_STATE_TRANSITION: Pubkey =
        static_pubkey!("7VrQy2dzGBff69rwz7XxfF9FB5Pvgi9UQFNGwept4dzt");
    pub const REBALANCE_STATE_TRANSITION_UNDERLYING: Pubkey =
        static_pubkey!("GrDw6ChbbA119r694Wvtnc9dYuBi2BbT1mHJLT5ovuB9");

    /// the address of the multi deposit vault's shares token account for the solend standalone vault
    pub const SOLEND_OPTIMIZER_SHARES_ACCOUNT: Pubkey =
        static_pubkey!("96jC6s6whzjxov8E8pL6t6jJsPPXkeKWQx1CZzLQiahp");
    /// the address of the multi deposit vault's shares token account for the tulip standalone vault
    pub const TULIP_OPTIMIZER_SHARES_ACCOUNT: Pubkey =
        static_pubkey!("qv1gRK7rQkuzyewtg4Xbqgmr8J99FnDhELYtsjCUuws");
    /// the address of the multi deposit vault's shares token account for the mango standalone vault
    pub const MANGO_OPTIMIZER_SHARES_ACCOUNT: Pubkey =
        static_pubkey!("7XdtJRhMYg5DS5uueCjpWHAxRYFaDUFu2MkG5AWE4Ywx");

    impl ProgramConfig {
        #[inline(always)]
        pub fn issue_shares_ix(user: Pubkey) -> impl IssueShares {
            DepositAddresses::new(user, ACCOUNT, PDA, SHARES_MINT, UNDERLYING_MINT)
        }
        #[inline(always)]
        pub fn permissioned_issue_shares_ix(user: Pubkey) -> impl IssueShares {
            DepositAddressesPermissioned::new(user, ACCOUNT, PDA, SHARES_MINT, UNDERLYING_MINT)
        }
        #[inline(always)]
        pub fn register_deposit_tracking_ix(user: Pubkey) -> impl RegisterDepositTracking {
            RegisterDepositTrackingAddresses::new(user, ACCOUNT, SHARES_MINT, UNDERLYING_MINT)
        }
        #[inline(always)]
        pub fn withdraw_deposit_tracking_ix(user: Pubkey) -> impl WithdrawDepositTracking {
            WithdrawDepositTrackingAddresses::new(user, ACCOUNT, SHARES_MINT)
        }
        pub fn withdraw_multi_deposit_optimizer_vault(
            user: Pubkey,
            platform: Platform,
        ) -> std::result::Result<Box<impl WithdrawMultiOptimizerVault>, std::io::Error> {
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
            Ok(Box::new(WithdrawAddresses::new(
                user,
                ACCOUNT,
                PDA,
                SHARES_MINT,
                UNDERLYING_MINT,
                UNDERLYING_WITHDRAW_QUEUE,
                platform_config,
                (&standalone_config.0, standalone_config.1),
            )?))
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
    impl MultiVaultProgramConfig for ProgramConfig {
        fn account(&self, ) -> Pubkey {
            ACCOUNT
        }
        fn pda(&self, ) -> Pubkey {
            PDA
        }
        fn shares_mint(&self, ) -> Pubkey {
            SHARES_MINT
        }
        fn underlying_compound_queue(&self, ) -> Pubkey {
            UNDERLYING_COMPOUND_QUEUE
        }
        fn underlying_deposit_queue(&self, ) -> Pubkey {
            UNDERLYING_DEPOSIT_QUEUE
        }
        fn underlying_withdraw_queue(&self, ) -> Pubkey {
            UNDERLYING_WITHDRAW_QUEUE
        }
        fn underlying_mint(&self, ) -> Pubkey {
            UNDERLYING_MINT
        }
        fn rebalance_state_transition(&self, ) -> Pubkey {
            REBALANCE_STATE_TRANSITION
        }
        fn rebalance_state_transition_underlying(&self, ) -> Pubkey {
            REBALANCE_STATE_TRANSITION_UNDERLYING
        }
        fn optimizer_shares_account(&self, platform: Platform) -> Pubkey {
            match platform {
                Platform::MangoV3 => MANGO_OPTIMIZER_SHARES_ACCOUNT,
                Platform::Solend => SOLEND_OPTIMIZER_SHARES_ACCOUNT,
                Platform::Tulip => TULIP_OPTIMIZER_SHARES_ACCOUNT
            }
        }
        fn issue_shares(&self, user: Pubkey) -> Box<dyn IssueShares> {
            Box::new(ProgramConfig::issue_shares_ix(user))
        }
        fn permissioned_issue_shares(&self, user: Pubkey) -> Box<dyn IssueShares> {
            Box::new(ProgramConfig::permissioned_issue_shares_ix(user))
        }
        fn register_deposit_tracking(&self, user: Pubkey) -> Box<dyn RegisterDepositTracking> {
            Box::new(ProgramConfig::register_deposit_tracking_ix(user))
        }
        fn withdraw_deposit_tracking(&self, user: Pubkey) -> Box<dyn WithdrawDepositTracking> {
            Box::new( ProgramConfig::withdraw_deposit_tracking_ix(user))
        }
        fn withdraw_multi_deposit_optimizer_vault(&self, user: Pubkey, platform: Platform) -> std::result::Result<Box<dyn WithdrawMultiOptimizerVault>, std::io::Error> {
            Ok(ProgramConfig::withdraw_multi_deposit_optimizer_vault(user, platform)?)
        }
        fn remaining_accounts(&self, platform: Platform) -> Vec<Pubkey> {
            match platform {
                Platform::MangoV3 => ProgramConfig::get_mango_remaining_accounts().to_vec(),
                Platform::Solend => ProgramConfig::get_solend_remaining_accounts().to_vec(),
                Platform::Tulip => ProgramConfig::get_tulip_remaining_accounts().to_vec(),
            }
        }
    }
}

/// bundles configuration information for the solend usdc standalone vault
pub mod solend {
    use super::*;

    pub const TAG_STRING: &str = "solend";
    pub const FARM_KEY: Farm = Farm::Lending { name: Lending::SOL };

    /// address of the standalone vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("2DPDpyd8pUoNDux8E7ZbzCgQtvyyVv8gPHUAzHDRXiWN");
    /// address of the standalone vault pda
    pub const PDA: Pubkey = static_pubkey!("9ftmG873yFyybF51v85QjrvYfYZTu83mSgQo6wHtEqTh");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("934f2qngiJN2XnCjqMkEJjECgMGzCBfJF6SCBqzRYBKw");
    /// address of the standalone vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("7q6UP1BhkAxbzkK9HSTbUU8GE3YgGnFTvL4sApMgN9Yt");
    /// address of the standalone vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("57V8aNbjFzZHLs7TrzqgMDvLxuaaMzouLapi3eawytoB");
    /// address of the standalone vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("GKXUBegwfoVBrTJmEbsUZ8sxRtx6Pk54rgerYNvVPP63");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("So11111111111111111111111111111111111111112");
    /// address of the configuration data account used the v2 vaults program
    pub const CONFIG_DATA_ACCOUNT: Pubkey =
        static_pubkey!("7sqrzaBWTf7apr5z2VmAb61g6CbzFr5mzjWcYur8arcA");
    /// address of the information account which stores configuration information for the config data account
    pub const INFORMATION_ACCOUNT: Pubkey =
        static_pubkey!("5vCZXDNn8NTFrvQvNGGRhNAEf5tHqXq83xnmjJMJSFmE");
    /// address of the program id this standalone vault deposits into
    pub const PROGRAM_ID: Pubkey = static_pubkey!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");
    /// address of the reserves collateral token mint
    pub const COLLATERAL_MINT: Pubkey =
        static_pubkey!("5h6ssFpeDeRbzsEHDbTQNH7nVGgsKrZydxdSTnLm6QdV");
    /// address of the lending market which owns the reserve
    pub const LENDING_MARKET_ACCOUNT: Pubkey =
        static_pubkey!("4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY");
    /// address of the derived lending market authority
    pub const LENDING_MARKET_AUTHORITY: Pubkey =
        static_pubkey!("DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby");
    /// address of the oracle that goes into the first element
    /// of the oracle keys array
    pub const PYTH_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG");
    /// address of the oracle that goes into the second element
    /// of the oracle keys array
    pub const SWITCHBOARD_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("AdtRGGhmqvom3Jemp5YNrxd9q9unX36BZk1pujkkXijL");
    /// address of the program which owns the first oracle
    pub const PYTH_PROGRAM_ID: Pubkey =
        static_pubkey!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
    /// address of the program which owns the second oracle
    pub const SWITCHBOARD_PROGRAM_ID: Pubkey =
        static_pubkey!("DtmE9D2CSB4L5D6A15mraeEjrGMm6auWVzgaD8hK2tZM");
    /// address of the lending market reserve account
    pub const RESERVE_ACCOUNT: Pubkey =
        static_pubkey!("8PbodeaosQP19SjYFx855UMqWxH2HynZLdBXmsrbac36");
    /// address of the reserve's token account which holders
    /// the underlying tokens
    pub const RESERVE_LIQUIDITY_ACCOUNT: Pubkey =
        static_pubkey!("8UviNr47S8eL6J3WfDxMRa3hvLta1VDJwNWqsDgtN3Cv");
    /// the address of the standalone vault's token account for the
    /// reserve's collateral token mint
    pub const COLLATERAL_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("4bKoFNye8HxwkK2jfq3XHaLmfcKWZtd1fAA6DsMYqx3f");

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
    pub const FARM_KEY: Farm = Farm::Lending { name: Lending::SOL };

    /// address of the standalone vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("5KrsibBDuJyifhFMnpKhicZxpdjse8o4M3viXgqyduoH");
    /// address of the standalone vault pda
    pub const PDA: Pubkey = static_pubkey!("3xk85iPpM4ni7WaiioCbLeq29KRXVVkx7myJgKHQ1v2S");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("DVumpNbQZNXvTpK8oY31DrQ9gNdK5hHRqhs7oApPF9Qo");
    /// address of the standalone vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("CbgZWaX2R2etnvb2xz86ehWf97ZaFWqGVSV3qLJAhNpV");
    /// address of the standalone vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("67FuierfDeYa1eru2rFh3EKxmrziYWaPS4uKnpW3fUoh");
    /// address of the standalone vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("DSgsM43L563zHaioGHAzAysVdT89EPLZyzAbBN83QyxT");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("So11111111111111111111111111111111111111112");
    /// address of the configuration data account used the v2 vaults program
    pub const CONFIG_DATA_ACCOUNT: Pubkey =
        static_pubkey!("Hv7bo9Y1jXRYs3gV9BJz8vYG3Gandw2NofPxrdKJ7ps5");
    /// address of the information account which stores configuration information for the config data account
    pub const INFORMATION_ACCOUNT: Pubkey =
        static_pubkey!("5j8x3giFz4fJfjKZAWELUP5DXHwjdn6xtch59od7EHxj");
    /// address of the program id this standalone vault deposits into
    pub const PROGRAM_ID: Pubkey = static_pubkey!("4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM");
    /// address of the reserves collateral token mint
    pub const COLLATERAL_MINT: Pubkey =
        static_pubkey!("H4Q3hDbuMUw8Bu72Ph8oV2xMQ7BFNbekpfQZKS2xF7jW");
    /// address of the lending market which owns the reserve
    pub const LENDING_MARKET_ACCOUNT: Pubkey =
        static_pubkey!("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym");
    /// address of the derived lending market authority
    pub const LENDING_MARKET_AUTHORITY: Pubkey =
        static_pubkey!("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s");
    /// address of the oracle that goes into the first element
    /// of the oracle keys array
    pub const PYTH_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("DQAcms41gjYzidRooXRE9GQM1jAauPXDcEpMbVh4FEc7");
    /// address of the program which owns the first oracle
    pub const PYTH_PROGRAM_ID: Pubkey =
        static_pubkey!("5JQ8Mhdp2wv3HWcfjq9Ts8kwzCAeBADFBDAgBznzRsE4");
    /// address of the lending market reserve account
    pub const RESERVE_ACCOUNT: Pubkey =
        static_pubkey!("FzbfXR7sopQL29Ubu312tkqWMxSre4dYSrFyYAjUYiC4");
    /// address of the reserve's token account which holders
    /// the underlying tokens
    pub const RESERVE_LIQUIDITY_ACCOUNT: Pubkey =
        static_pubkey!("CPs1jJ5XAjhcAJsmTToWksAiPEqoLwKMbb1Z83rzaaaU");
    /// the address of the standalone vault's token account for the
    /// reserve's collateral token mint
    pub const COLLATERAL_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("DrB1QrmkdrTN4BfrZUpKzHMG1h7MkBrnMwz5HRHDiiSo");

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
    pub const FARM_KEY: Farm = Farm::Lending { name: Lending::SOL };

    /// address of the standalone vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("4oJXs6Gx4Sj1YBY4DndMaB97RqUXzkW21t8YZGerEAvt");
    /// address of the standalone vault pda
    pub const PDA: Pubkey = static_pubkey!("BodRnr1TUSbzENAWEXr3QWbrPSFo1ahyWbsxZZd8TTP");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("4jpVgQaLL3aDPxo2q1NoaWVLp6TfcH6ZVeUMKKDFph22");
    /// address of the standalone vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("Dp4dE9n2DRNyLurBg9VP4xby9yUYLw28hqExopFe4rEd");
    /// address of the standalone vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("CGv35EfXdUxddZoWbKApZ7XRgHmyVt9XXydjT4ZfuCs6");
    /// address of the standalone vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("Aty8vpK1gHm5YoLJBSegwxpCY2QmoebFQXkUd3xq1f4w");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("So11111111111111111111111111111111111111112");
    /// address of the configuration data account used the v2 vaults program
    pub const CONFIG_DATA_ACCOUNT: Pubkey =
        static_pubkey!("6tGs1Yfejxo8pcyRiCovDkH5wvUAD5TsErpouQenPjfx");
    /// address of the information account which stores configuration information for the config data account
    pub const INFORMATION_ACCOUNT: Pubkey =
        static_pubkey!("CGfAEaUSv1soa8Z3XMNpdnstGwRM46c1E5WxuRkpBFRw");
    pub const PROGRAM_ID: Pubkey = static_pubkey!("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68");
    /// address of the mango cache
    pub const CACHE: Pubkey = static_pubkey!("EBDRoayCDDUvDgCimta45ajQeXbexv7aKqJubruqpyvu");
    /// address of the mango group account
    pub const GROUP: Pubkey = static_pubkey!("98pjRuQjK3qA6gXts96PqZT4Ze5QmnCmt3QYjhbUSPue");
    /// address of the mango signer pda
    pub const GROUP_SIGNER: Pubkey = static_pubkey!("9BVcYqEQxyccuwznvxXqDkSJFavvTyheiTYk231T1A8S");
    /// address of the mango group usdc token account
    pub const GROUP_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("AVn3JRGhifPCxjxZsU3tQuo4U4dTHizHzBDGW983tx47");
    /// address of the mango root bank
    pub const ROOT_BANK: Pubkey = static_pubkey!("7jH1uLmiB2zbHNe6juZZYjQCrvquakTwd3yMaQpeP8rR");
    /// address of the mango usdc node bank
    pub const NODE_BANK: Pubkey = static_pubkey!("2bqJYcA1A8gw4qJFjyE2G4akiUunpd9rP6QzfnxHqSqr");
    /// address of the standalone vault's mango account
    pub const OPTIMIZER_MANGO_ACCOUNT: Pubkey =
        static_pubkey!("7jH1uLmiB2zbHNe6juZZYjQCrvquakTwd3yMaQpeP8rR");

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
