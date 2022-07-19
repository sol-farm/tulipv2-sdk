use anchor_lang::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
#[repr(u64)]
pub enum Atrix {
    SOLUSDC = 0,
    GMTUSDC = 1,
    mSOLUSDC = 2,
    BTCUSDC = 3,
    USDrUSDC = 4,
    PLACEHOLDER_E = 5,
    PLACEHOLDER_F = 6,
    PLACEHOLDER_G = 7,
    PLACEHOLDER_H = 8,
    PLACEHOLDER_I = 9,
    PLACEHOLDER_J = 10,
    PLACEHOLDER_K = 11,
    PLACEHOLDER_L = 12,
    PLACEHOLDER_M = 13,
    PLACEHOLDER_N = 14,
    PLACEHOLDER_O = 15,
    PLACEHOLDER_P = 16,
    PLACEHOLDER_Q = 17,
    PLACEHOLDER_R = 18,
    PLACEHOLDER_S = 19,
    PLACEHOLDER_T = 20,
    PLACEHOLDER_U = 21,
    PLACEHOLDER_V = 22,
    PLACEHOLDER_W = 23,
    PLACEHOLDER_X = 24,
    PLACEHOLDER_Y = 25,
    PLACEHOLDER_Z = 26,
    UNKNOWN = u64::MAX,
}

impl From<Atrix> for u64 {
    fn from(val: Atrix) -> Self {
        match val {
            Atrix::SOLUSDC => 0,
            Atrix::GMTUSDC => 1,
            Atrix::mSOLUSDC => 2,
            Atrix::BTCUSDC => 3,
            Atrix::USDrUSDC => 4,
            Atrix::PLACEHOLDER_E => 5,
            Atrix::PLACEHOLDER_F => 6,
            Atrix::PLACEHOLDER_G => 7,
            Atrix::PLACEHOLDER_H => 8,
            Atrix::PLACEHOLDER_I => 9,
            Atrix::PLACEHOLDER_J => 10,
            Atrix::PLACEHOLDER_K => 11,
            Atrix::PLACEHOLDER_L => 12,
            Atrix::PLACEHOLDER_M => 13,
            Atrix::PLACEHOLDER_N => 14,
            Atrix::PLACEHOLDER_O => 15,
            Atrix::PLACEHOLDER_P => 16,
            Atrix::PLACEHOLDER_Q => 17,
            Atrix::PLACEHOLDER_R => 18,
            Atrix::PLACEHOLDER_S => 19,
            Atrix::PLACEHOLDER_T => 20,
            Atrix::PLACEHOLDER_U => 21,
            Atrix::PLACEHOLDER_V => 22,
            Atrix::PLACEHOLDER_W => 23,
            Atrix::PLACEHOLDER_X => 24,
            Atrix::PLACEHOLDER_Y => 25,
            Atrix::PLACEHOLDER_Z => 26,
            _ => u64::MAX,
        }
    }
}

impl From<u64> for Atrix {
    fn from(val: u64) -> Atrix {
        match val {
            0 => Atrix::SOLUSDC,
            1 => Atrix::GMTUSDC,
            2 => Atrix::mSOLUSDC,
            3 => Atrix::BTCUSDC,
            4 => Atrix::USDrUSDC,
            5 => Atrix::PLACEHOLDER_E,
            6 => Atrix::PLACEHOLDER_F,
            7 => Atrix::PLACEHOLDER_G,
            8 => Atrix::PLACEHOLDER_H,
            9 => Atrix::PLACEHOLDER_I,
            10 => Atrix::PLACEHOLDER_J,
            11 => Atrix::PLACEHOLDER_K,
            12 => Atrix::PLACEHOLDER_L,
            13 => Atrix::PLACEHOLDER_M,
            14 => Atrix::PLACEHOLDER_N,
            15 => Atrix::PLACEHOLDER_O,
            16 => Atrix::PLACEHOLDER_P,
            17 => Atrix::PLACEHOLDER_Q,
            18 => Atrix::PLACEHOLDER_R,
            19 => Atrix::PLACEHOLDER_S,
            20 => Atrix::PLACEHOLDER_T,
            21 => Atrix::PLACEHOLDER_U,
            22 => Atrix::PLACEHOLDER_V,
            23 => Atrix::PLACEHOLDER_W,
            24 => Atrix::PLACEHOLDER_X,
            25 => Atrix::PLACEHOLDER_Y,
            26 => Atrix::PLACEHOLDER_Z,
            _ => Atrix::UNKNOWN,
        }
    }
}

impl From<&str> for Atrix {
    fn from(val: &str) -> Self {
        match val {
            "SOL-USDC" => Atrix::SOLUSDC,
            "GMT-USDC" => Atrix::GMTUSDC,
            "mSOL-USDC" => Atrix::mSOLUSDC,
            "BTC-USDC" => Atrix::BTCUSDC,
            "USDr-USDC" => Atrix::USDrUSDC,
            _ => Atrix::UNKNOWN,
        }
    }
}

impl ToString for Atrix {
    fn to_string(&self) -> String {
        match self {
            Atrix::SOLUSDC => String::from("SOL-USDC"),
            Atrix::GMTUSDC => String::from("GMT-USDC"),
            Atrix::mSOLUSDC => String::from("mSOL-USDC"),
            Atrix::BTCUSDC => String::from("BTC-USDC"),
            Atrix::USDrUSDC => String::from("USDr-USDC"),
            _ => String::from("UNKNOWN"),
        }
    }
}

impl Atrix {
    pub fn name(&self) -> String {
        self.to_string()
    }
}
