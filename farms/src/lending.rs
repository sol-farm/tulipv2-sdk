//! lending plattforms

use anchor_lang::prelude::*;

/// represents a type of lending farm, which may be platform specific such as
/// obtaining the highest yield % on a platform like tulip, or asset specific
/// such as obtaining the highest yield % on a single asset like USDC,
///
/// since the enum is represented as a u64 type, numbers 0 -> 65535 are reserved
/// for platform identifiers, while numbers 65536 and above are to be taken
/// on a first come first serve basis
#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum Lending {
    /// a farm focused on tulip lending
    TULIP = 0,
    /// a farm focused on solend lending
    SOLEND = 1,
    /// a farm focused on mango lending
    MANGO = 2,
    /// a farm focused on port lending
    PORT = 3,
    /// a farm focused on larix lending
    LARIX = 4,
    /// a farm focused on parrot lending
    PARROT = 5,
    /// a farm focused on usdc lending
    USDC = 65536,
    /// a farm capable of depositing into multiple
    /// lending programs at the same time
    MULTI_DEPOSIT = 65537,
    /// a farm focused on ray lending
    RAY = 65538,
    USDT = 65539,
    SOL = 65540,
    PLACEHOLDER_E = 65541,
    PLACEHOLDER_F = 65542,
    PLACEHOLDER_G = 65543,
    PLACEHOLDER_H = 65544,
    PLACEHOLDER_I = 65545,
    PLACEHOLDER_J = 65546,
    PLACEHOLDER_K = 65547,
    PLACEHOLDER_L = 65548,
    PLACEHOLDER_M = 65549,
    PLACEHOLDER_N = 65550,
    PLACEHOLDER_O = 65551,
    PLACEHOLDER_P = 65552,
    PLACEHOLDER_Q = 65553,
    PLACEHOLDER_R = 65554,
    PLACEHOLDER_S = 65555,
    PLACEHOLDER_T = 65556,
    PLACEHOLDER_U = 65557,
    PLACEHOLDER_V = 65558,
    PLACEHOLDER_W = 65559,
    PLACEHOLDER_X = 65560,
    PLACEHOLDER_Y = 65561,
    PLACEHOLDER_Z = 65562,
    UNKNOWN = u64::MAX,
}

impl From<Lending> for u64 {
    fn from(val: Lending) -> Self {
        match val {
            Lending::TULIP => 0,
            Lending::SOLEND => 1,
            Lending::MANGO => 2,
            Lending::PORT => 3,
            Lending::LARIX => 4,
            Lending::PARROT => 5,
            Lending::USDC => 65536,
            Lending::MULTI_DEPOSIT => 65537,
            Lending::RAY => 65538,
            Lending::USDT => 65539,
            Lending::SOL => 65540,
            Lending::PLACEHOLDER_E => 65541,
            Lending::PLACEHOLDER_F => 65542,
            Lending::PLACEHOLDER_G => 65543,
            Lending::PLACEHOLDER_H => 65544,
            Lending::PLACEHOLDER_I => 65545,
            Lending::PLACEHOLDER_J => 65546,
            Lending::PLACEHOLDER_K => 65547,
            Lending::PLACEHOLDER_L => 65548,
            Lending::PLACEHOLDER_M => 65549,
            Lending::PLACEHOLDER_N => 65550,
            Lending::PLACEHOLDER_O => 65551,
            Lending::PLACEHOLDER_P => 65552,
            Lending::PLACEHOLDER_Q => 65553,
            Lending::PLACEHOLDER_R => 65554,
            Lending::PLACEHOLDER_S => 65555,
            Lending::PLACEHOLDER_T => 65556,
            Lending::PLACEHOLDER_U => 65557,
            Lending::PLACEHOLDER_V => 65558,
            Lending::PLACEHOLDER_W => 65559,
            Lending::PLACEHOLDER_X => 65560,
            Lending::PLACEHOLDER_Y => 65561,
            Lending::PLACEHOLDER_Z => 65562,
            _ => u64::MAX,
        }
    }
}

