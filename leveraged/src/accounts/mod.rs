pub mod aqua_farms;
pub mod derivations;
pub mod leveraged_farm;
pub mod position_info;
pub mod user_farm;

use anchor_lang::prelude::*;

pub const POSITION_INFO_ACCOUNT_SIZE: usize = 300;
pub const USER_FARM_ACCOUNT_SIZE: usize = 247;
pub const LEVERAGED_FARM_ACCOUNT_SIZE: usize = 1616;

/// Position is the current state of the obligation
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum Position {
    /// Opening is before lp tokens have been obtained and added to vault
    /// In this step collateral is deposited, liq is borrowed
    /// Tokens are swapped to appropriate amounts
    /// Finally liquidity is added and then deposited into vault
    Opening = 0,
    /// Swapped is an intermediate step in opening of a new position
    /// This is after tokens have been swapped to appropriate amounts
    /// After this step only valid path is to obtain lp tokens and deposit into vaults
    Swapped, // 1
    /// AddedLiquidity liquidity provided and lp tokens obtained
    /// This is the final step before opening a position
    AddedLiquidity, // 2
    /// Opened is after lp tokens have been deposited into a vault
    Opened, // 3
    /// Position is being closed by the user
    Withdrawing, // 4
    /// Removed liquidity from lp
    RemovedLiquidity, // 5
    /// Swapped tokens to repay debt
    SwappedForRepaying, // 6
    /// Debt repaid
    Repaid, // 7
    /// Collateral withdrawn
    Withdrawn, // 8
    /// indicates this obligation account is being closed
    /// and is undergoing liquidation
    Closing, // 9
    ClosingAndExiting, // 10
    /// This is after an existing obligation account has been liquidated or closed manually
    /// basically it has no borrows / deposits
    Closed,
    ExitingAndLiquidated,
    Liquidated,
    /// indicates we are "topping up" an opened position, adding extra funds into the position itself
    /// without allowing the position opener to borrow liquidity
    TopUp,
    /// Borrowed some liquidity from reserve
    Borrowed,
    /// Extra added collateral swapped
    TopUpSwapped,
    /// Extra liquidity added
    TopUpAddedLiquidity,
    /// Deposited lp tokens into aqua farm
    DepositedOrcaAquaFarm,
    WithdrawnOrcaDoubleDip,

    /// Swapped For Liquidation
    /// This is needed to make partial liquidation safe
    SwappedForLiquidation, // 20

    /// RepaidForLiquidation
    RepaidForLiquidation,

    /// DDPulledForLiquidation
    DDPulledForLiquidation,
    /// LPPulledForLiquidation
    LPPulledForLiquidation,
    /// RemovedLiquidityForLiquidation
    RemovedLiquidityForLiquidation,
}

impl From<u8> for Position {
    fn from(i: u8) -> Position {
        match i {
            0 => Position::Opening,
            1 => Position::Swapped,
            2 => Position::AddedLiquidity,
            3 => Position::Opened,
            4 => Position::Withdrawing,
            5 => Position::RemovedLiquidity,
            6 => Position::SwappedForRepaying,
            7 => Position::Repaid,
            8 => Position::Withdrawn,
            9 => Position::Closing,
            10 => Position::ClosingAndExiting,
            11 => Position::Closed,
            12 => Position::ExitingAndLiquidated,
            13 => Position::Liquidated,
            14 => Position::TopUp,
            15 => Position::Borrowed,
            16 => Position::TopUpSwapped,
            17 => Position::TopUpAddedLiquidity,
            18 => Position::DepositedOrcaAquaFarm,
            19 => Position::WithdrawnOrcaDoubleDip,
            20 => Position::SwappedForLiquidation,
            21 => Position::RepaidForLiquidation,
            22 => Position::DDPulledForLiquidation,
            23 => Position::LPPulledForLiquidation,
            24 => Position::RemovedLiquidityForLiquidation,
            _ => panic!("invalid index"),
        }
    }
}

