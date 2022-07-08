use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use tulipv2_sdk_common::config::deposit_tracking::traits::IssueShares;
use tulipv2_sdk_common::config::deposit_tracking::traits::RegisterDepositTracking;
use tulipv2_sdk_common::config::deposit_tracking::traits::WithdrawDepositTracking;
use tulipv2_sdk_common::config::lending::traits::WithdrawMultiOptimizerVault;
use tulipv2_sdk_common::msg_panic;
use tulipv2_sdk_farms::Farm;
use tulipv2_sdk_vaults::instructions::{
    deposit_tracking::{
        new_register_deposit_tracking_account_ix, new_withdraw_deposit_tracking_ix,
    },
    multi_deposit_optimizer::new_withdraw_multi_deposit_optimizer_vault_ix,
    new_issue_shares_ix,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod examples {
    use super::*;
    pub fn log_exchange_rate(
        ctx: Context<LogExchangeRate>,
        farm_type: [u64; 2]
    ) -> Result<()> {
        use tulipv2_sdk_common::traits::vault::TokenizedShares;
        let farm: Farm = farm_type.into();
        match farm {
            tulipv2_sdk_farms::Farm::Orca {
                name,
            } => {
                if name.is_double_dip() {
                    let loader: AccountLoader<tulipv2_sdk_vaults::accounts::orca_vault::OrcaDoubleDipVaultV1> = AccountLoader::try_from_unchecked(ctx.accounts.vault_program.key, &ctx.accounts.vault)?;
                    let vault = loader.load()?;
                    msg!("{}", vault.base.cached_exchange_rate(&ctx.accounts.shares_mint));
                } else {
                    let loader: AccountLoader<tulipv2_sdk_vaults::accounts::orca_vault::OrcaVaultV1> = AccountLoader::try_from_unchecked(ctx.accounts.vault_program.key, &ctx.accounts.vault)?;
                    let vault = loader.load()?;
                    msg!("{}", vault.base.cached_exchange_rate(&ctx.accounts.shares_mint));
                };
            }
            tulipv2_sdk_farms::Farm::Raydium {
                ..
            } => {
                let loader: AccountLoader<tulipv2_sdk_vaults::accounts::raydium_vault::RaydiumVaultV1> = AccountLoader::try_from_unchecked(ctx.accounts.vault_program.key, &ctx.accounts.vault)?;
                let vault = loader.load()?;
                msg!("{}", vault.base.cached_exchange_rate(&ctx.accounts.shares_mint));
            }
            tulipv2_sdk_farms::Farm::Lending {
                name
            } => {
                if name.eq(&tulipv2_sdk_farms::lending::Lending::MULTI_DEPOSIT) {
                    let loader: AccountLoader<tulipv2_sdk_vaults::accounts::multi_optimizer::MultiDepositOptimizerV1> = AccountLoader::try_from_unchecked(ctx.accounts.vault_program.key, &ctx.accounts.vault)?;
                    let vault = loader.load()?;

                    msg!("{}", vault.base.cached_exchange_rate(&ctx.accounts.shares_mint));
                } else {
                    let loader: AccountLoader<tulipv2_sdk_vaults::accounts::lending_optimizer::LendingOptimizerV1> = AccountLoader::try_from_unchecked(ctx.accounts.vault_program.key, &ctx.accounts.vault)?;
                    let vault = loader.load()?;
                    msg!("{}", vault.base.cached_exchange_rate(&ctx.accounts.shares_mint));
                }
            }
            _ => panic!("unsupported farm"),
        }
        Ok(())
    }
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
    /// if desired. generally speaking this should only be done if you want to
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
            &ctx.accounts
                .source_liquidity_token_account
                .to_account_info(),
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
            amount,
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
            amount,
        )?;
        Ok(())
    }
    pub fn withdraw_raydium_vault<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WithdrawRaydiumVault<'info>>,
        amount: u64,
    ) -> Result<()> {
        /// although this is not really needed since the configuration information
        /// is provided through the context, it's done to showcase how to use the sdk
        let raydium_vault_config = tulipv2_sdk_vaults::config::raydium::RaydiumVaultConfig::new(
            ctx.accounts.vault.key(),
            ctx.accounts.underlying_withdraw_queue.mint,
            ctx.accounts.pool_id.key(),
            ctx.accounts.raydium_stake_program.key(),
            ctx.accounts.pool_reward_a_token_account.mint,
            ctx.accounts.pool_reward_b_token_account.mint,
        );
        raydium_vault_config.instruction(
            ctx.accounts.authority.key(),
            ctx.accounts.pool_id.key(),
            ctx.accounts.pool_authority.key(),
            ctx.accounts.pool_lp_token_account.key(),
        )
        Ok(())
    }
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
    /// regardless of the underlying vault they are withdrawing from
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
    /// regardless of the underlying vault they are withdrawing from
    /// CHECK: .
    pub common_data: WithdrawMultiDepositOptimizerVault<'info>,
}

#[derive(Accounts)]
pub struct WithdrawTulipMultiDepositOptimizerVault<'info> {
    /// configuration data common to all multi deposit withdraw instructions
    /// regardless of the underlying vault they are withdrawing from
    /// CHECK: .
    pub common_data: WithdrawMultiDepositOptimizerVault<'info>,
}



#[derive(Accounts)]
pub struct WithdrawRaydiumVault<'info> {
    /// .
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountLoader<'info, RaydiumVaultV1>,
    /// .
    #[account(mut)]
    pub vault_pda: AccountInfo<'info>,
    /// .
    #[account(mut)]
    pub vault_stake_info_account: AccountInfo<'info>,
    /// .
    #[account(mut)]
    pub pool_id: AccountInfo<'info>,
    /// .
    #[account(mut)]
    pub pool_authority: AccountInfo<'info>,
    #[account(mut)]
    pub underlying_withdraw_queue: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_lp_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub vault_reward_a_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_reward_a_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub vault_reward_b_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_reward_b_token_account: Box<Account<'info, TokenAccount>>,
    /// the account from which shares will be burned in exchange for
    /// a corresponding amount of lp tokens
    #[account(mut)]
    pub burning_shares_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub receiving_underlying_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub shares_mint: Box<Account<'info, Mint>>,
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: .
    pub token_program: AccountInfo<'info>,
    /// CHECK: .
    pub raydium_stake_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LogExchangeRate<'info> {
    pub vault: AccountInfo<'info>,
    pub shares_mint: Box<Account<'info, Mint>>,
    pub vault_program: AccountInfo<'info>,
}