impl From<u64> for Lending {
    fn from(val: u64) -> Self {
        match val {
            0 => Lending::TULIP,
            1 => Lending::SOLEND,
            2 => Lending::MANGO,
            3 => Lending::PORT,
            4 => Lending::LARIX,
            5 => Lending::PARROT,
            65536 => Lending::USDC,
            65537 => Lending::MULTI_DEPOSIT,
            65538 => Lending::RAY,
            65539 => Lending::USDT,
            65540 => Lending::SOL,
            65541 => Lending::PLACEHOLDER_E,
            65542 => Lending::PLACEHOLDER_F,
            65543 => Lending::PLACEHOLDER_G,
            65544 => Lending::PLACEHOLDER_H,
            65545 => Lending::PLACEHOLDER_I,
            65546 => Lending::PLACEHOLDER_J,
            65547 => Lending::PLACEHOLDER_K,
            65548 => Lending::PLACEHOLDER_L,
            65549 => Lending::PLACEHOLDER_M,
            65550 => Lending::PLACEHOLDER_N,
            65551 => Lending::PLACEHOLDER_O,
            65552 => Lending::PLACEHOLDER_P,
            65553 => Lending::PLACEHOLDER_Q,
            65554 => Lending::PLACEHOLDER_R,
            65555 => Lending::PLACEHOLDER_S,
            65556 => Lending::PLACEHOLDER_T,
            65557 => Lending::PLACEHOLDER_U,
            65558 => Lending::PLACEHOLDER_V,
            65559 => Lending::PLACEHOLDER_W,
            65560 => Lending::PLACEHOLDER_X,
            65561 => Lending::PLACEHOLDER_Y,
            65562 => Lending::PLACEHOLDER_Z,
            _ => Lending::UNKNOWN,
        }
    }
}

impl From<&str> for Lending {
    fn from(val: &str) -> Self {
        match val {
            "TULIP" => Lending::TULIP,
            "SOLEND" => Lending::SOLEND,
            "MANGO" => Lending::MANGO,
            "PORT" => Lending::PORT,
            "LARIX" => Lending::LARIX,
            "PARROT" => Lending::PARROT,
            "USDC" => Lending::USDC,
            "MULTI_DEPOSIT" => Lending::MULTI_DEPOSIT,
            "RAY" => Lending::RAY,
            "USDT" => Lending::USDT,
            "SOL" => Lending::SOL,
            _ => Lending::UNKNOWN,
        }
    }
}

impl ToString for Lending {
    fn to_string(&self) -> String {
        match self {
            Lending::TULIP => "TULIP".to_string(),
            Lending::SOLEND => "SOLEND".to_string(),
            Lending::MANGO => "MANGO".to_string(),
            Lending::PORT => "PORT".to_string(),
            Lending::LARIX => "LARIX".to_string(),
            Lending::PARROT => "PARROT".to_string(),
            Lending::USDC => "USDC".to_string(),
            Lending::MULTI_DEPOSIT => "MULTI_DEPOSIT".to_string(),
            Lending::RAY => "RAY".to_string(),
            Lending::USDT => "USDT".to_string(),
            Lending::SOL => "SOL".to_string(),
            _ => "UNKNOWN".to_string(),
        }
    }
}

impl Lending {
    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lending() {
        let farm = Lending::from(0_u64);
        assert!(farm == Lending::TULIP);
        assert!(farm.name().eq(&"TULIP".to_string()));
        assert!(Lending::from("TULIP") == Lending::TULIP);
        let number: u64 = farm.into();
        assert_eq!(number, 0);

        let farm = Lending::from(1_u64);
        assert!(farm == Lending::SOLEND);
        assert!(farm.name().eq(&"SOLEND".to_string()));
        assert!(Lending::from("SOLEND") == Lending::SOLEND);
        let number: u64 = farm.into();
        assert_eq!(number, 1);

        let farm = Lending::from(2_u64);
        assert!(farm == Lending::MANGO);
        assert!(farm.name().eq(&"MANGO".to_string()));
        assert!(Lending::from("MANGO") == Lending::MANGO);
        let number: u64 = farm.into();
        assert_eq!(number, 2);

        let farm = Lending::from(3_u64);
        assert!(farm == Lending::PORT);
        assert!(farm.name().eq(&"PORT".to_string()));
        assert!(Lending::from("PORT") == Lending::PORT);
        let number: u64 = farm.into();
        assert_eq!(number, 3);

        let farm = Lending::from(4_u64);
        assert!(farm == Lending::LARIX);
        assert!(farm.name().eq(&"LARIX".to_string()));
        assert!(Lending::from("LARIX") == Lending::LARIX);
        let number: u64 = farm.into();
        assert_eq!(number, 4);

        let farm = Lending::from(5_u64);
        assert!(farm == Lending::PARROT);
        assert!(farm.name().eq(&"PARROT".to_string()));
        assert!(Lending::from("PARROT") == Lending::PARROT);
        let number: u64 = farm.into();
        assert_eq!(number, 5);

        let farm = Lending::from(65536_u64);
        assert!(farm == Lending::USDC);
        assert!(farm.name().eq(&"USDC".to_string()));
        assert!(Lending::from("USDC") == Lending::USDC);
        let number: u64 = farm.into();
        assert_eq!(number, 65536);

        let farm = Lending::from(u64::MAX / 2);
        assert!(farm == Lending::UNKNOWN);
        assert!(farm.name().eq(&"UNKNOWN".to_string()));
        assert!(Lending::from("UNKNOWN") == Lending::UNKNOWN);
        let number: u64 = farm.into();
        assert_eq!(number, u64::MAX);
    }
}
