use anchor_lang::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
#[repr(u64)]
pub enum Orca {
    ATLASUSDC = 0,
    POLISUSDC = 1,
    ORCASOL = 2,
    USDTUSDC = 3,
    ORCAUSDC = 4,
    BASISUSDC = 5,
    SAMOUSDC = 6,
    SHDWUSDC = 7,
    SHDWSOL = 8,
    stSOLUSDC = 9,
    wUSTUSDC = 10,
    CMFIUSDC = 11,
    PLACEHOLDER_B = 12,
    PLACEHOLDER_C = 13,
    PLACEHOLDER_D = 14,
    PLACEHOLDER_E = 15,
    PLACEHOLDER_F = 16,
    PLACEHOLDER_G = 17,
    PLACEHOLDER_H = 18,
    PLACEHOLDER_I = 19,
    PLACEHOLDER_J = 20,
    PLACEHOLDER_K = 21,
    PLACEHOLDER_L = 22,
    PLACEHOLDER_M = 23,
    PLACEHOLDER_N = 24,
    PLACEHOLDER_O = 25,
    PLACEHOLDER_P = 26,
    PLACEHOLDER_Q = 27,
    PLACEHOLDER_R = 28,
    PLACEHOLDER_S = 29,
    PLACEHOLDER_T = 30,
    PLACEHOLDER_U = 31,
    PLACEHOLDER_V = 32,
    PLACEHOLDER_W = 33,
    PLACEHOLDER_X = 34,
    PLACEHOLDER_Y = 35,
    PLACEHOLDER_Z = 36,
    UNKNOWN = u64::MAX,
}

impl From<Orca> for u64 {
    fn from(val: Orca) -> Self {
        match val {
            Orca::ATLASUSDC => 0,
            Orca::POLISUSDC => 1,
            Orca::ORCASOL => 2,
            Orca::USDTUSDC => 3,
            Orca::ORCAUSDC => 4,
            Orca::BASISUSDC => 5,
            Orca::SAMOUSDC => 6,
            Orca::SHDWUSDC => 7,
            Orca::SHDWSOL => 8,
            Orca::stSOLUSDC => 9,
            Orca::wUSTUSDC => 10,
            Orca::CMFIUSDC => 11,
            Orca::PLACEHOLDER_B => 12,
            Orca::PLACEHOLDER_C => 13,
            Orca::PLACEHOLDER_D => 14,
            Orca::PLACEHOLDER_E => 15,
            Orca::PLACEHOLDER_F => 16,
            Orca::PLACEHOLDER_G => 17,
            Orca::PLACEHOLDER_H => 18,
            Orca::PLACEHOLDER_I => 19,
            Orca::PLACEHOLDER_J => 20,
            Orca::PLACEHOLDER_K => 21,
            Orca::PLACEHOLDER_L => 22,
            Orca::PLACEHOLDER_M => 23,
            Orca::PLACEHOLDER_N => 24,
            Orca::PLACEHOLDER_O => 25,
            Orca::PLACEHOLDER_P => 26,
            Orca::PLACEHOLDER_Q => 27,
            Orca::PLACEHOLDER_R => 28,
            Orca::PLACEHOLDER_S => 29,
            Orca::PLACEHOLDER_T => 30,
            Orca::PLACEHOLDER_U => 31,
            Orca::PLACEHOLDER_V => 32,
            Orca::PLACEHOLDER_W => 33,
            Orca::PLACEHOLDER_X => 34,
            Orca::PLACEHOLDER_Y => 35,
            Orca::PLACEHOLDER_Z => 36,
            _ => u64::MAX,
        }
    }
}

impl From<u64> for Orca {
    fn from(val: u64) -> Self {
        match val {
            0 => Orca::ATLASUSDC,
            1 => Orca::POLISUSDC,
            2 => Orca::ORCASOL,
            3 => Orca::USDTUSDC,
            4 => Orca::ORCAUSDC,
            5 => Orca::BASISUSDC,
            6 => Orca::SAMOUSDC,
            7 => Orca::SHDWUSDC,
            8 => Orca::SHDWSOL,
            9 => Orca::stSOLUSDC,
            10 => Orca::wUSTUSDC,
            11 => Orca::CMFIUSDC,
            12 => Orca::PLACEHOLDER_B,
            13 => Orca::PLACEHOLDER_C,
            14 => Orca::PLACEHOLDER_D,
            15 => Orca::PLACEHOLDER_E,
            16 => Orca::PLACEHOLDER_F,
            17 => Orca::PLACEHOLDER_G,
            18 => Orca::PLACEHOLDER_H,
            19 => Orca::PLACEHOLDER_I,
            20 => Orca::PLACEHOLDER_J,
            21 => Orca::PLACEHOLDER_K,
            22 => Orca::PLACEHOLDER_L,
            23 => Orca::PLACEHOLDER_M,
            24 => Orca::PLACEHOLDER_N,
            25 => Orca::PLACEHOLDER_O,
            26 => Orca::PLACEHOLDER_P,
            27 => Orca::PLACEHOLDER_Q,
            28 => Orca::PLACEHOLDER_R,
            29 => Orca::PLACEHOLDER_S,
            30 => Orca::PLACEHOLDER_T,
            31 => Orca::PLACEHOLDER_U,
            32 => Orca::PLACEHOLDER_V,
            33 => Orca::PLACEHOLDER_W,
            34 => Orca::PLACEHOLDER_X,
            35 => Orca::PLACEHOLDER_Y,
            36 => Orca::PLACEHOLDER_Z,
            _ => Orca::UNKNOWN,
        }
    }
}

