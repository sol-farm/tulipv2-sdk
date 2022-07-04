use crate::accounts::{Farms, derivations::{derive_user_farm_address, derive_user_farm_obligation_vault_address, derive_user_farm_obligation_address}};

use super::*;
use anchor_lang::prelude::*;
use solana_program::{instruction::Instruction, sysvar, system_program};
use tulipv2_sdk_common::config::levfarm::{LevFarmConfig, LENDING_PROGRAM};
use crate::instructions::create_user_farm;

/// returns an instruction that can be used to create a user farm account, and initialize
/// the first obligation/position
pub fn new_create_user_farm(
    authority: Pubkey,
    farm: Farms,
) -> Option<Instruction> {
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
    })

}



pub fn lev_farm_config(farm: Farms) -> Option<LevFarmConfig> {
    match farm {
        Farms::RayUsdcVault => {
            #[cfg(not(feature = "rayusdc-levfarm"))]
            unimplemented!("requires rayusdc-levfarm feature to be activated");
            #[cfg(feature = "rayusdc-levfarm")]
            return Some(tulipv2_sdk_common::config::levfarm::ray_usdc::get_lev_farm_config())
        }
        _ => None
    }
}