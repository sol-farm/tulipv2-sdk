use super::error::LendingError;
use super::last_update::LastUpdate;
use super::{pack_bool, pack_decimal, UNINITIALIZED_VERSION, PROGRAM_VERSION, unpack_decimal, unpack_bool};
use crate::math::{decimal::Decimal, rate::Rate, common::{TryAdd, TryDiv, TryMul, TrySub}};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use anchor_lang::solana_program::{
    clock::Slot,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use std::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
};

/// Max number of collateral and liquidity reserve accounts combined for an obligation
pub const MAX_OBLIGATION_RESERVES: usize = 2;

/// Lending market obligation state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Obligation {
    /// Version of the struct
    pub version: u8,
    /// Last update to collateral, liquidity, or their market values
    pub last_update: LastUpdate,
    /// Lending market address
    pub lending_market: Pubkey,
    /// Vault address the obligation is associated with
    // pub vault_address: Pubkey,
    /// Owner authority which can borrow liquidity
    pub owner: Pubkey,
    /// Deposited collateral for the obligation, unique by deposit reserve address
    pub deposits: Vec<ObligationCollateral>,
    /// Borrowed liquidity for the obligation, unique by borrow reserve address
    pub borrows: Vec<ObligationLiquidity>,
    /// Market value of deposits
    // pub deposited_value: Decimal,
    /// Market value of borrows
    pub borrowed_value: Decimal,
    /// The maximum borrow value at the weighted average loan to value ratio
    // pub allowed_borrow_value: Decimal,
    /// The dangerous borrow value at the weighted average liquidation threshold
    // pub unhealthy_borrow_value: Decimal,
    /// vault shares
    pub vault_shares: u64,
    /// the number of lp token shares owned by this obligation
    /// when calculating loan to value ratio for obligations which have been used
    /// to borrow lp tokens, we calculate LTV using the pseudo deposits value
    /// instead of collateral value
    pub lp_tokens: u64,
    /// the number of coin tokens owned by this obligation
    /// coin is wrt the lp token for this obligation
    /// this is used for deposit value when calculating LTV
    pub coin_deposits: u64,
    /// the number of pc tokens owned by this obligation
    /// pc is wrt the lp token for this obligation
    /// this is used for deposit value when calculating LTV
    pub pc_deposits: u64,
    /// the market value of the user deposits
    /// includes lp (pseudo deposits), coin and pc tokens
    pub deposits_market_value: Decimal,
    /// the lp token pyth price account address
    // pub lp_token_price_account: Pubkey,
    // the coin / base pyth price account address
    // pub coin_price_account: Pubkey,
    /// the pc / quote pyth price account address
    // pub pc_price_account: Pubkey,
    /// Decimals in lp token
    pub lp_decimals: u8,
    /// Decimals in coin
    pub coin_decimals: u8,
    /// Decimals in pc
    pub pc_decimals: u8,
}

impl Obligation {
    /// Create a new obligation
    pub fn new(params: InitObligationParams) -> Self {
        let mut obligation = Self::default();
        Self::init(&mut obligation, params);
        obligation
    }

    /// Initialize an obligation
    pub fn init(&mut self, params: InitObligationParams) {
        self.version = PROGRAM_VERSION;
        self.last_update = LastUpdate::new(params.current_slot);
        self.lending_market = params.lending_market;
        self.owner = params.owner;
        self.deposits = params.deposits;
        self.borrows = params.borrows;
        self.vault_shares = 0;
        self.lp_tokens = 0;
        self.coin_deposits = 0;
        self.pc_deposits = 0;
        self.deposits_market_value = Decimal::zero();
        // self.lp_token_price_account = params.lp_token_pyth_price_account;
        // self.coin_price_account = params.coin_token_pyth_price_account;
        // self.pc_price_account = params.pc_token_pyth_price_account;
        self.lp_decimals = params.lp_decimals;
        self.pc_decimals = params.pc_decimals;
        self.coin_decimals = params.coin_decimals;
        // self.vault_address= params.vault_address;
    }

