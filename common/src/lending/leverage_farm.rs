//! accounts involved in leverage farming
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::pubkey::Pubkey;

use crate::DEFAULT_KEY;

/// used to cheap access the lp mint from a given leverage farm using the accessor method.
/// this is a potentially dangerous method unless sufficient validation of the provided account
/// is done beforehand
pub fn get_leverage_farm_lp_mint(
    account: &AccountInfo,
) -> std::result::Result<Pubkey, ProgramError> {
    use so_defi_utils::accessor::{to_pubkey, AccessorType};
    if account.data_len() - 1 < 487 {
        return Err(ProgramError::AccountDataTooSmall);
    }
    let data_bytes = AccessorType::Pubkey(488).access(account);
    if data_bytes.len() != 32 {
        return Err(ProgramError::InvalidAccountData);
    };
    let got_lp_mint = to_pubkey(&data_bytes);
    if got_lp_mint.eq(&DEFAULT_KEY) {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(got_lp_mint)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use anchor_client::solana_client::rpc_client::RpcClient;
    use solana_program::account_info::IntoAccountInfo;

    use super::*;
    #[test]
    fn test_get_leverage_farm_lp_mint_orca() {
        let basis_usdc_orca_lev_farm_account =
            Pubkey::from_str("7dKmQgDoXJ5gBeugwzwXHHE15ypVMBfqffmbJXqnvmcH").unwrap();
        let want_basis_usdc_orca_lp_mint =
            Pubkey::from_str("GoaAiajubRgeCFEz9L6mLnSmT2QFegoJDH5tpLfivpj").unwrap();
        let rpc_client = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let lev_farm_account = rpc_client
            .get_account(&basis_usdc_orca_lev_farm_account)
            .unwrap();
        let mut lev_farm_tuple = (basis_usdc_orca_lev_farm_account, lev_farm_account);
        let lev_farm_account_info = lev_farm_tuple.into_account_info();
        let got_lp_mint = get_leverage_farm_lp_mint(&lev_farm_account_info).unwrap();
        assert_eq!(got_lp_mint, want_basis_usdc_orca_lp_mint);
    }
    #[test]
    fn test_get_leverage_farm_lp_mint_raydium() {
        let stsol_usdc_ray_lev_farm_account =
            Pubkey::from_str("2RwEGydvxM7ZuHLgRmKyfve1qYc8ZNsd39BBZtK38CeX").unwrap();
        let want_stsol_usdc_ray_lp_mint =
            Pubkey::from_str("HDUJMwYZkjUZre63xUeDhdCi8c6LgUDiBqxmP3QC3VPX").unwrap();
        let rpc_client = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let lev_farm_account = rpc_client
            .get_account(&stsol_usdc_ray_lev_farm_account)
            .unwrap();
        let mut lev_farm_tuple = (want_stsol_usdc_ray_lp_mint, lev_farm_account);
        let lev_farm_account_info = lev_farm_tuple.into_account_info();
        let got_lp_mint = get_leverage_farm_lp_mint(&lev_farm_account_info).unwrap();
        assert_eq!(got_lp_mint, want_stsol_usdc_ray_lp_mint);
    }
}
