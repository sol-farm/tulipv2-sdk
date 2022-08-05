use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};
use tulipv2_sdk_common::config::levfarm::RAYDIUM_VAULT_PROGRAM;

#[cfg(not(target_arch = "bpf"))]
use type_layout::TypeLayout;

#[account]
#[derive(Debug)]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
// consumes 784 bytes with the account discriminator included
pub struct Vault {
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub pda_token_account: Pubkey,
    pub pda: Pubkey,
    pub nonce: u8,
    pub info_nonce: u8,
    pub reward_a_nonce: u8,
    pub reward_b_nonce: u8,
    pub swap_to_nonce: u8,
    pub total_vault_balance: u64,
    pub info_account: Pubkey,
    pub lp_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub reward_a_account: Pubkey,
    pub reward_b_account: Pubkey,
    pub swap_to_account: Pubkey,
    pub total_vlp_shares: u64,
    pub pool_id: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_lp_token_account: Pubkey,
    pub pool_reward_a_token_account: Pubkey,
    pub pool_reward_b_token_account: Pubkey,
    pub stake_program_id: Pubkey, 
    pub dual_reward: bool,
    pub metadata_account: Pubkey,
    pub open_orders_account: Pubkey,
    pub controller_fee: u64,
    pub platform_fee: u64,
    pub vault_fee: u64,
    pub entrance_fee: u64,
    pub withdrawal_fee: u64,
    pub fee_recipient: Pubkey,
    pub fee_authority: Pubkey,
    pub compound_authority: Pubkey,
    pub precision_factor: u64,
    pub last_compound_time: i64,
    pub compound_interval: i64,
    pub dual_fee_recipient: Pubkey,
    pub tulip_reward_per_slot: u64,
    pub tulip_reward_per_share: u128,
    pub tulip_reward_end_slot: u64,
    pub last_interaction_slot: u64,
    pub staking_state_refreshed: bool,
    pub disabled: bool,
    pub migrated: bool,
    pub old_user_info_account: Pubkey,
    pub associated_info_account: Pubkey,
    pub version_five: bool,
    pub deposit_queue: Pubkey,
    pub deposit_queue_nonce: u8,
    pub buffer: [u8; 30],
}

#[inline(never)]
pub fn load<'info>(info: &AccountInfo<'info>) -> Result<Vault> {
    let mut data: &[u8] = &info.try_borrow_data()?;
    Ok(Vault::try_deserialize_unchecked(&mut data)?)
}

pub fn derive_balance_account_address(
    vault_user_info_account: Pubkey,
    authority: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        vault_user_info_account.as_ref(),
        authority.as_ref(),
    ], &RAYDIUM_VAULT_PROGRAM)
}

pub fn derive_balance_account_metadata_address(
    balance_account: Pubkey,
    authority: Pubkey
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        balance_account.as_ref(),
        authority.as_ref(),
    ], &RAYDIUM_VAULT_PROGRAM)
}

pub fn derive_vault_pda_address(
    vault: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        vault.as_ref(),
        b"pda",
    ], &RAYDIUM_VAULT_PROGRAM)
}

pub fn derive_vault_user_info_address(
    vault: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        vault.as_ref(),
        b"info",
    ], &RAYDIUM_VAULT_PROGRAM)
}

pub fn derive_vault_reward_a_address(
    vault: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        vault.as_ref(),
        b"reward",
    ], &RAYDIUM_VAULT_PROGRAM)
}

pub fn derive_vault_reward_b_address(
    vault: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            vault.as_ref(),
            b"rewardb",
        ],
        &RAYDIUM_VAULT_PROGRAM
    )
}


