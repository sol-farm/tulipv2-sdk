use anchor_lang::{
    prelude::*,
    solana_program::{pubkey::Pubkey, instruction::Instruction, system_program, sysvar},
};

use crate::config::lending::usdc;

#[derive(Clone, Debug, Default)]
pub struct WithdrawAddresses {
    pub authority: Pubkey,
    pub multi_vault: Pubkey,
    pub multi_vault_pda: Pubkey,
    pub withdraw_vault: Pubkey,
    pub withdraw_vault_pda: Pubkey,
    pub platform_information: Pubkey,
    pub platform_config_data: Pubkey,
    pub multi_burning_shares_token_account: Pubkey,
    pub withdraw_burning_shares_token_account: Pubkey,
    pub receiving_underlying_token_account: Pubkey,
    pub multi_underlying_withdraw_queue: Pubkey,
    pub multi_shares_mint: Pubkey,
    pub withdraw_shares_mint: Pubkey,
    pub withdraw_vault_underlying_deposit_queue: Pubkey,
    pub lending_program: Pubkey,
}

impl WithdrawAddresses {
    pub fn new(user: Pubkey, platform: &str) -> WithdrawAddresses {
        let vault = usdc::multi_deposit::ACCOUNT;
        let vault_pda = usdc::multi_deposit::PDA;
        let shares_mint = usdc::multi_deposit::SHARES_MINT;
        let underlying_mint = usdc::multi_deposit::UNDERLYING_MINT;
        let underlying_withdraw_queue = usdc::multi_deposit::UNDERLYING_WITHDRAW_QUEUE;

        let platform_config = match platform {
            "tulip" => [
                usdc::tulip::ACCOUNT,
                usdc::tulip::PDA,
                usdc::tulip::INFORMATION_ACCOUNT,
                usdc::tulip::CONFIG_DATA_ACCOUNT,
                usdc::tulip::SHARES_MINT,
                usdc::tulip::UNDERLYING_DEPOSIT_QUEUE,
                usdc::tulip::PROGRAM_ID,
            ],
            "solend" => [
                usdc::solend::ACCOUNT,
                usdc::solend::PDA,
                usdc::solend::INFORMATION_ACCOUNT,
                usdc::solend::CONFIG_DATA_ACCOUNT,
                usdc::solend::SHARES_MINT,
                usdc::solend::UNDERLYING_DEPOSIT_QUEUE,
                usdc::solend::PROGRAM_ID,
            ],
            "mango" => [
                usdc::mango::ACCOUNT,
                usdc::mango::PDA,
                usdc::mango::INFORMATION_ACCOUNT,
                usdc::mango::CONFIG_DATA_ACCOUNT,
                usdc::mango::SHARES_MINT,
                usdc::mango::UNDERLYING_DEPOSIT_QUEUE,
                usdc::mango::PROGRAM_ID,
            ],
            _ => {}
        };

        let multi_burning_shares_token_account = spl_associated_token_account::get_associated_token_address(
            &user,
            &shares_mint
        );

        let withdraw_burning_shares_token_account = spl_associated_token_account::get_associated_token_address(
            &vault_pda,
            &platform_config[4]
        );

        let receiving_underlying_token_account = spl_associated_token_account::get_associated_token_address(
            &user,
            &underlying_mint
        );

        WithdrawAddresses{
            authority: user,
            multi_vault: vault,
            multi_vault_pda: vault_pda,
            withdraw_vault: platform_config[0],
            withdraw_vault_pda: platform_config[1],
            platform_information: platform_config[2],
            platform_config_data:platform_config[3],
            multi_burning_shares_token_account,
            withdraw_burning_shares_token_account,
            receiving_underlying_token_account,
            multi_underlying_withdraw_queue: underlying_withdraw_queue,
            multi_shares_mint: shares_mint,
            withdraw_shares_mint: platform_config[4],
            withdraw_vault_underlying_deposit_queue: platform_config[5],
            lending_program: platform_config[6]
        }
    }

    pub fn get_tulip_remaining_accounts() -> [Pubkey; 7] {
        [
            usdc::tulip::COLLATERAL_TOKEN_ACCOUNT,
            usdc::tulip::RESERVE_ACCOUNT,
            usdc::tulip::RESERVE_LIQUIDITY_ACCOUNT,
            usdc::tulip::COLLATERAL_MINT,
            usdc::tulip::LENDING_MARKET_ACCOUNT,
            usdc::tulip::LENDING_MARKET_AUTHORITY,
            usdc::tulip::PYTH_PRICE_ACCOUNT,
        ]
    }

    pub fn get_solend_remaining_accounts() -> [Pubkey; 8] {
        [
            usdc::solend::COLLATERAL_TOKEN_ACCOUNT,
            usdc::solend::RESERVE_ACCOUNT,
            usdc::solend::RESERVE_LIQUIDITY_ACCOUNT,
            usdc::solend::COLLATERAL_MINT,
            usdc::solend::LENDING_MARKET_ACCOUNT,
            usdc::solend::LENDING_MARKET_AUTHORITY,
            usdc::solend::PYTH_PRICE_ACCOUNT,
            usdc::solend::SWITCHBOARD_PRICE_ACCOUNT,
        ]
    }

    pub fn get_mango_remaining_accounts() -> [Pubkey; 8] {
        [
            usdc::mango::GROUP,
            usdc::mango::OPTIMIZER_MANGO_ACCOUNT,
            usdc::mango::CACHE,
            usdc::mango::ROOT_BANK,
            usdc::mango::NODE_BANK,
            usdc::mango::GROUP_TOKEN_ACCOUNT,
            usdc::mango::GROUP_SIGNER,
            system_program::id(),
        ]
    }
}