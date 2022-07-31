use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

pub struct RaydiumSwap {
    pub authority: Pubkey,
    pub leveraged_farm: Pubkey,
    pub user_farm: Pubkey,
    pub user_farm_obligation: Pubkey,
    pub token_program: Pubkey,
    pub vault_signer: Pubkey,
    pub swap_or_liquidity_program_id: Pubkey,
    pub amm_id: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub amm_quantities_or_target_orders: Pubkey,
    pub pool_coin_tokenaccount: Pubkey,
    pub pool_pc_tokenaccount: Pubkey,
    pub serum_program_id: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub coin_wallet: Pubkey,
    pub pc_wallet: Pubkey,
}

#[inline(always)]
pub fn swap_tokens_raydium_stats(
    accounts: Box<RaydiumSwap>,
    lending_market_account: Pubkey,
    lending_market_authority: Pubkey,
    lending_program: Pubkey,
    position_info_account: Pubkey,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("swap_tokens_raydium_stats")?;
    let mut ix_data = Vec::with_capacity(9);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());

    let mut accounts = accounts.to_account_metas(None);
    accounts.push(AccountMeta::new_readonly(
        lending_market_account.key(),
        false,
    ));
    accounts.push(AccountMeta::new_readonly(
        lending_market_authority.key(),
        false,
    ));
    accounts.push(AccountMeta::new_readonly(lending_program.key(), false));
    accounts.push(AccountMeta::new(position_info_account.key(), false));

    Some(Instruction {
        program_id: crate::ID,
        accounts,
        data: ix_data,
    })
}

impl ToAccountMetas for RaydiumSwap {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority, true),
            AccountMeta::new(self.leveraged_farm, false),
            AccountMeta::new(self.user_farm, false),
            AccountMeta::new(self.user_farm_obligation, false),
            AccountMeta::new_readonly(self.token_program, false),
            AccountMeta::new_readonly(self.leveraged_farm, false),
            AccountMeta::new_readonly(self.swap_or_liquidity_program_id, false),
            AccountMeta::new(self.amm_id, false),
            AccountMeta::new(self.amm_authority, false),
            AccountMeta::new(self.amm_open_orders, false),
            AccountMeta::new(self.amm_quantities_or_target_orders, false),
            AccountMeta::new(self.pool_coin_tokenaccount, false),
            AccountMeta::new(self.pool_pc_tokenaccount, false),
            AccountMeta::new_readonly(self.serum_program_id, false),
            AccountMeta::new(self.serum_market, false),
            AccountMeta::new(self.serum_bids, false),
            AccountMeta::new(self.serum_asks, false),
            AccountMeta::new(self.serum_event_queue, false),
            AccountMeta::new(self.serum_coin_vault_account, false),
            AccountMeta::new(self.serum_pc_vault_account, false),
            AccountMeta::new(self.vault_signer, false),
            AccountMeta::new(self.coin_wallet, false),
            AccountMeta::new(self.pc_wallet, false),
        ]
    }
}
