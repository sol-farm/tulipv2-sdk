//! usdc lending optimizer configuration variables

use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use static_pubkey::static_pubkey;
use tulipv2_sdk_farms::{lending::Lending, Farm};

/// bundles configuration information for the usdc lending optimizer multi deposit vault
pub mod multi_deposit {
    use super::*;
    use crate::config::deposit_tracking::issue_shares::DepositAddresses;
    use crate::config::deposit_tracking::register::RegisterDepositTrackingAddresses;
    use crate::config::deposit_tracking::traits::{
        IssueShares, RegisterDepositTracking, WithdrawDepositTracking,
    };
    use crate::config::deposit_tracking::withdraw::WithdrawDepositTrackingAddresses;
    use crate::config::lending::traits::WithdrawMultiOptimizerVault;
    use crate::config::lending::withdraw::WithdrawAddresses;
    use crate::config::lending::Platform;

    /// empty struct used to implement the various traits used
    /// to interact with the usdt lending optimizer vault
    pub struct ProgramConfig;

    pub const TAG_STRING: &str = "usdcv1";
    pub const FARM_KEY: Farm = Farm::Lending {
        name: Lending::MULTI_DEPOSIT,
    };

    /// address of the multi deposit vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("3wPiV9inTGexMZjp6x5Amqwp2sRNtpSheG8Hbv2rgq8W");
    /// address of the multi deposit vault pda
    pub const PDA: Pubkey = static_pubkey!("14fdy6YXbhDgnVQz4VcgSGgUcZ35eE48SKDrfqF87NUP");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("Cvvh8nsKZet59nsDDo3orMa3rZnPWQhpgrMCVcRDRgip");
    /// address of the multi depsoit vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("FAr7Xouceyn9Ms7Egx4JUQryy3RQXuM27RVvCqH6X1o3");
    /// address of the multi deposit vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("36KtHLHxcGnrfEb2GLwPcbN9nHUkeoi3gd6rMQj8wwVj");
    /// address of the multi deposit vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("HLVcpKPkBJJJGTHTSaZcAixDppy4R65x1is3k8Q7qZpj");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    pub const REBALANCE_STATE_TRANSITION: Pubkey =
        static_pubkey!("3Vjgd77xSaAeBX9DmtZ2Rw7EwVCqvb8aoPvDD7Z75HXP");
    pub const REBALANCE_STATE_TRANSITION_UNDERLYING: Pubkey =
        static_pubkey!("BBAcBhNSvGpHd4FHh1XF1VGpGjfeUDeNxxww9TRKra7r");

    /// the address of the multi deposit vault's shares token account for the solend standalone vault
    pub const SOLEND_OPTIMIZER_SHARES_ACCOUNT: Pubkey =
        static_pubkey!("UxVNZzzx3xRxKFAuGjRQgbDaa7mirSkFEAu7qiYQ1h4");
    /// the address of the multi deposit vault's shares token account for the tulip standalone vault
    pub const TULIP_OPTIMIZER_SHARES_ACCOUNT: Pubkey =
        static_pubkey!("M7akLS7xYVhp68LnMkBBCemvqkGcCycQ3qp7e3SsKR1");
    /// the address of the multi deposit vault's shares token account for the mango standalone vault
    pub const MANGO_OPTIMIZER_SHARES_ACCOUNT: Pubkey =
        static_pubkey!("A9kM8NKf3v29F3DgRQ5Rw7TJoadFZZDfBGLRBGNzASrr");

    impl ProgramConfig {
        pub fn issue_shares_ix(user: Pubkey) -> impl IssueShares {
            DepositAddresses::new(user, ACCOUNT, PDA, SHARES_MINT, UNDERLYING_MINT)
        }
        pub fn register_deposit_tracking_ix(user: Pubkey) -> impl RegisterDepositTracking {
            RegisterDepositTrackingAddresses::new(user, ACCOUNT, SHARES_MINT, UNDERLYING_MINT)
        }
        pub fn withdraw_deposit_tracking(user: Pubkey) -> impl WithdrawDepositTracking {
            WithdrawDepositTrackingAddresses::new(user, ACCOUNT, SHARES_MINT)
        }
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
            Ok(WithdrawAddresses::new(
                user,
                ACCOUNT,
                PDA,
                SHARES_MINT,
                UNDERLYING_MINT,
                UNDERLYING_WITHDRAW_QUEUE,
                platform_config,
                (&standalone_config.0, standalone_config.1),
            )?)
        }
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
    use crate::config::lending::withdraw::PlatformConfigAddresses;

    use super::*;

    pub const TAG_STRING: &str = "solend";
    pub const FARM_KEY: Farm = Farm::Lending {
        name: Lending::USDC,
    };

