//! tulip strategy vault configurations

use self::traits::{MultiVaultProgramConfig, StandaloneVaultProgramConfig};

pub mod traits;
pub mod withdraw;

#[cfg(feature = "usdc-optimizer")]
pub mod usdc;

#[cfg(feature = "sol-optimizer")]
pub mod sol;

#[cfg(feature = "ray-optimizer")]
pub mod ray;

#[cfg(feature = "usdt-optimizer")]
pub mod usdt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Platform {
    MangoV3,
    Tulip,
    Solend,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StrategyVaults {
    USDCv1,
    SOLv1,
    RAYv1,
    USDTv1,
}

impl StrategyVaults {
    /// returns the multi deposit vault program configuration trait for strategy vault `self`
    pub fn multi_deposit_config(&self) -> Box<dyn MultiVaultProgramConfig> {
        match self {
            Self::USDCv1 => {
                Box::new(usdc::multi_deposit::ProgramConfig) as Box<dyn MultiVaultProgramConfig>
            }
            Self::SOLv1 => {
                Box::new(sol::multi_deposit::ProgramConfig) as Box<dyn MultiVaultProgramConfig>
            }
            Self::RAYv1 => {
                Box::new(ray::multi_deposit::ProgramConfig) as Box<dyn MultiVaultProgramConfig>
            }
            Self::USDTv1 => {
                Box::new(usdt::multi_deposit::ProgramConfig) as Box<dyn MultiVaultProgramConfig>
            }
        }
    }
    /// returns the standalone vault program configuration for the standalone vault belonging to `platform`
    /// used by the strategy vault as indicated by `self`
    pub fn standalone_config(&self, platform: Platform) -> Box<dyn StandaloneVaultProgramConfig> {
        match self {
            Self::USDCv1 => self.multi_deposit_config().standalone_config(platform),
            Self::SOLv1 => self.multi_deposit_config().standalone_config(platform),
            Self::RAYv1 => self.multi_deposit_config().standalone_config(platform),
            Self::USDTv1 => self.multi_deposit_config().standalone_config(platform),
        }
    }
}

/// given address `vault`, return the corresponding multi deposit vault configuration trait.
/// 
/// returns None if the vault is not a strategy vault
pub fn get_multi_deposit_vault_config(vault: anchor_lang::solana_program::pubkey::Pubkey) -> Option<Box<dyn MultiVaultProgramConfig>> {
    match vault {
        usdc::multi_deposit::ACCOUNT => Some(StrategyVaults::USDCv1.multi_deposit_config()),
        sol::multi_deposit::ACCOUNT => Some(StrategyVaults::SOLv1.multi_deposit_config()),
        ray::multi_deposit::ACCOUNT => Some(StrategyVaults::RAYv1.multi_deposit_config()),
        usdt::multi_deposit::ACCOUNT => Some(StrategyVaults::USDTv1.multi_deposit_config()),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use tulipv2_sdk_farms::{lending::Lending, Farm};

    use super::*;
    #[test]
    fn test_sol_multi_deposit_config() {
        let conf = get_multi_deposit_vault_config(sol::multi_deposit::ACCOUNT).unwrap();


        assert_eq!(conf.account(), sol::multi_deposit::ACCOUNT);
        assert_eq!(conf.pda(), sol::multi_deposit::PDA);
        assert_eq!(conf.shares_mint(), sol::multi_deposit::SHARES_MINT);
        assert_eq!(
            conf.underlying_compound_queue(),
            sol::multi_deposit::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            conf.underlying_deposit_queue(),
            sol::multi_deposit::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            conf.underlying_withdraw_queue(),
            sol::multi_deposit::UNDERLYING_WITHDRAW_QUEUE
        );
        assert_eq!(conf.underlying_mint(), sol::multi_deposit::UNDERLYING_MINT);
        assert_eq!(
            conf.rebalance_state_transition(),
            sol::multi_deposit::REBALANCE_STATE_TRANSITION
        );
        assert_eq!(
            conf.rebalance_state_transition_underlying(),
            sol::multi_deposit::REBALANCE_STATE_TRANSITION_UNDERLYING
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::MangoV3),
            sol::multi_deposit::MANGO_OPTIMIZER_SHARES_ACCOUNT
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::Tulip),
            sol::multi_deposit::TULIP_OPTIMIZER_SHARES_ACCOUNT
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::Solend),
            sol::multi_deposit::SOLEND_OPTIMIZER_SHARES_ACCOUNT
        );

