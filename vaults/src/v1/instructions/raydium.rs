//! raydium v1 vault instructions

use anchor_lang::solana_program::instruction::Instruction;
use sighashdb::GlobalSighashDB;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use tulipv2_sdk_common::config::levfarm::RAYDIUM_VAULT_PROGRAM;

pub struct WithdrawVault {
    authority: Pubkey,
    authority_token_account: Pubkey,
    vault: Pubkey,
    lp_token_account: Pubkey,
    user_balance_account: Pubkey,
    user_info_account: Pubkey,
    user_lp_token_account: Pubkey,
    user_reward_a_token_account: Pubkey,
    pool_reward_a_token_account: Pubkey,
    user_reward_b_token_account: Pubkey,
    pool_reward_b_token_account: Pubkey,
    token_program_id: Pubkey,
    clock: Pubkey,
    vault_pda_account: Pubkey,
    pool_lp_token_account: Pubkey,
    pool_authority: Pubkey,
    pool_id: Pubkey,
    stake_program_id: Pubkey,
    user_balance_meta: Pubkey,
}

pub struct DepositVault {
    authority: Pubkey,
    authority_token_account: Pubkey,
    vault_pda_account: Pubkey,
    vault: Pubkey,
    lp_token_account: Pubkey,
    user_balance_account: Pubkey,
    system_program: Pubkey,
    stake_program_id: Pubkey,
    pool_id: Pubkey,
    pool_authority: Pubkey,
    user_info_account: Pubkey,
    user_lp_token_account: Pubkey,
    pool_lp_token_account: Pubkey,
    user_reward_a_token_account: Pubkey,
    pool_reward_a_token_account: Pubkey,
    user_reward_b_token_account: Pubkey,
    pool_reward_b_token_account: Pubkey,
    clock: Pubkey,
    rent: Pubkey,
    token_program_id: Pubkey,
    user_balance_metadata: Pubkey,
}


pub fn new_withdraw_vault_ix(
    accounts: WithdrawVault,
    amount: u64,
    meta_nonce: u8
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("withdraw_vault")?;
    let mut ix_data = Vec::with_capacity(17);
    ix_data.extend_from_slice(&ix_sighash);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&meta_nonce).ok()?[..]);
    Some(Instruction {
        program_id: RAYDIUM_VAULT_PROGRAM,
        accounts: accounts.to_account_metas(None),
        data: ix_data
    })
}

pub fn new_deposit_vault_vault_ix(
    accounts: DepositVault,
    nonce: u8,
    amount: u64,
    meta_nonce: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("deposit_vault")?;
    let mut ix_data = Vec::with_capacity(17);
    ix_data.extend_from_slice(&ix_sighash);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&nonce).ok()?[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&meta_nonce).ok()?[..]);
    Some(Instruction {
        program_id: RAYDIUM_VAULT_PROGRAM,
        accounts: accounts.to_account_metas(None),
        data: ix_data
    })
}

impl ToAccountMetas for WithdrawVault {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority, true),
            AccountMeta::new(self.authority_token_account, false),
            AccountMeta::new(self.vault, false),
            AccountMeta::new(self.lp_token_account, false),
            AccountMeta::new(self.user_balance_account, false),
            AccountMeta::new(self.user_info_account, false),
            AccountMeta::new(self.user_lp_token_account, false),
            AccountMeta::new(self.user_reward_a_token_account, false),
            AccountMeta::new(self.pool_reward_a_token_account, false),
            AccountMeta::new(self.user_reward_b_token_account, false),
            AccountMeta::new(self.pool_reward_b_token_account, false),
            AccountMeta::new_readonly(self.token_program_id, false),
            AccountMeta::new_readonly(self.clock, false),
            AccountMeta::new(self.vault_pda_account, false),
            AccountMeta::new(self.pool_lp_token_account, false),
            AccountMeta::new(self.pool_authority, false),
            AccountMeta::new(self.pool_id, false),
            AccountMeta::new_readonly(self.stake_program_id, false),
            AccountMeta::new(self.user_balance_meta, false),
        ]
    }
}

impl ToAccountMetas for DepositVault {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority, true),
            AccountMeta::new(self.authority_token_account, false),
            AccountMeta::new(self.vault_pda_account, false),
            AccountMeta::new(self.vault, false),
            AccountMeta::new(self.lp_token_account, false),
            AccountMeta::new(self.user_balance_account, false),
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new_readonly(self.stake_program_id, false),
            AccountMeta::new(self.pool_id, false),
            AccountMeta::new(self.pool_authority, false),
            AccountMeta::new(self.user_info_account, false),
            AccountMeta::new(self.user_lp_token_account, false),
            AccountMeta::new(self.pool_lp_token_account, false),
            AccountMeta::new(self.user_reward_a_token_account, false),
            AccountMeta::new(self.pool_reward_a_token_account, false),
            AccountMeta::new(self.user_reward_b_token_account, false),
            AccountMeta::new(self.pool_reward_b_token_account, false),
            AccountMeta::new_readonly(self.clock, false),
            AccountMeta::new_readonly(self.rent, false),
            AccountMeta::new_readonly(self.token_program_id, false),
            AccountMeta::new(self.user_balance_metadata, false),
        ]
    }
}