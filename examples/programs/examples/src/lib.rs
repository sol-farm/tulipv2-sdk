use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use tulipv2_sdk_common::config::deposit_tracking::traits::IssueShares;
use tulipv2_sdk_common::config::deposit_tracking::traits::RegisterDepositTracking;
use tulipv2_sdk_common::config::deposit_tracking::traits::WithdrawDepositTracking;
use tulipv2_sdk_common::config::lending::traits::WithdrawMultiOptimizerVault;
use tulipv2_sdk_common::msg_panic;
use tulipv2_sdk_farms::Farm;
use tulipv2_sdk_vaults::instructions::{
    new_issue_shares_ix, new_register_deposit_tracking_account_ix,
    new_withdraw_deposit_tracking_ix, new_withdraw_multi_deposit_optimizer_vault_ix,
};

pub mod implementations;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod examples {
    use super::*;
    pub fn register_deposit_tracking_account(
        ctx: Context<RegisterDepositTrackingAccount>,
        farm_type: [u64; 2],
    ) -> Result<()> {
        // create the associated
        {
            let ix = spl_associated_token_account::create_associated_token_account(
                ctx.accounts.authority.key,
                ctx.accounts.deposit_tracking_pda.key,
                &ctx.accounts.shares_mint.key(),
            );
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.authority.clone(),
                    ctx.accounts.deposit_tracking_pda.clone(),
                    ctx.accounts.shares_mint.to_account_info(),
                    ctx.accounts.token_program.clone(),
                    ctx.accounts.deposit_tracking_hold_account.clone(),
                    ctx.accounts.rent.to_account_info(),
                ],
            )?;
        }
        {
            let registration_trait = tulipv2_sdk_common::config::lending::usdc::multi_deposit::ProgramConfig::register_deposit_tracking_ix(
                *ctx.accounts.authority.key,
            );
            anchor_lang::solana_program::program::invoke(
                &registration_trait
                    .instruction(tulipv2_sdk_farms::Farm::Lending {
                        name: tulipv2_sdk_farms::lending::Lending::MULTI_DEPOSIT,
                    })
                    .unwrap(),
                &[
                    ctx.accounts.authority.clone(),
                    ctx.accounts.vault.clone(),
                    ctx.accounts.deposit_tracking_account.clone(),
                    ctx.accounts.deposit_tracking_queue_account.clone(),
                    ctx.accounts.deposit_tracking_hold_account.clone(),
                    ctx.accounts.shares_mint.to_account_info(),
                    ctx.accounts.deposit_tracking_pda.clone(),
                    ctx.accounts.rent.to_account_info(),
                    ctx.accounts.token_program.clone(),
                    ctx.accounts.rent.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }
        Ok(())
    }
    /// deposits `amount` of the underlying tokens in exchange for a corresponding
    /// amount of shares. these shares are locked within the deposit tracking account
    /// for 15 minutes, after which they can be removed from the deposit tracking account
    /// if desired. generaly speaking this should only be done if you want to
    /// use the tokenized shares elsewhere (ie friktion volts), otherwise
    /// its best to leave them within the deposit tracking account otherwise
    /// so that you can measure your accrued rewards automatically.
    pub fn issue_shares(
        ctx: Context<IssueSharesInstruction>,
        amount: u64,
        farm_type: [u64; 2],
    ) -> Result<()> {
        /*
            if this error is returned, it means the depositing_underlying_account
            has less tokens (X) then requested deposit amount (Y)
            Program log: RUNTIME ERROR: a(X) < b(Y)
            Program log: panicked at 'RUNTIME ERROR: a(0) < b(1)', programs/vaults/src/vault_instructions/deposit_tracking/acl_helpers.rs:198:9
        */

        let issue_trait = tulipv2_sdk_common::config::lending::usdc::multi_deposit::ProgramConfig::issue_shares_ix(
            *ctx.accounts.authority.key,
        );

        anchor_lang::solana_program::program::invoke(
            &issue_trait.instruction(farm_type.into(), amount).unwrap(),
            &[
                ctx.accounts.authority.clone(),
                ctx.accounts.vault.clone(),
                ctx.accounts.deposit_tracking_account.clone(),
                ctx.accounts.deposit_tracking_pda.clone(),
                ctx.accounts.vault_pda.clone(),
                ctx.accounts.vault_underlying_account.to_account_info(),
                ctx.accounts.shares_mint.to_account_info(),
                ctx.accounts.receiving_shares_account.to_account_info(),
                ctx.accounts.depositing_underlying_account.to_account_info(),
            ],
        )?;
        Ok(())
    }
    /// withdraws `amount` of shares from the deposit tracking account into the `receiving_shares_account`.
    /// these withdrawn shares still accrue rewards, the rewards accrued are no longer tracked by the deposit
    /// tracking account
    pub fn withdraw_deposit_tracking(
        ctx: Context<WithdrawDepositTrackingAccount>,
        amount: u64,
        farm_type: [u64; 2],
    ) -> Result<()> {
        let withdraw_trait = tulipv2_sdk_common::config::lending::usdc::multi_deposit::ProgramConfig::withdraw_deposit_tracking_ix(
            *ctx.accounts.authority.key,
        );
        anchor_lang::solana_program::program::invoke(
            &withdraw_trait
                .instruction(amount, farm_type.into())
                .unwrap(),
            &[
                ctx.accounts.authority.clone(),
                ctx.accounts.clock.to_account_info(),
                ctx.accounts.deposit_tracking_account.clone(),
                ctx.accounts.deposit_tracking_pda.clone(),
                ctx.accounts.deposit_tracking_hold_account.to_account_info(),
                ctx.accounts.receiving_shares_account.to_account_info(),
                ctx.accounts.shares_mint.to_account_info(),
                ctx.accounts.vault.clone(),
            ],
        )?;
        Ok(())
    }
    /// burns/redeems the `amount` of shares for their corresponding amount
    /// of underlying asset, using the mango standalone vault as the source of funds to withdraw from
    pub fn withdraw_multi_deposit_vault_through_mango(
        ctx: Context<WithdrawMangoMultiDepositOptimizerVault>,
        amount: u64,
    ) -> Result<()> {
        // you must scope the instruction creation function the way this is done
        // otherwise stack size will be blown, as the size of the `withdraw_trait`
        // and the instruction itself can't be on the stack when the instruction is
        // invoked through cpi
        let ix = {
            let withdraw_trait = tulipv2_sdk_common::config::lending::usdc::multi_deposit::ProgramConfig::withdraw_multi_deposit_optimizer_vault(
                *ctx.accounts.common_data.authority.key,
                tulipv2_sdk_common::config::lending::Platform::MangoV3,
            ).unwrap();
            let ix = withdraw_trait.instruction(amount).unwrap();
            ix
        };
        // this instruction fails in localnet as the localnet mainnet cloned state
        // is deposited into solend
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.common_data.authority.clone(),
                ctx.accounts.common_data.multi_vault.clone(),
                ctx.accounts.common_data.multi_vault_pda.clone(),
                ctx.accounts.common_data.withdraw_vault.clone(),
                ctx.accounts.common_data.withdraw_vault_pda.clone(),
                ctx.accounts
                    .common_data
                    .platform_information
                    .to_account_info(),
                ctx.accounts.common_data.platform_config_data.clone(),
                ctx.accounts.common_data.lending_program.clone(),
                ctx.accounts
                    .common_data
                    .multi_burning_shares_token_account
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .withdraw_burning_shares_token_account
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .receiving_underlying_token_account
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .multi_underlying_withdraw_queue
                    .to_account_info(),
                ctx.accounts.common_data.multi_shares_mint.to_account_info(),
                ctx.accounts
                    .common_data
                    .withdraw_shares_mint
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .withdraw_vault_underlying_deposit_queue
                    .to_account_info(),
                ctx.accounts.mango_group_account.clone(),
                ctx.accounts.withdraw_vault_mango_account.clone(),
                ctx.accounts.mango_cache.clone(),
                ctx.accounts.mango_root_bank.clone(),
                ctx.accounts.mango_node_bank.clone(),
                ctx.accounts.mango_token_account.to_account_info(),
                ctx.accounts.mango_group_signer.clone(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.common_data.clock.to_account_info(),
            ],
        )?;
        Ok(())
    }
    /// burns/redeems the `amount` of shares for their corresponding amount
    /// of underlying asset, using the solend standalone vault as the source of funds to withdraw from
    pub fn withdraw_multi_deposit_vault_through_solend<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WithdrawSolendMultiDepositOptimizerVault<'info>>,
        amount: u64,
    ) -> Result<()> {
        // you must scope the instruction creation function the way this is done
        // otherwise stack size will be blown, as the size of the `withdraw_trait`
        // and the instruction itself can't be on the stack when the instruction is
        // invoked through cpi
        let ix = {
            let withdraw_trait = tulipv2_sdk_common::config::lending::usdc::multi_deposit::ProgramConfig::withdraw_multi_deposit_optimizer_vault(
                *ctx.accounts.common_data.authority.key,
                tulipv2_sdk_common::config::lending::Platform::Solend,
            ).unwrap();
            let ix = withdraw_trait.instruction(amount).unwrap();
            ix
        };
        /*
            this will fail in the localnet environment, as there is no sweeper service
            that sweeps funds the user has deposited. when running this on localnet
            if you check the program logs and see:
                ```
                    Program log: RUNTIME ERROR: !!WARN!! farm balance 7251615747817 < deposited 7269688550615, rebase happening before sweep
                    Program log: panicked at 'RUNTIME ERROR: !!WARN!! farm balance 7251615747817 < deposited 7269688550615, rebase happening before sweep', programs/vaults/src/vault_accounts/lending_optimizer.rs:259:17
                ```
            then this is an expected error message
        */
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.common_data.authority.clone(),
                ctx.accounts.common_data.multi_vault.clone(),
                ctx.accounts.common_data.multi_vault_pda.clone(),
                ctx.accounts.common_data.withdraw_vault.clone(),
                ctx.accounts.common_data.withdraw_vault_pda.clone(),
                ctx.accounts
                    .common_data
                    .platform_information
                    .to_account_info(),
                ctx.accounts.common_data.platform_config_data.clone(),
                ctx.accounts.common_data.lending_program.clone(),
                ctx.accounts
                    .common_data
                    .multi_burning_shares_token_account
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .withdraw_burning_shares_token_account
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .receiving_underlying_token_account
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .multi_underlying_withdraw_queue
                    .to_account_info(),
                ctx.accounts.common_data.multi_shares_mint.to_account_info(),
                ctx.accounts
                    .common_data
                    .withdraw_shares_mint
                    .to_account_info(),
                ctx.accounts.common_data.clock.to_account_info(),
                ctx.accounts.common_data.token_program.clone(),
                ctx.accounts
                    .common_data
                    .withdraw_vault_underlying_deposit_queue
                    .to_account_info(),
                ctx.remaining_accounts.get(0).unwrap().clone(), // reserve collateral
                ctx.remaining_accounts.get(1).unwrap().clone(), // reserve account
                ctx.remaining_accounts.get(2).unwrap().clone(), // reserve liquidity supply
                ctx.remaining_accounts.get(3).unwrap().clone(), // reserve collateral mint
                ctx.remaining_accounts.get(4).unwrap().clone(), // lending market
                ctx.remaining_accounts.get(5).unwrap().clone(), // lending market authority
                ctx.remaining_accounts.get(6).unwrap().clone(), // pyth price account
                ctx.remaining_accounts.get(7).unwrap().clone(), // switchboard price account
            ],
        )?;
        Ok(())
    }
    /// burns/redeems the `amount` of shares for their corresponding amount
    /// of underlying asset, using the tulip standalone vault as the source of funds to withdraw from
    pub fn withdraw_multi_deposit_vault_through_tulip<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WithdrawTulipMultiDepositOptimizerVault<'info>>,
        amount: u64,
    ) -> Result<()> {
        // you must scope the instruction creation function the way this is done
        // otherwise stack size will be blown, as the size of the `withdraw_trait`
        // and the instruction itself can't be on the stack when the instruction is
        // invoked through cpi
        let ix = {
            let withdraw_trait = tulipv2_sdk_common::config::lending::usdc::multi_deposit::ProgramConfig::withdraw_multi_deposit_optimizer_vault(
                *ctx.accounts.common_data.authority.key,
                tulipv2_sdk_common::config::lending::Platform::Tulip,
            ).unwrap();
            let ix = withdraw_trait.instruction(amount).unwrap();
            ix
        };
        // this instruction fails in localnet as the localnet mainnet cloned state
        // is deposited into solend
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.common_data.authority.clone(),
                ctx.accounts.common_data.multi_vault.clone(),
                ctx.accounts.common_data.multi_vault_pda.clone(),
                ctx.accounts.common_data.withdraw_vault.clone(),
                ctx.accounts.common_data.withdraw_vault_pda.clone(),
                ctx.accounts
                    .common_data
                    .platform_information
                    .to_account_info(),
                ctx.accounts.common_data.platform_config_data.clone(),
                ctx.accounts.common_data.lending_program.clone(),
                ctx.accounts
                    .common_data
                    .multi_burning_shares_token_account
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .withdraw_burning_shares_token_account
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .receiving_underlying_token_account
                    .to_account_info(),
                ctx.accounts
                    .common_data
                    .multi_underlying_withdraw_queue
                    .to_account_info(),
                ctx.accounts.common_data.multi_shares_mint.to_account_info(),
                ctx.accounts
                    .common_data
                    .withdraw_shares_mint
                    .to_account_info(),
                ctx.accounts.common_data.clock.to_account_info(),
                ctx.accounts.common_data.token_program.clone(),
                ctx.accounts
                    .common_data
                    .withdraw_vault_underlying_deposit_queue
                    .to_account_info(),
                ctx.remaining_accounts.get(0).unwrap().clone(), // reserve collateral
                ctx.remaining_accounts.get(1).unwrap().clone(), // reserve account
                ctx.remaining_accounts.get(2).unwrap().clone(), // reserve liquidity supply
                ctx.remaining_accounts.get(3).unwrap().clone(), // reserve collateral mint
                ctx.remaining_accounts.get(4).unwrap().clone(), // lending market
                ctx.remaining_accounts.get(5).unwrap().clone(), // lending market authority
                ctx.remaining_accounts.get(6).unwrap().clone(), // pyth price account
            ],
        )?;
        Ok(())
    }
    pub fn deposit_reserve_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, DepositReserveLiquidity<'info>>,
        amount: u64,
    ) -> Result<()> {
        tulipv2_sdk_lending::helpers::refresh_reserve(
            &ctx.accounts.lending_program,
            &ctx.accounts.clock.to_account_info(),
            &ctx.accounts.reserve,
            &ctx.accounts.pyth_price_account,
        )?;
        tulipv2_sdk_lending::helpers::deposit_reserve_liquidity(
            &ctx.accounts.lending_program,
            &ctx.accounts.source_liquidity_token_account.to_account_info(),
            &ctx.accounts.destination_collateral.to_account_info(),
            &ctx.accounts.reserve,
            &ctx.accounts.reserve_liquidity.to_account_info(),
            &ctx.accounts.reserve_collateral_mint.to_account_info(),
            &ctx.accounts.lending_market,
            &ctx.accounts.lending_market_authority,
            &ctx.accounts.authority,
            &ctx.accounts.clock.to_account_info(),
            &ctx.accounts.token_program,
            &[],
            amount
        )?;
        Ok(())
    }
    pub fn redeem_reserve_collateral<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, RedeemReserveLiquidity<'info>>,
        amount: u64,
    ) -> Result<()> {
        tulipv2_sdk_lending::helpers::refresh_reserve(
            &ctx.accounts.lending_program,
            &ctx.accounts.clock.to_account_info(),
            &ctx.accounts.reserve,
            &ctx.accounts.pyth_price_account,
        )?;
        tulipv2_sdk_lending::helpers::redeem_reserve_collateral(
            &ctx.accounts.lending_program,
            &ctx.accounts.source_collateral.to_account_info(),
            &ctx.accounts.destination_liquidity.to_account_info(),
            &ctx.accounts.reserve,
            &ctx.accounts.reserve_collateral_mint.to_account_info(),
            &ctx.accounts.reserve_liquidity.to_account_info(),
            &ctx.accounts.lending_market,
            &ctx.accounts.lending_market_authority,
            &ctx.accounts.authority,
            &ctx.accounts.clock.to_account_info(),
            &ctx.accounts.token_program,
            &[],
            amount
        )?;
        Ok(())
    }
    pub fn create_user_farm<'info>(
        ctx: Context<CreateUserFarm>,
        farm: u64,
    ) -> Result<()> {
        {
            let ix = spl_associated_token_account::create_associated_token_account(
                ctx.accounts.authority.key,
                ctx.accounts.user_farm.key,
                ctx.accounts.lp_token_mint.key,
            );
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.authority.clone(),
                    ctx.accounts.user_farm.clone(),
                    ctx.accounts.lp_token_mint.clone(),
                    ctx.accounts.token_program.clone(),
                    ctx.accounts.user_farm_lp_token_account.clone(),
                    ctx.accounts.rent.clone(),
                ]
            )?
        }
        {
            let ix = spl_associated_token_account::create_associated_token_account(
                ctx.accounts.authority.key,
                ctx.accounts.user_farm.key,
                ctx.accounts.base_token_mint.key,
            );
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.authority.clone(),
                    ctx.accounts.user_farm.clone(),
                    ctx.accounts.base_token_mint.clone(),
                    ctx.accounts.token_program.clone(),
                    ctx.accounts.user_farm_base_token_account.clone(),
                    ctx.accounts.rent.clone(),
                ]
            )?
        }
        {
            let ix = spl_associated_token_account::create_associated_token_account(
                ctx.accounts.authority.key,
                ctx.accounts.user_farm.key,
                ctx.accounts.quote_token_mint.key,
            );
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.authority.clone(),
                    ctx.accounts.user_farm.clone(),
                    ctx.accounts.quote_token_mint.clone(),
                    ctx.accounts.token_program.clone(),
                    ctx.accounts.user_farm_quote_token_account.clone(),
                    ctx.accounts.rent.clone(),
                ]
            )?
        }
        let farm = tulipv2_sdk_levfarm::accounts::Farms::from(farm);
        let ix = tulipv2_sdk_levfarm::helpers::new_create_user_farm_ix(
            ctx.accounts.authority.key(),
            farm,
        ).unwrap();
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.authority.clone(),
                ctx.accounts.user_farm.clone(),
                ctx.accounts.user_farm_obligation.clone(),
                ctx.accounts.lending_market.clone(),
                ctx.accounts.global.clone(),
                ctx.accounts.leveraged_farm.clone(),
                ctx.accounts.clock.clone(),
                ctx.accounts.rent.clone(),
                ctx.accounts.system_program.clone(),
                ctx.accounts.lending_program.clone(),
                ctx.accounts.token_program.clone(),
                ctx.accounts.obligation_vault_address.clone(),
            ]
        )?;
        Ok(())
    }
    pub fn create_user_farm_obligation<'info>(
        ctx: Context<CreateUserFarmObligation>,
        farm: u64,
        obligation_index: u64,
    ) -> Result<()> {
        let farm = tulipv2_sdk_levfarm::accounts::Farms::from(farm);
        let ix = tulipv2_sdk_levfarm::helpers::new_create_user_farm_obligation_ix(
            ctx.accounts.authority.key(),
            ctx.accounts.user_farm.key(),
            farm,
            obligation_index,
        ).unwrap();
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.authority.clone(),
                ctx.accounts.user_farm.clone(),
                ctx.accounts.leveraged_farm.clone(),
                ctx.accounts.user_farm_obligation.clone(),
                ctx.accounts.lending_market.clone(),
                ctx.accounts.obligation_vault_address.clone(),
                ctx.accounts.clock.clone(),
                ctx.accounts.rent.clone(),
                ctx.accounts.lending_program.clone(),
                ctx.accounts.token_program.clone(),
                ctx.accounts.system_program.clone(),
            ]
        )?;
        Ok(())
    }
    pub fn deposit_borrow_dual<'info>(
        ctx: Context<DepositBorrowDual<'info>>,
        coin_amount: u64,
        pc_amount: u64,
        coin_borrow: u64,
        pc_borrow: u64,
        obligation_index: u8,
    ) -> Result<()> {
        let deposit_borrow_dual: tulipv2_sdk_levfarm::instructions::deposit_borrow_dual::DepositBorrowDual = ctx.accounts.into();
        let ix = tulipv2_sdk_levfarm::helpers::new_deposit_borrow_dual_ix(
            deposit_borrow_dual,
            ctx.accounts.position_info_account.key(),
            ctx.accounts.system_program.key(),
            coin_amount,
            pc_amount,
            coin_borrow,
            pc_borrow,
            obligation_index,
        ).unwrap();
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.authority.clone(),
                ctx.accounts.user_farm.clone(),
                ctx.accounts.leveraged_farm.clone(),
                ctx.accounts.user_farm_obligation.clone(),
                ctx.accounts.coin_source_token_account.clone(),
                ctx.accounts.coin_destination_token_account.clone(),
                ctx.accounts.pc_source_token_account.clone(),
                ctx.accounts.pc_destination_token_account.clone(),
                ctx.accounts.pc_deposit_reserve_account.clone(),
                ctx.accounts.coin_deposit_reserve_account.clone(),
                ctx.accounts.coin_reserve_liquidity_oracle.clone(),
                ctx.accounts.pc_reserve_liquidity_oracle.clone(),
                ctx.accounts.lending_market_account.clone(),
                ctx.accounts.derived_lending_market_authority.clone(),
                ctx.accounts.token_program.clone(),
                ctx.accounts.lending_program.clone(),
                ctx.accounts.coin_source_reserve_liquidity_token_account.clone(),
                ctx.accounts.pc_source_reserve_liquidity_token_account.clone(),
                ctx.accounts.coin_reserve_liquidity_fee_receiver.clone(),
                ctx.accounts.pc_reserve_liquidity_fee_receiver.clone(),
                ctx.accounts.borrow_authorizer.clone(),
                ctx.accounts.lp_pyth_price_account.clone(),
                ctx.accounts.vault_account.clone(),
                ctx.accounts.rent.clone(),
                ctx.accounts.position_info_account.clone(),
                ctx.accounts.system_program.clone(),
            ]
        )?;
        Ok(())
    }
    pub fn swap_tokens_raydium_stats<'info>(
        ctx: Context<'_, '_, '_, 'info, RaydiumSwap<'info>>,
        obligation_index: u64,
    ) -> Result<()> {
        let ix = {
            let swap_tokens: Box<tulipv2_sdk_levfarm::instructions::swap_tokens_raydium_stats::RaydiumSwap> = Box::new(ctx.accounts.into());
            tulipv2_sdk_levfarm::helpers::new_swap_tokens_raydium_stats_ix(
                swap_tokens,
                ctx.remaining_accounts.get(0).unwrap().key(),
                ctx.remaining_accounts.get(1).unwrap().key(),
                ctx.remaining_accounts.get(2).unwrap().key(),
                ctx.remaining_accounts.get(3).unwrap().key(),
                obligation_index as u8,
            ).unwrap()
        };
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.authority.clone(),
                ctx.accounts.leveraged_farm.clone(),
                ctx.accounts.user_farm.clone(),
                ctx.accounts.user_farm_obligation.clone(),
                ctx.accounts.token_program.clone(),
                ctx.accounts.vault_signer.clone(),
                ctx.accounts.swap_or_liquidity_program_id.clone(),
                ctx.accounts.amm_id.clone(),
                ctx.accounts.amm_authority.clone(),
                ctx.accounts.amm_open_orders.clone(),
                ctx.accounts.amm_quantities_or_target_orders.clone(),
                ctx.accounts.pool_coin_tokenaccount.clone(),
                ctx.accounts.pool_pc_tokenaccount.clone(),
                ctx.accounts.serum_program_id.clone(),
                ctx.accounts.serum_market.clone(),
                ctx.accounts.serum_bids.clone(),
                ctx.accounts.serum_asks.clone(),
                ctx.accounts.serum_event_queue.clone(),
                ctx.accounts.serum_coin_vault_account.clone(),
                ctx.accounts.serum_pc_vault_account.clone(),
                ctx.accounts.serum_vault_signer.clone(),
                ctx.accounts.coin_wallet.clone(),
                ctx.accounts.pc_wallet.clone(),
                ctx.remaining_accounts.get(0).unwrap().clone(),
                ctx.remaining_accounts.get(1).unwrap().clone(),
                ctx.remaining_accounts.get(2).unwrap().clone(),
                ctx.remaining_accounts.get(3).unwrap().clone(),
            ],
        )?;
        Ok(())
    }
    pub fn add_liquidity_stats<'info>(
        ctx: Context<'_, '_, '_, 'info, AddLiquidity<'info>>,
        obligation_index: u64
    ) -> Result<()> {
        {
            let ix = {
                let add_liq: Box<tulipv2_sdk_levfarm::instructions::add_liquidity_stats::AddLiquidity> = Box::new(ctx.accounts.into());
                tulipv2_sdk_levfarm::helpers::new_add_liquidity_stats_ix(
                    add_liq,
                    ctx.remaining_accounts.get(0).unwrap().key(),
                    obligation_index as u8
                ).unwrap()
            };
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.authority.to_account_info(),
                    ctx.accounts.user_farm.to_account_info(),
                    ctx.accounts.leveraged_farm.to_account_info(),
                    ctx.accounts.liquidity_program_id.to_account_info(),
                    ctx.accounts.amm_id.to_account_info(),
                    ctx.accounts.amm_authority.to_account_info(),
                    ctx.accounts.amm_open_orders.to_account_info(),
                    ctx.accounts.amm_quantities_or_target_orders.to_account_info(),
                    ctx.accounts.lp_mint_address.to_account_info(),
                    ctx.accounts.pool_coin_token_account.to_account_info(),
                    ctx.accounts.pool_pc_token_account.to_account_info(),
                    ctx.accounts.serum_market.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.lev_farm_coin_token_account.to_account_info(),
                    ctx.accounts.lev_farm_pc_token_account.to_account_info(),
                    ctx.accounts.user_lp_token_account.to_account_info(),
                    ctx.accounts.pyth_price_account.to_account_info(),
                    ctx.accounts.lending_market_account.to_account_info(),
                    ctx.accounts.user_farm_obligation.to_account_info(),
                    ctx.accounts.derived_lending_market_authority.to_account_info(),
                    ctx.accounts.lending_program.to_account_info(),
                    ctx.accounts.clock.to_account_info(),
                    ctx.accounts.dex_program.to_account_info(),
                    ctx.remaining_accounts.get(0).unwrap().clone(),
                ]
            )?;
        }
        Ok(())
    }
    // you would likely want to provide the nonce values off-chain in a struct
    // to avoid the pubkey derivation costs
    pub fn deposit_raydium_vault<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, DepositLevFarm<'info>>,
        obligation_index: u64,
    ) -> Result<()> {
        {
            let ix = {
            let deposit_vault:  Box<tulipv2_sdk_levfarm::instructions::deposit_raydium_vault::DepositFarm> = Box::new(ctx.accounts.into());
            tulipv2_sdk_levfarm::helpers::new_deposit_raydium_vault_ix(
                deposit_vault,
                ctx.accounts.lending_market_account.key(),
                ctx.accounts.user_farm_obligation.key(),
                ctx.accounts.lending_market_authority.key(),
                ctx.accounts.lending_program.key(),
                obligation_index
            ).unwrap()
            };
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.authority.clone(),
                    ctx.accounts.user_farm.clone(),
                    ctx.accounts.obligation_vault_address.clone(),
                    ctx.accounts.leveraged_farm.clone(),
                    ctx.accounts.vault_program.clone(),
                    ctx.accounts.authority_token_account.to_account_info(),
                    ctx.accounts.vault_pda_account.clone(),
                    ctx.accounts.vault.clone(),
                    ctx.accounts.lp_token_account.to_account_info(),
                    ctx.accounts.user_balance_account.clone(),
                    ctx.accounts.system_program.clone(),
                    ctx.accounts.stake_program_id.clone(),
                    ctx.accounts.pool_id.clone(),
                    ctx.accounts.pool_authority.clone(),
                    ctx.accounts.vault_info_account.clone(),
                    ctx.accounts.pool_lp_token_account.to_account_info(),
                    ctx.accounts.user_reward_a_token_account.to_account_info(),
                    ctx.accounts.pool_reward_a_token_account.to_account_info(),
                    ctx.accounts.user_reward_b_token_account.to_account_info(),
                    ctx.accounts.pool_reward_b_token_account.to_account_info(),
                    ctx.accounts.clock.clone(),
                    ctx.accounts.rent.clone(),
                    ctx.accounts.token_program_id.clone(),
                    ctx.accounts.user_balance_metadata.clone(),
                    ctx.accounts.lending_market_account.clone(),
                    ctx.accounts.user_farm_obligation.clone(),
                    ctx.accounts.lending_market_authority.clone(),
                    ctx.accounts.lending_program.clone(),
                ]
            )?;
        };
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositBorrowDualArgs {
    coin_amount: u64,
    pc_amount: u64,
    coin_borrow: u64,
    pc_borrow: u64,
    obligation_index: u8,
}