impl From<u64> for Position {
    fn from(i: u64) -> Position {
        match i {
            0 => Position::Opening,
            1 => Position::Swapped,
            2 => Position::AddedLiquidity,
            3 => Position::Opened,
            4 => Position::Withdrawing,
            5 => Position::RemovedLiquidity,
            6 => Position::SwappedForRepaying,
            7 => Position::Repaid,
            8 => Position::Withdrawn,
            9 => Position::Closing,
            10 => Position::ClosingAndExiting,
            11 => Position::Closed,
            12 => Position::ExitingAndLiquidated,
            13 => Position::Liquidated,
            14 => Position::TopUp,
            15 => Position::Borrowed,
            16 => Position::TopUpSwapped,
            17 => Position::TopUpAddedLiquidity,
            18 => Position::DepositedOrcaAquaFarm,
            19 => Position::WithdrawnOrcaDoubleDip,
            20 => Position::SwappedForLiquidation,
            21 => Position::RepaidForLiquidation,
            22 => Position::DDPulledForLiquidation,
            23 => Position::LPPulledForLiquidation,
            24 => Position::RemovedLiquidityForLiquidation,
            _ => panic!("invalid index"),
        }
    }
}

/// denotes available farms which can be used
/// additionally these serve as "seed words"
/// for generating pdas
#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub enum Farms {
    RayUsdcVault = 0,
    RaySolVault = 1,
    RayUsdtVault = 2,
    RaySrmVault = 3,
    MerUsdcVault = 4,
    MediaUsdcVault = 5,
    CopeUsdcVault = 6,
    RayEthVault = 7,
    StepUsdcVault = 8,
    RopeUsdcVault = 9,
    AlephUsdcVault = 10,
    TulipUsdcVault = 11,
    SnyUsdcVault = 12,
    BopRayVault = 13,
    SlrsUsdcVault = 14,
    SamoRayVault = 15,
    LikeUsdcVault = 16,
    OrcaUsdcVault = 17,
    OrcaSolVault = 18,
    AtlasUsdcVault = 19,
    AtlasRayVault = 20,
    PolisUsdcVault = 21,
    PolisRayVault = 22,
    EthUsdcOrcaVault = 23,
    SolUsdcOrcaVault = 24,
    SolUsdtOrcaVault = 25,
    EthSolOrcaVault = 26,
    AtlasUsdcOrcaVault = 27,
    PolisUsdcOrcaVault = 28,
    whEthUsdcOrcaVault = 29,
    whEthSolOrcaVault = 30,
    mSolUsdcRayVault = 31,
    mSolUsdtRayVault = 32,
    EthMSolRayVault = 33,
    BtcMSolRayVault = 34,
    mSolRayRayVault = 35,
    SamoRayRayVault = 36,
    SamoUsdcOrcaVault = 37,
    SrmUsdcRayVault = 38,
    whEthUsdcRayVault = 39,
    whEthSolRayVault = 40,
    weSushiUsdcRayVault = 41,
    weUniUsdcRayVault = 42,
    StarsUsdcRayVault = 43,
    weDydxUsdcRayVault = 44,
    GeneUsdcRayVault = 45,
    GeneRayRayVault = 46,
    DflUsdcRayVault = 47,
    CaveUsdcRayVault = 48,
    wbWBNBUsdcRayVault = 49,
    SolUsdcRayVault = 50,
    SolUsdtRayVault = 51,
    RealUsdcRayVault = 52,
    PrsmUsdcRayVault = 53,
    MbsUsdcRayVault = 54,
    ShdwUsdcOrcaVault = 55,
    ShdwSolOrcaVault = 56,
    BasisUsdcOrcVault = 57,
    stSolUsdcOrcaVault = 58,
    stSolUsdcRayVault = 59,
    stSolUsdtRayVault = 60,
    BtcstSolRayVault = 61,
    EthstSolRayVault = 62,
    stSolwUstOrcaVault = 63,
    GstUsdcOrcaVault = 64,
    ZbcUsdcRayVault = 65,
    wAlephUsdcRayVault = 66,
    SlclUsdcRayVault = 67,
    PrismUsdcRayVault = 68,
    SlcUsdcRayVault = 69,
    HbbUsdhRayVault = 70,
    UsdhUsdcRayVault = 71,
    sRlySolOrcaVault = 72,
    HawkUsdcRayVault = 73,
    GmtUsdcOrcaVault = 74,
    RaywhEthRayVault = 75,
    Unknown = 255,
}

