use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

pub struct WithdrawOrcaFarm {
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
    pub orca_user_farm: Pubkey,
    pub global_reward_token_vault: Pubkey,
    pub convert_authority: Pubkey,
    pub aqua_farm_program: Pubkey,
    pub receiving_token_account: Pubkey,
    pub clock: Pubkey,
    pub leveraged_user_farm: Pubkey,
    pub leveraged_farm: Pubkey,
    pub solfarm_vault_program: Pubkey,
    pub obligation_vault_address: Pubkey,
}

pub fn withdraw_orca_vault_close(
    accounts: Box<WithdrawOrcaFarm>,
    lending_market_account: Pubkey,
    user_farm_obligation: Pubkey,
    lending_market_authority: Pubkey,
    lending_program: Pubkey,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("withdraw_orca_vault_close")?;
    let mut ix_data = Vec::with_capacity(11);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&withdraw_percent).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&close_method).unwrap());

    let mut accounts = accounts.to_account_metas(None);
    accounts.push(AccountMeta::new_readonly(
        lending_market_account.key(),
        false,
    ));
    accounts.push(AccountMeta::new(user_farm_obligation.key(), false));
    accounts.push(AccountMeta::new_readonly(
        lending_market_authority.key(),
        false,
    ));
    accounts.push(AccountMeta::new(lending_program.key(), false));

    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts,
        data: ix_data,
    })
}

pub fn withdraw_orca_vault_without_shares(
    accounts: Box<WithdrawOrcaFarm>,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("withdraw_orca_vault_without_shares")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data.to_vec(),
    })
}

impl ToAccountMetas for WithdrawOrcaFarm {
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
            AccountMeta::new(self.obligation_vault_address, false),
            AccountMeta::new(self.global_reward_token_vault, false),
            AccountMeta::new_readonly(self.convert_authority, false),
            AccountMeta::new_readonly(self.aqua_farm_program, false),
            AccountMeta::new(self.receiving_token_account, false),
            AccountMeta::new(self.leveraged_farm, false),
            AccountMeta::new_readonly(self.solfarm_vault_program, false),
            AccountMeta::new(self.obligation_vault_address, false),
        ]
    }
}
