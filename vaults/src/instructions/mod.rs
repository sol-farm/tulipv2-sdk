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
    }
}

pub fn new_issue_shares_ix(
    authority: Pubkey,
    vault: Pubkey,
    deposit_tracking: Pubkey,
    deposit_tracking_pda: Pubkey,
    vault_pda: Pubkey,
    vault_underlying_account: Pubkey,
    shares_mint: Pubkey,
    receiving_shares_account: Pubkey,
    depositing_underlying_account: Pubkey,
    farm_type: Farm,
    amount: u64,
) -> Instruction {
    let mut ix_data = ISSUE_SHARES_SIGHASH.to_vec();
    let farm_type: [u64; 2] = farm_type.into();
    ix_data.append(&mut AnchorSerialize::try_to_vec(&farm_type).unwrap());
    ix_data.append(&mut AnchorSerialize::try_to_vec(&amount).unwrap());
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(vault, false),
            AccountMeta::new(deposit_tracking, false),
            AccountMeta::new(deposit_tracking_pda, false),
            AccountMeta::new_readonly(vault_pda, false),
            AccountMeta::new(vault_underlying_account, false),
            AccountMeta::new(shares_mint, false),
            AccountMeta::new(receiving_shares_account, false),
            AccountMeta::new(depositing_underlying_account, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: ix_data,
    }
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
) -> Instruction {
    let mut ix_data = WITHDRAW_DEPOSIT_TRACKING_SIGHASH.to_vec();
    let farm_type: [u64; 2] = farm_type.into();
    ix_data.append(&mut AnchorSerialize::try_to_vec(&amount).unwrap()); 
    ix_data.append(&mut AnchorSerialize::try_to_vec(&farm_type).unwrap());
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new(deposit_tracking_account, false),
            AccountMeta::new(deposit_tracking_pda, false),
            AccountMeta::new(deposit_tracking_hold_account, false),
            AccountMeta::new(receiving_shares_account, false),
            AccountMeta::new_readonly(shares_mint, false),
            AccountMeta::new(vault, false),
        ],
        data: ix_data,
    }
}

/// returns a new instruction to withdraw funds from the multi deposit optimizer vault.
/// the `standalone_vault_accounts` argument are ProgramType specific accounts, which are
/// expceted to be in the following order depending on the ProgramType.
/// 
/// You may request a withdrawal from any of the standalone vaults which have a deposited
/// balance greater than 0. It's up to the caller to decide which of the active standalone vaults to withdraw from.
/// 
/// ------------------------------
/// SOURCE_PROGRAM ACCOUNTS ORDER
/// ------------------------------
///
/// For platforms with a ProgramType::SplUnmodified are used, the following accounts are needed in order.
///
///         0 [writable]        -> source_collateral_token_account
///
///         1 [writable]        -> reserve_account
///
///         2 [writable]        -> reserve_liquidity_supply
///
///         3 [writable]        -> reserve_collateral_mint
///
///         4 []                -> lending_market_account
///    
///         5 []                -> derived_lending_market_authority
///
///         6 []                -> reserve_oracle
///
///
/// For platforms with a ProgramType::SplModifiedSolend are used, the following accounts are needed in order
///
///         1 [writable]        -> reserve_account
///
///         2 [writable]        -> reserve_liquidity_supply
///
///         3 [writable]        -> reserve_collateral_mint
///
///         4 []                -> lending_market_account
///    
///         5 []                -> derived_lending_market_authority
///
///         6 []                -> reserve_pyth_price_account
///
///         7 []                -> reserve_switchboard_price_account
///
///
/// For platforms with a ProgramType::MangoV3 are used, the following accounts are needed in order
///
///         0 []                -> mango_group_account
///
///         1 [writable]        -> optimizer_mango_account
///
///         2 []                -> mango_cache
///
///         3 []                -> root_bank
///
///         4 [writable]        -> node_bank
///
///         5 [writable]        -> mango_token_account
///
///         6 []                -> mango_group_signer
///
///         7 []                -> system_program
///
pub fn new_withdraw_multi_deposit_optimizer_vault_ix(
    authority: Pubkey,
    multi_vault: Pubkey,
    multi_vault_pda: Pubkey,
    withdraw_vault: Pubkey,
    withdraw_vault_pda: Pubkey,
    platform_information: Pubkey,
    platform_config_data: Pubkey,
    lending_program: Pubkey,
    multi_burning_shares_token_account: Pubkey,
    withdraw_burning_shares_token_account: Pubkey,
    receiving_underyling_token_account: Pubkey,
    multi_underlying_withdraw_queue: Pubkey,
    multi_shares_mint: Pubkey,
    withdraw_shares_mint: Pubkey,
    withdraw_vault_underlying_deposit_queue: Pubkey,
    amount: u64,
    standalone_vault_accounts: Vec<AccountMeta>,
) -> Instruction {
    let mut ix_data = WITHDRAW_MULTI_DEPOSIT_OPTIMIZER_VAULT_SIGHASH.to_vec();
    ix_data.append(&mut AnchorSerialize::try_to_vec(&amount).unwrap());
    let mut accounts = vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(multi_vault, false),
            AccountMeta::new_readonly(multi_vault_pda, false),
            AccountMeta::new(withdraw_vault, false),
            AccountMeta::new_readonly(withdraw_vault_pda, false),
            AccountMeta::new_readonly(platform_information, false),
            AccountMeta::new_readonly(platform_config_data, false),
            AccountMeta::new_readonly(lending_program, false),
            AccountMeta::new(multi_burning_shares_token_account, false),
            AccountMeta::new(withdraw_burning_shares_token_account, false),
            AccountMeta::new(receiving_underyling_token_account, false),
            AccountMeta::new(multi_underlying_withdraw_queue, false),
            AccountMeta::new(multi_shares_mint, false),
            AccountMeta::new(withdraw_shares_mint, false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(withdraw_vault_underlying_deposit_queue, false),
    ];
    accounts.extend_from_slice(&standalone_vault_accounts[..]);
    Instruction {
        program_id: crate::ID,
        accounts,
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