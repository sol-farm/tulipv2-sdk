use anchor_lang::prelude::*;
use super::Position;

/// represents a collection of olibgations managed by this particular UserFarm account
#[account(zero_copy)]
pub struct UserFarm {
    /// the authority which controlls the UserFarmManager
    pub authority: Pubkey,
    /// the key of the leveraged farm account which contains
    /// configuration information about this particular farm
    /// largely related to serum market information
    pub leveraged_farm: Pubkey,
    /// which user farm is this
    pub user_farm_number: u8,
    /// indicates the current number of obligation accounts
    /// managed by this particular UserFarm account
    /// once this reaches the limit of the obligations array 3
    /// because arrays are indexed starting at 0 a new UserFarm
    /// account is needed
    pub number_of_obligations: u8,
    /// this value only exists for first user farm and is used to track how many user farms exist
    pub number_of_user_farms: u8,
    pub nonce: u8,
    /// each UserFarm supports managing 3 obligation accounts
    /// where a given Obligation represents an individual position
    pub obligations: [Obligation; 3],
}


/// represents a single obligation account
/// to derive the obligation account we use the following seeds
/// - find_program_address([authority, user_farm_addr, olibgation_index], lending_program_id)
///
/// 57 bytes in size
#[zero_copy]
pub struct Obligation {
    // the address of the lending program obligation naccount
    pub obligation_account: Pubkey,
    pub coin_amount: u64,
    pub pc_amount: u64,
    pub deposited_lp_tokens: u64,
    pub position_state: Position,
}