#[derive(Accounts)]
pub struct RedeemReserveLiquidity<'info> {
    /// CHECK: ..
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK: ..
    #[account(mut)]
    pub source_collateral: Box<Account<'info, TokenAccount>>,
    /// CHECK: ..
    #[account(mut)]
    pub destination_liquidity: Box<Account<'info, TokenAccount>>,
    /// CHECK: ..
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    /// CHECK: ..
    #[account(mut)]
    pub reserve_liquidity: Box<Account<'info, TokenAccount>>,
    /// CHECK: ..
    #[account(mut)]
    reserve_collateral_mint: Box<Account<'info, Mint>>,
    /// CHECK: ..
    pub lending_market: AccountInfo<'info>,
    /// CHECK: ..
    pub lending_market_authority: AccountInfo<'info>,
    /// CHECK: ..
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: ..
    pub lending_program: AccountInfo<'info>,
    /// CHECK: ..
    pub token_program: AccountInfo<'info>,
    /// CHECK: ..
    pub pyth_price_account: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DepositReserveLiquidity<'info> {
    /// CHECK: ..
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK: ..
    #[account(mut)]
    pub source_liquidity_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: ..
    #[account(mut)]
    pub destination_collateral: Box<Account<'info, TokenAccount>>,
    /// CHECK: ..
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    /// CHECK: ..
    #[account(mut)]
    pub reserve_liquidity: Box<Account<'info, TokenAccount>>,
    /// CHECK: ..
    #[account(mut)]
    reserve_collateral_mint: Box<Account<'info, Mint>>,
    /// CHECK: ..
    pub lending_market: AccountInfo<'info>,
    /// CHECK: ..
    pub lending_market_authority: AccountInfo<'info>,
    /// CHECK: ..
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: ..
    pub lending_program: AccountInfo<'info>,
    /// CHECK: ..
    pub token_program: AccountInfo<'info>,
    /// CHECK: ..
    pub pyth_price_account: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct RegisterDepositTrackingAccount<'info> {
    #[account(mut, signer)]
    /// CHECK: .
    pub authority: AccountInfo<'info>,
    /// CHECK: .
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub deposit_tracking_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub deposit_tracking_queue_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub deposit_tracking_hold_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub shares_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK: .
    pub underlying_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK: .
    pub deposit_tracking_pda: AccountInfo<'info>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
    /// CHECK: .
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: .
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: .
    pub system_program: Program<'info, System>,
    /// CHECK: .
    pub vault_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct IssueSharesInstruction<'info> {
    #[account(signer)]
    /// CHECK: .
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub deposit_tracking_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub deposit_tracking_pda: AccountInfo<'info>,
    /// CHECK: .
    pub vault_pda: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub shares_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK: .
    /// the account which will receive the issued shares
    /// this is the deposit_tracking_hold_account
    pub receiving_shares_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: .
    /// the account owned by the authority which contains the underlying tokens
    /// we want to deposit in exchange for the vault shares
    pub depositing_underlying_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: .
    /// the underlying token account that is owned by the vault pda
    /// which holds the underlying tokens until they are swept into the farm.
    ///
    /// also known as the deposit queue account
    pub vault_underlying_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    pub system_program: Program<'info, System>,
    /// CHECK: .
    pub vault_program: AccountInfo<'info>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawDepositTrackingAccount<'info> {
    #[account(signer)]
    /// CHECK: .
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub deposit_tracking_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub deposit_tracking_pda: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub deposit_tracking_hold_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: .
    /// this is the token account owned by the authority that
    /// should receive the tokenized shares which are being removed
    /// from the deposit tracking account. do note that this means
    /// these shares are no longer being tracked by the deposit tracking
    /// account, and any newly accrued rewards tracked by the deposit tracking
    /// account will reflect the remaining balance that hasn't been withdrawn
    ///
    /// **the shares that are being withdrawn still accrue rewards the same as shares that are held by the deposit tracking account**
    pub receiving_shares_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    pub shares_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub vault: AccountInfo<'info>,
    /// CHECK: .
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: .
    pub vault_program: AccountInfo<'info>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawMultiDepositOptimizerVault<'info> {
    #[account(signer)]
    /// CHECK: .
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub multi_vault: AccountInfo<'info>,
    /// CHECK: .
    pub multi_vault_pda: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub withdraw_vault: AccountInfo<'info>,
    /// CHECK: .
    pub withdraw_vault_pda: AccountInfo<'info>,
    /// CHECK: .
    pub platform_information:
        Box<Account<'info, tulipv2_sdk_vaults::accounts::lending_optimizer::LendingPlatformV1>>,
    /// CHECK: .
    pub platform_config_data: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    /// this is the token account owned by the authority for the multi vault
    /// shares mint, which are the tokens we are burning/redeeming in exchange
    /// for the underlying asset
    pub multi_burning_shares_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: .
    /// this is the account owned by the multi vault pda that holds the tokenized
    /// shares issued by the withdraw vault.
    pub withdraw_burning_shares_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: .
    /// this is the account owned by the authority which will receive the underlying
    pub receiving_underlying_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: .
    /// this is the underlying token account owned by the multi deposit vault
    /// which is used to temporarily store tokens during the token withdraw process
    pub multi_underlying_withdraw_queue: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    /// CHECK: .
    pub multi_shares_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK: .
    pub withdraw_shares_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK: .
    /// this is the underlying token account owned by the withdraw vault we are
    /// removing underlying assets from
    pub withdraw_vault_underlying_deposit_queue: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
    /// CHECK: .
    pub lending_program: AccountInfo<'info>,
    /// CHECK: .
    pub vault_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawMangoMultiDepositOptimizerVault<'info> {
    /// configuration data common to all multi deposit withdraw instructions
    /// regardless of the underlying vault htey are withdrawing from
    /// CHECK: .
    pub common_data: WithdrawMultiDepositOptimizerVault<'info>,
    /// CHECK: .
    pub mango_group_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub withdraw_vault_mango_account: AccountInfo<'info>,
    /// CHECK: .
    pub mango_cache: AccountInfo<'info>,
    /// CHECK: .
    pub mango_root_bank: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub mango_node_bank: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: .
    pub mango_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    pub mango_group_signer: AccountInfo<'info>,
    /// CHECK: .
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSolendMultiDepositOptimizerVault<'info> {
    /// configuration data common to all multi deposit withdraw instructions
    /// regardless of the underlying vault htey are withdrawing from
    /// CHECK: .
    pub common_data: WithdrawMultiDepositOptimizerVault<'info>,
}

#[derive(Accounts)]
pub struct WithdrawTulipMultiDepositOptimizerVault<'info> {
    /// configuration data common to all multi deposit withdraw instructions
    /// regardless of the underlying vault htey are withdrawing from
    /// CHECK: .
    pub common_data: WithdrawMultiDepositOptimizerVault<'info>,
}

