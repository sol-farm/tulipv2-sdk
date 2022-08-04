use anchor_lang::solana_program::pubkey::Pubkey;
use tulipv2_sdk_common::config::levfarm::RAYDIUM_VAULT_PROGRAM;


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