    /// Calculate the current ratio of borrowed value using their pseudo deposited value
    /// instead of deposited value, allowing us to measure LTV using lp token value instead
    ///
    /// todo(bonedaddy): add a test for this function
    pub fn pseudo_loan_to_value(&self) -> Result<Decimal, ProgramError> {
        #[cfg(target = "bpfel-unknown-unknown")]
        msg!("[ltv]: {} {}", self.coin_deposits, self.pc_deposits);
        // if self.deposits_market_value ==  Decimal::zero() {
        //     return self.borrowed_value.try_div(self.deposited_value)
        // }

        self.borrowed_value.try_div(self.deposits_market_value)
    }

    /// Calculate the current ratio of borrowed value to deposited value
    pub fn loan_to_value(&self) -> Result<Decimal, ProgramError> {
        self.borrowed_value.try_div(self.deposits_market_value)
    }

    /// Repay liquidity and remove it from borrows if zeroed out
    pub fn repay(&mut self, settle_amount: Decimal, liquidity_index: usize) -> ProgramResult {
        let liquidity = &mut self.borrows[liquidity_index];
        if settle_amount == liquidity.borrowed_amount_wads {
            self.borrows.remove(liquidity_index);
        } else {
            liquidity.repay(settle_amount)?;
        }
        Ok(())
    }

    /// Withdraw collateral and remove it from deposits if zeroed out
    pub fn withdraw(&mut self, withdraw_amount: u64, collateral_index: usize) -> ProgramResult {
        let collateral = &mut self.deposits[collateral_index];
        if withdraw_amount == collateral.deposited_amount {
            self.deposits.remove(collateral_index);
        } else {
            collateral.withdraw(withdraw_amount)?;
        }
        Ok(())
    }

    /// Calculate the maximum collateral value that can be withdrawn
    pub fn max_withdraw_value(&self) -> Result<Decimal, ProgramError> {
        // let required_deposit_value = self
        //     .borrowed_value
        //     .try_mul(self.deposited_value)?
        //     .try_div(self.allowed_borrow_value)?;
        // if required_deposit_value >= self.deposited_value {
        //     return Ok(Decimal::zero());
        // }
        // self.deposited_value.try_sub(required_deposit_value)
        Ok(Decimal::zero())
    }

    /// Calculate the maximum liquidity value that can be borrowed
    pub fn remaining_borrow_value(&self) -> Result<Decimal, ProgramError> {
        // self.allowed_borrow_value.try_sub(self.borrowed_value)
        return Err(LendingError::MethodNotAllowed.into());
    }

    /// Calculate the maximum liquidation amount for a given liquidity
    pub fn max_liquidation_amount(
        &self,
        liquidity: &ObligationLiquidity,
    ) -> Result<Decimal, ProgramError> {
        // let max_liquidation_value = self
        //     .borrowed_value
        //     .try_mul(Rate::from_percent(LIQUIDATION_CLOSE_FACTOR))?
        //     .min(liquidity.market_value);
        // let max_liquidation_pct = max_liquidation_value.try_div(liquidity.market_value)?;
        // liquidity.borrowed_amount_wads.try_mul(max_liquidation_pct)
        return Err(LendingError::MethodNotAllowed.into());
    }

    /// Find collateral by deposit reserve
    pub fn find_collateral_in_deposits(
        &self,
        deposit_reserve: Pubkey,
    ) -> Result<(&ObligationCollateral, usize), ProgramError> {
        // if self.deposits.is_empty() {
        //     msg!("Obligation has no deposits");
        //     return Err(LendingError::ObligationDepositsEmpty.into());
        // }
        // let collateral_index = self
        //     ._find_collateral_index_in_deposits(deposit_reserve)
        //     .ok_or(LendingError::InvalidObligationCollateral)?;
        // Ok((&self.deposits[collateral_index], collateral_index))
        return Err(LendingError::MethodNotAllowed.into());
    }

