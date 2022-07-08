use crate::accounts::raydium_vault::derive_associated_stake_info_address;
use crate::accounts::{
    derive_compound_queue_address, derive_pda_address, derive_shares_mint_address,
    derive_withdraw_queue_address, raydium_vault::derive_user_stake_info_address,
};

use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::pubkey::Pubkey;

use super::VaultBaseConfig;

pub struct RaydiumVaultConfig {
    pub vault: Pubkey,
    pub pda: Pubkey,
    pub withdraw_queue: Pubkey,
    pub compound_queue: Pubkey,
    pub deposit_queue: Pubkey,
    pub underlying_mint: Pubkey,
    pub shares_mint: Pubkey,
    pub user_stake_info: Pubkey,
    pub associated_stake_info: Pubkey,
    pub vault_reward_a_token_account: Pubkey,
    pub vault_reward_b_token_account: Pubkey,
}

impl RaydiumVaultConfig {
    pub fn new(
        vault: Pubkey,
        underlying_mint: Pubkey,
        raydium_pool_id: Pubkey,
        raydium_stake_program: Pubkey,
        reward_a_token_mint: Pubkey,
        reward_b_token_mint: Pubkey,
    ) -> Self {
        let pda = derive_pda_address(&vault).0;
        let shares_mint = derive_shares_mint_address(&vault, &underlying_mint).0;
        let withdraw_queue = derive_withdraw_queue_address(&vault, &underlying_mint).0;
        let compound_queue = derive_compound_queue_address(&vault, &underlying_mint).0;
        let user_stake_info = derive_user_stake_info_address(&pda).0;
        let associated_stake_info =
            derive_associated_stake_info_address(&raydium_pool_id, &pda, &raydium_stake_program).0;
        Self {
            vault,
            pda,
            shares_mint,
            withdraw_queue,
            compound_queue,
            underlying_mint,
            deposit_queue: spl_associated_token_account::get_associated_token_address(
                &pda,
                &underlying_mint,
            ),
            user_stake_info,
            associated_stake_info,
            vault_reward_a_token_account: spl_associated_token_account::get_associated_token_address(
                &pda,
                &reward_a_token_mint,
            ),
            vault_reward_b_token_account: spl_associated_token_account::get_associated_token_address(
                &pda,
                &reward_b_token_mint,
            ),
        }
    }
    pub fn to_ix(
        &self,
        authority: Pubkey,
        pool_id: Pubkey,
        pool_authority: Pubkey,
        pool_lp_token_account: Pubkey,
        burning_shares_token_account: Pubkey,
        receiving_shares_token_account: Pubkey,
        pool_reward_a_token_account: Pubkey,
        pool_reward_b_token_account: Pubkey,
        fee_collector_reward_a_token_account: Pubkey,
        fee_collector_reward_b_token_account: Option<Pubkey>,
        raydium_stake_program: Pubkey,
        amount: u64,
    ) -> Option<Instruction> {
        crate::instructions::raydium::new_withdraw_raydium_vault_ix(
            authority,
            self.vault,
            self.pda,
            self.associated_stake_info,
            pool_id,
            pool_authority,
            self.withdraw_queue,
            pool_lp_token_account,
            self.vault_reward_a_token_account,
            pool_reward_a_token_account,
            self.vault_reward_b_token_account,
            pool_reward_b_token_account,
            burning_shares_token_account,
            receiving_shares_token_account,
            self.shares_mint,
            raydium_stake_program,
            fee_collector_reward_a_token_account,
            fee_collector_reward_b_token_account,
            amount
        )
    }
}

impl VaultBaseConfig for RaydiumVaultConfig {
    fn compound_queue(&self) -> Pubkey {
        self.compound_queue
    }
    fn deposit_queue(&self) -> Pubkey {
        self.deposit_queue
    }
    fn vault(&self) -> Pubkey {
        self.vault
    }
    fn vault_pda(&self) -> Pubkey {
        self.pda
    }
    fn withdraw_queue(&self) -> Pubkey {
        self.withdraw_queue
    }
    fn shares_mint(&self) -> Pubkey {
        self.shares_mint
    }
}
