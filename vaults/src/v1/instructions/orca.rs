//! orca v1 vault instructions

use anchor_lang::solana_program::instruction::Instruction;
use sighashdb::GlobalSighashDB;
use anchor_lang::prelude::*;
use tulipv2_sdk_common::config::levfarm::ORCA_VAULT_PROGRAM;



pub struct DepositVault {
    pub authority: Pubkey,
    pub vault_account: Pubkey,
    pub vault_user_account: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub vault_pda: Pubkey,
    pub system_program: Pubkey,
    pub user_farm_owner: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_farm_token_account: Pubkey,
    pub user_reward_token_account: Pubkey,
    pub global_base_token_vault: Pubkey,
    pub farm_token_mint: Pubkey,
    pub global_farm: Pubkey,
    pub user_farm: Pubkey,
    pub global_reward_token_vault: Pubkey,
    pub convert_authority: Pubkey,
    pub aqua_farm_program: Pubkey,
    pub funding_token_account: Pubkey,
}

pub fn new_deposit_vault_ix(
    accounts: DepositVault,
    amount: u64,
    account_nonce: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get("deposit_vault")?;
    let mut ix_data = Vec::with_capacity(17);
    ix_data.extend_from_slice(&ix_sighash);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&amount).ok()?[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&account_nonce).ok()?[..]);

    Some(Instruction {
        program_id: ORCA_VAULT_PROGRAM,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}

impl ToAccountMetas for DepositVault {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority, true),
            AccountMeta::new(self.vault_account, false),
            AccountMeta::new(self.vault_user_account, false),
            AccountMeta::new_readonly(self.token_program, false),
            AccountMeta::new_readonly(self.rent, false),
            AccountMeta::new(self.vault_pda, false),
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new(self.user_farm_owner, false),
            AccountMeta::new_readonly(self.user_transfer_authority, false),
            AccountMeta::new(self.user_base_token_account, false),
            AccountMeta::new(self.user_farm_token_account, false),
            AccountMeta::new(self.user_reward_token_account, false),
            AccountMeta::new(self.global_base_token_vault, false),
            AccountMeta::new(self.farm_token_mint, false),
            AccountMeta::new(self.global_farm, false),
            AccountMeta::new(self.user_farm, false),
            AccountMeta::new(self.global_reward_token_vault, false),
            AccountMeta::new_readonly(self.convert_authority, false),
            AccountMeta::new_readonly(self.aqua_farm_program, false),
            AccountMeta::new(self.funding_token_account, false),
        ]
    }
}