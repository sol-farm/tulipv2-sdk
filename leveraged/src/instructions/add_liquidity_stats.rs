use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;


pub struct AddLiquidity {
    pub authority: Pubkey,
    pub user_farm: Pubkey,
    pub leveraged_farm: Pubkey,
    pub liquidity_program_id: Pubkey,
    pub amm_id: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_quantities_or_target_orders: Pubkey,
    pub lp_mint_address: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub serum_market: Pubkey,
    pub token_program: Pubkey,
    pub lev_farm_coin_token_account: Pubkey,
    pub lev_farm_pc_token_account: Pubkey,
    pub user_lp_token_account: Pubkey,
    pub pyth_price_account: Pubkey,
    pub lending_market_account: Pubkey,
    pub user_farm_obligation: Pubkey,
    pub derived_lending_market_authority: Pubkey,
    pub lending_program: Pubkey,
    pub clock: Pubkey,
    pub dex_program: Pubkey,
}

pub fn add_liquidity_stats(
    accounts: Box<AddLiquidity>,
    position_info_account: Pubkey,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("add_liquidity_stats")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());

    let mut accounts = accounts.to_account_metas(None);
    accounts.push(AccountMeta::new(position_info_account.key(), false));

    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts,
        data: ix_data,
    })
}

impl ToAccountMetas for AddLiquidity {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority, true),
            AccountMeta::new(self.user_farm, false),
            AccountMeta::new(self.leveraged_farm, false),
            AccountMeta::new_readonly(self.liquidity_program_id, false),
            AccountMeta::new(self.amm_id, false),
            AccountMeta::new(self.amm_authority, false),
            AccountMeta::new(self.amm_open_orders, false),
            AccountMeta::new(self.amm_quantities_or_target_orders, false),
            AccountMeta::new(self.lp_mint_address, false),
            AccountMeta::new(self.pool_coin_token_account, false),
            AccountMeta::new(self.pool_pc_token_account, false),
            AccountMeta::new(self.serum_market, false),
            AccountMeta::new_readonly(self.token_program, false),
            AccountMeta::new(self.lev_farm_coin_token_account, false),
            AccountMeta::new(self.lev_farm_pc_token_account, false),
            AccountMeta::new(self.user_lp_token_account, false),
            AccountMeta::new_readonly(self.pyth_price_account, false),
            AccountMeta::new_readonly(self.lending_market_account, false),
            AccountMeta::new(self.user_farm_obligation, false),
            AccountMeta::new_readonly(self.derived_lending_market_authority, false),
            AccountMeta::new_readonly(self.lending_program, false),
            AccountMeta::new_readonly(self.clock, false),
            AccountMeta::new_readonly(self.dex_program, false),
        ]
    }
}