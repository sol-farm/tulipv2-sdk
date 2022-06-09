use super::{traits::WithdrawMultiOptimizerVault, Platform};
use crate::config::{lending::usdc, ID};
use anchor_lang::AnchorSerialize;
use anchor_lang::{
    solana_program::{pubkey::Pubkey, system_program},
    ToAccountMetas,
};
use sighashdb::GlobalSighashDB;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::sysvar;
use solana_sdk::{instruction::Instruction, msg, native_token::Sol};

#[derive(Clone, Copy)]
pub struct WithdrawAddresses {
    pub authority: Pubkey,
    pub multi_vault: Pubkey,
    pub multi_vault_pda: Pubkey,
    pub multi_burning_shares_token_account: Pubkey,
    pub withdraw_burning_shares_token_account: Pubkey,
    pub receiving_underlying_token_account: Pubkey,
    pub multi_underlying_withdraw_queue: Pubkey,
    pub multi_shares_mint: Pubkey,
    pub platform_config: PlatformConfigAddresses,
    pub tulip_standalone_addresses: Option<TulipStandaloneAddresses>,
    pub solend_standalone_addresses: Option<SolendStandaloneAddresses>,
    pub mango_standalone_addresses: Option<MangoStandaloneAddresses>,
}

#[derive(Clone, Copy)]
pub struct PlatformConfigAddresses {
    pub vault: Pubkey,
    pub vault_pda: Pubkey,
    pub information_account: Pubkey,
    pub config_data_account: Pubkey,
    pub shares_mint: Pubkey,
    pub underlying_deposit_queue: Pubkey,
    pub lending_program: Pubkey,
}

#[derive(Clone, Copy)]
pub struct TulipStandaloneAddresses {
    pub collateral_token_account: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity: Pubkey,
    pub collateral_mint: Pubkey,
    pub lending_market_account: Pubkey,
    pub lending_market_authority: Pubkey,
    pub pyth_price_account: Pubkey,
}

#[derive(Clone, Copy)]
pub struct SolendStandaloneAddresses {
    pub collateral_token_account: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity: Pubkey,
    pub collateral_mint: Pubkey,
    pub lending_market_account: Pubkey,
    pub lending_market_authority: Pubkey,
    pub pyth_price_account: Pubkey,
    pub switchboard_price_account: Pubkey,
}

#[derive(Clone, Copy)]
pub struct MangoStandaloneAddresses {
    pub group: Pubkey,
    pub optimizer_mango_account: Pubkey,
    pub cache: Pubkey,
    pub root_bank: Pubkey,
    pub node_bank: Pubkey,
    pub group_token_account: Pubkey,
    pub group_signer: Pubkey,
}

impl WithdrawAddresses {
    pub fn new(
        user: Pubkey,
        vault: Pubkey,
        vault_pda: Pubkey,
        shares_mint: Pubkey,
        underlying_mint: Pubkey,
        underlying_withdraw_queue: Pubkey,
        platform_config: PlatformConfigAddresses,
        standalone_config: (&[Pubkey], Platform),
    ) -> std::result::Result<WithdrawAddresses, std::io::Error> {
        let multi_burning_shares_token_account =
            spl_associated_token_account::get_associated_token_address(&user, &shares_mint);

        let withdraw_burning_shares_token_account =
            spl_associated_token_account::get_associated_token_address(
                &vault_pda,
                &platform_config.shares_mint,
            );

        let receiving_underlying_token_account =
            spl_associated_token_account::get_associated_token_address(&user, &underlying_mint);

        let mut withdraw_addresses = WithdrawAddresses {
            authority: user,
            multi_vault: vault,
            multi_vault_pda: vault_pda,
            multi_burning_shares_token_account,
            withdraw_burning_shares_token_account,
            receiving_underlying_token_account,
            multi_underlying_withdraw_queue: underlying_withdraw_queue,
            multi_shares_mint: shares_mint,
            platform_config,
            tulip_standalone_addresses: None,
            solend_standalone_addresses: None,
            mango_standalone_addresses: None,
        };
        if standalone_config.1.eq(&Platform::MangoV3) {
            let mango_standalone_addresses: MangoStandaloneAddresses =
                standalone_config.0.try_into()?;
            withdraw_addresses.mango_standalone_addresses = Some(mango_standalone_addresses);
        } else if standalone_config.1.eq(&Platform::Solend) {
            let solend_standalone_addresses: SolendStandaloneAddresses =
                standalone_config.0.try_into()?;
            withdraw_addresses.solend_standalone_addresses = Some(solend_standalone_addresses);
        } else {
            let tulip_standalone_addresses: TulipStandaloneAddresses =
                standalone_config.0.try_into()?;
            withdraw_addresses.tulip_standalone_addresses = Some(tulip_standalone_addresses);
        }
        Ok(withdraw_addresses)
    }
}

