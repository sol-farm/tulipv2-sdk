//! tulip strategy vault configurations

use self::traits::MultiVaultProgramConfig;

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


#[derive(Clone, Copy)]
pub enum StrategyVaults {
    USDCv1,
    SOLv1,
    RAYv1,
    USDTv1,
}

impl StrategyVaults {
    pub fn multi_deposit_config(&self) -> Box<dyn MultiVaultProgramConfig> {
        match self {
            Self::USDCv1 => Box::new(usdc::multi_deposit::ProgramConfig) as Box<dyn MultiVaultProgramConfig>,
            Self::SOLv1 => Box::new(sol::multi_deposit::ProgramConfig) as Box<dyn MultiVaultProgramConfig>,
            Self::RAYv1 => Box::new(ray::multi_deposit::ProgramConfig) as Box<dyn MultiVaultProgramConfig>,
            Self::USDTv1 => Box::new(usdt::multi_deposit::ProgramConfig) as Box<dyn MultiVaultProgramConfig>,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sol_multi_deposit_config() {
        let conf = StrategyVaults::SOLv1.multi_deposit_config();   

        assert_eq!(conf.account(), sol::multi_deposit::ACCOUNT);
        assert_eq!(conf.pda(), sol::multi_deposit::PDA);
        assert_eq!(conf.shares_mint(), sol::multi_deposit::SHARES_MINT);
        assert_eq!(conf.underlying_compound_queue(), sol::multi_deposit::UNDERLYING_COMPOUND_QUEUE);
        assert_eq!(conf.underlying_deposit_queue(), sol::multi_deposit::UNDERLYING_DEPOSIT_QUEUE);
        assert_eq!(conf.underlying_withdraw_queue(), sol::multi_deposit::UNDERLYING_WITHDRAW_QUEUE);
        assert_eq!(conf.underlying_mint(), sol::multi_deposit::UNDERLYING_MINT);
        assert_eq!(conf.rebalance_state_transition(), sol::multi_deposit::REBALANCE_STATE_TRANSITION);
        assert_eq!(conf.rebalance_state_transition_underlying(), sol::multi_deposit::REBALANCE_STATE_TRANSITION_UNDERLYING);
        assert_eq!(conf.optimizer_shares_account(Platform::MangoV3), sol::multi_deposit::MANGO_OPTIMIZER_SHARES_ACCOUNT);
        assert_eq!(conf.optimizer_shares_account(Platform::Tulip), sol::multi_deposit::TULIP_OPTIMIZER_SHARES_ACCOUNT);
        assert_eq!(conf.optimizer_shares_account(Platform::Solend), sol::multi_deposit::SOLEND_OPTIMIZER_SHARES_ACCOUNT);

        // TODO: validate the issue_shares, etc... methods

        assert_eq!(conf.remaining_accounts(Platform::MangoV3), sol::multi_deposit::ProgramConfig::get_mango_remaining_accounts());
        assert_eq!(conf.remaining_accounts(Platform::Tulip), sol::multi_deposit::ProgramConfig::get_tulip_remaining_accounts());
        assert_eq!(conf.remaining_accounts(Platform::Solend), sol::multi_deposit::ProgramConfig::get_solend_remaining_accounts());
    }
    #[test]
    fn test_ray_multi_deposit_config() {
        let conf = StrategyVaults::RAYv1.multi_deposit_config();   

        assert_eq!(conf.account(), ray::multi_deposit::ACCOUNT);
        assert_eq!(conf.pda(), ray::multi_deposit::PDA);
        assert_eq!(conf.shares_mint(), ray::multi_deposit::SHARES_MINT);
        assert_eq!(conf.underlying_compound_queue(), ray::multi_deposit::UNDERLYING_COMPOUND_QUEUE);
        assert_eq!(conf.underlying_deposit_queue(), ray::multi_deposit::UNDERLYING_DEPOSIT_QUEUE);
        assert_eq!(conf.underlying_withdraw_queue(), ray::multi_deposit::UNDERLYING_WITHDRAW_QUEUE);
        assert_eq!(conf.underlying_mint(), ray::multi_deposit::UNDERLYING_MINT);
        assert_eq!(conf.rebalance_state_transition(), ray::multi_deposit::REBALANCE_STATE_TRANSITION);
        assert_eq!(conf.rebalance_state_transition_underlying(), ray::multi_deposit::REBALANCE_STATE_TRANSITION_UNDERLYING);
        assert_eq!(conf.optimizer_shares_account(Platform::MangoV3), ray::multi_deposit::MANGO_OPTIMIZER_SHARES_ACCOUNT);
        assert_eq!(conf.optimizer_shares_account(Platform::Tulip), ray::multi_deposit::TULIP_OPTIMIZER_SHARES_ACCOUNT);
        assert_eq!(conf.optimizer_shares_account(Platform::Solend), ray::multi_deposit::SOLEND_OPTIMIZER_SHARES_ACCOUNT);

        // TODO: validate the issue_shares, etc... methods

        assert_eq!(conf.remaining_accounts(Platform::MangoV3), ray::multi_deposit::ProgramConfig::get_mango_remaining_accounts());
        assert_eq!(conf.remaining_accounts(Platform::Tulip), ray::multi_deposit::ProgramConfig::get_tulip_remaining_accounts());
        assert_eq!(conf.remaining_accounts(Platform::Solend), ray::multi_deposit::ProgramConfig::get_solend_remaining_accounts());
    }
    #[test]
    fn test_usdc_multi_deposit_config() {
        let conf = StrategyVaults::USDCv1.multi_deposit_config();   

        assert_eq!(conf.account(), usdc::multi_deposit::ACCOUNT);
        assert_eq!(conf.pda(), usdc::multi_deposit::PDA);
        assert_eq!(conf.shares_mint(), usdc::multi_deposit::SHARES_MINT);
        assert_eq!(conf.underlying_compound_queue(), usdc::multi_deposit::UNDERLYING_COMPOUND_QUEUE);
        assert_eq!(conf.underlying_deposit_queue(), usdc::multi_deposit::UNDERLYING_DEPOSIT_QUEUE);
        assert_eq!(conf.underlying_withdraw_queue(), usdc::multi_deposit::UNDERLYING_WITHDRAW_QUEUE);
        assert_eq!(conf.underlying_mint(), usdc::multi_deposit::UNDERLYING_MINT);
        assert_eq!(conf.rebalance_state_transition(), usdc::multi_deposit::REBALANCE_STATE_TRANSITION);
        assert_eq!(conf.rebalance_state_transition_underlying(), usdc::multi_deposit::REBALANCE_STATE_TRANSITION_UNDERLYING);
        assert_eq!(conf.optimizer_shares_account(Platform::MangoV3), usdc::multi_deposit::MANGO_OPTIMIZER_SHARES_ACCOUNT);
        assert_eq!(conf.optimizer_shares_account(Platform::Tulip), usdc::multi_deposit::TULIP_OPTIMIZER_SHARES_ACCOUNT);
        assert_eq!(conf.optimizer_shares_account(Platform::Solend), usdc::multi_deposit::SOLEND_OPTIMIZER_SHARES_ACCOUNT);

        // TODO: validate the issue_shares, etc... methods

        assert_eq!(conf.remaining_accounts(Platform::MangoV3), usdc::multi_deposit::ProgramConfig::get_mango_remaining_accounts());
        assert_eq!(conf.remaining_accounts(Platform::Tulip), usdc::multi_deposit::ProgramConfig::get_tulip_remaining_accounts());
        assert_eq!(conf.remaining_accounts(Platform::Solend), usdc::multi_deposit::ProgramConfig::get_solend_remaining_accounts());
    }
    #[test]
    fn test_usdt_multi_deposit_config() {
        let conf = StrategyVaults::USDTv1.multi_deposit_config();   

        assert_eq!(conf.account(), usdt::multi_deposit::ACCOUNT);
        assert_eq!(conf.pda(), usdt::multi_deposit::PDA);
        assert_eq!(conf.shares_mint(), usdt::multi_deposit::SHARES_MINT);
        assert_eq!(conf.underlying_compound_queue(), usdt::multi_deposit::UNDERLYING_COMPOUND_QUEUE);
        assert_eq!(conf.underlying_deposit_queue(), usdt::multi_deposit::UNDERLYING_DEPOSIT_QUEUE);
        assert_eq!(conf.underlying_withdraw_queue(), usdt::multi_deposit::UNDERLYING_WITHDRAW_QUEUE);
        assert_eq!(conf.underlying_mint(), usdt::multi_deposit::UNDERLYING_MINT);
        assert_eq!(conf.rebalance_state_transition(), usdt::multi_deposit::REBALANCE_STATE_TRANSITION);
        assert_eq!(conf.rebalance_state_transition_underlying(), usdt::multi_deposit::REBALANCE_STATE_TRANSITION_UNDERLYING);
        assert_eq!(conf.optimizer_shares_account(Platform::MangoV3), usdt::multi_deposit::MANGO_OPTIMIZER_SHARES_ACCOUNT);
        assert_eq!(conf.optimizer_shares_account(Platform::Tulip), usdt::multi_deposit::TULIP_OPTIMIZER_SHARES_ACCOUNT);
        assert_eq!(conf.optimizer_shares_account(Platform::Solend), usdt::multi_deposit::SOLEND_OPTIMIZER_SHARES_ACCOUNT);

        // TODO: validate the issue_shares, etc... methods

        assert_eq!(conf.remaining_accounts(Platform::MangoV3), usdt::multi_deposit::ProgramConfig::get_mango_remaining_accounts());
        assert_eq!(conf.remaining_accounts(Platform::Tulip), usdt::multi_deposit::ProgramConfig::get_tulip_remaining_accounts());
        assert_eq!(conf.remaining_accounts(Platform::Solend), usdt::multi_deposit::ProgramConfig::get_solend_remaining_accounts());
    }
}