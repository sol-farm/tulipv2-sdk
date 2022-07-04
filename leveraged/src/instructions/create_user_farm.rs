use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

pub struct CreateUserFarm {
    //#[account(signer)]
    pub authority: Pubkey,
    //#[account(mut)]
    pub user_farm: Pubkey,
    //#[account(mut)]
    pub user_farm_obligation: Pubkey,
    //#[account(mut)]
    pub lending_market: Pubkey,
    pub global: Pubkey,
    pub leveraged_farm: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub lending_program: Pubkey,
    pub token_program: Pubkey,
    //#[account(mut)]
    pub obligation_vault_address: Pubkey,
}


pub fn create_user_farm(
    accounts: CreateUserFarm,
) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("create_user_farm")?;
    let mut ix_data = Vec::with_capacity(40);
    ix_data.extend_from_slice(&ix_sighash[..]);
    ix_data.extend_from_slice(&AnchorSerialize::try_to_vec(&crate::ID).unwrap());
    Some(Instruction {
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
        data: ix_data,
    })
}

impl ToAccountMetas for CreateUserFarm {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority, true),
            AccountMeta::new(self.user_farm, false),
            AccountMeta::new(self.user_farm_obligation, false),
            AccountMeta::new(self.lending_market, false),
            AccountMeta::new_readonly(self.global, false),
            AccountMeta::new_readonly(self.leveraged_farm, false),
            AccountMeta::new_readonly(self.clock, false),
            AccountMeta::new_readonly(self.rent, false),
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new_readonly(self.lending_program, false),
            AccountMeta::new_readonly(self.token_program, false),
            AccountMeta::new(self.obligation_vault_address, false),
        ]   
    }
}