//! Provides a `Farm` type used to describe different platforms (ie Raydium), and
//! and farms within those platforms (RAY-USDC). The underlying representation of the
//! `Farm` type consists of a 2 element u64 array. The first element of the array identifiers the platform, 
//! while the second element of the array identifies the farm type. In addition to identifying vaults
//! the `Farm` type serves a secondary purpose, which is to ensure vault address generation is deterministic.
//! 
//! In certain circumstances, the same `Farm` type may be used for multiple vaults, so to prevent
//! deterministic address generation of Solana PDA's from causing collisions, a the underlying 
//! representation of the `Farm` type is combined with a 32 byte `tag`, defaulting to [0_u8; 32]
//! 
//! For the rest of the comments/docs the first element of the underlying representation will be called
//! the `Farm Identifier` and the second element will be called the `Farm Name`. Farm names are defined
//! within modules in appropriately named files (lending.rs, orca.rs, etc..)
//! 
//! Within the files for the farm names there may be enum variants with names such as
//! `PLACEHOLDER_XYZ`. These are intended to allow creation of new vault types without
//! needing to upgrade the program to enable a new set of farm names to be valid.

#![deny(unused_must_use)]
#![deny(clippy::all)]
#![allow(clippy::bool_assert_comparison)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]

pub mod lending;
pub mod orca;
pub mod quarry;
pub mod raydium;
pub mod unknown;

use arrform::{arrform, ArrForm};
use anchor_lang::prelude::*;
use crate::{
    raydium::Raydium,
    lending::Lending,
    orca::Orca,
    quarry::Quarry,
    unknown::Unknown,
};

#[derive(Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Debug)]
pub enum Farm {
    /// 0
    /// indicates the farm is of type raydium
    Raydium {
        /// indicates the name of the given farm
        name: Raydium,
    },
    /// 1
    /// indicates the farm is of type lending
    Lending {
        /// indicates the name of the given lending platform
        name: Lending,
    },
    /// 2
    /// indicates the farm is of type orca
    Orca {
        /// indicates the name of the given orca farm
        name: Orca,
    },
    /// 3
    /// indicates the farm is of type quarry
    Quarry {
        /// indicates the name of the given quarry farm
        name: Quarry,
    },
    /// u64::MAX
    /// an unknown farm type, indicates an error
    Unknown { name: Unknown },
}

impl Farm {
    pub fn name(&self) -> String {
        self.to_string()
    }
    /// strips the farm identifier portion of the name
    /// returning the farm name, which corresponds to the market name
    /// of the underlying asset. 
    /// 
    /// This is irrelevant for lending farm identifiers as the farm name is likely to be a single asset
    /// and is also irrelevant for quarry farm identifier as the farm name indicates the 
    /// variant of quarry implementations, wrappers, etc..
    pub fn market_name(&self) -> Option<String> {
        if let Farm::Raydium { name: _name } = self {
            let name = self.name();
            let lp_name_parts: Vec<_> = name.split('-').collect();
            if lp_name_parts.len() < 2 {
                return None;
            }
            return Some(farm_identifier_stripper(&name, &lp_name_parts));
        }
        if let Farm::Orca { name: _name } = self {
            let name = self.name();
            let lp_name_parts: Vec<_> = name.split('-').collect();
            if lp_name_parts.len() < 2 {
                return None;
            }
            return Some(farm_identifier_stripper(&name, &lp_name_parts));
        }
        if let Farm::Lending { name } = self {
            return Some(name.name());
        }
        if let Farm::Quarry { name } = self {
            return Some(name.name());
        }
        None
    }
}

impl ToString for Farm {
    fn to_string(&self) -> String {
        match self {
            Farm::Raydium { name } => arrform!(128, "RAYDIUM-{}", name.name()).as_str().to_owned(),
            Farm::Lending { name } => arrform!(128, "LENDING-{}", name.name()).as_str().to_owned(),
            Farm::Orca { name } => arrform!(128, "ORCA-{}", name.name()).as_str().to_owned(),
            Farm::Quarry { name } => arrform!(128, "QUARRY-{}", name.name()).as_str().to_owned(),
            _ => String::from("UNKNOWN"),
        }
    }
}

