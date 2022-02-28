use anchor_lang::{prelude::*, solana_program::{instruction::Instruction, sysvar, system_program}};
use farms::Farm;

/// sighash used by the `register_deposit_tracking_account` instruction
pub const REGISTER_DEPOSIT_TRACKING_ACCOUNT_SIGHASH: [u8; 8] = [55, 114, 97, 238, 33, 173, 193, 225];
/// sighash used by the `withdraw_deposit_tracking` instruction
pub const WITHDRAW_DEPOSIT_TRACKING_SIGHASH: [u8; 8] = [3, 232, 22, 105, 242, 88, 178, 172];
/// sighash used by the `withdraw_multi_deposit_optimizer_vault` instruction
pub const WITHDRAW_MULTI_DEPOSIT_OPTIMIZER_VAULT_SIGHASH: [u8; 8] = [94, 147, 111, 141, 204, 247, 197, 86];
/// sighash used by the `issue_shares` istruction
pub const ISSUE_SHARES_SIGHASH: [u8; 8] = [110, 72, 179, 47, 131, 109, 115, 103];


/// returns a new instruction used to regsiter a deposit tracking account
/// with a given vault.
pub fn new_register_deposit_tracking_account_ix(
    authority: Pubkey,
    vault: Pubkey,
    deposit_tracking_account: Pubkey,
    deposit_tracking_queue_account: Pubkey,
    deposit_tracking_hold_account: Pubkey,
    shares_mint: Pubkey,
    deposit_tracking_pda: Pubkey,
    farm_type: Farm
) -> Instruction {
    let mut ix_data = REGISTER_DEPOSIT_TRACKING_ACCOUNT_SIGHASH.to_vec();
    let farm_type: [u64; 2] = farm_type.into();
    ix_data.append(
        &mut AnchorSerialize::try_to_vec(&farm_type).unwrap(),
    );
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
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
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anchor_lang::solana_program::pubkey::Pubkey;
    use ring::digest::{Context, SHA256};
    use std::str::FromStr;
    #[test]
    fn test_sighashes() {
        {
            let mut context = Context::new(&SHA256);
            context.update(b"global:register_deposit_tracking_account");
            let digest = context.finish();
            println!(
                "pub const REGISTER_DEPOSIT_TRACKING_ACCOUNT_SIGHASH: [u8; 8] = {:?};",
                &digest.as_ref()[0..8]
            );
        }
        {
            let mut context = Context::new(&SHA256);
            context.update(b"global:withdraw_deposit_tracking");
            let digest = context.finish();
            println!(
                "pub const WITHDRAW_DEPOSIT_TRACKING_SIGHASH: [u8; 8] = {:?};",
                &digest.as_ref()[0..8]
            );
        }
        {
            let mut context = Context::new(&SHA256);
            context.update(b"global:withdraw_multi_deposit_optimizer_vault");
            let digest = context.finish();
            println!(
                "pub const WITHDRAW_MULTI_DEPOSIT_OPTIMIZER_VAULT_SIGHASH: [u8; 8] = {:?};",
                &digest.as_ref()[0..8]
            );
        }
        {
            let mut context = Context::new(&SHA256);
            context.update(b"global:issue_shares");
            let digest = context.finish();
            println!(
                "pub const ISSUE_SHARES_SIGHASH: [u8; 8] = {:?};",
                &digest.as_ref()[0..8]
            );            
        }
    }
}