    /// Find or add collateral by deposit reserve
    pub fn find_or_add_collateral_to_deposits(
        &mut self,
        deposit_reserve: Pubkey,
    ) -> Result<&mut ObligationCollateral, ProgramError> {
        if let Some(collateral_index) = self._find_collateral_index_in_deposits(deposit_reserve) {
            return Ok(&mut self.deposits[collateral_index]);
        }
        if self.deposits.len() + self.borrows.len() >= MAX_OBLIGATION_RESERVES {
            msg!(
                "Obligation cannot have more than {} deposits and borrows combined",
                MAX_OBLIGATION_RESERVES
            );
            return Err(LendingError::ObligationReserveLimit.into());
        }
        let collateral = ObligationCollateral::new(deposit_reserve);
        self.deposits.push(collateral);
        Ok(self.deposits.last_mut().unwrap())
    }

    fn _find_collateral_index_in_deposits(&self, deposit_reserve: Pubkey) -> Option<usize> {
        self.deposits
            .iter()
            .position(|collateral| collateral.deposit_reserve == deposit_reserve)
    }

    /// Find liquidity by borrow reserve
    pub fn find_liquidity_in_borrows(
        &self,
        borrow_reserve: Pubkey,
    ) -> Result<(&ObligationLiquidity, usize), ProgramError> {
        if self.borrows.is_empty() {
            msg!("Obligation has no borrows");
            return Err(LendingError::ObligationBorrowsEmpty.into());
        }
        let liquidity_index = self
            ._find_liquidity_index_in_borrows(borrow_reserve)
            .ok_or(LendingError::InvalidObligationLiquidity)?;
        Ok((&self.borrows[liquidity_index], liquidity_index))
    }

    /// Find or add liquidity by borrow reserve
    pub fn find_or_add_liquidity_to_borrows(
        &mut self,
        borrow_reserve: Pubkey,
        cumulative_borrow_rate_wads: Decimal,
    ) -> Result<&mut ObligationLiquidity, ProgramError> {
        if let Some(liquidity_index) = self._find_liquidity_index_in_borrows(borrow_reserve) {
            return Ok(&mut self.borrows[liquidity_index]);
        }
        if self.deposits.len() + self.borrows.len() >= MAX_OBLIGATION_RESERVES {
            msg!(
                "Obligation cannot have more than {} deposits and borrows combined",
                MAX_OBLIGATION_RESERVES
            );
            return Err(LendingError::ObligationReserveLimit.into());
        }
        let liquidity = ObligationLiquidity::new(borrow_reserve, cumulative_borrow_rate_wads);
        self.borrows.push(liquidity);
        Ok(self.borrows.last_mut().unwrap())
    }

    fn _find_liquidity_index_in_borrows(&self, borrow_reserve: Pubkey) -> Option<usize> {
        self.borrows
            .iter()
            .position(|liquidity| liquidity.borrow_reserve == borrow_reserve)
    }

}

/// Initialize an obligation
pub struct InitObligationParams {
    /// Last update to collateral, liquidity, or their market values
    pub current_slot: Slot,
    /// Lending market address
    pub lending_market: Pubkey,
    // Vault address
    // pub vault_address: Pubkey,
    /// Owner authority which can borrow liquidity
    pub owner: Pubkey,
    // pyth price account for the lp token
    // pub lp_token_pyth_price_account: Pubkey,
    // pyth price account for the coin token
    // pub coin_token_pyth_price_account: Pubkey,
    // pyth price account for the pc token
    // pub pc_token_pyth_price_account: Pubkey,
    /// Deposited collateral for the obligation, unique by deposit reserve address
    pub deposits: Vec<ObligationCollateral>,
    /// Borrowed liquidity for the obligation, unique by borrow reserve address
    pub borrows: Vec<ObligationLiquidity>,
    /// lp decimals
    pub lp_decimals: u8,
    /// coin decimals
    pub coin_decimals: u8,
    /// pc decimals
    pub pc_decimals: u8,
}

impl Sealed for Obligation {}
impl IsInitialized for Obligation {
    fn is_initialized(&self) -> bool {
        self.version != UNINITIALIZED_VERSION
    }
}

/// Obligation collateral state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObligationCollateral {
    /// Reserve collateral is deposited to
    pub deposit_reserve: Pubkey,
    /// Amount of collateral deposited
    pub deposited_amount: u64,
    /// Collateral market value in quote currency
    pub market_value: Decimal,
}

