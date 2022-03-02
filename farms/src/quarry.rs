//! farm types supported by Quarry
use anchor_lang::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
#[repr(u64)]
pub enum Quarry {
    VANILLA = 0_u64,
    /// indicates this is a Saber quarry farm
    SABER = 1_u64,
    /// indicates this is a Sunny quarry farm
    SUNNY = 2_u64,
    UNKNOWN = u64::MAX,
}

impl From<Quarry> for u64 {
    fn from(val: Quarry) -> Self {
        match val {
            Quarry::VANILLA => 0,
            Quarry::SABER => 1,
            Quarry::SUNNY => 2,
            _ => u64::MAX,
        }
    }
}

impl From<u64> for Quarry {
    fn from(val: u64) -> Self {
        match val {
            0 => Quarry::VANILLA,
            1 => Quarry::SABER,
            2 => Quarry::SUNNY,
            _ => Quarry::UNKNOWN,
        }
    }
}

impl From<&str> for Quarry {
    fn from(val: &str) -> Self {
        match val {
            "VANILLA" => Quarry::VANILLA,
            "SABER" => Quarry::SABER,
            "SUNNY" => Quarry::SUNNY,
            _ => Quarry::UNKNOWN,
        }
    }
}

impl Quarry {
    pub fn name(&self) -> String {
        match self {
            Quarry::SABER => String::from("SABER"),
            Quarry::SUNNY => String::from("SUNNY"),
            Quarry::VANILLA => String::from("VANILLA"),
            _ => "UNKNOWN".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quarry_farms() {
        let farm: Quarry = 0.into();
        assert_eq!(farm, Quarry::VANILLA);
        let farm2: Quarry = "VANILLA".into();
        assert_eq!(farm, farm2);
        assert_eq!(farm.name(), "VANILLA".to_string());
    }
}