    /// address of the standalone vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("85JXjDiyianDpvz8y8efkRyFsxpnSJJpmyxrJ7bncKHM");
    /// address of the standalone vault pda
    pub const PDA: Pubkey = static_pubkey!("963HGaUjwGqvqwwqwJZayUXvCC21AAqZU5SLw1kU4gVc");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("CS8cJicaSpphTTboMJD1UeNpU7vpkZc86vKrtqzRVnG5");
    /// address of the standalone vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("2KpoR9CquEuUTgVqFTtZi8nqrTA1YFsYmsHjp957YVcX");
    /// address of the standalone vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("2Li9Q5Vx9BEnFTGJTLWc5pVqerYGjhgyGYzSAA2WhYKQ");
    /// address of the standalone vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("BoGjVzrz2yhPRnFoj2QPPMLZTkptG7pESc3PyHDGV6WJ");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    /// address of the configuration data account used the v2 vaults program
    pub const CONFIG_DATA_ACCOUNT: Pubkey =
        static_pubkey!("6AzcPNmNWomkdMRgcZJPVAs4zF1jev9wqxzzzxVzDDsi");
    /// address of the information account which stores configuration information for the config data account
    pub const INFORMATION_ACCOUNT: Pubkey =
        static_pubkey!("GRL5rtnvzCfQRdKJXkG2A8LvDSNXkbxENEnF4SwJ3pTF");
    /// address of the program id this standalone vault deposits into
    pub const PROGRAM_ID: Pubkey = static_pubkey!("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");
    /// address of the reserves collateral token mint
    pub const COLLATERAL_MINT: Pubkey =
        static_pubkey!("993dVFL2uXWYeoXuEBFXR4BijeXdTv4s6BzsCjJZuwqk");
    /// address of the lending market which owns the reserve
    pub const LENDING_MARKET_ACCOUNT: Pubkey =
        static_pubkey!("4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY");
    /// address of the derived lending market authority
    pub const LENDING_MARKET_AUTHORITY: Pubkey =
        static_pubkey!("DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby");
    /// address of the oracle that goes into the first element
    /// of the oracle keys array
    pub const PYTH_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");
    /// address of the oracle that goes into the second element
    /// of the oracle keys array
    pub const SWITCHBOARD_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("CZx29wKMUxaJDq6aLVQTdViPL754tTR64NAgQBUGxxHb");
    /// address of the program which owns the first oracle
    pub const PYTH_PROGRAM_ID: Pubkey =
        static_pubkey!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
    /// address of the program which owns the second oracle
    pub const SWITCHBOARD_PROGRAM_ID: Pubkey =
        static_pubkey!("DtmE9D2CSB4L5D6A15mraeEjrGMm6auWVzgaD8hK2tZM");
    /// address of the lending market reserve account
    pub const RESERVE_ACCOUNT: Pubkey =
        static_pubkey!("BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw");
    /// address of the reserve's token account which holders
    /// the underlying tokens
    pub const RESERVE_LIQUIDITY_ACCOUNT: Pubkey =
        static_pubkey!("8SheGtsopRUDzdiD6v6BR9a6bqZ9QwywYQY99Fp5meNf");
    /// the address of the standalone vault's token account for the
    /// reserve's collateral token mint
    pub const COLLATERAL_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("6EaiG2gRVu9u7QzVmX59AWLSmiaEYvMrKWQfPMCgNxsZ");

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
    use crate::config::lending::withdraw::PlatformConfigAddresses;

    use super::*;

    pub const TAG_STRING: &str = "tulip";
    pub const FARM_KEY: Farm = Farm::Lending {
        name: Lending::USDC,
    };

    /// address of the standalone vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("8KLrrsnUv3DjC9Q89xSQDVdiGLZHUEUuyPedfHrtuVRr");
    /// address of the standalone vault pda
    pub const PDA: Pubkey = static_pubkey!("mrT9Q45iuC2HLYxpceaQFjzcAgd6Trks7bXAGbKYpwL");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("D2PLcwGR1wsXUPhb1BHysSVEsHVVCQ129qGpgXXaTNR1");
    /// address of the standalone vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("E9XiAr49FzCGeLHwLjz5H1AiPKWNodizA5e8So7zLmjz");
    /// address of the standalone vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("8eDHmS15CWd8d88uckg6oKxrfwijZVudZsDgdtgGqS49");
    /// address of the standalone vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("AMdJL2UzEpd54a7wDqq6aV99ix6iXMHMmvs1w8CRs3Gc");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    /// address of the configuration data account used the v2 vaults program
    pub const CONFIG_DATA_ACCOUNT: Pubkey =
        static_pubkey!("7XTtoiHfkYzjLDxKDMoaVYncmdBW1yLfphmisSbBrnuZ");
    /// address of the information account which stores configuration information for the config data account
    pub const INFORMATION_ACCOUNT: Pubkey =
        static_pubkey!("4QVuedVSCMLA3eQ643czUy95uQFxXKAcCMJ1ChpkVA2B");
    /// address of the program id this standalone vault deposits into
    pub const PROGRAM_ID: Pubkey = static_pubkey!("4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM");
    /// address of the reserves collateral token mint
    pub const COLLATERAL_MINT: Pubkey =
        static_pubkey!("Amig8TisuLpzun8XyGfC5HJHHGUQEscjLgoTWsCCKihg");
    /// address of the lending market which owns the reserve
    pub const LENDING_MARKET_ACCOUNT: Pubkey =
        static_pubkey!("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym");
    /// address of the derived lending market authority
    pub const LENDING_MARKET_AUTHORITY: Pubkey =
        static_pubkey!("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s");
    /// address of the oracle that goes into the first element
    /// of the oracle keys array
    pub const PYTH_PRICE_ACCOUNT: Pubkey =
        static_pubkey!("ExzpbWgczTgd8J58BrnESndmzBkRVfc6PhFjSGiQXgAB");
    /// address of the program which owns the first oracle
    pub const PYTH_PROGRAM_ID: Pubkey =
        static_pubkey!("5JQ8Mhdp2wv3HWcfjq9Ts8kwzCAeBADFBDAgBznzRsE4");
    /// address of the lending market reserve account
    pub const RESERVE_ACCOUNT: Pubkey =
        static_pubkey!("FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt");
    /// address of the reserve's token account which holders
    /// the underlying tokens
    pub const RESERVE_LIQUIDITY_ACCOUNT: Pubkey =
        static_pubkey!("64QJd6MYXUjCBvCaZKaqxiKmaMkPUdNonE1KuY1YoGGb");
    /// the address of the standalone vault's token account for the
    /// reserve's collateral token mint
    pub const COLLATERAL_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("2U6kk4iTVqeypBydVPKA8mLTLAQEBfWf4KYfmkcvomPE");

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
    use crate::config::lending::withdraw::PlatformConfigAddresses;

