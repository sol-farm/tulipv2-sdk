//! farm types supported by raydium, some of these may or may not be activate at any given moment
use anchor_lang::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
#[repr(u64)]
pub enum Raydium {
    ALEPHUSDC = 0_u64,
    BOPRAY = 1_u64,
    COPEUSDC = 2_u64,
    LIKEUSDC = 3_u64,
    PSYUSDC = 4_u64,
    MERUSDC = 5_u64,
    stSOLUSDC = 6_u64,
    RAY = 7_u64,
    RAYUSDT = 8_u64,
    RAYUSDC = 9_u64,
    RAYSRM = 10_u64,
    RAYSOL = 11_u64,
    RAYETH = 12_u64,
    ROPEUSDC = 13_u64,
    SAMORAY = 14_u64,
    // farm ended, identifier can be reused
    SNYUSDC = 15_u64,
    // farm ended, identifier can be reused
    stSOLUSDT = 16_u64,
    TULIPUSDC = 17_u64,
    ATLASRAY = 18_u64,
    POLISRAY = 19_u64,
    PLACEHOLDER_A = 20,
    PLACEHOLDER_B = 21,
    PLACEHOLDER_C = 22,
    PLACEHOLDER_D = 23,
    PLACEHOLDER_E = 24,
    PLACEHOLDER_F = 25,
    PLACEHOLDER_G = 26,
    PLACEHOLDER_H = 27,
    PLACEHOLDER_I = 28,
    PLACEHOLDER_J = 29,
    PLACEHOLDER_K = 30,
    PLACEHOLDER_L = 31,
    PLACEHOLDER_M = 32,
    PLACEHOLDER_N = 33,
    PLACEHOLDER_O = 34,
    PLACEHOLDER_P = 35,
    PLACEHOLDER_Q = 36,
    PLACEHOLDER_R = 37,
    PLACEHOLDER_S = 38,
    PLACEHOLDER_T = 39,
    PLACEHOLDER_U = 40,
    PLACEHOLDER_V = 41,
    PLACEHOLDER_W = 42,
    PLACEHOLDER_X = 43,
    PLACEHOLDER_Y = 44,
    PLACEHOLDER_Z = 55,
    UNKNOWN = u64::MAX,
}

impl From<Raydium> for u64 {
    fn from(val: Raydium) -> Self {
        match val {
            Raydium::ALEPHUSDC => 0,
            Raydium::BOPRAY => 1,
            Raydium::COPEUSDC => 2,
            Raydium::LIKEUSDC => 3,
            Raydium::PSYUSDC => 4,
            Raydium::MERUSDC => 5,
            Raydium::stSOLUSDC => 6,
            Raydium::RAY => 7,
            Raydium::RAYUSDT => 8,
            Raydium::RAYUSDC => 9,
            Raydium::RAYSRM => 10,
            Raydium::RAYSOL => 11,
            Raydium::RAYETH => 12,
            Raydium::ROPEUSDC => 13,
            Raydium::SAMORAY => 14,
            Raydium::SNYUSDC => 15,
            Raydium::stSOLUSDT => 16,
            Raydium::TULIPUSDC => 17,
            Raydium::POLISRAY => 18,
            Raydium::ATLASRAY => 19,
            Raydium::PLACEHOLDER_A => 20,
            Raydium::PLACEHOLDER_B => 21,
            Raydium::PLACEHOLDER_C => 22,
            Raydium::PLACEHOLDER_D => 23,
            Raydium::PLACEHOLDER_E => 24,
            Raydium::PLACEHOLDER_F => 25,
            Raydium::PLACEHOLDER_G => 26,
            Raydium::PLACEHOLDER_H => 27,
            Raydium::PLACEHOLDER_I => 28,
            Raydium::PLACEHOLDER_J => 29,
            Raydium::PLACEHOLDER_K => 30,
            Raydium::PLACEHOLDER_L => 31,
            Raydium::PLACEHOLDER_M => 32,
            Raydium::PLACEHOLDER_N => 33,
            Raydium::PLACEHOLDER_O => 34,
            Raydium::PLACEHOLDER_P => 35,
            Raydium::PLACEHOLDER_Q => 36,
            Raydium::PLACEHOLDER_R => 37,
            Raydium::PLACEHOLDER_S => 38,
            Raydium::PLACEHOLDER_T => 39,
            Raydium::PLACEHOLDER_U => 40,
            Raydium::PLACEHOLDER_V => 41,
            Raydium::PLACEHOLDER_W => 42,
            Raydium::PLACEHOLDER_X => 43,
            Raydium::PLACEHOLDER_Y => 44,
            Raydium::PLACEHOLDER_Z => 55,
            _ => u64::MAX,
        }
    }
}

