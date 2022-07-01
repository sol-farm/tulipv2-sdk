use anchor_lang::{prelude::*, accounts::loader::Loader};
use solana_program::entrypoint::ProgramResult;
use tulipv2_sdk_common::math::decimal::Decimal;
use tulipv2_sdk_common::math::common::TryMul;

pub fn init_position_info_account<'info>(
    authority: &AccountInfo<'info>,
    position_info_account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    user_farm: &AccountInfo<'info>,
    coin_price: Decimal,
    pc_price: Decimal,
    obligation_index: u8,
    coin_deposit_amount: u64,
    pc_deposit_amount: u64,
    clock: u32,
) -> ProgramResult {
    let rent = Rent::get()?;
    let (position_info_addr, position_info_nonce) = crate::accounts::derivations::derive_user_position_info_address(
        *user_farm.key,
        crate::ID,
        obligation_index,
    );

    assert_eq!(position_info_addr, *position_info_account.key);

    if position_info_account.data_is_empty() {
        // create position info account
        {
            let ix = anchor_lang::solana_program::system_instruction::create_account(
                &authority.key,
                position_info_account.key,
                rent.minimum_balance(crate::accounts::POSITION_INFO_ACCOUNT_SIZE),
                crate::accounts::POSITION_INFO_ACCOUNT_SIZE as u64,
                &crate::ID,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    authority.clone(),
                    position_info_account.clone(),
                    system_program.clone(),
                ],
                &[&[
                    b"position_info",
                    user_farm.key.as_ref(),
                    &(obligation_index as u64).to_le_bytes()[..],
                    &[position_info_nonce],
                ]],
            )?;
        }

        // set position info account values
        {
            let loader: Loader<crate::accounts::position_info::PositionInfo> =
                Loader::try_from_unchecked(&crate::ID, position_info_account)?;
            {
                let mut position_info_accnt = loader.load_init()?;
                position_info_accnt.nonce = position_info_nonce;
                position_info_accnt.coin_deposit = coin_deposit_amount;
                position_info_accnt.pc_deposit = pc_deposit_amount;
                position_info_accnt.open_time = clock;
                position_info_accnt.open_coin_cost = (coin_price.try_mul(coin_deposit_amount).unwrap()).try_round_u64().unwrap();
                position_info_accnt.open_pc_cost = (pc_price.try_mul(pc_deposit_amount).unwrap()).try_round_u64().unwrap();
            }
            loader.exit(&crate::ID)?;
        }
    } else {
        let loader: Loader<crate::accounts::position_info::PositionInfo> =
            Loader::try_from(&crate::ID, position_info_account)?;
        {
            let mut position_info_accnt = loader.load_mut()?;
            position_info_accnt.nonce = position_info_nonce;
            position_info_accnt.coin_deposit = coin_deposit_amount;
            position_info_accnt.pc_deposit = pc_deposit_amount;
            position_info_accnt.withdraw_coin = 0;
            position_info_accnt.withdraw_pc = 0;
            position_info_accnt.open_time = clock;
            position_info_accnt.withdraw_percent = 0;
            position_info_accnt.repay_coin_percent = 0;
            position_info_accnt.repay_pc_percent = 0;
            position_info_accnt.buffer_coin = 0;
            position_info_accnt.buffer_pc = 0;
            position_info_accnt.deposit_lp = 0;
            position_info_accnt.withdraw_lp = 0;
            position_info_accnt.open_coin_cost = (coin_price.try_mul(coin_deposit_amount).unwrap()).try_round_u64().unwrap();
            position_info_accnt.open_pc_cost = (pc_price.try_mul(pc_deposit_amount).unwrap()).try_round_u64().unwrap();
            position_info_accnt.withdraw_coin_cost = 0;
            position_info_accnt.withdraw_pc_cost = 0;
            position_info_accnt.coin_swap = 0;
            position_info_accnt.pc_swap = 0;
            position_info_accnt.coin_deposit_lp = 0;
            position_info_accnt.pc_deposit_lp = 0;
            position_info_accnt.coin_withdraw_lp = 0;
            position_info_accnt.pc_withdraw_lp = 0;
        }
        loader.exit(&crate::ID)?;
    }

    Ok(())
}