impl From<&String> for Farm {
    fn from(val: &String) -> Self {
        farm_from_str(val)
    }
}

impl From<&str> for Farm {
    fn from(val: &str) -> Self {
        farm_from_str(val)
    }
}

impl Default for Farm {
    fn default() -> Self {
        Self::Unknown {
            name: Unknown::Uknown,
        }
    }
}

impl From<Raydium> for Farm {
    fn from(val: Raydium) -> Self {
        Farm::Raydium { name: val }
    }
}

impl From<Lending> for Farm {
    fn from(val: Lending) -> Self {
        Farm::Lending { name: val }
    }
}

impl From<Orca> for Farm {
    fn from(val: Orca) -> Self {
        Farm::Orca { name: val }
    }
}

impl From<Quarry> for Farm {
    fn from(val: Quarry) -> Self {
        Farm::Quarry { name: val }
    }
}

impl From<[u64; 2]> for Farm {
    fn from(val: [u64; 2]) -> Self {
        match val[0] {
            0 => Farm::Raydium {
                name: val[1].into(),
            },
            1 => Farm::Lending {
                name: val[1].into(),
            },
            2 => Farm::Orca {
                name: val[1].into(),
            },
            3 => Farm::Quarry {
                name: val[1].into(),
            },
            _ => Farm::Unknown {
                name: Unknown::Uknown,
            },
        }
    }
}

impl From<Farm> for [u64; 2] {
    fn from(value: Farm) -> Self {
        match value {
            Farm::Raydium { name } => [0_u64, name.into()],
            Farm::Lending { name } => [1_u64, name.into()],
            Farm::Orca { name } => [2_u64, name.into()],
            Farm::Quarry { name } => [3_u64, name.into()],
            _ => [u64::MAX, u64::MAX],
        }
    }
}

/// used to strip the farm identifier portion of a farm type, returning only the farm name
pub fn farm_identifier_stripper(
    farm_type: &str,
    parts: &Vec<&str>,
) -> String {
    let mut farm_name = String::with_capacity(farm_type.len() - parts[0].len());
    for (idx, part) in parts.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        farm_name.push_str(part);
        if idx < parts.len() - 1 {
            farm_name.push('-');
        }
    }
    farm_name
}