impl WithdrawMultiOptimizerVault for WithdrawAddresses {
    fn authority(&self) -> Pubkey {
        self.authority
    }
    fn multi_deposit_vault(&self) -> Pubkey {
        self.multi_vault
    }
    fn multi_deposit_vault_pda(&self) -> Pubkey {
        self.multi_vault_pda
    }
    fn withdraw_vault(&self) -> Pubkey {
        self.platform_config.vault
    }
    fn withdraw_vault_pda(&self) -> Pubkey {
        self.platform_config.vault_pda
    }
    fn platform_information(&self) -> Pubkey {
        self.platform_config.information_account
    }
    fn platform_config_data(&self) -> Pubkey {
        self.platform_config.config_data_account
    }
    fn lending_program(&self) -> Pubkey {
        self.platform_config.lending_program
    }
    fn multi_burning_shares_token_account(&self) -> Pubkey {
        self.multi_burning_shares_token_account
    }
    fn withdraw_burning_shares_token_account(&self) -> Pubkey {
        self.withdraw_burning_shares_token_account
    }
    fn receiving_underlying_token_account(&self) -> Pubkey {
        self.receiving_underlying_token_account
    }
    fn multi_underlying_withdraw_queue(&self) -> Pubkey {
        self.multi_underlying_withdraw_queue
    }
    fn multi_shares_mint(&self) -> Pubkey {
        self.multi_shares_mint
    }
    fn withdraw_shares_mint(&self) -> Pubkey {
        self.platform_config.shares_mint
    }
    fn withdraw_vault_underlying_deposit_queue(&self) -> Pubkey {
        self.platform_config.underlying_deposit_queue
    }
    fn standalone_vault_accounts(&self) -> Option<Vec<solana_sdk::instruction::AccountMeta>> {
        if let Some(mango_accounts) = self.mango_standalone_addresses {
            Some(mango_accounts.to_account_metas(None))
        } else if let Some(solend_accounts) = self.solend_standalone_addresses {
            Some(solend_accounts.to_account_metas(None))
        } else if let Some(tulip_accounts) = self.tulip_standalone_addresses {
            Some(tulip_accounts.to_account_metas(None))
        } else {
            #[cfg(feature = "logs")]
            msg!("mango, solend, and tulip accounts are None");
            None
        }
    }
    fn instruction(&self, amount: u64) -> Option<solana_sdk::instruction::Instruction> {
        let ix_sighash = self.ix_data()?;
        // 8 for the sighash, 8 for the amount
        let mut ix_data = Vec::with_capacity(16);
        ix_data.extend_from_slice(&ix_sighash[..]);
        match AnchorSerialize::try_to_vec(&amount) {
            Ok(amount_data) => ix_data.extend_from_slice(&amount_data[..]),
            Err(err) => {
                #[cfg(feature = "logs")]
                msg!("failed to serialize amount {:#?}", err);
                return None;
            }
        }
        let mut accounts = self.to_account_meta(None);
        let mut standalone_metas = self.standalone_vault_accounts()?;
        accounts.append(&mut standalone_metas);
        Some(Instruction {
            program_id: ID,
            accounts,
            data: ix_data,
        })
    }
    fn ix_data(&self) -> Option<[u8; 8]> {
        GlobalSighashDB.get("withdraw_multi_deposit_optimizer_vault")
    }
    // returns the account metas for the main instruction object
    fn to_account_meta(
        &self,
        _is_signer: Option<bool>,
    ) -> Vec<solana_sdk::instruction::AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.authority(), true),
            AccountMeta::new(self.multi_deposit_vault(), false),
            AccountMeta::new_readonly(self.multi_deposit_vault_pda(), false),
            AccountMeta::new(self.withdraw_vault(), false),
            AccountMeta::new_readonly(self.withdraw_vault_pda(), false),
            AccountMeta::new_readonly(self.platform_information(), false),
            AccountMeta::new_readonly(self.platform_config_data(), false),
            AccountMeta::new_readonly(self.lending_program(), false),
            AccountMeta::new(self.multi_burning_shares_token_account(), false),
            AccountMeta::new(self.withdraw_burning_shares_token_account(), false),
            AccountMeta::new(self.receiving_underlying_token_account(), false),
            AccountMeta::new(self.multi_shares_mint(), false),
            AccountMeta::new(self.withdraw_shares_mint(), false),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(self.withdraw_vault_underlying_deposit_queue(), false),
        ]
    }
}

