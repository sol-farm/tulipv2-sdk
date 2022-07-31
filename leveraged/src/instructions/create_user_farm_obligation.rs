use anchor_lang::prelude::*;
use sighashdb::GlobalSighashDB;
use solana_program::instruction::Instruction;

pub struct CreateUserFarmObligation {
    pub authority: Pubkey,
    pub user_farm: Pubkey,
    pub leveraged_farm: Pubkey,
    pub user_farm_obligation: Pubkey,
    pub lending_market: Pubkey,
    pub obligation_vault_address: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub lending_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}

pub fn create_user_farm_obligation(accounts: CreateUserFarmObligation) -> Option<Instruction> {
    let ix_sighash = GlobalSighashDB.get_deprecated("create_user_farm_obligation")?;
    Some(Instruction {
        data: ix_sighash.to_vec(),
        program_id: crate::ID,
        accounts: accounts.to_account_metas(None),
    })
}

impl ToAccountMetas for CreateUserFarmObligation {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.authority, true),
            AccountMeta::new(self.user_farm, false),
            AccountMeta::new_readonly(self.leveraged_farm, false),
            AccountMeta::new(self.user_farm_obligation, false),
            AccountMeta::new(self.lending_market, false),
            AccountMeta::new(self.obligation_vault_address, false),
            AccountMeta::new_readonly(self.clock, false),
            AccountMeta::new_readonly(self.rent, false),
            AccountMeta::new_readonly(self.lending_program, false),
            AccountMeta::new_readonly(self.token_program, false),
            AccountMeta::new_readonly(self.system_program, false),
        ]
    }
}
