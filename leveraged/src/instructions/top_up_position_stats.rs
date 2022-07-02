use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct DepositObligationCollateral<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    pub leveraged_farm:AccountInfo<'info>,
    // this is the obligation account
    #[account(mut)]
    pub user_farm_obligation: AccountInfo<'info>,
    #[account(mut)]
    pub coin_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    // false positive
    //#[soteria(ignore)]
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
    pub clock: Sysvar<'info, Clock>,
    pub lending_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

pub fn top_up_position_stats<'info>(
    accounts: DepositObligationCollateral<'info>,
    coin_amount: u64,
    pc_amount: u64,
    obligation_index: u8,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("top_up_position_stats")?;
    let mut ix_data = Vec::with_capacity((8 * 3) + 1);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&coin_amount).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&pc_amount).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}