impl From<u64> for Raydium {
    fn from(val: u64) -> Self {
        match val {
            0 => Raydium::ALEPHUSDC,
            1 => Raydium::BOPRAY,
            2 => Raydium::COPEUSDC,
            3 => Raydium::LIKEUSDC,
            4 => Raydium::PSYUSDC,
            5 => Raydium::MERUSDC,
            6 => Raydium::stSOLUSDC,
            7 => Raydium::RAY,
            8 => Raydium::RAYUSDT,
            9 => Raydium::RAYUSDC,
            10 => Raydium::RAYSRM,
            11 => Raydium::RAYSOL,
            12 => Raydium::RAYETH,
            13 => Raydium::ROPEUSDC,
            14 => Raydium::SAMORAY,
            15 => Raydium::SNYUSDC,
            16 => Raydium::stSOLUSDT,
            17 => Raydium::TULIPUSDC,
            18 => Raydium::POLISRAY,
            19 => Raydium::ATLASRAY,
            20 => Raydium::PLACEHOLDER_A,
            21 => Raydium::PLACEHOLDER_B,
            22 => Raydium::PLACEHOLDER_C,
            23 => Raydium::PLACEHOLDER_D,
            24 => Raydium::PLACEHOLDER_E,
            25 => Raydium::PLACEHOLDER_F,
            26 => Raydium::PLACEHOLDER_G,
            27 => Raydium::PLACEHOLDER_H,
            28 => Raydium::PLACEHOLDER_I,
            29 => Raydium::PLACEHOLDER_J,
            30 => Raydium::PLACEHOLDER_K,
            31 => Raydium::PLACEHOLDER_L,
            32 => Raydium::PLACEHOLDER_M,
            33 => Raydium::PLACEHOLDER_N,
            34 => Raydium::PLACEHOLDER_O,
            35 => Raydium::PLACEHOLDER_P,
            36 => Raydium::PLACEHOLDER_Q,
            37 => Raydium::PLACEHOLDER_R,
            38 => Raydium::PLACEHOLDER_S,
            39 => Raydium::PLACEHOLDER_T,
            40 => Raydium::PLACEHOLDER_U,
            41 => Raydium::PLACEHOLDER_V,
            42 => Raydium::PLACEHOLDER_W,
            43 => Raydium::PLACEHOLDER_X,
            44 => Raydium::PLACEHOLDER_Y,
            55 => Raydium::PLACEHOLDER_Z,
            _ => Raydium::UNKNOWN,
        }
    }
}

impl From<&str> for Raydium {
    fn from(val: &str) -> Self {
        match val {
            "ALEPH-USDC" => Raydium::ALEPHUSDC,
            "BOP-RAY" => Raydium::BOPRAY,
            "COPE-USDC" => Raydium::COPEUSDC,
            "LIKE-USDC" => Raydium::LIKEUSDC,
            "PSY-USDC" => Raydium::PSYUSDC,
            "MER-USDC" => Raydium::MERUSDC,
            "stSOL-USDC" => Raydium::stSOLUSDC,
            "RAY" => Raydium::RAY,
            "RAY-USDT" => Raydium::RAYUSDT,
            "RAY-USDC" => Raydium::RAYUSDC,
            "RAY-SRM" => Raydium::RAYSRM,
            "RAY-SOL" => Raydium::RAYSOL,
            "RAY-ETH" => Raydium::RAYETH,
            "ROPE-USDC" => Raydium::ROPEUSDC,
            "SAMO-RAY" => Raydium::SAMORAY,
            "SNY-USDC" => Raydium::SNYUSDC,
            "stSOL-USDT" => Raydium::stSOLUSDT,
            "TULIP-USDC" => Raydium::TULIPUSDC,
            "POLIS-RAY" => Raydium::POLISRAY,
            "ATLAS-RAY" => Raydium::ATLASRAY,
            _ => Raydium::UNKNOWN,
        }
    }
}