impl From<&str> for Orca {
    fn from(val: &str) -> Self {
        match val {
            "ATLAS-USDC" => Orca::ATLASUSDC,
            "POLIS-USDC" => Orca::POLISUSDC,
            "ORCA-SOL" => Orca::ORCASOL,
            "USDT-USDC" => Orca::USDTUSDC,
            "ORCA-USDC" => Orca::ORCAUSDC,
            "BASIS-USDC" => Orca::BASISUSDC,
            "SAMO-USDC" => Orca::SAMOUSDC,
            "SHDW-USDC" => Orca::SHDWUSDC,
            "SHDW-SOL" => Orca::SHDWSOL,
            "stSOL-USDC" => Orca::stSOLUSDC,
            "wUST-USDC" => Orca::wUSTUSDC,
            "CMFI-USDC" => Orca::CMFIUSDC,
            _ => Orca::UNKNOWN,
        }
    }
}

impl ToString for Orca {
    fn to_string(&self) -> String {
        match self {
            Orca::ATLASUSDC => String::from("ATLAS-USDC"),
            Orca::POLISUSDC => String::from("POLIS-USDC"),
            Orca::ORCASOL => String::from("ORCA-SOL"),
            Orca::USDTUSDC => String::from("USDT-USDC"),
            Orca::ORCAUSDC => String::from("ORCA-USDC"),
            Orca::BASISUSDC => String::from("BASIS-USDC"),
            Orca::SAMOUSDC => String::from("SAMO-USDC"),
            Orca::SHDWUSDC => String::from("SHDW-USDC"),
            Orca::SHDWSOL => String::from("SHDW-SOL"),
            Orca::stSOLUSDC => String::from("stSOL-USDC"),
            Orca::wUSTUSDC => String::from("wUST-USDC"),
            Orca::CMFIUSDC => String::from("CMFI-USDC"),
            _ => String::from("UNKNOWN"),
        }
    }
}

impl Orca {
    pub fn name(&self) -> String {
        self.to_string()
    }
    pub fn is_double_dip(&self) -> bool {
        matches!(
            self,
            Orca::ATLASUSDC
                | Orca::POLISUSDC
                | Orca::BASISUSDC
                | Orca::SAMOUSDC
                | Orca::SHDWUSDC
                | Orca::SHDWSOL
                | Orca::stSOLUSDC
                | Orca::wUSTUSDC
                | Orca::CMFIUSDC
        )
    }
    pub fn is_stable(&self) -> bool {
        matches!(self, Orca::USDTUSDC | Orca::wUSTUSDC)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn orca() {
        let farm = Orca::from(0_u64);
        assert!(farm == Orca::ATLASUSDC);
        assert!(farm.name().eq(&"ATLAS-USDC".to_string()));
        assert_eq!(farm.is_double_dip(), true);
        assert!(Orca::from("ATLAS-USDC") == Orca::ATLASUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 0);

        let farm = Orca::from(1_u64);
        assert!(farm == Orca::POLISUSDC);
        assert!(farm.name().eq(&"POLIS-USDC".to_string()));
        assert_eq!(farm.is_double_dip(), true);
        assert!(Orca::from("POLIS-USDC") == Orca::POLISUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 1);

        let farm = Orca::from(2_u64);
        assert!(farm == Orca::ORCASOL);
        assert!(farm.name().eq(&"ORCA-SOL".to_string()));
        assert_eq!(farm.is_double_dip(), false);
        assert!(Orca::from("ORCA-SOL") == Orca::ORCASOL);
        let number: u64 = farm.into();
        assert_eq!(number, 2);

        let farm = Orca::from(42069_u64);
        assert!(farm == Orca::UNKNOWN);
        assert!(farm.name().eq(&"UNKNOWN".to_string()));
        assert_eq!(farm.is_double_dip(), false);
        assert!(Orca::from("UNKNONWASANDJASDJASJD") == Orca::UNKNOWN);
        let number: u64 = farm.into();
        assert_eq!(number, u64::MAX);
    }
}
