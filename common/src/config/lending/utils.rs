use anchor_lang::prelude::Pubkey;

/// derive the address used to store tracking account information in
pub fn derive_tracking_address(
    vault: &Pubkey,
    owner: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"tracking", vault.as_ref(), owner.as_ref()], program_id)
}

/// derive the address used by the tracking account for signing
pub fn derive_tracking_pda_address(tracking_account: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[tracking_account.as_ref()], program_id)
}

/// derive the address used as the temporary account
/// for temporarily storing share tokens dudring a withdrawal process
pub fn derive_tracking_queue_address(tracking_pda: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"queue", tracking_pda.as_ref()], program_id)
}
