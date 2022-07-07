use crate::{accounts::{Farms, derivations::{derive_user_farm_address, derive_user_farm_obligation_vault_address, derive_user_farm_obligation_address}}, instructions::{deposit_borrow_dual, deposit_raydium_vault, swap_tokens_raydium_stats, add_liquidity_stats, withdraw_raydium_vault_close, orca_add_liquidity_queue, withdraw_orca_vault}};

use super::*;
use anchor_lang::prelude::*;
use solana_program::{instruction::Instruction, sysvar, system_program};
use tulipv2_sdk_common::{config::levfarm::{LevFarmConfig, LENDING_PROGRAM}, DEFAULT_KEY};
use crate::instructions::{create_user_farm, create_user_farm_obligation};

/// returns an instruction that can be used to create a user farm account, and initialize
/// the first obligation/position
pub fn new_create_user_farm_ix(
    authority: Pubkey,
    farm: Farms,
) -> Option<Instruction> {
    msg!("farm {:#?}", farm);
    let levfarm_config = lev_farm_config(farm)?;
    let user_farm_address = derive_user_farm_address(
        authority,
        crate::ID,
        0,
        farm,
    ).0;
    let obligation_vault_address = derive_user_farm_obligation_vault_address(
        user_farm_address,
        crate::ID,
        0,
    ).0;
    let user_farm_obligation = derive_user_farm_obligation_address(
        authority,
        user_farm_address,
        crate::ID,
        0,
    ).0;
    create_user_farm::create_user_farm(create_user_farm::CreateUserFarm {
        authority,
        user_farm: user_farm_address,
        lending_market: levfarm_config.lending_market,
        global: levfarm_config.global,
        leveraged_farm:  levfarm_config.account,
        clock: sysvar::clock::id(),
        rent: sysvar::rent::id(),
        system_program: system_program::id(),
        lending_program: LENDING_PROGRAM,
        token_program: spl_token::id(),
        obligation_vault_address,
        user_farm_obligation,
    }, levfarm_config.solfarm_vault_program)

}

/// returns an instruction that can be used to create additional positions
/// withing a user farm, up to a maximum of 2 additional
pub fn new_create_user_farm_obligation_ix(
    authority: Pubkey,
    user_farm_address: Pubkey,
    farm: Farms,
    obligation_index: u64,
) -> Option<Instruction> {
    let levfarm_config = lev_farm_config(farm)?;
    let obligation_vault_address = derive_user_farm_obligation_vault_address(
        user_farm_address,
        crate::ID,
        obligation_index as u8,
    ).0;
    let user_farm_obligation = derive_user_farm_obligation_address(
        authority,
        user_farm_address,
        crate::ID,
        obligation_index as u8,
    ).0;
    create_user_farm_obligation::create_user_farm_obligation(create_user_farm_obligation::CreateUserFarmObligation {
        authority,
        user_farm: user_farm_address,
        lending_market: levfarm_config.lending_market,
        leveraged_farm:  levfarm_config.account,
        clock: sysvar::clock::id(),
        rent: sysvar::rent::id(),
        system_program: system_program::id(),
        lending_program: LENDING_PROGRAM,
        token_program: spl_token::id(),
        obligation_vault_address,
        user_farm_obligation,
    })
}


pub fn new_deposit_borrow_dual_ix(
    accounts: deposit_borrow_dual::DepositBorrowDual,
    position_info_account: Pubkey,
    system_program: Pubkey,
    coin_amount: u64,
    pc_amount: u64,
    coin_borrow_amount: u64,
    pc_borrow_amount: u64,
    obligation_index: u8,
) -> Option<Instruction> {
    deposit_borrow_dual::deposit_borrow_dual(
        accounts,
        position_info_account,
        system_program,
        coin_amount,
        pc_amount,
        coin_borrow_amount,
        pc_borrow_amount,
        obligation_index,
    )
}

