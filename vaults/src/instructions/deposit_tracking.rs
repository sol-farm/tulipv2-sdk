use super::*;

/// returns a new instruction used to register a deposit tracking account
/// with a given vault.
pub fn new_register_deposit_tracking_account_ix(
    authority: Pubkey,
    vault: Pubkey,
    deposit_tracking_account: Pubkey,
    deposit_tracking_queue_account: Pubkey,
    deposit_tracking_hold_account: Pubkey,
    shares_mint: Pubkey,
    deposit_tracking_pda: Pubkey,
    farm_type: Farm,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("register_deposit_tracking_account")?;
    let farm_type: [u64; 2] = farm_type.into();
    let mut ix_data = Vec::with_capacity(32);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&farm_type).ok()?[..]);
    Some(Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(vault, false),
            AccountMeta::new(deposit_tracking_account, false),
            AccountMeta::new(deposit_tracking_queue_account, false),
            AccountMeta::new(deposit_tracking_hold_account, false),
            AccountMeta::new(shares_mint, false),
            AccountMeta::new(deposit_tracking_pda, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: ix_data,
    })
}


pub fn new_withdraw_deposit_tracking_ix(
    authority: Pubkey,
    deposit_tracking_account: Pubkey,
    deposit_tracking_pda: Pubkey,
    deposit_tracking_hold_account: Pubkey,
    receiving_shares_account: Pubkey,
    shares_mint: Pubkey,
    vault: Pubkey,
    farm_type: Farm,
    amount: u64,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("withdraw_deposit_tracking")?;
    let farm_type: [u64; 2] = farm_type.into();
    let mut ix_data = Vec::with_capacity(32);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&farm_type).ok()?[..]);
    Some(Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(deposit_tracking_account, false),
            AccountMeta::new(deposit_tracking_pda, false),
            AccountMeta::new(deposit_tracking_hold_account, false),
            AccountMeta::new(receiving_shares_account, false),
            AccountMeta::new_readonly(shares_mint, false),
            AccountMeta::new(vault, false),
        ],
        data: ix_data,
    })
}