impl From<u64> for Farms {
    fn from(num: u64) -> Farms {
        match num {
            0 => Farms::RayUsdcVault,
            1 => Farms::RaySolVault,
            2 => Farms::RayUsdtVault,
            3 => Farms::RaySrmVault,
            4 => Farms::MerUsdcVault,
            5 => Farms::MediaUsdcVault,
            6 => Farms::CopeUsdcVault,
            7 => Farms::RayEthVault,
            8 => Farms::StepUsdcVault,
            9 => Farms::RopeUsdcVault,
            10 => Farms::AlephUsdcVault,
            11 => Farms::TulipUsdcVault,
            12 => Farms::SnyUsdcVault,
            13 => Farms::BopRayVault,
            14 => Farms::SlrsUsdcVault,
            15 => Farms::SamoRayVault,
            16 => Farms::LikeUsdcVault,
            17 => Farms::OrcaUsdcVault,
            18 => Farms::OrcaSolVault,
            19 => Farms::AtlasUsdcVault,
            20 => Farms::AtlasRayVault,
            21 => Farms::PolisUsdcVault,
            22 => Farms::PolisRayVault,
            23 => Farms::EthUsdcOrcaVault,
            24 => Farms::SolUsdcOrcaVault,
            25 => Farms::SolUsdtOrcaVault,
            26 => Farms::EthSolOrcaVault,
            27 => Farms::AtlasUsdcOrcaVault,
            28 => Farms::PolisUsdcOrcaVault,
            29 => Farms::whEthUsdcOrcaVault,
            30 => Farms::whEthSolOrcaVault,
            31 => Farms::mSolUsdcRayVault,
            32 => Farms::mSolUsdtRayVault,
            33 => Farms::EthMSolRayVault,
            34 => Farms::BtcMSolRayVault,
            35 => Farms::mSolRayRayVault,
            36 => Farms::SamoRayRayVault,
            37 => Farms::SamoUsdcOrcaVault,
            38 => Farms::SrmUsdcRayVault,
            39 => Farms::whEthUsdcRayVault,
            40 => Farms::whEthSolRayVault,
            41 => Farms::weSushiUsdcRayVault,
            42 => Farms::weUniUsdcRayVault,
            43 => Farms::StarsUsdcRayVault,
            44 => Farms::weDydxUsdcRayVault,
            45 => Farms::GeneUsdcRayVault,
            46 => Farms::GeneRayRayVault,
            47 => Farms::DflUsdcRayVault,
            48 => Farms::CaveUsdcRayVault,
            49 => Farms::wbWBNBUsdcRayVault,
            50 => Farms::SolUsdcRayVault,
            51 => Farms::SolUsdtRayVault,
            52 => Farms::RealUsdcRayVault,
            53 => Farms::PrsmUsdcRayVault,
            54 => Farms::MbsUsdcRayVault,
            55 => Farms::ShdwUsdcOrcaVault,
            56 => Farms::ShdwSolOrcaVault,
            57 => Farms::BasisUsdcOrcVault,
            58 => Farms::stSolUsdcOrcaVault,
            59 => Farms::stSolUsdcRayVault,
            60 => Farms::stSolUsdtRayVault,
            61 => Farms::BtcstSolRayVault,
            62 => Farms::EthstSolRayVault,
            63 => Farms::stSolwUstOrcaVault,
            64 => Farms::GstUsdcOrcaVault,
            65 => Farms::ZbcUsdcRayVault,
            66 => Farms::wAlephUsdcRayVault,
            67 => Farms::SlclUsdcRayVault,
            68 => Farms::PrismUsdcRayVault,
            69 => Farms::SlcUsdcRayVault,
            70 => Farms::HbbUsdhRayVault,
            71 => Farms::UsdhUsdcRayVault,
            72 => Farms::sRlySolOrcaVault,
            73 => Farms::HawkUsdcRayVault,
            74 => Farms::GmtUsdcOrcaVault,
            75 => Farms::RaywhEthRayVault,
            _ => Farms::Unknown,
        }
    }
}

