use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

pub struct WithdrawFarm{
    pub authority: Pubkey,
    pub user_farm: Pubkey,
    pub obligation_vault_address: Pubkey,
    pub leveraged_farm: Pubkey,
    pub authority_token_account: Pubkey,
    pub vault: Pubkey,
    pub vault_program: Pubkey,
    pub user_balance_account: Pubkey,
    pub user_info_account: Pubkey,
    pub user_lp_token_account: Pubkey,
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

pub fn withdraw_raydium_vault_close(
    accounts: Box<WithdrawFarm>,
    lending_market_account: Pubkey,
    user_farm_obligation: Pubkey,
    lending_market_authority: Pubkey,
    lending_program: Pubkey,
    position_info_account: Pubkey,
    system_program: Pubkey,
    rent: Pubkey,
    meta_nonce: u8,
    nonce: u8,
    obligation_index: u8,
    withdraw_percent: u8,
    close_method: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("withdraw_raydium_vault_close")?;
    let mut ix_data = Vec::with_capacity(12);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&meta_nonce).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&nonce).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&withdraw_percent).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&close_method).unwrap());

    let mut accounts = accounts.to_account_metas(None);
    accounts.push(AccountMeta::new_readonly(lending_market_account.key(), false));
    accounts.push(AccountMeta::new(user_farm_obligation.key(), false));
    accounts.push(AccountMeta::new_readonly(lending_market_authority.key(), false));
    accounts.push(AccountMeta::new_readonly(lending_program.key(), false));
    accounts.push(AccountMeta::new(position_info_account.key(), false));
    accounts.push(AccountMeta::new_readonly(system_program.key(), false));
    accounts.push(AccountMeta::new_readonly(rent.key(), false));

    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts,
        data: ix_data,
    })
}

impl ToAccountMetas for WithdrawFarm {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority, true),
            AccountMeta::new(self.user_farm, false),
            AccountMeta::new(self.obligation_vault_address, false),
            AccountMeta::new_readonly(self.leveraged_farm, false),
            AccountMeta::new(self.authority_token_account, false),
            AccountMeta::new(self.vault, false),
            AccountMeta::new_readonly(self.vault_program, false),
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