impl ObligationCollateral {
    /// Create new obligation collateral
    pub fn new(deposit_reserve: Pubkey) -> Self {
        Self {
            deposit_reserve,
            deposited_amount: 0,
            market_value: Decimal::zero(),
        }
    }

    /// Increase deposited collateral
    pub fn deposit(&mut self, collateral_amount: u64) -> ProgramResult {
        self.deposited_amount = self
            .deposited_amount
            .checked_add(collateral_amount)
            .ok_or(LendingError::MathOverflow)?;
        Ok(())
    }

    /// Decrease deposited collateral
    pub fn withdraw(&mut self, collateral_amount: u64) -> ProgramResult {
        self.deposited_amount = self
            .deposited_amount
            .checked_sub(collateral_amount)
            .ok_or(LendingError::MathOverflow)?;
        Ok(())
    }
}

/// Obligation liquidity state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObligationLiquidity {
    /// Reserve liquidity is borrowed from
    pub borrow_reserve: Pubkey,
    /// Borrow rate used for calculating interest
    pub cumulative_borrow_rate_wads: Decimal,
    /// Amount of liquidity borrowed plus interest
    pub borrowed_amount_wads: Decimal,
    /// Liquidity market value in quote currency
    pub market_value: Decimal,
}

impl ObligationLiquidity {
    /// Create new obligation liquidity
    pub fn new(borrow_reserve: Pubkey, cumulative_borrow_rate_wads: Decimal) -> Self {
        Self {
            borrow_reserve,
            cumulative_borrow_rate_wads,
            borrowed_amount_wads: Decimal::zero(),
            market_value: Decimal::zero(),
        }
    }

    /// Decrease borrowed liquidity
    pub fn repay(&mut self, settle_amount: Decimal) -> ProgramResult {
        self.borrowed_amount_wads = self.borrowed_amount_wads.try_sub(settle_amount)?;
        Ok(())
    }

    /// Increase borrowed liquidity
    pub fn borrow(&mut self, borrow_amount: Decimal) -> ProgramResult {
        self.borrowed_amount_wads = self.borrowed_amount_wads.try_add(borrow_amount)?;
        Ok(())
    }

    /// Accrue interest
    pub fn accrue_interest(&mut self, cumulative_borrow_rate_wads: Decimal) -> ProgramResult {
        match cumulative_borrow_rate_wads.cmp(&self.cumulative_borrow_rate_wads) {
            Ordering::Less => {
                msg!("Interest rate cannot be negative");
                return Err(LendingError::NegativeInterestRate.into());
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                let compounded_interest_rate: Rate = cumulative_borrow_rate_wads
                    .try_div(self.cumulative_borrow_rate_wads)?
                    .try_into()?;

                self.borrowed_amount_wads = self
                    .borrowed_amount_wads
                    .try_mul(compounded_interest_rate)?;
                self.cumulative_borrow_rate_wads = cumulative_borrow_rate_wads;
            }
        }

        Ok(())
    }
}

const OBLIGATION_COLLATERAL_LEN: usize = 56; // 32 + 8 + 16
const OBLIGATION_LIQUIDITY_LEN: usize = 80; // 32 + 16 + 16 + 16
const OBLIGATION_LEN: usize = 303; // 1 + 8 + 1 + 32 + 32 + 32 + 32 + 32 + 32 + 16 + 16 + 16 + 16 + + 16 + 16 + 16 + 16 + 16 + 1 + 1 + 1 + 1 + 1 + (56 * 1) + (80 * 9)
                                   // @TODO: break this up by obligation / collateral / liquidity https://git.io/JOCca