impl From<&str> for Farms {
    fn from(s: &str) -> Farms {
        match s {
            "RAY-USDC" => Farms::RayUsdcVault,
            "RAY-SOL" => Farms::RaySolVault,
            "RAY-USDT" => Farms::RayUsdtVault,
            "RAY-SRM" => Farms::RaySrmVault,
            "MEDIA-USDC" => Farms::MediaUsdcVault,
            "MER-USDC" => Farms::MerUsdcVault,
            "COPE-USDC" => Farms::CopeUsdcVault,
            "RAY-ETH" => Farms::RayEthVault,
            "STEP-USDC" => Farms::StepUsdcVault,
            "ROPE-USDC" => Farms::RopeUsdcVault,
            "ALEPH-USDC" => Farms::AlephUsdcVault,
            "TULIP-USDC" => Farms::TulipUsdcVault,
            "SNY-USDC" => Farms::SnyUsdcVault,
            "BOP-RAY" => Farms::BopRayVault,
            "SLRS-USDC" => Farms::SlrsUsdcVault,
            "SAMO-RAY" => Farms::SamoRayVault,
            "LIKE-USDC" => Farms::LikeUsdcVault,
            "ORCA-SOL" => Farms::OrcaSolVault,
            "ORCA-USDC" => Farms::OrcaUsdcVault,
            "ATLAS-USDC" => Farms::AtlasUsdcVault,
            "ATLAS-RAY" => Farms::AtlasRayVault,
            "POLIS-USDC" => Farms::PolisUsdcVault,
            "POLIS-RAY" => Farms::PolisRayVault,
            "ETH-USDC-ORCA" => Farms::EthUsdcOrcaVault,
            "SOL-USDC-ORCA" => Farms::SolUsdcOrcaVault,
            "SOL-USDT-ORCA" => Farms::SolUsdtOrcaVault,
            "ETH-SOL-ORCA" => Farms::EthSolOrcaVault,
            "ATLAS-USDC-ORCA" => Farms::AtlasUsdcOrcaVault,
            "POLIS-USDC-ORCA" => Farms::PolisUsdcOrcaVault,
            "whETH-USDC-ORCA" => Farms::whEthUsdcOrcaVault,
            "whETH-SOL-ORCA" => Farms::whEthSolOrcaVault,
            "mSOL-USDC-RAY" => Farms::mSolUsdcRayVault,
            "mSOL-USDT-RAY" => Farms::mSolUsdtRayVault,
            "ETH-mSOL-RAY" => Farms::EthMSolRayVault,
            "BTC-mSOL-RAY" => Farms::BtcMSolRayVault,
            "mSOL-RAY-RAY" => Farms::mSolRayRayVault,
            "SAMO-RAY-RAY" => Farms::SamoRayRayVault,
            "SAMO-USDC-ORCA" => Farms::SamoUsdcOrcaVault,
            "SRM-USDC-RAY" => Farms::SrmUsdcRayVault,
            "whETH-USDC-RAY" => Farms::whEthUsdcRayVault,
            "whETH-SOL-RAY" => Farms::whEthSolRayVault,
            "weUNI-USDC-RAY" => Farms::weUniUsdcRayVault,
            "weSUSHI-USDC-RAY" => Farms::weSushiUsdcRayVault,
            "STARS-USDC-RAY" => Farms::StarsUsdcRayVault,
            "weDYDX-USDC-RAY" => Farms::weDydxUsdcRayVault,
            "GENE-USDC-RAY" => Farms::GeneUsdcRayVault,
            "GENE-RAY-RAY" => Farms::GeneRayRayVault,
            "DFL-USDC-RAY" => Farms::DflUsdcRayVault,
            "wbWBNB-USDC-RAY" => Farms::wbWBNBUsdcRayVault,
            "CAVE-USDC-RAY" => Farms::CaveUsdcRayVault,
            "SOL-USDT-RAY" => Farms::SolUsdtRayVault,
            "SOL-USDC-RAY" => Farms::SolUsdcRayVault,
            "REAL-USDC-RAY" => Farms::RealUsdcRayVault,
            "MBS-USDC-RAY" => Farms::MbsUsdcRayVault,
            "PRSM-USDC-RAY" => Farms::PrsmUsdcRayVault,
            "SHDW-SOL-ORCA" => Farms::ShdwSolOrcaVault,
            "SHDW-USDC-ORCA" => Farms::ShdwUsdcOrcaVault,
            "BASIS-USDC-ORCA" => Farms::BasisUsdcOrcVault,
            "stSOL-USDC-ORCA" => Farms::stSolUsdcOrcaVault,
            "GST-USDC-ORCA" => Farms::GstUsdcOrcaVault,
            "stSOL-USDC-RAY" => Farms::stSolUsdcRayVault,
            "stSOL-USDT-RAY" => Farms::stSolUsdtRayVault,
            "ETH-stSOL-RAY" => Farms::EthstSolRayVault,
            "BTC-stSOL-RAY" => Farms::BtcstSolRayVault,
            "stSOL-wUST-ORCA" => Farms::stSolwUstOrcaVault,
            "ZBC-USDC-RAY" => Farms::ZbcUsdcRayVault,
            "wALEPH-USDC-RAY" => Farms::wAlephUsdcRayVault,
            "SLCL-USDC-RAY" => Farms::SlclUsdcRayVault,
            "PRISM-USDC-RAY" => Farms::PrismUsdcRayVault,
            "SLC-USDC-RAY" => Farms::SlcUsdcRayVault,
            "HBB-USDH-RAY" => Farms::HbbUsdhRayVault,
            "USDH-USDC-RAY" => Farms::UsdhUsdcRayVault,
            "sRLY-SOL-ORCA" => Farms::sRlySolOrcaVault,
            "HAWK-USDC-RAY" => Farms::HawkUsdcRayVault,
            "GMT-USDC-ORCA" => Farms::GmtUsdcOrcaVault,
            "RAY-whETH-RAY" => Farms::RaywhEthRayVault,
            _ => Farms::Unknown,
        }
    }
}