impl Raydium {
    pub fn name(&self) -> String {
        match self {
            Raydium::ALEPHUSDC => String::from("ALEPH-USDC"),
            Raydium::BOPRAY => String::from("BOP-RAY"),
            Raydium::COPEUSDC => String::from("COPE-USDC"),
            Raydium::LIKEUSDC => String::from("LIKE-USDC"),
            Raydium::PSYUSDC => String::from("PSY-USDC"),
            Raydium::MERUSDC => String::from("MER-USDC"),
            Raydium::stSOLUSDC => String::from("stSOL-USDC"),
            Raydium::RAY => String::from("RAY"),
            Raydium::RAYUSDT => String::from("RAY-USDT"),
            Raydium::RAYUSDC => String::from("RAY-USDC"),
            Raydium::RAYSRM => String::from("RAY-SRM"),
            Raydium::RAYSOL => String::from("RAY-SOL"),
            Raydium::RAYETH => String::from("RAY-ETH"),
            Raydium::ROPEUSDC => String::from("ROPE-USDC"),
            Raydium::SAMORAY => String::from("SAMO-RAY"),
            Raydium::SNYUSDC => String::from("SNY-USDC"),
            Raydium::stSOLUSDT => String::from("stSOL-USDT"),
            Raydium::TULIPUSDC => String::from("TULIP-USDC"),
            Raydium::ATLASRAY => String::from("ATLAS-RAY"),
            Raydium::POLISRAY => String::from("POLIS-RAY"),
            _ => String::from("UNKNOWN"),
        }
    }
    pub fn dual_reward(&self) -> bool {
        matches!(
            self,
            Raydium::BOPRAY | Raydium::SAMORAY | Raydium::ATLASRAY | Raydium::POLISRAY
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn raydium() {
        let farm = Raydium::from(0_u64);
        assert!(farm == Raydium::ALEPHUSDC);
        assert!(farm.name().eq(&"ALEPH-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("ALEPH-USDC") == Raydium::ALEPHUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 0);

        let farm = Raydium::from(1_u64);
        assert!(farm == Raydium::BOPRAY);
        assert!(farm.name().eq(&"BOP-RAY".to_string()));
        assert_eq!(farm.dual_reward(), true);
        assert!(Raydium::from("BOP-RAY") == Raydium::BOPRAY);
        let number: u64 = farm.into();
        assert_eq!(number, 1);

        let farm = Raydium::from(2_u64);
        assert!(farm == Raydium::COPEUSDC);
        assert!(farm.name().eq(&"COPE-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("COPE-USDC") == Raydium::COPEUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 2);

        let farm = Raydium::from(3_u64);
        assert!(farm == Raydium::LIKEUSDC);
        assert!(farm.name().eq(&"LIKE-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("LIKE-USDC") == Raydium::LIKEUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 3);

        let farm = Raydium::from(4_u64);
        assert!(farm == Raydium::PSYUSDC);
        assert!(farm.name().eq(&"PSY-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("PSY-USDC") == Raydium::PSYUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 4);

        let farm = Raydium::from(5_u64);
        assert!(farm == Raydium::MERUSDC);
        assert!(farm.name().eq(&"MER-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("MER-USDC") == Raydium::MERUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 5);

        let farm = Raydium::from(6_u64);
        assert!(farm == Raydium::stSOLUSDC);
        assert!(farm.name().eq(&"stSOL-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("stSOL-USDC") == Raydium::stSOLUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 6);

        let farm = Raydium::from(7_u64);
        assert!(farm == Raydium::RAY);
        assert!(farm.name().eq(&"RAY".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("RAY") == Raydium::RAY);
        let number: u64 = farm.into();
        assert_eq!(number, 7);

        let farm = Raydium::from(8_u64);
        assert!(farm == Raydium::RAYUSDT);
        assert!(farm.name().eq(&"RAY-USDT".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("RAY-USDT") == Raydium::RAYUSDT);
        let number: u64 = farm.into();
        assert_eq!(number, 8);

        let farm = Raydium::from(9_u64);
        assert!(farm == Raydium::RAYUSDC);
        assert!(farm.name().eq(&"RAY-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("RAY-USDC") == Raydium::RAYUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 9);

        let farm = Raydium::from(10_u64);
        assert!(farm == Raydium::RAYSRM);
        assert!(farm.name().eq(&"RAY-SRM".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("RAY-SRM") == Raydium::RAYSRM);
        let number: u64 = farm.into();
        assert_eq!(number, 10);

        let farm = Raydium::from(11_u64);
        assert!(farm == Raydium::RAYSOL);
        assert!(farm.name().eq(&"RAY-SOL".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("RAY-SOL") == Raydium::RAYSOL);
        let number: u64 = farm.into();
        assert_eq!(number, 11);

        let farm = Raydium::from(12_u64);
        assert!(farm == Raydium::RAYETH);
        assert!(farm.name().eq(&"RAY-ETH".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("RAY-ETH") == Raydium::RAYETH);
        let number: u64 = farm.into();
        assert_eq!(number, 12);

        let farm = Raydium::from(13_u64);
        assert!(farm == Raydium::ROPEUSDC);
        assert!(farm.name().eq(&"ROPE-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("ROPE-USDC") == Raydium::ROPEUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 13);

        let farm = Raydium::from(14_u64);
        assert!(farm == Raydium::SAMORAY);
        assert!(farm.name().eq(&"SAMO-RAY".to_string()));
        assert_eq!(farm.dual_reward(), true);
        assert!(Raydium::from("SAMO-RAY") == Raydium::SAMORAY);
        let number: u64 = farm.into();
        assert_eq!(number, 14);

        let farm = Raydium::from(15_u64);
        assert!(farm == Raydium::SNYUSDC);
        assert!(farm.name().eq(&"SNY-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("SNY-USDC") == Raydium::SNYUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 15);

        let farm = Raydium::from(16_u64);
        assert!(farm == Raydium::stSOLUSDT);
        assert!(farm.name().eq(&"stSOL-USDT".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("stSOL-USDT") == Raydium::stSOLUSDT);
        let number: u64 = farm.into();
        assert_eq!(number, 16);

        let farm = Raydium::from(17_u64);
        assert!(farm == Raydium::TULIPUSDC);
        assert!(farm.name().eq(&"TULIP-USDC".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("TULIP-USDC") == Raydium::TULIPUSDC);
        let number: u64 = farm.into();
        assert_eq!(number, 17);

        let farm = Raydium::from(18_u64);
        assert!(farm == Raydium::POLISRAY);
        assert!(farm.name().eq(&"POLIS-RAY".to_string()));
        assert_eq!(farm.dual_reward(), true);
        assert!(Raydium::from("POLIS-RAY") == Raydium::POLISRAY);
        let number: u64 = farm.into();
        assert_eq!(number, 18);

        let farm = Raydium::from(19_u64);
        assert!(farm == Raydium::ATLASRAY);
        assert!(farm.name().eq(&"ATLAS-RAY".to_string()));
        assert_eq!(farm.dual_reward(), true);
        assert!(Raydium::from("ATLAS-RAY") == Raydium::ATLASRAY);
        let number: u64 = farm.into();
        assert_eq!(number, 19);

        let farm = Raydium::from(u64::MAX);
        assert!(farm == Raydium::UNKNOWN);
        assert!(farm.name().eq(&"UNKNOWN".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("abababab") == Raydium::UNKNOWN);
        let number: u64 = farm.into();
        assert_eq!(number, u64::MAX);

        let farm = Raydium::from(u64::MAX - 1);
        assert!(farm == Raydium::UNKNOWN);
        assert!(farm.name().eq(&"UNKNOWN".to_string()));
        assert_eq!(farm.dual_reward(), false);
        assert!(Raydium::from("bcbcbcb") == Raydium::UNKNOWN);
        let number: u64 = farm.into();
        assert_eq!(number, u64::MAX);
    }
}
