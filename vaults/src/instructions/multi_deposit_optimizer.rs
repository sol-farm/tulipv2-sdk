use super::*;


/// returns a new instruction to withdraw funds from the multi deposit optimizer vault.
/// the `standalone_vault_accounts` argument are ProgramType specific accounts, which are
/// expected to be in the following order depending on the ProgramType.
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
    receiving_underlying_token_account: Pubkey,
    multi_underlying_withdraw_queue: Pubkey,
    multi_shares_mint: Pubkey,
    withdraw_shares_mint: Pubkey,
    withdraw_vault_underlying_deposit_queue: Pubkey,
    amount: u64,
    standalone_vault_accounts: Vec<AccountMeta>,
) -> Option<Instruction> {
    let mut ix_sighash = GlobalSighashDB.get("withdraw_multi_deposit_optimizer_vault")?;
    let mut ix_data = Vec::with_capacity(16);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);
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
        AccountMeta::new(receiving_underlying_token_account, false),
        AccountMeta::new(multi_underlying_withdraw_queue, false),
        AccountMeta::new(multi_shares_mint, false),
        AccountMeta::new(withdraw_shares_mint, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(withdraw_vault_underlying_deposit_queue, false),
    ];
    accounts.extend_from_slice(&standalone_vault_accounts[..]);
    Some(Instruction {
        program_id: crate::ID,
        accounts,
        data: ix_data,
    })
}