impl Pack for Obligation {
    const LEN: usize = OBLIGATION_LEN;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let output = array_mut_ref![dst, 0, OBLIGATION_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
            let (
            version,
            last_update_slot,
            last_update_stale,
            lending_market,
            // vault_address,
            owner,
            // deposited_value,
            borrowed_value,
            // allowed_borrow_value,
            // unhealthy_borrow_value,
            vault_shares,
            lp_tokens,
            coin_deposits,
            pc_deposits,
            deposits_market_value,
            // lp_token_price_account,
            // coin_price_account,
            // pc_price_account,
            lp_decimals,
            coin_decimals,
            pc_decimals,
            deposits_len,
            borrows_len,
            data_flat,
        ) = mut_array_refs![
            output,
            1, // version
            8, // last update slot
            1, // last update stale
            PUBKEY_BYTES, // lending market
            // PUBKEY_BYTES, // vault address
            PUBKEY_BYTES, // owner
            16, // borrowed value
            8, // vault shares
            8, // lp tokens
            8, // coin deposits
            8, // pc deposits
            16, // deposits market value
            // PUBKEY_BYTES, // lp token price account
            // PUBKEY_BYTES, // coin price account
            // PUBKEY_BYTES, // pc price account
            1, // lp decimals
            1, // coin decimals
            1, // pc decimals
            1, // deposits len
            1, // borrows len
            OBLIGATION_LIQUIDITY_LEN * MAX_OBLIGATION_RESERVES
        ];

        // obligation
        *version = self.version.to_le_bytes();
        *last_update_slot = self.last_update.slot.to_le_bytes();
        pack_bool(self.last_update.stale, last_update_stale);
        lending_market.copy_from_slice(self.lending_market.as_ref());
        owner.copy_from_slice(self.owner.as_ref());
        pack_decimal(self.borrowed_value, borrowed_value);
        *vault_shares = self.vault_shares.to_le_bytes();
        *lp_tokens = self.lp_tokens.to_le_bytes();
        *coin_deposits = self.coin_deposits.to_le_bytes();
        *pc_deposits = self.pc_deposits.to_le_bytes();
        pack_decimal(self.deposits_market_value, deposits_market_value);
        *lp_decimals = self.lp_decimals.to_le_bytes();
        *coin_decimals = self.coin_decimals.to_le_bytes();
        *pc_decimals = self.pc_decimals.to_le_bytes();
        *deposits_len = u8::try_from(self.deposits.len()).unwrap().to_le_bytes();
        *borrows_len = u8::try_from(self.borrows.len()).unwrap().to_le_bytes();

        let mut offset = 0;

        // deposits
        for collateral in &self.deposits {
            let deposits_flat = array_mut_ref![data_flat, offset, OBLIGATION_COLLATERAL_LEN];
            #[allow(clippy::ptr_offset_with_cast)]
            let (deposit_reserve, deposited_amount, market_value) =
                mut_array_refs![deposits_flat, PUBKEY_BYTES, 8, 16];
            deposit_reserve.copy_from_slice(collateral.deposit_reserve.as_ref());
            *deposited_amount = collateral.deposited_amount.to_le_bytes();
            pack_decimal(collateral.market_value, market_value);
            offset += OBLIGATION_COLLATERAL_LEN;
        }

        // borrows
        for liquidity in &self.borrows {
            let borrows_flat = array_mut_ref![data_flat, offset, OBLIGATION_LIQUIDITY_LEN];
            #[allow(clippy::ptr_offset_with_cast)]
            let (borrow_reserve, cumulative_borrow_rate_wads, borrowed_amount_wads, market_value) =
                mut_array_refs![borrows_flat, PUBKEY_BYTES, 16, 16, 16];
            borrow_reserve.copy_from_slice(liquidity.borrow_reserve.as_ref());
            pack_decimal(
                liquidity.cumulative_borrow_rate_wads,
                cumulative_borrow_rate_wads,
            );
            pack_decimal(liquidity.borrowed_amount_wads, borrowed_amount_wads);
            pack_decimal(liquidity.market_value, market_value);
            offset += OBLIGATION_LIQUIDITY_LEN;
        }
    }

