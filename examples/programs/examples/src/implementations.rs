use super::*;
impl<'info> From<&mut DepositBorrowDual<'info>> for tulipv2_sdk_levfarm::instructions::deposit_borrow_dual::DepositBorrowDual {
    fn from(farm_accounts: &mut DepositBorrowDual<'info>) -> Self {
        Self {
            authority: farm_accounts.authority.key(),
            user_farm: farm_accounts.user_farm.key(),
            leveraged_farm: farm_accounts.leveraged_farm.key(),
            user_farm_obligation: farm_accounts.user_farm_obligation.key(),
            coin_source_token_account: farm_accounts.coin_source_token_account.key(),
            coin_destination_token_account: farm_accounts.coin_destination_token_account.key(),
            pc_source_token_account: farm_accounts.pc_source_token_account.key(),
            pc_destination_token_account: farm_accounts.pc_destination_token_account.key(),
            coin_deposit_reserve_account: farm_accounts.coin_deposit_reserve_account.key(),
            pc_deposit_reserve_account: farm_accounts.pc_deposit_reserve_account.key(),
            coin_reserve_liquidity_oracle: farm_accounts.coin_reserve_liquidity_oracle.key(),
            pc_reserve_liquidity_oracle: farm_accounts.pc_reserve_liquidity_oracle.key(),
            lending_market_account: farm_accounts.lending_market_account.key(),
            derived_lending_market_authority: farm_accounts.derived_lending_market_authority.key(),
            token_program: farm_accounts.token_program.key(),
            lending_program: farm_accounts.lending_program.key(),
            coin_source_reserve_liquidity_token_account: farm_accounts.coin_source_reserve_liquidity_token_account.key(),
            pc_source_reserve_liquidity_token_account: farm_accounts.pc_source_reserve_liquidity_token_account.key(),
            coin_reserve_liquidity_fee_receiver: farm_accounts.coin_reserve_liquidity_fee_receiver.key(),
            pc_reserve_liquidity_fee_receiver: farm_accounts.pc_reserve_liquidity_fee_receiver.key(),
            borrow_authorizer: farm_accounts.borrow_authorizer.key(),
            lp_pyth_price_account: farm_accounts.lp_pyth_price_account.key(),
            vault_account: farm_accounts.vault_account.key(),
            rent:             farm_accounts.rent.key(),
        }
    }
}

impl<'info> From<&mut DepositLevFarm<'info>> for tulipv2_sdk_levfarm::instructions::deposit_raydium_vault::DepositFarm {
    #[inline(always)]
    fn from(farm_accounts: &mut DepositLevFarm<'info>) -> Self {
        Self {
            authority: farm_accounts.authority.key(), 
            user_farm: tulipv2_sdk_common::DEFAULT_KEY,  // this needs to be manually overwrritten
            obligation_vault_address: farm_accounts.obligation_vault_address.key(), 
            leveraged_farm: tulipv2_sdk_common::DEFAULT_KEY,  // this needs to be manually overwrritten
            vault_program: farm_accounts.vault_program.key(), 
            authority_token_account: farm_accounts.authority_token_account.key(), 
            vault_pda_account: farm_accounts.vault_pda_account.key(), 
            vault: tulipv2_sdk_common::DEFAULT_KEY, 
            lp_token_account: farm_accounts.lp_token_account.key(), 
            user_balance_account: farm_accounts.user_balance_account.key(), 
            system_program: anchor_lang::solana_program::system_program::id(), 
            stake_program_id: farm_accounts.stake_program_id.key(), 
            pool_id: farm_accounts.pool_id.key(), 
            pool_authority: farm_accounts.pool_authority.key(), 
            vault_info_account: farm_accounts.vault_info_account.key(), 
            pool_lp_token_account: farm_accounts.pool_lp_token_account.key(), 
            user_reward_a_token_account: farm_accounts.user_reward_a_token_account.key(), 
            pool_reward_a_token_account: farm_accounts.pool_reward_a_token_account.key(), 
            user_reward_b_token_account: farm_accounts.user_reward_b_token_account.key(), 
            pool_reward_b_token_account: farm_accounts.pool_reward_b_token_account.key(), 
            clock: anchor_lang::solana_program::sysvar::clock::id(), 
            rent: anchor_lang::solana_program::sysvar::rent::id(), 
            token_program_id: spl_token::id(), 
            user_balance_metadata: farm_accounts.user_balance_metadata.key(), 
        }
    }
}