pub fn new_swap_tokens_raydium_stats_ix(
    accounts: Box<swap_tokens_raydium_stats::RaydiumSwap>,
    lending_market: Pubkey,
    lending_market_authority: Pubkey,
    lending_program: Pubkey,
    position_info_account: Pubkey,
    obligation_index: u8,
) -> Option<Instruction> {
    swap_tokens_raydium_stats::swap_tokens_raydium_stats(
        accounts,
        lending_market,
        lending_market_authority,
        lending_program,
        position_info_account,
        obligation_index,
    )
}
pub fn new_add_liquidity_stats_ix(
    accounts: Box<add_liquidity_stats::AddLiquidity>,
    position_info_account: Pubkey,
    obligation_index: u8
) -> Option<Instruction> {
    add_liquidity_stats::add_liquidity_stats(
        accounts,
        position_info_account,
        obligation_index
    )
}

#[inline(always)]
pub fn new_deposit_raydium_vault_ix(
    accounts: Box<deposit_raydium_vault::DepositFarm>,
    lending_market: Pubkey,
    user_farm_obligation: Pubkey,
    lending_market_authority: Pubkey,
    lending_program: Pubkey,
    obligation_index: u64,
    farm: Farms,
) -> Option<Instruction> {
    let (user_balance_account, balance_account_nonce) = Pubkey::find_program_address(
        &[
            vault_info_account(farm).unwrap().as_ref(),
            accounts.obligation_vault_address.as_ref(),
        ],
        &accounts.vault_program
    );
    let (_, balanace_metadata_nonce) = Pubkey::find_program_address(
        &[
            user_balance_account.as_ref(),
            accounts.obligation_vault_address.as_ref(),
        ],
        &accounts.vault_program,
    );
    deposit_raydium_vault::deposit_vault(
        accounts,
        lending_market,
        user_farm_obligation,
        lending_market_authority,
        lending_program,
        balance_account_nonce,
        balanace_metadata_nonce,
        obligation_index
    )
}

pub fn new_withdraw_raydium_vault_ix(
    accounts: Box<withdraw_raydium_vault_close::WithdrawFarm>,
    lending_market_account: Pubkey,
    user_farm_obligation: Pubkey,
    lending_market_authority: Pubkey,
    lending_program: Pubkey,
    position_info_account: Pubkey,
    system_program: Pubkey,
    rent: Pubkey,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
    farm: Farms,
) -> Option<Instruction> {
    let (user_balance_account, balance_account_nonce) = Pubkey::find_program_address(
        &[
            vault_info_account(farm).unwrap().as_ref(),
            accounts.obligation_vault_address.as_ref(),
        ],
        &accounts.vault_program
    );
    let (_, balanace_metadata_nonce) = Pubkey::find_program_address(
        &[
            user_balance_account.as_ref(),
            accounts.obligation_vault_address.as_ref(),
        ],
        &accounts.vault_program,
    );
    withdraw_raydium_vault_close::withdraw_raydium_vault_close(
        accounts,
        lending_market_account,
        user_farm_obligation,
        lending_market_authority,
        lending_program,
        position_info_account,
        system_program,
        rent,
        balanace_metadata_nonce,
        balance_account_nonce,
        obligation_index,
        withdraw_percent,
        close_method,
    )   
}

