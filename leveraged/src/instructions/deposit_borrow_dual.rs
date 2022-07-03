use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct DepositBorrowDual<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    pub leveraged_farm: AccountInfo<'info>,
    // this is the obligation account
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    #[account(mut)]
    pub coin_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub coin_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub coin_deposit_reserve_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_deposit_reserve_account: AccountInfo<'info>,
    pub coin_reserve_liquidity_oracle: AccountInfo<'info>,
    pub pc_reserve_liquidity_oracle: AccountInfo<'info>,
    pub lending_market_account: AccountInfo<'info>,
    pub derived_lending_market_authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub lending_program: AccountInfo<'info>,
    #[account(mut)]
    pub coin_source_reserve_liquidity_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub pc_source_reserve_liquidity_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub coin_reserve_liquidity_fee_receiver: AccountInfo<'info>,
    #[account(mut)]
    pub pc_reserve_liquidity_fee_receiver: AccountInfo<'info>,
    pub borrow_authorizer: AccountInfo<'info>,
    pub lp_pyth_price_account: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn deposit_borrow_dual<'info>(
    accounts: DepositBorrowDual<'info>,
    position_info_account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    coin_amount: u64,
    pc_amount: u64,
    coin_borrow_amount: u64,
    pc_borrow_amount: u64,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("deposit_borrow_dual")?;
    let mut ix_data = Vec::with_capacity((8 * 5) + 1);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&coin_amount).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&pc_amount).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&coin_borrow_amount).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&pc_borrow_amount).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());


    let mut accounts = accounts.to_account_metas(None);
    accounts.push(AccountMeta::new(position_info_account.key(), false));
    accounts.push(AccountMeta::new_readonly(system_program.key(), false));

    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts,
        data: ix_data,
    })
}