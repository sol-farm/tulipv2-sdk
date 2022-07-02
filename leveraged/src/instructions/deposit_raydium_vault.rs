use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

#[derive(Accounts)]
pub struct DepositFarm<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub user_farm: AccountInfo<'info>,
    #[account(mut)]
    pub obligation_vault_address: AccountInfo<'info>,
    #[account(mut)]
    pub leveraged_farm: AccountInfo<'info>,
    pub vault_program: AccountInfo<'info>,

    #[account(mut)]
    // the lp token account owned by authority
    authority_token_account: AccountInfo<'info>,
    #[account(mut)]
    // the account of the vault pda
    vault_pda_account: AccountInfo<'info>,
    #[account(mut)]
    vault: AccountInfo<'info>,
    #[account(mut)]
    // lp token account owned by vault pda which holds the lp tokens
    lp_token_account: AccountInfo<'info>,

    #[account(mut)]
    user_balance_account: AccountInfo<'info>,

    system_program: AccountInfo<'info>,

    stake_program_id: AccountInfo<'info>,
    #[account(mut)]
    pool_id: AccountInfo<'info>,
    #[account(mut)]
    pool_authority: AccountInfo<'info>,
    #[account(mut)]
    user_info_account: AccountInfo<'info>,
    #[account(mut)]
    pool_lp_token_account: AccountInfo<'info>,

    #[account(mut)]
    user_reward_a_token_account: AccountInfo<'info>,
    #[account(mut)]
    pool_reward_a_token_account: AccountInfo<'info>,
    #[account(mut)]
    user_reward_b_token_account: AccountInfo<'info>,
    #[account(mut)]
    pool_reward_b_token_account: AccountInfo<'info>,

    clock: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,

    #[account(mut)]
    user_balance_metadata: AccountInfo<'info>,
}


pub fn deposit_vault<'info>(
    accounts: DepositFarm<'info>,
    nonce: u8,
    meta_nonce: u8,
    obligation_index: u64,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("deposit_vault")?;
    let mut ix_data = Vec::with_capacity(11);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&nonce).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&meta_nonce).unwrap());
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&obligation_index).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}