    /// Unpacks a byte buffer into an [ObligationInfo](struct.ObligationInfo.html).
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![src, 0, OBLIGATION_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            version,
            last_update_slot,
            last_update_stale,
            lending_market,
            // vault_address,
            owner,
            // deposited_value,
            borrowed_value,
            // allowed_borrow_value,
            // unhealthy_borrow_value,
            vault_shares,
            lp_tokens,
            coin_deposits,
            pc_deposits,
            deposits_market_value,
            // lp_token_price_account,
            // coin_price_account,
            // pc_price_account,
            lp_decimals,
            coin_decimals,
            pc_decimals,
            deposits_len,
            borrows_len,
            data_flat,
        ) = array_refs![
            input,
            1, // version
            8, // last update slot
            1, // last update stale
            PUBKEY_BYTES, // lending market
            // PUBKEY_BYTES, // vault address
            PUBKEY_BYTES, // owner
            16, // borrowed value
            8, // vault shares
            8, // lp tokens
            8, // coin deposits
            8, // pc deposits
            16, // deposits market value
            // PUBKEY_BYTES, // lp token price account
            // PUBKEY_BYTES, // coin price account
            // PUBKEY_BYTES, // pc price account
            1, // lp decimals
            1, // coin decimals
            1, // pc decimals
            1, // deposits len
            1, // borrows len
            OBLIGATION_LIQUIDITY_LEN * MAX_OBLIGATION_RESERVES
        ];

        let version = u8::from_le_bytes(*version);
        if version > PROGRAM_VERSION {
            msg!("Obligation version does not match lending program version");
            return Err(ProgramError::InvalidAccountData);
        }

        let deposits_len = u8::from_le_bytes(*deposits_len);
        let borrows_len = u8::from_le_bytes(*borrows_len);
        let mut deposits = Vec::with_capacity(deposits_len as usize + 1);
        let mut borrows = Vec::with_capacity(borrows_len as usize + 1);

        let mut offset = 0;
        for _ in 0..deposits_len {
            let deposits_flat = array_ref![data_flat, offset, OBLIGATION_COLLATERAL_LEN];
            #[allow(clippy::ptr_offset_with_cast)]
            let (deposit_reserve, deposited_amount, market_value) =
                array_refs![deposits_flat, PUBKEY_BYTES, 8, 16];
            deposits.push(ObligationCollateral {
                deposit_reserve: Pubkey::new(deposit_reserve),
                deposited_amount: u64::from_le_bytes(*deposited_amount),
                market_value: unpack_decimal(market_value),
            });
            offset += OBLIGATION_COLLATERAL_LEN;
        }
        for _ in 0..borrows_len {
            let borrows_flat = array_ref![data_flat, offset, OBLIGATION_LIQUIDITY_LEN];
            #[allow(clippy::ptr_offset_with_cast)]
            let (borrow_reserve, cumulative_borrow_rate_wads, borrowed_amount_wads, market_value) =
                array_refs![borrows_flat, PUBKEY_BYTES, 16, 16, 16];
            borrows.push(ObligationLiquidity {
                borrow_reserve: Pubkey::new(borrow_reserve),
                cumulative_borrow_rate_wads: unpack_decimal(cumulative_borrow_rate_wads),
                borrowed_amount_wads: unpack_decimal(borrowed_amount_wads),
                market_value: unpack_decimal(market_value),
            });
            offset += OBLIGATION_LIQUIDITY_LEN;
        }

        Ok(Self {
            version,
            last_update: LastUpdate {
                slot: u64::from_le_bytes(*last_update_slot),
                stale: unpack_bool(last_update_stale)?,
            },
            lending_market: Pubkey::new_from_array(*lending_market),
            owner: Pubkey::new_from_array(*owner),
            deposits,
            borrows,
            borrowed_value: unpack_decimal(borrowed_value),
            vault_shares: u64::from_le_bytes(*vault_shares),
            lp_tokens: u64::from_le_bytes(*lp_tokens),
            coin_deposits: u64::from_le_bytes(*coin_deposits),
            pc_deposits: u64::from_le_bytes(*pc_deposits),
            deposits_market_value: unpack_decimal(deposits_market_value),
            lp_decimals: u8::from_le_bytes(*lp_decimals),
            coin_decimals: u8::from_le_bytes(*coin_decimals),
            pc_decimals: u8::from_le_bytes(*pc_decimals),
        })
    }
}
