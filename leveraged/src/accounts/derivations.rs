use anchor_lang::prelude::*;

/// used to derive the address of a user farm account
pub fn derive_user_farm_address(
    authority: Pubkey,
    program_id: Pubkey,
    index: u64,
    farm: super::Farms,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            authority.as_ref(),
            &index.to_le_bytes()[..],
            &(farm as u64).to_le_bytes()[..],
        ],
        &program_id,
    )
}


pub fn derive_leveraged_farm_address(
    solfarm_vault_program: Pubkey,
    // for non-serum based farms such as Orca
    // is just a generalized "swap account" value
    // as a swap account is the spl token swap equivalent
    // of a serum market
    serum_market: Pubkey,
    program_id: Pubkey,
    farm: super::Farms,
    legacy: bool,
) -> (Pubkey, u8) {
    if legacy {
        return Pubkey::find_program_address(
            &[
                solfarm_vault_program.as_ref(),
                serum_market.as_ref(),
                &(farm as u64).to_le_bytes()[..],
            ],
            &program_id,
        )
    }
    Pubkey::find_program_address(
        &[
            b"new",
            solfarm_vault_program.as_ref(),
            serum_market.as_ref(),
            &(farm as u64).to_le_bytes()[..],
        ],
        &program_id,
    )    
}

/// used to derive the address that is used by a user farm account
/// to manage a specific vault account.
pub fn derive_user_farm_obligation_vault_address(
    user_farm_account: Pubkey,
    program_id: Pubkey,
    obligation_index: u8,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            user_farm_account.as_ref(),
            &(obligation_index as u64).to_le_bytes()[..],
        ],
        &program_id,
    )
}

pub fn derive_user_position_info_address(
    user_farm_addr: Pubkey,
    program_id: Pubkey, // this is the farm rpogram id
    obligation_index: u8,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"position_info",
            user_farm_addr.as_ref(),
            &(obligation_index as u64).to_le_bytes()[..],
        ],
        &program_id,
    )
}