//! raydium v1 vault instructions

use anchor_lang::solana_program::instruction::Instruction;
use sighashdb::GlobalSighashDB;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use tulipv2_sdk_common::config::levfarm::RAYDIUM_VAULT_PROGRAM;

pub struct WithdrawVault {
    pub authority: Pubkey,
    pub authority_token_account: Pubkey,
    pub vault: Pubkey,
    pub lp_token_account: Pubkey,
    pub user_balance_account: Pubkey,
    pub user_info_account: Pubkey,
    pub user_reward_a_token_account: Pubkey,
    pub pool_reward_a_token_account: Pubkey,
    pub user_reward_b_token_account: Pubkey,
    pub pool_reward_b_token_account: Pubkey,
    pub token_program_id: Pubkey,
    pub clock: Pubkey,
    pub vault_pda_account: Pubkey,
    pub pool_lp_token_account: Pubkey,
    pub pool_authority: Pubkey,
    pub pool_id: Pubkey,
    pub stake_program_id: Pubkey,
    pub user_balance_meta: Pubkey,
}

pub struct DepositVault {
    pub authority: Pubkey,
    pub authority_token_account: Pubkey,
    pub vault_pda_account: Pubkey,
    pub vault: Pubkey,
    pub lp_token_account: Pubkey,
    pub user_balance_account: Pubkey,
    pub system_program: Pubkey,
    pub stake_program_id: Pubkey,
    pub pool_id: Pubkey,
    pub pool_authority: Pubkey,
    pub user_info_account: Pubkey,
    pub pool_lp_token_account: Pubkey,
    pub user_reward_a_token_account: Pubkey,
    pub pool_reward_a_token_account: Pubkey,
    pub user_reward_b_token_account: Pubkey,
    pub pool_reward_b_token_account: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub token_program_id: Pubkey,
    pub user_balance_metadata: Pubkey,
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

pub fn new_deposit_vault_ix(
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