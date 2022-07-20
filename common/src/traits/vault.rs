//! defines a common interface for an implementation of tokenized vaults that may or may not compound

use anchor_lang::{prelude::*, solana_program::account_info::AccountInfo};
use anchor_spl::token::Mint;

pub trait TokenizedShares {
    /// used to check if a withdraw attempt is locked. whether or
    /// not a vault is locked depends on the underlying implementation
    fn is_locked(&self, last_deposit_time: i64) -> bool;
    /// used to compound the underlying token represented by the shares
    /// returns true if the implementation supports compound operations
    /// and returns false if the implementation does not support compound operations
    fn compound(&mut self, balance_to_add: u64) -> bool;
    /// returns true if the implementation supports compound operations
    fn supports_compound(&self) -> bool;
    /// returns the amount of shares to give in exchange for depositing the amount
    /// of underlying asset
    fn shares_to_give(&self, amount: u64) -> u64;
    /// returns the amount of underlying to redeem in exchange for burning the amount of shares
    fn underlying_to_redeem(&self, amount: u64) -> u64;
    /// used to record the effects of depositing underlying asset into the vault
    fn record_deposit(&mut self, amount: u64) -> u64;
    /// used to record the effect of withdrawing and burning the shares for their
    /// underlying assets. note you will need to calculate the shares to burn
    /// and the corresponding balance to remove
    fn record_withdraw(&mut self, shares_to_burn: u64, balance_to_remove: u64);
    /// returns the total deposited underlying tokens
    fn total_deposited_tokens(&self) -> u64;
    /// returns the total shares issued, which should match
    /// the shares mint supply
    fn total_shares(&self) -> u64;
    /// returns the share's mint account address
    fn shares_mint(&self) -> Pubkey;
    /// returns the share's underlying asset mint account address
    fn underlying_mint(&self) -> Pubkey;
    /// used to issue the actual shares
    fn issue_shares<'info>(
        &mut self,
        mint: &AccountInfo<'info>,
        // token account which will receive the shares
        receiver: &AccountInfo<'info>,
        pda: &AccountInfo<'info>,
        token_program: &AccountInfo<'info>,
        signer_seeds: &[&[&[u8]]],
        // number of shares to mint
        amount: u64,
    ) -> Result<()>;
    /// used to burn the actual shares
    fn burn_shares<'info>(
        &mut self,
        // the shares token account from which we are burning funds
        shares_account: &AccountInfo<'info>,
        // the mint account of the shares we are burning
        shares_mint: &mut Box<Account<'info, Mint>>,
        authority: &AccountInfo<'info>,
        // the underlying token account for which we are transferring
        // the redeemed underlying assets to
        token_program: &AccountInfo<'info>,
        signer_seeds: &[&[&[u8]]],
        // amount of shares we are burning
        shares_to_burn: u64,
        // any additional signers,
        additional_signers: Option<Vec<AccountInfo<'info>>>,
    ) -> Result<()>;
    /// used to transfer underlying tokens
    fn transfer_underlying<'info>(
        &mut self,
        // account from which the underlying tokens will be sent from
        underlying_account: &AccountInfo<'info>,
        // token account which will receive the underlying asset
        receiver: &AccountInfo<'info>,
        pda: &AccountInfo<'info>,
        token_program: &AccountInfo<'info>,
        signer_seeds: &[&[&[u8]]],
        // number of shares to mint
        amount: u64,
    ) -> Result<()>;
    /// is used to check whether or not the incoming deposited amount
    /// will push the vault over the cap. if the deposit cap is reached,
    /// returns true, otherwise returns false
    fn deposits_capped(&self, incoming_deposit_amount: u64) -> bool;
    /// syncs the variable tracked by the vault with the shares supply of a mint
    fn sync_shares(&mut self, mint: &spl_token::state::Mint);
    /// returns the amount of underlying backing 1 share
    fn exchange_rate(&mut self, mint: &spl_token::state::Mint) -> f64;
    /// returns the cached exchange rate value without first performing a shares sync
    /// this is generally only useful for on-chain programs which aren't the owner of the account
    /// backing this particular trait.
    fn cached_exchange_rate(&self, mint: &spl_token::state::Mint) -> f64;
}

/// implementation of a holder of tokenized vault shares
#[cfg(not(tarpaulin_include))]
pub trait TokenizedSharesHolder {
    /// returns the amount of shares to given in exchange for depositing the specified amount of underlying otkens
    fn shares_to_give(&self, vault: &impl TokenizedShares, amount: u64) -> u64;
    /// returns the amount of underlying to redeem in exchange for burning the amount of shares
    /// returns None if vault is locked for the share holder
    fn underlying_to_redeem(&self, vault: &impl TokenizedShares, amount: u64) -> Option<u64>;
    /// used to record the effects of depositing underlying asset into the vault
    fn record_deposit(&mut self, amount: u64, shares: u64);
    /// used to record the effect of withdrawing and burning the shares for their
    /// underlying assets. note you will need to calculate the shares to burn
    /// and the corresponding balance to remove
    fn record_withdraw(&mut self, vault: &impl TokenizedShares, amount: u64) -> u64;
    /// returns the current deposited underlying managed by this account, not considering
    /// compounding rewards
    fn deposited_balance(&self) -> u64;
    /// returns the current number of shares managed by this account
    fn issued_shares(&self) -> u64;
    /// returns the total amount of underlying that has been deposited into this account over time
    fn total_deposited_underlying(&self) -> u64;
    /// returns the total amount of underlying that has ever been withdrawn from this account over time
    fn total_withdrawn_underlying(&self) -> u64;
    /// withdraw shares from the deposit tracking account
    fn withdraw_shares<'info>(
        &mut self,
        deposit_account: &AccountInfo<'info>,
        hold_account: &AccountInfo<'info>,
        pda_account: &AccountInfo<'info>,
        receiving_shares_account: &AccountInfo<'info>,
        token_program: &AccountInfo<'info>,
        amount: u64,
    ) -> Result<()>;
}