#[derive(Accounts)]
pub struct CreateUserFarm<'info> {
    #[account(signer)]
    /// CHECK: .
    pub authority: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    /// CHECK: .
    pub global: AccountInfo<'info>,
    /// CHECK: .
    pub leveraged_farm: AccountInfo<'info>,
    /// CHECK: .
    pub clock: AccountInfo<'info>,
    /// CHECK: .
    pub rent: AccountInfo<'info>,
    /// CHECK: .
    pub system_program: AccountInfo<'info>,
    /// CHECK: .
    pub lending_program: AccountInfo<'info>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm_lp_token_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm_base_token_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm_quote_token_account: AccountInfo<'info>,
    /// CHECK: .
    /// CHECK: .
    pub lp_token_mint: AccountInfo<'info>,
    /// CHECK: .
    pub base_token_mint: AccountInfo<'info>,
    /// CHECK: .
    pub quote_token_mint: AccountInfo<'info>,
    /// CHECK: .
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: .
    pub tulip_leveraged_farm_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateUserFarmObligation<'info> {
    #[account(mut, signer)]
    /// CHECK: .
    pub authority: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    /// CHECK: .
    pub leveraged_farm: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
    /// CHECK: .
    pub clock: AccountInfo<'info>,
    /// CHECK: .
    pub rent: AccountInfo<'info>,
    /// CHECK: .
    pub lending_program: AccountInfo<'info>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
    /// CHECK: .
    pub system_program: AccountInfo<'info>,
    /// CHECK: .
    pub tulip_leveraged_farm_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositBorrowDual<'info> {
    #[account(signer)]
    /// CHECK: .
    pub authority: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    /// CHECK: .
    pub leveraged_farm: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub coin_source_token_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub coin_destination_token_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pc_source_token_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pc_destination_token_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub coin_deposit_reserve_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pc_deposit_reserve_account: AccountInfo<'info>,
    /// CHECK: .
    pub coin_reserve_liquidity_oracle: AccountInfo<'info>,
    /// CHECK: .
    pub pc_reserve_liquidity_oracle: AccountInfo<'info>,
    /// CHECK: .
    pub lending_market_account: AccountInfo<'info>,
    /// CHECK: .
    pub derived_lending_market_authority: AccountInfo<'info>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
    /// CHECK: .
    pub lending_program: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub coin_source_reserve_liquidity_token_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pc_source_reserve_liquidity_token_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub coin_reserve_liquidity_fee_receiver: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pc_reserve_liquidity_fee_receiver: AccountInfo<'info>,
    /// CHECK: .
    pub borrow_authorizer: AccountInfo<'info>,
    /// CHECK: .
    pub lp_pyth_price_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,
    /// CHECK: .
    pub rent: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub position_info_account: AccountInfo<'info>,
    /// CHECK: .
    pub system_program: AccountInfo<'info>,
    /// CHECK: .
    pub tulip_leveraged_farm_program: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct DepositLevFarm<'info> {
    /// CHECK: .
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    /// CHECK: .
    pub vault_program: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub authority_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    #[account(mut)]
    pub vault_pda_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub lp_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    #[account(mut)]
    pub user_balance_account: AccountInfo<'info>,
    /// CHECK: .
    pub system_program: AccountInfo<'info>,
    /// CHECK: .
    pub stake_program_id: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pool_id: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pool_authority: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub vault_info_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pool_lp_token_account:Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    #[account(mut)]
    pub user_reward_a_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    #[account(mut)]
    pub pool_reward_a_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    #[account(mut)]
    pub user_reward_b_token_account:Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    #[account(mut)]
    pub pool_reward_b_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    pub clock: AccountInfo<'info>,
    /// CHECK: .
    pub rent: AccountInfo<'info>,
    /// CHECK: .
    pub token_program_id: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_balance_metadata: AccountInfo<'info>,
    /// CHECK: .
    pub lending_market_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    /// CHECK: .
    pub lending_market_authority: AccountInfo<'info>,
    /// CHECK: .
    pub lending_program: AccountInfo<'info>,
    /// CHECK: .
    pub tulip_leveraged_farm_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RaydiumSwap<'info> {
    /// CHECK: .
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
    /// CHECK: .
    pub vault_signer: AccountInfo<'info>,
    /// CHECK: .
    pub swap_or_liquidity_program_id: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub amm_id: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub amm_quantities_or_target_orders: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pool_coin_tokenaccount: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pool_pc_tokenaccount: AccountInfo<'info>,
    /// CHECK: .
    pub serum_program_id: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub serum_vault_signer: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub coin_wallet: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub pc_wallet: AccountInfo<'info>,
    /// CHECK: .
    pub tulip_leveraged_farm_program: AccountInfo<'info>,
    ///// CHECK: .
    //pub lending_market: AccountInfo<'info>,
    ///// CHECK: .
    //pub lending_market_authority: AccountInfo<'info>,
    ///// CHECK: .
    //pub lending_program: AccountInfo<'info>,
    //#[account(mut)]
    ///// CHECK: .
    //pub position_info_account: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    /// CHECK: .
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    /// CHECK: .
    pub liquidity_program_id: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub amm_id: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub amm_quantities_or_target_orders: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub lp_mint_address: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub pool_coin_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_pc_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub lev_farm_coin_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub lev_farm_pc_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_lp_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: .
    pub pyth_price_account: AccountInfo<'info>,
    /// CHECK: .
    pub lending_market_account: AccountInfo<'info>,
    /// CHECK: .
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    /// CHECK: .
    pub derived_lending_market_authority: AccountInfo<'info>,
    /// CHECK: .
    pub lending_program: AccountInfo<'info>,
    /// CHECK: .
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: .
    pub dex_program: AccountInfo<'info>,
    /// CHECK: .
    pub tulip_leveraged_farm_program: AccountInfo<'info>,
}
