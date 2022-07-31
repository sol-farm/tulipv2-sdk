use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

pub struct OrcaAddLiquidityQueue {
    pub authority: Pubkey,
    pub user_farm: Pubkey,
    pub leveraged_farm: Pubkey,
    pub vault_account: Pubkey,
    pub vault_user_account: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub vault_pda: Pubkey,
    pub system_program: Pubkey,
    pub lev_farm_coin_token_account: Pubkey,
    pub lev_farm_pc_token_account: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub liquidity_program_id: Pubkey,
    pub amm_id: Pubkey,
    pub amm_authority: Pubkey,
    pub vault_deposit_queue: Pubkey,
    pub lp_mint_address: Pubkey,
    pub lending_market_account: Pubkey,
    pub user_farm_obligation: Pubkey,
    pub derived_lending_market_authority: Pubkey,
    pub lending_program: Pubkey,
    pub dex_program: Pubkey,
    pub solfarm_vault_program: Pubkey,
    pub obligation_vault_address: Pubkey,
}

pub fn orca_add_liquidity_queue(
    accounts: Box<OrcaAddLiquidityQueue>,
    position_info_account: Pubkey,
    account_nonce: u8,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("orca_add_liquidity_queue")?;
    let mut ix_data = Vec::with_capacity(10);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&account_nonce).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());

    let mut accounts = accounts.to_account_metas(None);
    accounts.push(AccountMeta::new(position_info_account.key(), false));

    Some(Instruction {
        program_id: crate::ID,
        accounts,
        data: ix_data,
    })
}

impl ToAccountMetas for OrcaAddLiquidityQueue {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority, true),
            AccountMeta::new(self.user_farm, false),
            AccountMeta::new(self.leveraged_farm, false),
            AccountMeta::new(self.vault_account, false),
            AccountMeta::new(self.vault_user_account, false),
            AccountMeta::new_readonly(self.token_program, false),
            AccountMeta::new_readonly(self.rent, false),
            AccountMeta::new(self.vault_pda, false),
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new(self.lev_farm_coin_token_account, false),
            AccountMeta::new(self.lev_farm_pc_token_account, false),
            AccountMeta::new(self.pool_coin_token_account, false),
            AccountMeta::new(self.pool_pc_token_account, false),
            AccountMeta::new_readonly(self.liquidity_program_id, false),
            AccountMeta::new(self.amm_id, false),
            AccountMeta::new(self.amm_authority, false),
            AccountMeta::new(self.vault_deposit_queue, false),
            AccountMeta::new(self.lp_mint_address, false),
            AccountMeta::new(self.lending_market_account, false),
            AccountMeta::new(self.user_farm_obligation, false),
            AccountMeta::new(self.derived_lending_market_authority, false),
            AccountMeta::new_readonly(self.lending_program, false),
            AccountMeta::new_readonly(self.dex_program, false),
            AccountMeta::new_readonly(self.solfarm_vault_program, false),
            AccountMeta::new(self.obligation_vault_address, false),
        ]
    }
}