pub fn new_orca_add_liquidity_queue_ix(
    accounts: Box<orca_add_liquidity_queue::OrcaAddLiquidityQueue>,
    position_info_account: Pubkey,
    obligation_index: u8,
) -> Option<Instruction> {
    let orca_user_nonce = crate::accounts::derivations::derive_orca_vault_user_address(
        &accounts.solfarm_vault_program,
        &accounts.vault_account,
        &accounts.authority,
    ).1;
    orca_add_liquidity_queue::orca_add_liquidity_queue(
        accounts,
        position_info_account,
        orca_user_nonce,
        obligation_index,
    )
}
pub fn new_withdraw_orca_vault_close_ix<'info>(
    accounts: Box<withdraw_orca_vault::WithdrawOrcaFarm>,
    lending_market: Pubkey,
    user_farm_obligation: Pubkey,
    lending_market_authority: Pubkey,
    lending_program: Pubkey,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8
) -> Option<Instruction> {
    withdraw_orca_vault::withdraw_orca_vault_close(
        accounts,
        lending_market,
        user_farm_obligation,
        lending_market_authority,
        lending_program,
        obligation_index,
        withdraw_percent,
        close_method
    )
}
pub fn new_withdraw_orca_vault_without_shares_ix<'info>(
    accounts: Box<withdraw_orca_vault::WithdrawOrcaFarm>,
    obligation_index: u8,
) -> Option<Instruction> {
    withdraw_orca_vault::withdraw_orca_vault_without_shares(
        accounts,
        obligation_index,
    )
}
pub fn lev_farm_config(farm: Farms) -> Option<LevFarmConfig> {
    match farm {
        Farms::RayUsdcVault => {
            #[cfg(not(feature = "ray-rayusdc-levfarm"))]
            unimplemented!("requires rayusdc-levfarm feature to be activated");
            #[cfg(feature = "ray-rayusdc-levfarm")]
            return Some(tulipv2_sdk_common::config::levfarm::ray_rayusdc::get_lev_farm_config())
        }
        Farms::RaySrmVault => {
            #[cfg(not(feature = "ray-raysrm-levfarm"))]
            unimplemented!("requires raysrm-levfarm feature to be activated");
            #[cfg(feature = "ray-raysrm-levfarm")]
            return Some(tulipv2_sdk_common::config::levfarm::ray_rayusdc::get_lev_farm_config())
        }
        Farms::RayUsdtVault => {
            #[cfg(not(feature = "ray-rayusdt-levfarm"))]
            unimplemented!("requires rayusdt-levfarm feature to be activated");
            #[cfg(feature = "ray-rayusdt-levfarm")]
            return Some(tulipv2_sdk_common::config::levfarm::ray_rayusdt::get_lev_farm_config())
        }
        Farms::OrcaUsdcVault => {
            #[cfg(not(feature = "orca-orcausdc-levfarm"))]
            unimplemented!("requires orcausdc-levfarm feature to be activated");
            #[cfg(feature = "orca-orcausdc-levfarm")]
            return Some(tulipv2_sdk_common::config::levfarm::orca_orcausdc::get_lev_farm_config())
        }
        _ => None
    }
}

pub fn vault_info_account(farm: Farms) -> Option<Pubkey> {
    match farm {
        Farms::RayUsdcVault => {
            #[cfg(not(feature = "ray-rayusdc-levfarm"))]
            unimplemented!("requires rayusdc-levfarm feature to be activated");
            #[cfg(feature = "ray-rayusdc-levfarm")]
            if tulipv2_sdk_common::config::levfarm::ray_rayusdc::vault_config::OLD_VAULT_INFO_ACCOUNT.eq(&DEFAULT_KEY) {
                Some(tulipv2_sdk_common::config::levfarm::ray_rayusdc::vault_config::VAULT_INFO_ACCOUNT)
            } else {
                Some(tulipv2_sdk_common::config::levfarm::ray_rayusdc::vault_config::OLD_VAULT_INFO_ACCOUNT)
            }
        }
        Farms::RaySrmVault => {
            #[cfg(not(feature = "ray-raysrm-levfarm"))]
            unimplemented!("requires raysrm-levfarm feature to be activated");
            #[cfg(feature = "ray-raysrm-levfarm")]
            if tulipv2_sdk_common::config::levfarm::ray_raysrm::vault_config::OLD_VAULT_INFO_ACCOUNT.eq(&DEFAULT_KEY) {
                Some(tulipv2_sdk_common::config::levfarm::ray_raysrm::vault_config::VAULT_INFO_ACCOUNT)
            } else {
                Some(tulipv2_sdk_common::config::levfarm::ray_raysrm::vault_config::OLD_VAULT_INFO_ACCOUNT)
            }
        }
        Farms::RayUsdtVault => {
            #[cfg(not(feature = "ray-rayusdt-levfarm"))]
            unimplemented!("requires rayusdt-levfarm feature to be activated");
            #[cfg(feature = "ray-rayusdt-levfarm")]
            if tulipv2_sdk_common::config::levfarm::ray_rayusdt::vault_config::OLD_VAULT_INFO_ACCOUNT.eq(&DEFAULT_KEY) {
                Some(tulipv2_sdk_common::config::levfarm::ray_rayusdt::vault_config::VAULT_INFO_ACCOUNT)
            } else {
                Some(tulipv2_sdk_common::config::levfarm::ray_rayusdt::vault_config::OLD_VAULT_INFO_ACCOUNT)
            }
        }
        _ => None
    }
}