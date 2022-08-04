use anchor_lang::solana_program::pubkey::Pubkey;
use tulipv2_sdk_levfarm::accounts::aqua_farms::AquaFarms;
pub fn derive_vault_user_address(
    program_id: &Pubkey,
    vault: &Pubkey,
    authority: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        vault.as_ref(),
        authority.as_ref(),
    ], program_id)
}

pub fn derive_vault_address(
    program_id: &Pubkey,
    farm_key: AquaFarms,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        &(farm_key as u64).to_le_bytes()[..],
    ], program_id)
}

pub fn derive_vault_pda(
    program_id: &Pubkey,
    vault_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"pda",
            vault_address.as_ref(),
        ],
        program_id,
    )
}

pub fn derive_deposit_queue(
    program_id: &Pubkey,
    vault_pda: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[vault_pda.as_ref(), b"deposit_queue"],
        &program_id,
    )
}


#[cfg(test)]
mod test {
    use tulipv2_sdk_common::config::levfarm::ORCA_VAULT_PROGRAM;

    use super::*;

    #[test]
    fn test_vault_derives() {
        let orca_usdc_vault = derive_vault_address(&ORCA_VAULT_PROGRAM, AquaFarms::ORCAUSDC).0;
        let orca_pda = derive_vault_pda(&ORCA_VAULT_PROGRAM, &orca_usdc_vault).0;
        let deposit_queue = derive_deposit_queue(&ORCA_VAULT_PROGRAM, &orca_pda).0;
        assert_eq!(orca_usdc_vault.to_string().as_str(), "2Dts63SfTz2yivx57izMcVfDMAdpxuBgqM99ChWeJXun");
        assert_eq!(orca_pda.to_string().as_str(), "2d2xTss5cCzemByg55CUVB5dZsxrpaLk8BdiZe9WVuXe");
        assert_eq!(deposit_queue.to_string().as_str(), "GEogPAbFyMyUuxjjgMDBh1of9qaz5BGEtTyYX9LVDfSA");

    }
}