    use super::*;

    pub const TAG_STRING: &str = "mango";
    pub const FARM_KEY: Farm = Farm::Lending {
        name: Lending::USDC,
    };

    /// address of the standalone vault itself
    pub const ACCOUNT: Pubkey = static_pubkey!("ZH9GWNBtwxcWeU9kHk77DSciwQnoJcSm8VVvYfmHXfe");
    /// address of the standalone vault pda
    pub const PDA: Pubkey = static_pubkey!("Dhry4TVd862yzcAuxFZgiuh6juS4R6FesfRZkWG3pCe7");
    /// address of the shares mint
    pub const SHARES_MINT: Pubkey = static_pubkey!("5u6jxB7En2LF5aroeq8U5JUbnHa5WSYB5JTemh3gYaMj");
    /// address of the standalone vaults underlying asset compound queue
    pub const UNDERLYING_COMPOUND_QUEUE: Pubkey =
        static_pubkey!("D8DwEfi3sqr2MDaFzT2bsWHi4Sqy9gokXTkGgLqJ4imW");
    /// address of the standalone vaults underlying asset deposit queue
    pub const UNDERLYING_DEPOSIT_QUEUE: Pubkey =
        static_pubkey!("3CnAyCjpA9mcxayed12cwJNGue7YbmyuBjpT9KN6meVT");
    /// address of the standalone vaults underlying asset withdraw queue
    pub const UNDERLYING_WITHDRAW_QUEUE: Pubkey =
        static_pubkey!("GFnGoQmW8HZzGpUVKPt7gSY8itvmeaTbAXJVYFTpbCTd");
    pub const UNDERLYING_MINT: Pubkey =
        static_pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    /// address of the configuration data account used the v2 vaults program
    pub const CONFIG_DATA_ACCOUNT: Pubkey =
        static_pubkey!("EecDX1xHrjKXQWbE5WtwU7fAiEkKxAv7w196oe8jaoqY");
    /// address of the information account which stores configuration information for the config data account
    pub const INFORMATION_ACCOUNT: Pubkey =
        static_pubkey!("GDzbqzebKTxJaQVfxfqejkqU4HLcBmgNBwun3rADvm8J");
    pub const PROGRAM_ID: Pubkey = static_pubkey!("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68");
    /// address of the mango cache
    pub const CACHE: Pubkey = static_pubkey!("EBDRoayCDDUvDgCimta45ajQeXbexv7aKqJubruqpyvu");
    /// address of the mango group account
    pub const GROUP: Pubkey = static_pubkey!("98pjRuQjK3qA6gXts96PqZT4Ze5QmnCmt3QYjhbUSPue");
    /// address of the mango signer pda
    pub const GROUP_SIGNER: Pubkey = static_pubkey!("9BVcYqEQxyccuwznvxXqDkSJFavvTyheiTYk231T1A8S");
    /// address of the mango group usdc token account
    pub const GROUP_TOKEN_ACCOUNT: Pubkey =
        static_pubkey!("8Vw25ZackDzaJzzBBqcgcpDsCsDfRSkMGgwFQ3gbReWF");
    /// address of the mango root bank
    pub const ROOT_BANK: Pubkey = static_pubkey!("AMzanZxMirPCgGcBoH9kw4Jzi9LFMomyUCXbpzDeL2T8");
    /// address of the mango usdc node bank
    pub const NODE_BANK: Pubkey = static_pubkey!("BGcwkj1WudQwUUjFk78hAjwd1uAm8trh1N4CJSa51euh");
    /// address of the standalone vault's mango account
    pub const OPTIMIZER_MANGO_ACCOUNT: Pubkey =
        static_pubkey!("3cZkd5eVyZhMhE8nJcR3rA7GgVQ6gCJt2qofr2GQd8ca");

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