fn farm_from_str(val: &str) -> Farm {
    let parts: Vec<_> = val.split('-').collect();
    if parts.len() <= 1 {
        return Farm::Unknown {
            name: Unknown::Uknown,
        };
    }
    match parts[0] {
        "RAYDIUM" => {
            let farm_name = farm_identifier_stripper(val, &parts);
            Farm::Raydium {
                name: Raydium::from(farm_name.as_str()),
            }
        }
        "LENDING" => {
            let farm_name = farm_identifier_stripper(val, &parts);
            Farm::Lending {
                name: Lending::from(farm_name.as_str()),
            }
        }
        "ORCA" => {
            let farm_name = farm_identifier_stripper(val, &parts);
            Farm::Orca {
                name: Orca::from(farm_name.as_str()),
            }
        }
        "QUARRY" => {
            let farm_name = farm_identifier_stripper(val, &parts);
            Farm::Quarry {
                name: Quarry::from(farm_name.as_str()),
            }
        }
        _ => Farm::Unknown {
            name: Unknown::Uknown,
        },
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn size_of() {
        println!("size {}", std::mem::size_of::<Farm>());
    }
    #[test]
    fn test_farm_raydium() {
        let farm = Farm::from(Raydium::from(0_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::ALEPHUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-ALEPH-USDC".to_string()));
        assert!(Farm::from(Raydium::from("ALEPH-USDC")).eq(&Farm::Raydium {
            name: Raydium::ALEPHUSDC
        }));
        let farm2 = Farm::from([0_u64, 0_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(1_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::BOPRAY
        }));
        assert!(farm.name().eq(&"RAYDIUM-BOP-RAY".to_string()));
        assert!(Farm::from(Raydium::from("BOP-RAY")).eq(&Farm::Raydium {
            name: Raydium::BOPRAY
        }));
        let farm2 = Farm::from([0_u64, 1_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(2_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::COPEUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-COPE-USDC".to_string()));
        assert!(Farm::from(Raydium::from("COPE-USDC")).eq(&Farm::Raydium {
            name: Raydium::COPEUSDC
        }));
        let farm2 = Farm::from([0_u64, 2_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(3_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::LIKEUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-LIKE-USDC".to_string()));
        assert!(Farm::from(Raydium::from("LIKE-USDC")).eq(&Farm::Raydium {
            name: Raydium::LIKEUSDC
        }));
        let farm2 = Farm::from([0_u64, 3_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(4_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::PSYUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-PSY-USDC".to_string()));
        assert!(Farm::from(Raydium::from("PSY-USDC")).eq(&Farm::Raydium {
            name: Raydium::PSYUSDC
        }));
        let farm2 = Farm::from([0_u64, 4_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(5_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::MERUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-MER-USDC".to_string()));
        assert!(Farm::from(Raydium::from("MER-USDC")).eq(&Farm::Raydium {
            name: Raydium::MERUSDC
        }));
        let farm2 = Farm::from([0_u64, 5_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(6_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::stSOLUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-stSOL-USDC".to_string()));
        assert!(Farm::from(Raydium::from("stSOL-USDC")).eq(&Farm::Raydium {
            name: Raydium::stSOLUSDC
        }));
        let farm2 = Farm::from([0_u64, 6_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(7_u64));
        assert!(farm.eq(&Farm::Raydium { name: Raydium::RAY }));
        assert!(farm.name().eq(&"RAYDIUM-RAY".to_string()));
        assert!(Farm::from(Raydium::from("RAY")).eq(&Farm::Raydium { name: Raydium::RAY }));
        let farm2 = Farm::from([0_u64, 7_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(8_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::RAYUSDT
        }));
        assert!(farm.name().eq(&"RAYDIUM-RAY-USDT".to_string()));
        assert!(Farm::from(Raydium::from("RAY-USDT")).eq(&Farm::Raydium {
            name: Raydium::RAYUSDT
        }));
        let farm2 = Farm::from([0_u64, 8_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(9_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::RAYUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-RAY-USDC".to_string()));
        assert!(Farm::from(Raydium::from("RAY-USDC")).eq(&Farm::Raydium {
            name: Raydium::RAYUSDC
        }));
        let farm2 = Farm::from([0_u64, 9_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(10_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::RAYSRM
        }));
        assert!(farm.name().eq(&"RAYDIUM-RAY-SRM".to_string()));
        assert!(Farm::from(Raydium::from("RAY-SRM")).eq(&Farm::Raydium {
            name: Raydium::RAYSRM
        }));
        let farm2 = Farm::from([0_u64, 10_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(11_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::RAYSOL
        }));
        assert!(farm.name().eq(&"RAYDIUM-RAY-SOL".to_string()));
        assert!(Farm::from(Raydium::from("RAY-SOL")).eq(&Farm::Raydium {
            name: Raydium::RAYSOL
        }));
        let farm2 = Farm::from([0_u64, 11_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(12_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::RAYETH
        }));
        assert!(farm.name().eq(&"RAYDIUM-RAY-ETH".to_string()));
        assert!(Farm::from(Raydium::from("RAY-ETH")).eq(&Farm::Raydium {
            name: Raydium::RAYETH
        }));
        let farm2 = Farm::from([0_u64, 12_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(13_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::ROPEUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-ROPE-USDC".to_string()));
        assert!(Farm::from(Raydium::from("ROPE-USDC")).eq(&Farm::Raydium {
            name: Raydium::ROPEUSDC
        }));
        let farm2 = Farm::from([0_u64, 13_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(14_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::SAMORAY
        }));
        assert!(farm.name().eq(&"RAYDIUM-SAMO-RAY".to_string()));
        assert!(Farm::from(Raydium::from("SAMO-RAY")).eq(&Farm::Raydium {
            name: Raydium::SAMORAY
        }));
        let farm2 = Farm::from([0_u64, 14_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(15_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::SNYUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-SNY-USDC".to_string()));
        assert!(Farm::from(Raydium::from("SNY-USDC")).eq(&Farm::Raydium {
            name: Raydium::SNYUSDC
        }));
        let farm2 = Farm::from([0_u64, 15_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(16_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::stSOLUSDT
        }));
        assert!(farm.name().eq(&"RAYDIUM-stSOL-USDT".to_string()));
        assert!(Farm::from(Raydium::from("stSOL-USDT")).eq(&Farm::Raydium {
            name: Raydium::stSOLUSDT
        }));
        let farm2 = Farm::from([0_u64, 16_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(17_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::TULIPUSDC
        }));
        assert!(farm.name().eq(&"RAYDIUM-TULIP-USDC".to_string()));
        assert!(Farm::from(Raydium::from("TULIP-USDC")).eq(&Farm::Raydium {
            name: Raydium::TULIPUSDC
        }));
        let farm2 = Farm::from([0_u64, 17_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(18_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::POLISRAY
        }));
        assert!(farm.name().eq(&"RAYDIUM-POLIS-RAY".to_string()));
        assert!(Farm::from(Raydium::from("POLIS-RAY")).eq(&Farm::Raydium {
            name: Raydium::POLISRAY
        }));
        let farm2 = Farm::from([0_u64, 18_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(19_u64));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::ATLASRAY
        }));
        assert!(farm.name().eq(&"RAYDIUM-ATLAS-RAY".to_string()));
        assert!(Farm::from(Raydium::from("ATLAS-RAY")).eq(&Farm::Raydium {
            name: Raydium::ATLASRAY
        }));
        let farm2 = Farm::from([0_u64, 19_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(u64::MAX));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::UNKNOWN
        }));
        assert!(farm.name().eq(&"RAYDIUM-UNKNOWN".to_string()));
        assert!(Farm::from(Raydium::from("abababab")).eq(&Farm::Raydium {
            name: Raydium::UNKNOWN
        }));
        let farm2 = Farm::from([0_u64, 128_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Raydium::from(u64::MAX - 1));
        assert!(farm.eq(&Farm::Raydium {
            name: Raydium::UNKNOWN
        }));
        assert!(farm.name().eq(&"RAYDIUM-UNKNOWN".to_string()));
        assert!(Farm::from(Raydium::from("bcbcbcb")).eq(&Farm::Raydium {
            name: Raydium::UNKNOWN
        }));
        let farm2 = Farm::from([0_u64, 255_u64]);
        assert!(farm.eq(&farm2));
    }
    #[test]
    fn test_farm_from() {
        assert!(
            Farm::from([0_u64, 0_u64])
                == Farm::Raydium {
                    name: Raydium::ALEPHUSDC
                }
        );
        assert!(Farm::from([0_u64, 0_u64]).name() == *"RAYDIUM-ALEPH-USDC");

        assert!(
            Farm::from([0_u64, 1_u64])
                == Farm::Raydium {
                    name: Raydium::BOPRAY
                }
        );
        assert!(Farm::from([0_u64, 1_u64]).name() == *"RAYDIUM-BOP-RAY");

        assert!(
            Farm::from([0_u64, 17_u64])
                == Farm::Raydium {
                    name: Raydium::TULIPUSDC
                }
        );
        assert!(Farm::from([0_u64, 17_u64]).name() == *"RAYDIUM-TULIP-USDC");

        assert!(Farm::from([1_u64, 0_u64]).name() == *"LENDING-TULIP");
        assert!(Farm::from([1_u64, 1_u64]).name() == *"LENDING-SOLEND");

        assert!(
            Farm::from([100_u64, 0_u64])
                == Farm::Unknown {
                    name: Unknown::Uknown
                }
        );
        assert!(Farm::from([100_u64, 0_u64]).name() == *"UNKNOWN");
    }
    #[test]
    fn test_farm_lending() {
        let farm = Farm::from(Lending::from(0_u64));
        assert!(farm.eq(&Farm::Lending {
            name: Lending::TULIP,
        }));
        assert!(farm.name().eq(&"LENDING-TULIP".to_string()));

        let farm = Farm::from(Lending::from(1_u64));
        assert!(farm.eq(&Farm::Lending {
            name: Lending::SOLEND,
        }));
        assert!(farm.name().eq(&"LENDING-SOLEND".to_string()));

        let farm = Farm::from(Lending::from(2_u64));
        assert!(farm.eq(&Farm::Lending {
            name: Lending::MANGO,
        }));
        assert!(farm.name().eq(&"LENDING-MANGO".to_string()));

        let farm = Farm::from(Lending::from(3_u64));
        assert!(farm.eq(&Farm::Lending {
            name: Lending::PORT,
        }));
        assert!(farm.name().eq(&"LENDING-PORT".to_string()));

        let farm = Farm::from(Lending::from(4_u64));
        assert!(farm.eq(&Farm::Lending {
            name: Lending::LARIX,
        }));
        assert!(farm.name().eq(&"LENDING-LARIX".to_string()));

        let farm = Farm::from(Lending::from(5_u64));
        assert!(farm.eq(&Farm::Lending {
            name: Lending::PARROT,
        }));
        assert!(farm.name().eq(&"LENDING-PARROT".to_string()));
    }
    #[test]
    fn test_farm_orca() {
        let farm = Farm::from(Orca::from(0_u64));
        assert!(farm.eq(&Farm::Orca {
            name: Orca::ATLASUSDC
        }));
        assert!(farm.name().eq(&"ORCA-ATLAS-USDC".to_string()));
        assert!(Farm::from(Orca::from("ATLAS-USDC")).eq(&Farm::Orca {
            name: Orca::ATLASUSDC
        }));
        let farm2 = Farm::from([2_u64, 0_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Orca::from(1_u64));
        assert!(farm.eq(&Farm::Orca {
            name: Orca::POLISUSDC
        }));
        assert!(farm.name().eq(&"ORCA-POLIS-USDC".to_string()));
        assert!(Farm::from(Orca::from("POLIS-USDC")).eq(&Farm::Orca {
            name: Orca::POLISUSDC
        }));
        let farm2 = Farm::from([2_u64, 1_u64]);
        assert!(farm.eq(&farm2));

        let farm = Farm::from(Orca::from(6969420_u64));
        assert!(farm.eq(&Farm::Orca {
            name: Orca::UNKNOWN
        }));
        assert!(farm.name().eq(&"ORCA-UNKNOWN".to_string()));
        assert!(Farm::from(Orca::from("UNKNOWN")).eq(&Farm::Orca {
            name: Orca::UNKNOWN
        }));
        let farm2 = Farm::from([2_u64, 6969420_u64]);
        assert!(farm.eq(&farm2));
    }
    #[test]
    fn test_lending_market_name() {
        let farm = Farm::Lending {
            name: Lending::USDC,
        };
        println!("{}", farm.market_name().unwrap())
    }
    #[test]
    fn test_farm_quarry() {
        let farm = Farm::from(Quarry::from(0_u64));
        assert!(farm.eq(&Farm::Quarry {
            name: Quarry::VANILLA
        }));
        assert_eq!(farm.name(), "QUARRY-VANILLA".to_string());
        assert_eq!(
            Farm::from(Quarry::from("VANILLA")),
            Farm::Quarry {
                name: Quarry::VANILLA
            }
        );
        let farm2: Farm = [3_u64, 0_u64].into();
        assert_eq!(farm, farm2);
        let farm3: [u64; 2] = farm2.into();
        let farm4: [u64; 2] = farm.into();
        assert_eq!(farm3, farm4);
    }
}
