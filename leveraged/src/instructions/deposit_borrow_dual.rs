use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;


pub struct DepositBorrowDual {
    pub authority: Pubkey,
    pub user_farm: Pubkey,
    pub leveraged_farm: Pubkey,
    pub user_farm_obligation: Pubkey,
    pub coin_source_token_account: Pubkey,
    pub coin_destination_token_account: Pubkey,
    pub pc_source_token_account: Pubkey,
    pub pc_destination_token_account: Pubkey,
    pub coin_deposit_reserve_account: Pubkey,
    pub pc_deposit_reserve_account: Pubkey,
    pub coin_reserve_liquidity_oracle: Pubkey,
    pub pc_reserve_liquidity_oracle: Pubkey,
    pub lending_market_account: Pubkey,
    pub derived_lending_market_authority: Pubkey,
    pub token_program: Pubkey,
    pub lending_program: Pubkey,
    pub coin_source_reserve_liquidity_token_account: Pubkey,
    pub pc_source_reserve_liquidity_token_account: Pubkey,
    pub coin_reserve_liquidity_fee_receiver: Pubkey,
    pub pc_reserve_liquidity_fee_receiver: Pubkey,
    pub borrow_authorizer: Pubkey,
    pub lp_pyth_price_account: Pubkey,
    pub vault_account: Pubkey,
    pub rent: Pubkey,
}

pub fn deposit_borrow_dual(
    accounts: DepositBorrowDual,
    position_info_account: Pubkey,
    system_program: Pubkey,
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

impl ToAccountMetas for DepositBorrowDual {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.authority, true),
            AccountMeta::new(self.user_farm, false),
            AccountMeta::new_readonly(self.leveraged_farm, false),
            AccountMeta::new(self.user_farm_obligation, false),
            AccountMeta::new(self.coin_source_token_account, false),
            AccountMeta::new(self.coin_destination_token_account, false),
            AccountMeta::new(self.pc_source_token_account, false),
            AccountMeta::new(self.pc_destination_token_account, false),
            AccountMeta::new(self.pc_deposit_reserve_account, false),
            AccountMeta::new(self.coin_deposit_reserve_account, false),
            AccountMeta::new_readonly(self.pc_reserve_liquidity_oracle, false),
            AccountMeta::new_readonly(self.pc_reserve_liquidity_oracle, false),
            AccountMeta::new_readonly(self.lending_market_account, false),
            AccountMeta::new_readonly(self.derived_lending_market_authority, false),
            AccountMeta::new_readonly(self.token_program, false),
            AccountMeta::new_readonly(self.lending_program, false),
            AccountMeta::new(self.coin_source_reserve_liquidity_token_account, false),
            AccountMeta::new(self.pc_source_reserve_liquidity_token_account, false),
            AccountMeta::new(self.pc_reserve_liquidity_fee_receiver, false),
            AccountMeta::new(self.borrow_authorizer, false),
            AccountMeta::new_readonly(self.lp_pyth_price_account, false),
            AccountMeta::new(self.vault_account, false),
            AccountMeta::new_readonly(self.rent, false),
        ]
    }
}