        // TODO: validate the issue_shares, etc... methods

        assert_eq!(
            conf.remaining_accounts(Platform::MangoV3),
            sol::multi_deposit::ProgramConfig::get_mango_remaining_accounts()
        );
        assert_eq!(
            conf.remaining_accounts(Platform::Tulip),
            sol::multi_deposit::ProgramConfig::get_tulip_remaining_accounts()
        );
        assert_eq!(
            conf.remaining_accounts(Platform::Solend),
            sol::multi_deposit::ProgramConfig::get_solend_remaining_accounts()
        );

        assert_eq!(conf.tag(), "solv1");
        assert_eq!(
            conf.farm(),
            Farm::Lending {
                name: Lending::MULTI_DEPOSIT
            }
        );

        let standalone_config = StrategyVaults::SOLv1.standalone_config(Platform::MangoV3);
        assert_eq!(standalone_config.account(), sol::mango::ACCOUNT);
        assert_eq!(standalone_config.pda(), sol::mango::PDA);
        assert_eq!(standalone_config.shares_mint(), sol::mango::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            sol::mango::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            sol::mango::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            sol::mango::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            sol::mango::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            sol::mango::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), sol::mango::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_some());
        assert!(standalone_config.tulip_config().is_none());
        assert!(standalone_config.solend_config().is_none());
        assert!(standalone_config.is_platform(Platform::MangoV3));
        assert_eq!(standalone_config.tag(), "mango");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending { name: Lending::SOL }
        );

        let standalone_config = StrategyVaults::SOLv1.standalone_config(Platform::Solend);
        assert_eq!(standalone_config.account(), sol::solend::ACCOUNT);
        assert_eq!(standalone_config.pda(), sol::solend::PDA);
        assert_eq!(standalone_config.shares_mint(), sol::solend::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            sol::solend::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            sol::solend::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            sol::solend::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            sol::solend::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            sol::solend::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), sol::solend::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_none());
        assert!(standalone_config.tulip_config().is_none());
        assert!(standalone_config.solend_config().is_some());
        assert!(standalone_config.is_platform(Platform::Solend));
        assert_eq!(standalone_config.tag(), "solend");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending { name: Lending::SOL }
        );

        let standalone_config = StrategyVaults::SOLv1.standalone_config(Platform::Tulip);
        assert_eq!(standalone_config.account(), sol::tulip::ACCOUNT);
        assert_eq!(standalone_config.pda(), sol::tulip::PDA);
        assert_eq!(standalone_config.shares_mint(), sol::tulip::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            sol::tulip::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            sol::tulip::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            sol::tulip::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            sol::tulip::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            sol::tulip::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), sol::tulip::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_none());
        assert!(standalone_config.tulip_config().is_some());
        assert!(standalone_config.solend_config().is_none());
        assert!(standalone_config.is_platform(Platform::Tulip));
        assert_eq!(standalone_config.tag(), "tulip");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending { name: Lending::SOL }
        );
    }
    #[test]
    fn test_ray_multi_deposit_config() {
        let conf = get_multi_deposit_vault_config(ray::multi_deposit::ACCOUNT).unwrap();

        assert_eq!(conf.account(), ray::multi_deposit::ACCOUNT);
        assert_eq!(conf.pda(), ray::multi_deposit::PDA);
        assert_eq!(conf.shares_mint(), ray::multi_deposit::SHARES_MINT);
        assert_eq!(
            conf.underlying_compound_queue(),
            ray::multi_deposit::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            conf.underlying_deposit_queue(),
            ray::multi_deposit::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            conf.underlying_withdraw_queue(),
            ray::multi_deposit::UNDERLYING_WITHDRAW_QUEUE
        );
        assert_eq!(conf.underlying_mint(), ray::multi_deposit::UNDERLYING_MINT);
        assert_eq!(
            conf.rebalance_state_transition(),
            ray::multi_deposit::REBALANCE_STATE_TRANSITION
        );
        assert_eq!(
            conf.rebalance_state_transition_underlying(),
            ray::multi_deposit::REBALANCE_STATE_TRANSITION_UNDERLYING
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::MangoV3),
            ray::multi_deposit::MANGO_OPTIMIZER_SHARES_ACCOUNT
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::Tulip),
            ray::multi_deposit::TULIP_OPTIMIZER_SHARES_ACCOUNT
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::Solend),
            ray::multi_deposit::SOLEND_OPTIMIZER_SHARES_ACCOUNT
        );

        // TODO: validate the issue_shares, etc... methods

        assert_eq!(
            conf.remaining_accounts(Platform::MangoV3),
            ray::multi_deposit::ProgramConfig::get_mango_remaining_accounts()
        );
        assert_eq!(
            conf.remaining_accounts(Platform::Tulip),
            ray::multi_deposit::ProgramConfig::get_tulip_remaining_accounts()
        );
        assert_eq!(
            conf.remaining_accounts(Platform::Solend),
            ray::multi_deposit::ProgramConfig::get_solend_remaining_accounts()
        );

        assert_eq!(conf.tag(), "rayv1");
        assert_eq!(
            conf.farm(),
            Farm::Lending {
                name: Lending::MULTI_DEPOSIT
            }
        );

        let standalone_config = StrategyVaults::RAYv1.standalone_config(Platform::MangoV3);
        assert_eq!(standalone_config.account(), ray::mango::ACCOUNT);
        assert_eq!(standalone_config.pda(), ray::mango::PDA);
        assert_eq!(standalone_config.shares_mint(), ray::mango::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            ray::mango::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            ray::mango::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            ray::mango::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            ray::mango::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            ray::mango::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), ray::mango::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_some());
        assert!(standalone_config.tulip_config().is_none());
        assert!(standalone_config.solend_config().is_none());
        assert!(standalone_config.is_platform(Platform::MangoV3));
        assert_eq!(standalone_config.tag(), "mango");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending { name: Lending::RAY }
        );

        let standalone_config = StrategyVaults::RAYv1.standalone_config(Platform::Solend);
        assert_eq!(standalone_config.account(), ray::solend::ACCOUNT);
        assert_eq!(standalone_config.pda(), ray::solend::PDA);
        assert_eq!(standalone_config.shares_mint(), ray::solend::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            ray::solend::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            ray::solend::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            ray::solend::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            ray::solend::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            ray::solend::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), ray::solend::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_none());
        assert!(standalone_config.tulip_config().is_none());
        assert!(standalone_config.solend_config().is_some());
        assert!(standalone_config.is_platform(Platform::Solend));
        assert_eq!(standalone_config.tag(), "solend");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending { name: Lending::RAY }
        );

        let standalone_config = StrategyVaults::RAYv1.standalone_config(Platform::Tulip);
        assert_eq!(standalone_config.account(), ray::tulip::ACCOUNT);
        assert_eq!(standalone_config.pda(), ray::tulip::PDA);
        assert_eq!(standalone_config.shares_mint(), ray::tulip::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            ray::tulip::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            ray::tulip::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            ray::tulip::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            ray::tulip::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            ray::tulip::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), ray::tulip::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_none());
        assert!(standalone_config.tulip_config().is_some());
        assert!(standalone_config.solend_config().is_none());
        assert!(standalone_config.is_platform(Platform::Tulip));
        assert_eq!(standalone_config.tag(), "tulip");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending { name: Lending::RAY }
        );
    }
    #[test]
    fn test_usdc_multi_deposit_config() {
        let conf = get_multi_deposit_vault_config(usdc::multi_deposit::ACCOUNT).unwrap();

        assert_eq!(conf.account(), usdc::multi_deposit::ACCOUNT);
        assert_eq!(conf.pda(), usdc::multi_deposit::PDA);
        assert_eq!(conf.shares_mint(), usdc::multi_deposit::SHARES_MINT);
        assert_eq!(
            conf.underlying_compound_queue(),
            usdc::multi_deposit::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            conf.underlying_deposit_queue(),
            usdc::multi_deposit::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            conf.underlying_withdraw_queue(),
            usdc::multi_deposit::UNDERLYING_WITHDRAW_QUEUE
        );
        assert_eq!(conf.underlying_mint(), usdc::multi_deposit::UNDERLYING_MINT);
        assert_eq!(
            conf.rebalance_state_transition(),
            usdc::multi_deposit::REBALANCE_STATE_TRANSITION
        );
        assert_eq!(
            conf.rebalance_state_transition_underlying(),
            usdc::multi_deposit::REBALANCE_STATE_TRANSITION_UNDERLYING
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::MangoV3),
            usdc::multi_deposit::MANGO_OPTIMIZER_SHARES_ACCOUNT
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::Tulip),
            usdc::multi_deposit::TULIP_OPTIMIZER_SHARES_ACCOUNT
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::Solend),
            usdc::multi_deposit::SOLEND_OPTIMIZER_SHARES_ACCOUNT
        );

        // TODO: validate the issue_shares, etc... methods

        assert_eq!(
            conf.remaining_accounts(Platform::MangoV3),
            usdc::multi_deposit::ProgramConfig::get_mango_remaining_accounts()
        );
        assert_eq!(
            conf.remaining_accounts(Platform::Tulip),
            usdc::multi_deposit::ProgramConfig::get_tulip_remaining_accounts()
        );
        assert_eq!(
            conf.remaining_accounts(Platform::Solend),
            usdc::multi_deposit::ProgramConfig::get_solend_remaining_accounts()
        );

        assert_eq!(conf.tag(), "usdcv1");
        assert_eq!(
            conf.farm(),
            Farm::Lending {
                name: Lending::MULTI_DEPOSIT
            }
        );

        let standalone_config = StrategyVaults::USDCv1.standalone_config(Platform::MangoV3);
        assert_eq!(standalone_config.account(), usdc::mango::ACCOUNT);
        assert_eq!(standalone_config.pda(), usdc::mango::PDA);
        assert_eq!(standalone_config.shares_mint(), usdc::mango::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            usdc::mango::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            usdc::mango::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            usdc::mango::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            usdc::mango::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            usdc::mango::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), usdc::mango::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_some());
        assert!(standalone_config.tulip_config().is_none());
        assert!(standalone_config.solend_config().is_none());
        assert!(standalone_config.is_platform(Platform::MangoV3));
        assert_eq!(standalone_config.tag(), "mango");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending {
                name: Lending::USDC
            }
        );

        let standalone_config = StrategyVaults::USDCv1.standalone_config(Platform::Solend);
        assert_eq!(standalone_config.account(), usdc::solend::ACCOUNT);
        assert_eq!(standalone_config.pda(), usdc::solend::PDA);
        assert_eq!(standalone_config.shares_mint(), usdc::solend::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            usdc::solend::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            usdc::solend::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            usdc::solend::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            usdc::solend::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            usdc::solend::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), usdc::solend::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_none());
        assert!(standalone_config.tulip_config().is_none());
        assert!(standalone_config.solend_config().is_some());
        assert!(standalone_config.is_platform(Platform::Solend));
        assert_eq!(standalone_config.tag(), "solend");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending {
                name: Lending::USDC
            }
        );

        let standalone_config = StrategyVaults::USDCv1.standalone_config(Platform::Tulip);
        assert_eq!(standalone_config.account(), usdc::tulip::ACCOUNT);
        assert_eq!(standalone_config.pda(), usdc::tulip::PDA);
        assert_eq!(standalone_config.shares_mint(), usdc::tulip::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            usdc::tulip::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            usdc::tulip::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            usdc::tulip::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            usdc::tulip::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            usdc::tulip::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), usdc::tulip::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_none());
        assert!(standalone_config.tulip_config().is_some());
        assert!(standalone_config.solend_config().is_none());
        assert!(standalone_config.is_platform(Platform::Tulip));
        assert_eq!(standalone_config.tag(), "tulip");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending {
                name: Lending::USDC
            }
        );
    }
    #[test]
    fn test_usdt_multi_deposit_config() {
        let conf = get_multi_deposit_vault_config(usdt::multi_deposit::ACCOUNT).unwrap();

        assert_eq!(conf.account(), usdt::multi_deposit::ACCOUNT);
        assert_eq!(conf.pda(), usdt::multi_deposit::PDA);
        assert_eq!(conf.shares_mint(), usdt::multi_deposit::SHARES_MINT);
        assert_eq!(
            conf.underlying_compound_queue(),
            usdt::multi_deposit::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            conf.underlying_deposit_queue(),
            usdt::multi_deposit::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            conf.underlying_withdraw_queue(),
            usdt::multi_deposit::UNDERLYING_WITHDRAW_QUEUE
        );
        assert_eq!(conf.underlying_mint(), usdt::multi_deposit::UNDERLYING_MINT);
        assert_eq!(
            conf.rebalance_state_transition(),
            usdt::multi_deposit::REBALANCE_STATE_TRANSITION
        );
        assert_eq!(
            conf.rebalance_state_transition_underlying(),
            usdt::multi_deposit::REBALANCE_STATE_TRANSITION_UNDERLYING
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::MangoV3),
            usdt::multi_deposit::MANGO_OPTIMIZER_SHARES_ACCOUNT
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::Tulip),
            usdt::multi_deposit::TULIP_OPTIMIZER_SHARES_ACCOUNT
        );
        assert_eq!(
            conf.optimizer_shares_account(Platform::Solend),
            usdt::multi_deposit::SOLEND_OPTIMIZER_SHARES_ACCOUNT
        );

        // TODO: validate the issue_shares, etc... methods

        assert_eq!(
            conf.remaining_accounts(Platform::MangoV3),
            usdt::multi_deposit::ProgramConfig::get_mango_remaining_accounts()
        );
        assert_eq!(
            conf.remaining_accounts(Platform::Tulip),
            usdt::multi_deposit::ProgramConfig::get_tulip_remaining_accounts()
        );
        assert_eq!(
            conf.remaining_accounts(Platform::Solend),
            usdt::multi_deposit::ProgramConfig::get_solend_remaining_accounts()
        );

        assert_eq!(conf.tag(), "usdtv1");
        assert_eq!(
            conf.farm(),
            Farm::Lending {
                name: Lending::MULTI_DEPOSIT
            }
        );

        let standalone_config = StrategyVaults::USDTv1.standalone_config(Platform::MangoV3);
        assert_eq!(standalone_config.account(), usdt::mango::ACCOUNT);
        assert_eq!(standalone_config.pda(), usdt::mango::PDA);
        assert_eq!(standalone_config.shares_mint(), usdt::mango::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            usdt::mango::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            usdt::mango::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            usdt::mango::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            usdt::mango::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            usdt::mango::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), usdt::mango::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_some());
        assert!(standalone_config.tulip_config().is_none());
        assert!(standalone_config.solend_config().is_none());
        assert!(standalone_config.is_platform(Platform::MangoV3));
        assert_eq!(standalone_config.tag(), "mango");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending {
                name: Lending::USDT
            }
        );

        let standalone_config = StrategyVaults::USDTv1.standalone_config(Platform::Solend);
        assert_eq!(standalone_config.account(), usdt::solend::ACCOUNT);
        assert_eq!(standalone_config.pda(), usdt::solend::PDA);
        assert_eq!(standalone_config.shares_mint(), usdt::solend::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            usdt::solend::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            usdt::solend::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            usdt::solend::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            usdt::solend::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            usdt::solend::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), usdt::solend::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_none());
        assert!(standalone_config.tulip_config().is_none());
        assert!(standalone_config.solend_config().is_some());
        assert!(standalone_config.is_platform(Platform::Solend));
        assert_eq!(standalone_config.tag(), "solend");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending {
                name: Lending::USDT
            }
        );

        let standalone_config = StrategyVaults::USDTv1.standalone_config(Platform::Tulip);
        assert_eq!(standalone_config.account(), usdt::tulip::ACCOUNT);
        assert_eq!(standalone_config.pda(), usdt::tulip::PDA);
        assert_eq!(standalone_config.shares_mint(), usdt::tulip::SHARES_MINT);
        assert_eq!(
            standalone_config.underlying_compound_queue(),
            usdt::tulip::UNDERLYING_COMPOUND_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_deposit_queue(),
            usdt::tulip::UNDERLYING_DEPOSIT_QUEUE
        );
        assert_eq!(
            standalone_config.underlying_mint(),
            usdt::tulip::UNDERLYING_MINT
        );
        assert_eq!(
            standalone_config.config_data_account(),
            usdt::tulip::CONFIG_DATA_ACCOUNT
        );
        assert_eq!(
            standalone_config.information_account(),
            usdt::tulip::INFORMATION_ACCOUNT
        );
        assert_eq!(standalone_config.program_id(), usdt::tulip::PROGRAM_ID);
        assert!(standalone_config.mango_config().is_none());
        assert!(standalone_config.tulip_config().is_some());
        assert!(standalone_config.solend_config().is_none());
        assert!(standalone_config.is_platform(Platform::Tulip));
        assert_eq!(standalone_config.tag(), "tulip");
        assert_eq!(
            standalone_config.farm(),
            Farm::Lending {
                name: Lending::USDT
            }
        );
    }
}
