use anchor_lang::solana_program::{pubkey::Pubkey, system_program};

use crate::config::lending::usdc;
use solana_sdk::{msg, native_token::Sol};

use super::Platform;

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
    pub program_id: Pubkey,
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
    pub system_program_id: Pubkey,
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

    pub fn get_tulip_remaining_accounts() -> [Pubkey; 7] {
        [
            usdc::tulip::COLLATERAL_TOKEN_ACCOUNT,
            usdc::tulip::RESERVE_ACCOUNT,
            usdc::tulip::RESERVE_LIQUIDITY_ACCOUNT,
            usdc::tulip::COLLATERAL_MINT,
            usdc::tulip::LENDING_MARKET_ACCOUNT,
            usdc::tulip::LENDING_MARKET_AUTHORITY,
            usdc::tulip::PYTH_PRICE_ACCOUNT,
        ]
    }

    pub fn get_solend_remaining_accounts() -> [Pubkey; 8] {
        [
            usdc::solend::COLLATERAL_TOKEN_ACCOUNT,
            usdc::solend::RESERVE_ACCOUNT,
            usdc::solend::RESERVE_LIQUIDITY_ACCOUNT,
            usdc::solend::COLLATERAL_MINT,
            usdc::solend::LENDING_MARKET_ACCOUNT,
            usdc::solend::LENDING_MARKET_AUTHORITY,
            usdc::solend::PYTH_PRICE_ACCOUNT,
            usdc::solend::SWITCHBOARD_PRICE_ACCOUNT,
        ]
    }

    pub fn get_mango_remaining_accounts() -> [Pubkey; 8] {
        [
            usdc::mango::GROUP,
            usdc::mango::OPTIMIZER_MANGO_ACCOUNT,
            usdc::mango::CACHE,
            usdc::mango::ROOT_BANK,
            usdc::mango::NODE_BANK,
            usdc::mango::GROUP_TOKEN_ACCOUNT,
            usdc::mango::GROUP_SIGNER,
            system_program::id(),
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
            system_program_id: system_program::id(),
        })
    }
}
