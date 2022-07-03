use anchor_lang::prelude::*;

/// Position info account to track user position
#[account(zero_copy)]
pub struct PositionInfo {
    pub nonce: u8, // 1
    pub coin_deposit: u64, // 9
    pub pc_deposit: u64, // 17
    pub withdraw_coin: u64, // 25
    pub withdraw_pc: u64, // 33
    pub open_time: u32, // 37
    pub useless_field: u16, // not used 39
    pub repay_coin_percent: u16, // 41
    pub repay_pc_percent: u16, // 43
    pub buffer_coin: u64, // 51
    pub buffer_pc: u64, // 59
    pub deposit_lp: u64, // 67
    pub withdraw_lp: u64, // 75
    pub open_coin_cost: u64, // 83
    pub open_pc_cost: u64, // 91
    pub withdraw_coin_cost: u64, // 99
    pub withdraw_pc_cost: u64, // 107
    pub coin_swap: i64, // 115
    pub pc_swap: i64, // 123
    pub coin_deposit_lp: u64, // 131
    pub pc_deposit_lp: u64, // 139
    pub coin_withdraw_lp: u64, // 147
    pub pc_withdraw_lp: u64, // 155
    pub settle_method: u8, // 156
    pub withdraw_percent: u32, // 160
    pub old_account: bool, // 161
    pub close_method: u8, // 162
    pub buffer: [u8; 130]
}