impl Farms {
    pub fn is_aquafarm(&self) -> bool {
        match self {
            Farms::OrcaUsdcVault
            | Farms::OrcaSolVault
            | Farms::EthUsdcOrcaVault
            | Farms::SolUsdcOrcaVault
            | Farms::SolUsdtOrcaVault
            | Farms::EthSolOrcaVault
            | Farms::AtlasUsdcOrcaVault
            | Farms::PolisUsdcOrcaVault
            | Farms::whEthSolOrcaVault
            | Farms::whEthUsdcOrcaVault
            | Farms::SamoUsdcOrcaVault
            | Farms::ShdwUsdcOrcaVault
            | Farms::ShdwSolOrcaVault
            | Farms::BasisUsdcOrcVault
            | Farms::stSolUsdcOrcaVault
            | Farms::GstUsdcOrcaVault
            | Farms::stSolwUstOrcaVault
            | Farms::sRlySolOrcaVault
            | Farms::GmtUsdcOrcaVault => true,
            _ => false,
        }
    }
    pub fn str(&self) -> &str {
        match self {
            Farms::RayUsdcVault => "RAY-USDC",
            Farms::RaySolVault => "RAY-SOL",
            Farms::RayUsdtVault => "RAY-USDT",
            Farms::RaySrmVault => "RAY-SRM",
            Farms::MerUsdcVault => "MER-USDC",
            Farms::MediaUsdcVault => "MEDIA-USDC",
            Farms::CopeUsdcVault => "COPE-USDC",
            Farms::RayEthVault => "RAY-ETH",
            Farms::StepUsdcVault => "STEP-USDC",
            Farms::RopeUsdcVault => "ROPE-USDC",
            Farms::AlephUsdcVault => "ALEPH-USDC",
            Farms::TulipUsdcVault => "TULIP-USDC",
            Farms::SnyUsdcVault => "SNY-USDC",
            Farms::BopRayVault => "BOP-RAY",
            Farms::SlrsUsdcVault => "SLRS-USDC",
            Farms::SamoRayVault => "SAMO-RAY",
            Farms::LikeUsdcVault => "LIKE-USDC",
            Farms::OrcaSolVault => "ORCA-SOL",
            Farms::OrcaUsdcVault => "ORCA-USDC",
            Farms::AtlasUsdcVault => "ATLAS-USDC",
            Farms::AtlasRayVault => "ATLAS-RAY",
            Farms::PolisUsdcVault => "POLIS-USDC",
            Farms::PolisRayVault => "POLIS-RAY",
            Farms::EthUsdcOrcaVault => "ETH-USDC-ORCA",
            Farms::SolUsdcOrcaVault => "SOL-USDC-ORCA",
            Farms::SolUsdtOrcaVault => "SOL-USDT-ORCA",
            Farms::EthSolOrcaVault => "ETH-SOL-ORCA",
            Farms::AtlasUsdcOrcaVault => "ATLAS-USDC-ORCA",
            Farms::PolisUsdcOrcaVault => "POLIS-USDC-ORCA",
            Farms::whEthUsdcOrcaVault => "whETH-USDC-ORCA",
            Farms::whEthSolOrcaVault => "whETH-SOL-ORCA",
            Farms::mSolUsdcRayVault => "mSOL-USDC-RAY",
            Farms::mSolUsdtRayVault => "mSOL-USDT-RAY",
            Farms::EthMSolRayVault => "ETH-mSOL-RAY",
            Farms::BtcMSolRayVault => "BTC-mSOL-RAY",
            Farms::mSolRayRayVault => "mSOL-RAY-RAY",
            Farms::SamoRayRayVault => "SAMO-RAY-RAY",
            Farms::SamoUsdcOrcaVault => "SAMO-USDC-ORCA",
            Farms::SrmUsdcRayVault => "SRM-USDC-RAY",
            Farms::whEthUsdcRayVault => "whETH-USDC-RAY",
            Farms::whEthSolRayVault => "whETH-SOL-RAY",
            Farms::weUniUsdcRayVault => "weUNI-USDC-RAY",
            Farms::weSushiUsdcRayVault => "weSUSHI-USDC-RAY",
            Farms::StarsUsdcRayVault => "STARS-USDC-RAY",
            Farms::weDydxUsdcRayVault => "weDYDX-USDC-RAY",
            Farms::GeneUsdcRayVault => "GENE-USDC-RAY",
            Farms::GeneRayRayVault => "GENE-RAY-RAY",
            Farms::DflUsdcRayVault => "DFL-USDC-RAY",
            Farms::CaveUsdcRayVault => "CAVE-USDC-RAY",
            Farms::wbWBNBUsdcRayVault => "wbWBNB-USDC-RAY",
            Farms::SolUsdcRayVault => "SOL-USDC-RAY",
            Farms::SolUsdtRayVault => "SOL-USDT-RAY",
            Farms::RealUsdcRayVault => "REAL-USDC-RAY",
            Farms::MbsUsdcRayVault => "MBS-USDC-RAY",
            Farms::PrsmUsdcRayVault => "PRSM-USDC-RAY",
            Farms::ShdwUsdcOrcaVault => "SHDW-USDC-ORCA",
            Farms::ShdwSolOrcaVault => "SHDW-SOL-ORCA",
            Farms::BasisUsdcOrcVault => "BASIS-USDC-ORCA",
            Farms::stSolUsdcOrcaVault => "stSOL-USDC-ORCA",
            Farms::GstUsdcOrcaVault => "GST-USDC-ORCA",
            Farms::stSolUsdcRayVault => "stSOL-USDC-RAY",
            Farms::stSolUsdtRayVault => "stSOL-USDT-RAY",
            Farms::BtcstSolRayVault => "BTC-stSOL-RAY",
            Farms::EthstSolRayVault => "ETH-stSOL-RAY",
            Farms::stSolwUstOrcaVault => "stSOL-wUST-ORCA",
            Farms::ZbcUsdcRayVault => "ZBC-USDC-RAY",
            Farms::wAlephUsdcRayVault => "wALEPH-USDC-RAY",
            Farms::SlclUsdcRayVault => "SLCL-USDC-RAY",
            Farms::PrismUsdcRayVault => "PRISM-USDC-RAY",
            Farms::SlcUsdcRayVault => "SLC-USDC-RAY",
            Farms::HbbUsdhRayVault => "HBB-USDH-RAY",
            Farms::UsdhUsdcRayVault => "USDH-USDC-RAY",
            Farms::sRlySolOrcaVault => "sRLY-SOL-ORCA",
            Farms::HawkUsdcRayVault => "HAWK-USDC-RAY",
            Farms::GmtUsdcOrcaVault => "GMT-USDC-ORCA",
            Farms::RaywhEthRayVault => "RAY-whETH-RAY",
            Farms::Unknown => "Unknown",
        }
    }
}