impl TryFrom<&[Pubkey]> for TulipStandaloneAddresses {
    type Error = std::io::ErrorKind;
    fn try_from(accounts: &[Pubkey]) -> Result<Self, Self::Error> {
        if accounts.len() != 7 {
            #[cfg(feature = "logs")]
            msg!("insufficient accounts");
            return Err(std::io::ErrorKind::InvalidInput.into());
        }
        Ok(Self {
            collateral_token_account: accounts[0],
            reserve: accounts[1],
            reserve_liquidity: accounts[2],
            collateral_mint: accounts[3],
            lending_market_account: accounts[4],
            lending_market_authority: accounts[5],
            pyth_price_account: accounts[6],
        })
    }
}

impl TryFrom<&[Pubkey]> for SolendStandaloneAddresses {
    type Error = std::io::ErrorKind;
    fn try_from(accounts: &[Pubkey]) -> Result<Self, Self::Error> {
        if accounts.len() != 8 {
            #[cfg(feature = "logs")]
            msg!("insufficient accounts");
            return Err(std::io::ErrorKind::InvalidInput.into());
        }
        Ok(Self {
            collateral_token_account: accounts[0],
            reserve: accounts[1],
            reserve_liquidity: accounts[2],
            collateral_mint: accounts[3],
            lending_market_account: accounts[4],
            lending_market_authority: accounts[5],
            pyth_price_account: accounts[6],
            switchboard_price_account: accounts[7],
        })
    }
}

impl TryFrom<&[Pubkey]> for MangoStandaloneAddresses {
    type Error = std::io::ErrorKind;
    fn try_from(accounts: &[Pubkey]) -> Result<Self, Self::Error> {
        // although 8 accounts are required we dont need to provide system program through input
        if accounts.len() != 7 {
            #[cfg(feature = "logs")]
            msg!("insufficient accounts");
            return Err(std::io::ErrorKind::InvalidInput.into());
        }
        Ok(Self {
            group: accounts[0],
            optimizer_mango_account: accounts[1],
            cache: accounts[2],
            root_bank: accounts[3],
            node_bank: accounts[4],
            group_token_account: accounts[5],
            group_signer: accounts[6],
        })
    }
}

impl ToAccountMetas for MangoStandaloneAddresses {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new_readonly(self.group, false),
            AccountMeta::new(self.optimizer_mango_account, false),
            AccountMeta::new(self.cache, false),
            AccountMeta::new(self.root_bank, false),
            AccountMeta::new(self.node_bank, false),
            AccountMeta::new(self.optimizer_mango_account, false),
            AccountMeta::new_readonly(self.group_signer, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ]
    }
}

impl ToAccountMetas for SolendStandaloneAddresses {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<solana_sdk::instruction::AccountMeta> {
        vec![
            AccountMeta::new(self.collateral_token_account, false),
            AccountMeta::new(self.reserve, false),
            AccountMeta::new(self.reserve_liquidity, false),
            AccountMeta::new(self.collateral_mint, false),
            AccountMeta::new_readonly(self.lending_market_account, false),
            AccountMeta::new_readonly(self.lending_market_authority, false),
            AccountMeta::new_readonly(self.pyth_price_account, false),
            AccountMeta::new(self.switchboard_price_account, false),
        ]
    }
}

impl ToAccountMetas for TulipStandaloneAddresses {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<solana_sdk::instruction::AccountMeta> {
        vec![
            AccountMeta::new(self.collateral_token_account, false),
            AccountMeta::new(self.reserve, false),
            AccountMeta::new(self.reserve_liquidity, false),
            AccountMeta::new(self.collateral_mint, false),
            AccountMeta::new_readonly(self.lending_market_account, false),
            AccountMeta::new_readonly(self.lending_market_authority, false),
            AccountMeta::new_readonly(self.pyth_price_account, false),
        ]
    }
}
