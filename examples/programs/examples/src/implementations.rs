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
impl<'info> From<&mut RaydiumSwap<'info>> for tulipv2_sdk_levfarm::instructions::swap_tokens_raydium_stats::RaydiumSwap {
    #[inline(always)]
    fn from(farm_accounts: &mut RaydiumSwap<'info>) -> Self {
        Self {
            authority: farm_accounts.authority.key(),
            leveraged_farm: farm_accounts.leveraged_farm.key(),
            user_farm: farm_accounts.user_farm.key(),
            user_farm_obligation: farm_accounts.user_farm_obligation.key(),
            token_program: farm_accounts.token_program.key(),
            vault_signer: farm_accounts.vault_signer.key(),
            swap_or_liquidity_program_id: farm_accounts.swap_or_liquidity_program_id.key(),
            amm_id: farm_accounts.amm_id.key(),
            amm_authority: farm_accounts.amm_authority.key(),
            amm_open_orders: farm_accounts.amm_open_orders.key(),
            amm_quantities_or_target_orders: farm_accounts.amm_quantities_or_target_orders.key(),
            pool_coin_tokenaccount: farm_accounts.pool_coin_tokenaccount.key(),
            pool_pc_tokenaccount: farm_accounts.pool_pc_tokenaccount.key(),
            serum_program_id: farm_accounts.serum_program_id.key(),
            serum_market: farm_accounts.serum_market.key(),
            serum_bids: farm_accounts.serum_bids.key(),
            serum_asks: farm_accounts.serum_asks.key(),
            serum_event_queue: farm_accounts.serum_event_queue.key(),
            serum_coin_vault_account: farm_accounts.serum_coin_vault_account.key(),
            serum_pc_vault_account: farm_accounts.serum_pc_vault_account.key(),
            serum_vault_signer: farm_accounts.serum_vault_signer.key(),
            coin_wallet: farm_accounts.coin_wallet.key(),
            pc_wallet: farm_accounts.pc_wallet.key(),
        }
    }
}