#[cfg(test)]
mod test {
    use super::*;
    use solana_client::rpc_client::RpcClient;
    #[test]
    fn log_layout() {
        use tulipv2_sdk_common::vaults::v1::raydium::{RAY_USDC, RAY_USDC_OLD_INFO, RAY_USDC_LP_TOKEN_ACCOUNT, ATLAS_USDC, ATLAS_USDC_INFO_ACCOUNT, ATLAS_USDC_LP_TOKEN_ACCOUNT};
        println!("{}", Vault::type_layout());
        let rpc = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let valt_acct = rpc.get_account(&RAY_USDC).unwrap();
        let vault_info = Vault::try_deserialize_unchecked(&mut &valt_acct.data[..]).unwrap();
        println!("{:#?}", vault_info);
        assert_eq!(vault_info.authority.to_string().as_str(), "D283q9dABgGLeERA5LpnHV5XnEaYTdnTp6Hq7SntcsNm");
        assert_eq!(vault_info.pda.to_string().as_str(), "38dsJ6n4y6ffCDSZXhYYiMXQCgfzqHK5XSytL2fApeGc");
    }
    #[test]
    fn derive_ray_usdc_addresses() {
        use std::str::FromStr;
        use tulipv2_sdk_common::vaults::v1::raydium::{RAY_USDC, RAY_USDC_OLD_INFO, RAY_USDC_LP_TOKEN_ACCOUNT, ATLAS_USDC, ATLAS_USDC_INFO_ACCOUNT, ATLAS_USDC_LP_TOKEN_ACCOUNT};

        let vault_pda = derive_vault_pda_address(RAY_USDC).0;
        let vault_info = derive_vault_user_info_address(RAY_USDC).0;
        let vault_reward_a_account = derive_vault_reward_a_address(RAY_USDC).0;
        let vault_reward_b_account = derive_vault_reward_b_address(RAY_USDC).0;
        assert_eq!(vault_pda.to_string().as_str(), "38dsJ6n4y6ffCDSZXhYYiMXQCgfzqHK5XSytL2fApeGc");
        assert_eq!(vault_info.to_string().as_str(), RAY_USDC_OLD_INFO.to_string().as_str());
        assert_eq!(vault_reward_a_account.to_string().as_str(), "9VQe52wd4GUFfyib2jwahsWsAAgiiJv7gZQ28HTS5GzB");
        assert_eq!(vault_reward_b_account.to_string().as_str(), "4fTYCyFfSsPX58LAfj2AWWBjzqi3D3jYh6EEUAaSHrAK");
        assert_eq!(RAY_USDC_LP_TOKEN_ACCOUNT, spl_associated_token_account::get_associated_token_address(
            &vault_pda,
            &Pubkey::from_str("FbC6K13MzHvN42bXrtGaWsvZY9fxrackRSZcBGfjPc7m").unwrap()
        ));

        let vault_pda = derive_vault_pda_address(ATLAS_USDC).0;
        let vault_info = derive_vault_user_info_address(ATLAS_USDC).0;
        let vault_reward_a_account = derive_vault_reward_a_address(ATLAS_USDC).0;
        let vault_reward_b_account = derive_vault_reward_b_address(ATLAS_USDC).0;
        assert_eq!(vault_pda.to_string().as_str(), "E9gcs2csKYETUVmAmUkZqh2jKQUgFjjEiTGcwazY5doT");
        assert_eq!(vault_info.to_string().as_str(), ATLAS_USDC_INFO_ACCOUNT.to_string().as_str());
        assert_eq!(vault_reward_a_account.to_string().as_str(), "J5KSiQVzk22Xt7Stdvqq1aBdHVsHRw6PhCTYJQMx2M98");
        assert_eq!(vault_reward_b_account.to_string().as_str(), "BTnELeB9pciBFPy8t2fnVQNX4e12Whdq4v36xGqV9b4r");
        assert_eq!(ATLAS_USDC_LP_TOKEN_ACCOUNT, spl_associated_token_account::get_associated_token_address(
            &vault_pda,
            &Pubkey::from_str("9shGU9f1EsxAbiR567MYZ78WUiS6ZNCYbHe53WUULQ7n").unwrap()
        ));

    }
}