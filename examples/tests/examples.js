"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const anchor = __importStar(require("@project-serum/anchor"));
const serumAssoToken = __importStar(require("@project-serum/associated-token"));
const splToken = __importStar(require("@solana/spl-token"));
const utils_1 = require("./utils");
const spl_token_1 = require("@solana/spl-token");
const usdcv1Vault = new anchor.web3.PublicKey("3wPiV9inTGexMZjp6x5Amqwp2sRNtpSheG8Hbv2rgq8W");
const usdcv1VaultPda = new anchor.web3.PublicKey("14fdy6YXbhDgnVQz4VcgSGgUcZ35eE48SKDrfqF87NUP");
const usdcv1SharesMint = new anchor.web3.PublicKey("Cvvh8nsKZet59nsDDo3orMa3rZnPWQhpgrMCVcRDRgip");
const usdcv1DepositQueue = new anchor.web3.PublicKey("36KtHLHxcGnrfEb2GLwPcbN9nHUkeoi3gd6rMQj8wwVj");
const usdcv1MangoVaultSharesAccount = new anchor.web3.PublicKey("A9kM8NKf3v29F3DgRQ5Rw7TJoadFZZDfBGLRBGNzASrr");
const usdcv1TulipVaultSharesAccount = new anchor.web3.PublicKey("M7akLS7xYVhp68LnMkBBCemvqkGcCycQ3qp7e3SsKR1");
const usdcv1SolendVaultSharesAccount = new anchor.web3.PublicKey("UxVNZzzx3xRxKFAuGjRQgbDaa7mirSkFEAu7qiYQ1h4");
const usdcv1WithdrawQueue = new anchor.web3.PublicKey("HLVcpKPkBJJJGTHTSaZcAixDppy4R65x1is3k8Q7qZpj");
const usdcTokenMint = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const associatedTokenProgramId = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const v2VaultsProgramId = new anchor.web3.PublicKey("TLPv2tuSVvn3fSk8RgW3yPddkp5oFivzZV3rA9hQxtX");
const mangoVault = new anchor.web3.PublicKey("ZH9GWNBtwxcWeU9kHk77DSciwQnoJcSm8VVvYfmHXfe");
const mangoVaultPda = new anchor.web3.PublicKey("Dhry4TVd862yzcAuxFZgiuh6juS4R6FesfRZkWG3pCe7");
const mangoVaultSharesMint = new anchor.web3.PublicKey("5u6jxB7En2LF5aroeq8U5JUbnHa5WSYB5JTemh3gYaMj");
const mangoVaultPlatformInformation = new anchor.web3.PublicKey("GDzbqzebKTxJaQVfxfqejkqU4HLcBmgNBwun3rADvm8J");
const mangoVaultPlatformConfigDataAccount = new anchor.web3.PublicKey("EecDX1xHrjKXQWbE5WtwU7fAiEkKxAv7w196oe8jaoqY");
const mangoVaultDepositQueue = new anchor.web3.PublicKey("3CnAyCjpA9mcxayed12cwJNGue7YbmyuBjpT9KN6meVT");
const mangoVaultMangoAccount = new anchor.web3.PublicKey("3cZkd5eVyZhMhE8nJcR3rA7GgVQ6gCJt2qofr2GQd8ca");
const mangoCache = new anchor.web3.PublicKey("EBDRoayCDDUvDgCimta45ajQeXbexv7aKqJubruqpyvu");
const mangoRootBank = new anchor.web3.PublicKey("AMzanZxMirPCgGcBoH9kw4Jzi9LFMomyUCXbpzDeL2T8");
const mangoNodeBank = new anchor.web3.PublicKey("BGcwkj1WudQwUUjFk78hAjwd1uAm8trh1N4CJSa51euh");
const mangoGroupAccount = new anchor.web3.PublicKey("98pjRuQjK3qA6gXts96PqZT4Ze5QmnCmt3QYjhbUSPue");
const mangoTokenAccount = new anchor.web3.PublicKey("8Vw25ZackDzaJzzBBqcgcpDsCsDfRSkMGgwFQ3gbReWF");
const mangoGroupSigner = new anchor.web3.PublicKey("9BVcYqEQxyccuwznvxXqDkSJFavvTyheiTYk231T1A8S");
const mangoProgramId = new anchor.web3.PublicKey("mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68");
const solendVault = new anchor.web3.PublicKey("85JXjDiyianDpvz8y8efkRyFsxpnSJJpmyxrJ7bncKHM");
const solendVaultPda = new anchor.web3.PublicKey("963HGaUjwGqvqwwqwJZayUXvCC21AAqZU5SLw1kU4gVc");
const solendVaultSharesMint = new anchor.web3.PublicKey("CS8cJicaSpphTTboMJD1UeNpU7vpkZc86vKrtqzRVnG5");
const solendVaultPlatformInformation = new anchor.web3.PublicKey("GRL5rtnvzCfQRdKJXkG2A8LvDSNXkbxENEnF4SwJ3pTF");
const solendVaultPlatformConfigDataAccount = new anchor.web3.PublicKey("6AzcPNmNWomkdMRgcZJPVAs4zF1jev9wqxzzzxVzDDsi");
const solendVaultDepositQueue = new anchor.web3.PublicKey("2Li9Q5Vx9BEnFTGJTLWc5pVqerYGjhgyGYzSAA2WhYKQ");
const solendReserveAccount = new anchor.web3.PublicKey("BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw");
const solendReserveLiquiditySupply = new anchor.web3.PublicKey("8SheGtsopRUDzdiD6v6BR9a6bqZ9QwywYQY99Fp5meNf");
const solendReserveCollateralMint = new anchor.web3.PublicKey("993dVFL2uXWYeoXuEBFXR4BijeXdTv4s6BzsCjJZuwqk");
const solendLendingMarketAccount = new anchor.web3.PublicKey("4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY");
const solendDerivedLendingMarketAuthority = new anchor.web3.PublicKey("DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby");
const solendReservePythPriceAccount = new anchor.web3.PublicKey("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");
const solendReserveSwitchboardPriceAccount = new anchor.web3.PublicKey("CZx29wKMUxaJDq6aLVQTdViPL754tTR64NAgQBUGxxHb");
const solendProgramId = new anchor.web3.PublicKey("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");
const tulipVault = new anchor.web3.PublicKey("8KLrrsnUv3DjC9Q89xSQDVdiGLZHUEUuyPedfHrtuVRr");
const tulipVaultPda = new anchor.web3.PublicKey("mrT9Q45iuC2HLYxpceaQFjzcAgd6Trks7bXAGbKYpwL");
const tulipVaultSharesMint = new anchor.web3.PublicKey("D2PLcwGR1wsXUPhb1BHysSVEsHVVCQ129qGpgXXaTNR1");
const tulipVaultPlatformInformation = new anchor.web3.PublicKey("4QVuedVSCMLA3eQ643czUy95uQFxXKAcCMJ1ChpkVA2B");
const tulipVaultPlatformConfigDataAccount = new anchor.web3.PublicKey("7XTtoiHfkYzjLDxKDMoaVYncmdBW1yLfphmisSbBrnuZ");
const tulipVaultDepositQueue = new anchor.web3.PublicKey("8eDHmS15CWd8d88uckg6oKxrfwijZVudZsDgdtgGqS49");
const tulipReserveAccount = new anchor.web3.PublicKey("FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt");
const tulipReserveLiquiditySupply = new anchor.web3.PublicKey("64QJd6MYXUjCBvCaZKaqxiKmaMkPUdNonE1KuY1YoGGb");
const tulipReserveCollateralMint = new anchor.web3.PublicKey("Amig8TisuLpzun8XyGfC5HJHHGUQEscjLgoTWsCCKihg");
const tulipLendingMarketAccount = new anchor.web3.PublicKey("D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym");
const tulipDerivedLendingMarketAuthority = new anchor.web3.PublicKey("8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s");
const tulipReservePythPriceAccount = new anchor.web3.PublicKey("ExzpbWgczTgd8J58BrnESndmzBkRVfc6PhFjSGiQXgAB");
const tulipLendingProgramId = new anchor.web3.PublicKey("4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM");
const solendCollateralTokenAccount = new anchor.web3.PublicKey("6EaiG2gRVu9u7QzVmX59AWLSmiaEYvMrKWQfPMCgNxsZ");
const tulipCollateralTokenAccount = new anchor.web3.PublicKey("2U6kk4iTVqeypBydVPKA8mLTLAQEBfWf4KYfmkcvomPE");
const tulipLeveragedFarmProgramId = new anchor.web3.PublicKey("Bt2WPMmbwHPk36i4CRucNDyLcmoGdC7xEdrVuxgJaNE6");
const tulipLevFarmGlobalAccount = new anchor.web3.PublicKey("HLuVf6p3SqgEKy8poYA6g26CDGuQddcbETmf8VdJKqjF");
const tulipRayUsdcLevFarmAccount = new anchor.web3.PublicKey("84ayseJgpJavzfeESgRdyfMoDo2bs4J2YUBjMT4iTs66");
const rayUsdcLpTokenMint = new anchor.web3.PublicKey("FbC6K13MzHvN42bXrtGaWsvZY9fxrackRSZcBGfjPc7m");
const rayTokenMint = new anchor.web3.PublicKey("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");
const rayUsdcLevFarmBaseTokenAccount = new anchor.web3.PublicKey("9k3gGV5WCWug8BNCemv3McAC12p8Tqhxb6GN5hBkmfDE");
const rayUsdcLevFarmQuoteTokenAccount = new anchor.web3.PublicKey("2nHaWRW4PkutKbpDGvVwJ2JkcW1dKA6gMgLv4rPAmqLk");
const borrowAuthorizer = new anchor.web3.PublicKey("Gp1oj71gwapSBjSQoPkWxEyjXxDxrtBVe1ijsVThknXT");
const coinDepositReserveAccount = new anchor.web3.PublicKey("9Bm8d2izGsf9eT6Wr79DTnXBkW2LHYVQa57QzeoTbsAF");
const pcDepositReserveAccount = new anchor.web3.PublicKey("FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt");
const coinReserveLiquidityOracle = new anchor.web3.PublicKey("83fYH17UZaTCpr6GNcPcS5pZkfxwR1CaEVhYKfkqE8YF");
const pcReserveLiquidityOracle = new anchor.web3.PublicKey("ExzpbWgczTgd8J58BrnESndmzBkRVfc6PhFjSGiQXgAB");
const lpPythPriceAccount = new anchor.web3.PublicKey("AV5GeH126btrRE9uq36tZWjdgCuLc1DdzKEatdjmoNex");
const coinSourceReserveLiquidityTokenAccount = new anchor.web3.PublicKey("9SG6E3jBTTHLNgpV6ueUYypMYMkm4K5zyS9tk9Rsjm8Y");
const pcSourceReserveLiquidityTokenAccount = new anchor.web3.PublicKey("64QJd6MYXUjCBvCaZKaqxiKmaMkPUdNonE1KuY1YoGGb");
const coinReserveLiquidityFeeReceiver = new anchor.web3.PublicKey("4bRQL2hLqfinNJTsiQW6odhYtYjKXH7zsPc2tafadgoj");
const pcReserveLiquidityFeeReceiver = new anchor.web3.PublicKey("GPf4tD3q71BzPU79YCadYB2NnLciXAVmYuxfgbKKzUdU");
const v1RayUsdcVaultAccount = new anchor.web3.PublicKey("HvNpbHuQUqGG748ZzgzcH5216wdQdTc283CEyFMc3RdG");
const v1RayUsdcOldVaultInfoAccount = new anchor.web3.PublicKey("8vnMSWpzW2RVdAeMaqXKGbQ3r11ijf6vrCm28Ks1bXRA");
const v1RayUsdcVaultInfoAccount = new anchor.web3.PublicKey("Gf38RxSF3FguiBVYfsB8AmpPyNkGCrNDE7LNvr6U8n7C");
const rayUsdcAmmId = new anchor.web3.PublicKey("6UmmUiYoBjSrhakAobJw8BvkmJtDVxaeBtbt7rxWo1mg");
const v1RaydiumVaultsProgram = new anchor.web3.PublicKey("7vxeyaXGLqcp66fFShqUdHxdacp4k4kwUpRSSeoZLCZ4");
const rayUsdcSerumVaultSigner = new anchor.web3.PublicKey("FmhXe9uG6zun49p222xt3nG1rBAkWvzVz7dxERQ6ouGw");
let raydiumStakeProgramId = new anchor.web3.PublicKey("EhhTKczWMGQt46ynNeRX1WfeagwwJd7ufHvCDjRxjo5Q");
let raydiumStakeProgramIdV5 = new anchor.web3.PublicKey("9KEPoZmtHUrBbhWN1v1KWLMkkvwY6WLtAVUCPRtRjP4z");
const raydiumLiquidityProgram = new anchor.web3.PublicKey("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
const v1RayUsdcVaultPda = new anchor.web3.PublicKey("38dsJ6n4y6ffCDSZXhYYiMXQCgfzqHK5XSytL2fApeGc");
const v1RayUsdcVaultRewardATokenAccount = new anchor.web3.PublicKey("9VQe52wd4GUFfyib2jwahsWsAAgiiJv7gZQ28HTS5GzB");
const v1RayUsdcVaultRewardBTokenAccount = new anchor.web3.PublicKey("9VQe52wd4GUFfyib2jwahsWsAAgiiJv7gZQ28HTS5GzB");
const v1RayUsdcVaultLpTokenAccount = new anchor.web3.PublicKey("E8gJAEcHDB4be9sCKSytLUyBe3V5SEDHgn4192REJhaB");
const v1RayUsdcPoolLpTokenAccount = new anchor.web3.PublicKey("BNnXLFGva3K8ACruAc1gaP49NCbLkyE6xWhGV4G2HLrs");
const v1RayUsdcPoolRewardATokenAccount = new anchor.web3.PublicKey("DpRueBHHhrQNvrjZX7CwGitJDJ8eZc3AHcyFMG4LqCQR");
const v1RayUsdcPoolRewardBTokenAccount = new anchor.web3.PublicKey("DpRueBHHhrQNvrjZX7CwGitJDJ8eZc3AHcyFMG4LqCQR");
const v1RayUsdcPoolAuthority = new anchor.web3.PublicKey("5KQFnDd33J5NaMC9hQ64P5XzaaSz8Pt7NBCkZFYn1po");
const rayUsdcPoolId = new anchor.web3.PublicKey("CHYrUBX2RKX8iBg7gYTkccoGNBzP44LdaazMHCLcdEgS");
const rayUsdcAmmAuthority = new anchor.web3.PublicKey("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1");
const rayUsdcAmmOpenOrders = new anchor.web3.PublicKey("J8u8nTHYtvudyqwLrXZboziN95LpaHFHpd97Jm5vtbkW");
const rayUsdcAmmQuantitiesOrTargetOrders = new anchor.web3.PublicKey("3cji8XW5uhtsA757vELVFAeJpskyHwbnTSceMFY5GjVT");
const rayUsdcAmmPoolCoinTokenAccount = new anchor.web3.PublicKey("FdmKUE4UMiJYFK5ogCngHzShuVKrFXBamPWcewDr31th");
const rayUsdcAmmPoolPcTokenAccount = new anchor.web3.PublicKey("Eqrhxd7bDUCH3MepKmdVkgwazXRzY6iHhEoBpY7yAohk");
const serumProgramId = new anchor.web3.PublicKey("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
const rayUsdcSerumMarket = new anchor.web3.PublicKey("2xiv8A5xrJ7RnGdxXB42uFEkYHJjszEhaJyKKt4WaLep");
const rayUsdcSerumBids = new anchor.web3.PublicKey("Hf84mYadE1VqSvVWAvCWc9wqLXak4RwXiPb4A91EAUn5");
const rayUsdcSerumAsks = new anchor.web3.PublicKey("DC1HsWWRCXVg3wk2NndS5LTbce3axwUwUZH1RgnV4oDN");
const rayUsdcSerumEventQueue = new anchor.web3.PublicKey("H9dZt8kvz1Fe5FyRisb77KcYTaN8LEbuVAfJSnAaEABz");
const rayUsdcSerumCoinVault = new anchor.web3.PublicKey("GGcdamvNDYFhAXr93DWyJ8QmwawUHLCyRqWL3KngtLRa");
const rayUsdcSerumPcVault = new anchor.web3.PublicKey("GGcdamvNDYFhAXr93DWyJ8QmwawUHLCyRqWL3KngtLRa");
const orcaAquaFarmProgram = new anchor.web3.PublicKey("82yxjeMsvaURa4MbZZ7WZZHfobirZYkH1zF8fmeGtyaQ");
const atlasTokenMint = new anchor.web3.PublicKey("ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx");
const orcaUsdcLevFarmAccount = new anchor.web3.PublicKey("5o3EsLS1NTciKHXVsGNYqQQ8iBBK3dBfSPwCH7wsdtRT");
const orcaUsdcLpTokenMint = new anchor.web3.PublicKey("n8Mpu28RjeYD7oUX3LG1tPxzhRZh3YYLRSHcHRdS3Zx");
const orcaTokenMint = new anchor.web3.PublicKey("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE");
const rayUsdcV2Vault = new anchor.web3.PublicKey("6tkFEgE6zry2gGC4yqLrTghdqtqadyT5H3H2AJd4w5rz");
const rayUsdcV2VaultPda = new anchor.web3.PublicKey("G6V4Kohr4PFpuLjhzNKGaTWB5p93dPS4cGe7pQ3MhiK1");
const rayUsdcV2VaultAssociatedStakeInfo = new anchor.web3.PublicKey("HyXpbhK7aubL257mZYDbCzGbcLMjAFWiQp9XnrvrcnE8");
const rayUsdcV2VaultSharesMint = new anchor.web3.PublicKey("9qLZgUPVe7r7YetwCWxBkY1uQAs8UNKuqQbGs3cdHYU8");
const rayUsdcV2VaultCompoundQueue = new anchor.web3.PublicKey("GNgMgCtnYS26XSjpbu8GtS5CDjEfMcHHM3zrH6ieuvLB");
const rayUsdcV2VaultDepositQueue = new anchor.web3.PublicKey("6iQTNqWi9EPs2ST8FqmbE1QZFy6tPLnNrnt7Z5zsrty8");
const rayUsdcV2VaultWithdrawQueue = new anchor.web3.PublicKey("4vvCisFXys52FatAVd9aT8UafCd5zEBpkL5DTnNTVA5u");
const rayUsdcV2VaultRewardATokenAccount = new anchor.web3.PublicKey("7DcoC6MGB6T4U5Tqwaq1qefv6JgrHuPmzeiACxyYjcYn");
const rayUsdcV2VaultStakeInfo = new anchor.web3.PublicKey("8nUUtLjoRg1HZmLSB48keMX5Dc36HL2jJNrJ5xDxtv9g");
let yourUsdcTokenAccount;
let yourOrcaTokenAccount;
let yourRayUsdcLpTokenAccount;
//const nine = new anchor.BN(9).mul(new anchor.BN(10).pow(new anchor.BN(6)));
const one = new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(6)));
describe("prepares test data", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    it("derives accounts", () => __awaiter(void 0, void 0, void 0, function* () {
        yourRayUsdcLpTokenAccount = yield (0, spl_token_1.getAssociatedTokenAddress)(rayUsdcLpTokenMint, provider.wallet.publicKey);
    }));
});
describe("examples", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Examples;
    const programId = program.programId;
    let depositTrackingAccount;
    let depositTrackingPda;
    let depositTrackingQueueAccount;
    let depositTrackingHoldAccount;
    let yourUnderlyingTokenAccount;
    let yourSharesTokenAccount;
    it("registers deposit tracking account", () => __awaiter(void 0, void 0, void 0, function* () {
        console.log("progrmaId ", programId);
        console.log("usdcv1 vault ", usdcv1Vault);
        console.log("provider", provider.wallet.publicKey);
        let [_depositTrackingAccount, _trackingNonce] = yield (0, utils_1.deriveTrackingAddress)(v2VaultsProgramId, usdcv1Vault, provider.wallet.publicKey);
        depositTrackingAccount = _depositTrackingAccount;
        let [_depositTrackingPda, _depositTrackingPdaNonce] = yield (0, utils_1.deriveTrackingPdaAddress)(v2VaultsProgramId, depositTrackingAccount);
        depositTrackingPda = _depositTrackingPda;
        let [_depositTrackingQueueAccount, _queueNonce] = yield (0, utils_1.deriveTrackingQueueAddress)(v2VaultsProgramId, depositTrackingPda);
        depositTrackingQueueAccount = _depositTrackingQueueAccount;
        depositTrackingHoldAccount = yield serumAssoToken.getAssociatedTokenAddress(depositTrackingPda, usdcv1SharesMint);
        console.log("deposit tracking queue", depositTrackingQueueAccount.toString());
        console.log("deposit tracking hold", depositTrackingHoldAccount.toString());
        console.log("deposit tracking pda", depositTrackingPda.toString());
        console.log("deposit tracking", depositTrackingAccount.toString());
        const authority = provider.wallet;
        console.log("sending register deposit tracking account tx");
        let tx = yield program.rpc.registerDepositTrackingAccount([new anchor.BN(1), new anchor.BN(65537)], {
            options: {
                skipPreflight: false,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                vault: usdcv1Vault,
                depositTrackingAccount,
                depositTrackingQueueAccount,
                depositTrackingHoldAccount,
                sharesMint: usdcv1SharesMint,
                underlyingMint: usdcTokenMint,
                depositTrackingPda,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                associatedTokenProgram: associatedTokenProgramId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultProgram: v2VaultsProgramId,
            },
        });
        console.log("sent register deposit tracking account tx ", tx);
    }));
    it("issues shares", () => __awaiter(void 0, void 0, void 0, function* () {
        yourUnderlyingTokenAccount = yield serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, usdcTokenMint);
        yourSharesTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, usdcv1SharesMint);
        let tx = yield program.rpc.issueShares(one.mul(new anchor.BN(4)), [new anchor.BN(1), new anchor.BN(65537)], {
            options: {
                skipPreflight: false,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                vault: usdcv1Vault,
                depositTrackingAccount,
                depositTrackingPda,
                vaultPda: usdcv1VaultPda,
                sharesMint: usdcv1SharesMint,
                receivingSharesAccount: depositTrackingHoldAccount,
                depositingUnderlyingAccount: yourUnderlyingTokenAccount,
                vaultUnderlyingAccount: usdcv1DepositQueue,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultProgram: v2VaultsProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
            },
        });
        console.log("sent issue shares tx");
    }));
    it("withdraws from deposit tracking account", () => __awaiter(void 0, void 0, void 0, function* () {
        console.log("wait 3 seconds");
        freeze(3000);
        console.log("in production this would fail, as 15 minutes need to pass before lockup is expired");
        let tx = yield program.rpc.withdrawDepositTracking(
        // fixed amount we get for 10 USDC based on the program state which has been dumped to disk
        new anchor.BN(3987587), [new anchor.BN(1), new anchor.BN(65537)], {
            options: { skipPreflight: true },
            accounts: {
                authority: provider.wallet.publicKey,
                depositTrackingAccount,
                depositTrackingPda,
                depositTrackingHoldAccount,
                receivingSharesAccount: yourSharesTokenAccount,
                sharesMint: usdcv1SharesMint,
                vault: usdcv1Vault,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                vaultProgram: v2VaultsProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
            },
        });
        console.log("sent withdraw deposit tracking tx");
    }));
    it("withdraws multi deposit vault through tulip", () => __awaiter(void 0, void 0, void 0, function* () {
        let tx = yield program.rpc.withdrawMultiDepositVaultThroughTulip(new anchor.BN(3987587), {
            options: { skipPreflight: true },
            accounts: {
                commonData: {
                    authority: provider.wallet.publicKey,
                    multiVault: usdcv1Vault,
                    multiVaultPda: usdcv1VaultPda,
                    withdrawVault: tulipVault,
                    withdrawVaultPda: tulipVaultPda,
                    platformInformation: tulipVaultPlatformInformation,
                    platformConfigData: tulipVaultPlatformConfigDataAccount,
                    multiBurningSharesTokenAccount: yourSharesTokenAccount,
                    withdrawBurningSharesTokenAccount: usdcv1TulipVaultSharesAccount,
                    receivingUnderlyingTokenAccount: yourUnderlyingTokenAccount,
                    multiUnderlyingWithdrawQueue: usdcv1WithdrawQueue,
                    multiSharesMint: usdcv1SharesMint,
                    withdrawSharesMint: tulipVaultSharesMint,
                    withdrawVaultUnderlyingDepositQueue: tulipVaultDepositQueue,
                    clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                    tokenProgram: splToken.TOKEN_PROGRAM_ID,
                    lendingProgram: tulipLendingProgramId,
                    vaultProgram: v2VaultsProgramId
                },
            },
            remainingAccounts: [
                {
                    pubkey: tulipCollateralTokenAccount,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: tulipReserveAccount,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: tulipReserveLiquiditySupply,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: tulipReserveCollateralMint,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: tulipLendingMarketAccount,
                    isSigner: false,
                    isWritable: false
                },
                {
                    pubkey: tulipDerivedLendingMarketAuthority,
                    isSigner: false,
                    isWritable: false
                },
                {
                    pubkey: tulipReservePythPriceAccount,
                    isSigner: false,
                    isWritable: false
                }
            ]
        }).then(() => {
            console.log("this instruction should not pass in localnet testing");
            process.exit(255);
        }).catch(() => {
            console.log("this test is expected to fail in localnet testing");
        });
    }));
    it("withdraws multi deposit vault through mango", () => __awaiter(void 0, void 0, void 0, function* () {
        program.rpc.withdrawMultiDepositVaultThroughMango(new anchor.BN(3987587), {
            options: { skipPreflight: true },
            accounts: {
                commonData: {
                    authority: provider.wallet.publicKey,
                    multiVault: usdcv1Vault,
                    multiVaultPda: usdcv1VaultPda,
                    withdrawVault: mangoVault,
                    withdrawVaultPda: mangoVaultPda,
                    platformInformation: mangoVaultPlatformInformation,
                    platformConfigData: mangoVaultPlatformConfigDataAccount,
                    multiBurningSharesTokenAccount: yourSharesTokenAccount,
                    withdrawBurningSharesTokenAccount: usdcv1MangoVaultSharesAccount,
                    receivingUnderlyingTokenAccount: yourUnderlyingTokenAccount,
                    multiUnderlyingWithdrawQueue: usdcv1WithdrawQueue,
                    multiSharesMint: usdcv1SharesMint,
                    withdrawSharesMint: mangoVaultSharesMint,
                    withdrawVaultUnderlyingDepositQueue: mangoVaultDepositQueue,
                    clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                    tokenProgram: splToken.TOKEN_PROGRAM_ID,
                    lendingProgram: mangoProgramId,
                    vaultProgram: v2VaultsProgramId,
                },
                withdrawVaultMangoAccount: mangoVaultMangoAccount,
                mangoCache: mangoCache,
                mangoRootBank,
                mangoNodeBank,
                mangoGroupAccount,
                mangoTokenAccount,
                mangoGroupSigner,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
        }).then(() => {
            console.log("test should not pass in localnet");
            process.exit(255);
        }).catch(() => {
            console.log("due to localnet restrictions, this test is expected to fail");
        });
    }));
    it("withdraws multi deposit vault through solend", () => __awaiter(void 0, void 0, void 0, function* () {
        program.rpc.withdrawMultiDepositVaultThroughSolend(new anchor.BN(3987587), {
            options: { skipPreflight: true },
            accounts: {
                commonData: {
                    authority: provider.wallet.publicKey,
                    multiVault: usdcv1Vault,
                    multiVaultPda: usdcv1VaultPda,
                    withdrawVault: solendVault,
                    withdrawVaultPda: solendVaultPda,
                    platformInformation: solendVaultPlatformInformation,
                    platformConfigData: solendVaultPlatformConfigDataAccount,
                    multiBurningSharesTokenAccount: yourSharesTokenAccount,
                    withdrawBurningSharesTokenAccount: usdcv1SolendVaultSharesAccount,
                    receivingUnderlyingTokenAccount: yourUnderlyingTokenAccount,
                    multiUnderlyingWithdrawQueue: usdcv1WithdrawQueue,
                    multiSharesMint: usdcv1SharesMint,
                    withdrawSharesMint: solendVaultSharesMint,
                    withdrawVaultUnderlyingDepositQueue: solendVaultDepositQueue,
                    clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                    tokenProgram: splToken.TOKEN_PROGRAM_ID,
                    lendingProgram: solendProgramId,
                    vaultProgram: v2VaultsProgramId,
                },
            },
            remainingAccounts: [
                {
                    pubkey: solendCollateralTokenAccount,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: solendReserveAccount,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: solendReserveLiquiditySupply,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: solendReserveCollateralMint,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: solendLendingMarketAccount,
                    isSigner: false,
                    isWritable: false
                },
                {
                    pubkey: solendDerivedLendingMarketAuthority,
                    isSigner: false,
                    isWritable: false
                },
                {
                    pubkey: solendReservePythPriceAccount,
                    isSigner: false,
                    isWritable: false
                },
                {
                    pubkey: solendReserveSwitchboardPriceAccount,
                    isSigner: false,
                    isWritable: false
                }
            ]
        }).catch(() => {
            console.log("test failure is expected in localnet");
        })
            .then(() => { });
    }));
});
describe("test lending instructions via usdc ", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Examples;
    const programId = program.programId;
    let yourCollateralTokenAccount;
    let yourUnderlyingTokenAccount;
    it("gets underlying token accounts", () => __awaiter(void 0, void 0, void 0, function* () {
        yourUnderlyingTokenAccount = yield serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, usdcTokenMint);
    }));
    it("deposits reserve liquidity", () => __awaiter(void 0, void 0, void 0, function* () {
        yourCollateralTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, tulipReserveCollateralMint);
        const tx = yield program.rpc.depositReserveLiquidity(one, {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                sourceLiquidityTokenAccount: yourUnderlyingTokenAccount,
                destinationCollateral: yourCollateralTokenAccount,
                reserve: tulipReserveAccount,
                reserveLiquidity: tulipReserveLiquiditySupply,
                reserveCollateralMint: tulipReserveCollateralMint,
                lendingMarket: tulipLendingMarketAccount,
                lendingMarketAuthority: tulipDerivedLendingMarketAuthority,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                lendingProgram: tulipLendingProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                pythPriceAccount: tulipReservePythPriceAccount,
            }
        });
        console.log("sent deposit reserve liquidity tx ", tx);
    }));
    it("redeems reserve collateral", () => __awaiter(void 0, void 0, void 0, function* () {
        const bal = yield provider.connection.getTokenAccountBalance(yourCollateralTokenAccount);
        const tx = yield program.rpc.redeemReserveCollateral(new anchor.BN(bal.value.amount), {
            accounts: {
                authority: provider.wallet.publicKey,
                destinationLiquidity: yourUnderlyingTokenAccount,
                sourceCollateral: yourCollateralTokenAccount,
                reserve: tulipReserveAccount,
                reserveLiquidity: tulipReserveLiquiditySupply,
                reserveCollateralMint: tulipReserveCollateralMint,
                lendingMarket: tulipLendingMarketAccount,
                lendingMarketAuthority: tulipDerivedLendingMarketAuthority,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                lendingProgram: tulipLendingProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                pythPriceAccount: tulipReservePythPriceAccount,
            }
        });
    }));
});
describe("tests leverage farm instructions via ray-usdc", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Examples;
    const programId = program.programId;
    let userFarmAddress;
    let userFarmObligation1VaultAddress;
    let userFarmObligation1Address;
    let userFarmObligationVault1LpTokenAccount;
    it("creates user farm", () => __awaiter(void 0, void 0, void 0, function* () {
        let [_userFarm, _userFarmNonce] = yield (0, utils_1.findUserFarmAddress)(provider.wallet.publicKey, tulipLeveragedFarmProgramId, new anchor.BN(0), new anchor.BN(0));
        userFarmAddress = _userFarm;
        let [_userFarmObligationVault, _userFarmObligationVaultNonce] = yield (0, utils_1.findUserFArmObligationVaultAddress)(userFarmAddress, new anchor.BN(0), tulipLeveragedFarmProgramId);
        userFarmObligation1VaultAddress = _userFarmObligationVault;
        let [_userFarmObligation, _userFarmObligationNonce] = yield (0, utils_1.findUserFarmObligationAddress)(provider.wallet.publicKey, userFarmAddress, tulipLeveragedFarmProgramId, new anchor.BN(0));
        userFarmObligation1Address = _userFarmObligation;
        userFarmObligationVault1LpTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, userFarmObligation1VaultAddress, rayUsdcLpTokenMint);
        const tx = yield program.rpc.createUserFarm(new anchor.BN(0), {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                userFarm: userFarmAddress,
                userFarmObligation: userFarmObligation1Address,
                lendingMarket: tulipLendingMarketAccount,
                global: tulipLevFarmGlobalAccount,
                leveragedFarm: tulipRayUsdcLevFarmAccount,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                lendingProgram: tulipLendingProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                obligationVaultAddress: userFarmObligation1VaultAddress,
                userFarmLpTokenAccount: yield serumAssoToken.getAssociatedTokenAddress(userFarmAddress, rayUsdcLpTokenMint),
                userFarmBaseTokenAccount: yield serumAssoToken.getAssociatedTokenAddress(userFarmAddress, rayTokenMint),
                userFarmQuoteTokenAccount: yield serumAssoToken.getAssociatedTokenAddress(userFarmAddress, usdcTokenMint),
                lpTokenMint: rayUsdcLpTokenMint,
                baseTokenMint: rayTokenMint,
                quoteTokenMint: usdcTokenMint,
                associatedTokenProgram: spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID,
                tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId,
            }
        });
        console.log("sent create user farm token account tx ", tx);
    }));
    let userFarmObligation2VaultAddress;
    let userFarmObligation2Address;
    let yourUsdcTokenAccount;
    let yourRayTokenAccount;
    it("create user farm obligation", () => __awaiter(void 0, void 0, void 0, function* () {
        yourUsdcTokenAccount = yield (0, spl_token_1.getAssociatedTokenAddress)(usdcTokenMint, provider.wallet.publicKey);
        yourRayTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, rayTokenMint);
        let [_userFarmObligationVault, _userFarmObligationVaultNonce] = yield (0, utils_1.findUserFArmObligationVaultAddress)(userFarmAddress, new anchor.BN(1), tulipLeveragedFarmProgramId);
        userFarmObligation2VaultAddress = _userFarmObligationVault;
        let [_userFarmObligation, _userFarmObligationNonce] = yield (0, utils_1.findUserFarmObligationAddress)(provider.wallet.publicKey, userFarmAddress, tulipLeveragedFarmProgramId, new anchor.BN(1));
        userFarmObligation2Address = _userFarmObligation;
        const tx = yield program.rpc.createUserFarmObligation(new anchor.BN(0), new anchor.BN(1), {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                userFarm: userFarmAddress,
                userFarmObligation: userFarmObligation2Address,
                lendingMarket: tulipLendingMarketAccount,
                leveragedFarm: tulipRayUsdcLevFarmAccount,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                lendingProgram: tulipLendingProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                obligationVaultAddress: userFarmObligation2VaultAddress,
                tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId,
            }
        });
        console.log("sent create user farm token account tx ", tx);
    }));
    let positionInfoAccount;
    it("invokes deposit borrow dual pc, borrow pc", () => __awaiter(void 0, void 0, void 0, function* () {
        let [_posInfo, _posNonce] = yield (0, utils_1.findUserPositionInfoAddress)(userFarmAddress, tulipLeveragedFarmProgramId, new anchor.BN(0));
        positionInfoAccount = _posInfo;
        const tx = yield program.rpc.depositBorrowDual(new anchor.BN(0), one.mul(new anchor.BN(4)), one, new anchor.BN(0), new anchor.BN(0), {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                userFarm: userFarmAddress,
                leveragedFarm: tulipRayUsdcLevFarmAccount,
                userFarmObligation: userFarmObligation1Address,
                coinSourceTokenAccount: yourRayTokenAccount,
                coinDestinationTokenAccount: rayUsdcLevFarmBaseTokenAccount,
                pcSourceTokenAccount: yourUsdcTokenAccount,
                pcDestinationTokenAccount: rayUsdcLevFarmQuoteTokenAccount,
                coinDepositReserveAccount,
                pcDepositReserveAccount,
                coinReserveLiquidityOracle,
                pcReserveLiquidityOracle,
                lendingMarketAccount: tulipLendingMarketAccount,
                derivedLendingMarketAuthority: tulipDerivedLendingMarketAuthority,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                lendingProgram: tulipLendingProgramId,
                coinSourceReserveLiquidityTokenAccount,
                pcSourceReserveLiquidityTokenAccount,
                coinReserveLiquidityFeeReceiver,
                pcReserveLiquidityFeeReceiver,
                borrowAuthorizer,
                lpPythPriceAccount,
                vaultAccount: v1RayUsdcVaultAccount,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                positionInfoAccount,
                systemProgram: anchor.web3.SystemProgram.programId,
                tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId
            }
        });
        console.log("sent deposit_dual_borrw tx ", tx);
    }));
    it("swaps tokens", () => __awaiter(void 0, void 0, void 0, function* () {
        const tx = yield program.rpc.swapTokensRaydiumStats(new anchor.BN(0), {
            options: {
                skipPreflight: true
            },
            accounts: {
                authority: provider.wallet.publicKey,
                leveragedFarm: tulipRayUsdcLevFarmAccount,
                userFarm: userFarmAddress,
                userFarmObligation: userFarmObligation1Address,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                vaultSigner: rayUsdcSerumVaultSigner,
                swapOrLiquidityProgramId: raydiumLiquidityProgram,
                ammId: rayUsdcAmmId,
                ammAuthority: rayUsdcAmmAuthority,
                ammOpenOrders: rayUsdcAmmOpenOrders,
                ammQuantitiesOrTargetOrders: rayUsdcAmmQuantitiesOrTargetOrders,
                poolCoinTokenaccount: rayUsdcAmmPoolCoinTokenAccount,
                poolPcTokenaccount: rayUsdcAmmPoolPcTokenAccount,
                serumProgramId,
                serumMarket: rayUsdcSerumMarket,
                serumBids: rayUsdcSerumBids,
                serumAsks: rayUsdcSerumAsks,
                serumEventQueue: rayUsdcSerumEventQueue,
                serumCoinVaultAccount: rayUsdcSerumCoinVault,
                serumPcVaultAccount: rayUsdcSerumPcVault,
                serumVaultSigner: rayUsdcSerumVaultSigner,
                coinWallet: rayUsdcLevFarmBaseTokenAccount,
                pcWallet: rayUsdcLevFarmQuoteTokenAccount,
                tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId
            },
            remainingAccounts: [
                {
                    pubkey: tulipLendingMarketAccount,
                    isWritable: false,
                    isSigner: false
                },
                {
                    pubkey: tulipDerivedLendingMarketAuthority,
                    isWritable: false,
                    isSigner: false,
                },
                {
                    pubkey: tulipLendingProgramId,
                    isWritable: false,
                    isSigner: false,
                },
                {
                    pubkey: positionInfoAccount,
                    isWritable: true,
                    isSigner: false,
                }
            ],
        });
    }));
    it("adds liquidity stats", () => __awaiter(void 0, void 0, void 0, function* () {
        const tx = yield program.rpc.addLiquidityStats(new anchor.BN(0), {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                leveragedFarm: tulipRayUsdcLevFarmAccount,
                userFarm: userFarmAddress,
                userFarmObligation: userFarmObligation1Address,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                ammId: rayUsdcAmmId,
                ammAuthority: rayUsdcAmmAuthority,
                ammOpenOrders: rayUsdcAmmOpenOrders,
                ammQuantitiesOrTargetOrders: rayUsdcAmmQuantitiesOrTargetOrders,
                serumMarket: rayUsdcSerumMarket,
                lendingProgram: tulipLendingProgramId,
                liquidityProgramId: raydiumLiquidityProgram,
                lpMintAddress: rayUsdcLpTokenMint,
                poolCoinTokenAccount: rayUsdcAmmPoolCoinTokenAccount,
                poolPcTokenAccount: rayUsdcAmmPoolPcTokenAccount,
                userLpTokenAccount: userFarmObligationVault1LpTokenAccount,
                levFarmCoinTokenAccount: rayUsdcLevFarmBaseTokenAccount,
                levFarmPcTokenAccount: rayUsdcLevFarmQuoteTokenAccount,
                pythPriceAccount: lpPythPriceAccount,
                lendingMarketAccount: tulipLendingMarketAccount,
                derivedLendingMarketAuthority: tulipDerivedLendingMarketAuthority,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                dexProgram: serumProgramId,
                tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId
            },
            remainingAccounts: [
                {
                    pubkey: positionInfoAccount,
                    isWritable: true,
                    isSigner: false,
                }
            ]
        });
    }));
    let vaultBalanceAccount;
    let vaultBalanceMetadataAccount;
    let vaultRewardAccount;
    it("deposits vault", () => __awaiter(void 0, void 0, void 0, function* () {
        let a;
        let b;
        let c;
        [vaultBalanceAccount, a] = yield (0, utils_1.findVaultBalanceAccount)(v1RayUsdcOldVaultInfoAccount, userFarmObligation1VaultAddress, v1RaydiumVaultsProgram);
        [vaultBalanceMetadataAccount, b] = yield (0, utils_1.findVaultBalanceMetadataAccount)(vaultBalanceAccount, userFarmObligation1VaultAddress, v1RaydiumVaultsProgram);
        [vaultRewardAccount, c] = yield (0, utils_1.findVaultRewardAccount)(vaultBalanceAccount, userFarmObligation1Address, v1RaydiumVaultsProgram);
        program.rpc.depositRaydiumVault(new anchor.BN(0), new anchor.BN(0), {
            accounts: {
                authority: provider.wallet.publicKey,
                userFarm: userFarmAddress,
                obligationVaultAddress: userFarmObligation1VaultAddress,
                leveragedFarm: tulipRayUsdcLevFarmAccount,
                vaultProgram: v1RaydiumVaultsProgram,
                authorityTokenAccount: userFarmObligationVault1LpTokenAccount,
                vaultPdaAccount: v1RayUsdcVaultPda,
                vault: v1RayUsdcVaultAccount,
                lpTokenAccount: v1RayUsdcVaultLpTokenAccount,
                userBalanceAccount: vaultBalanceAccount,
                systemProgram: anchor.web3.SystemProgram.programId,
                stakeProgramId: raydiumStakeProgramId,
                poolId: rayUsdcPoolId,
                poolAuthority: v1RayUsdcPoolAuthority,
                vaultInfoAccount: v1RayUsdcVaultInfoAccount,
                poolLpTokenAccount: v1RayUsdcPoolLpTokenAccount,
                userRewardATokenAccount: v1RayUsdcVaultRewardATokenAccount,
                poolRewardATokenAccount: v1RayUsdcPoolRewardATokenAccount,
                userRewardBTokenAccount: v1RayUsdcVaultRewardBTokenAccount,
                poolRewardBTokenAccount: v1RayUsdcPoolRewardBTokenAccount,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                tokenProgramId: splToken.TOKEN_PROGRAM_ID,
                userBalanceMetadata: vaultBalanceMetadataAccount,
                lendingMarketAccount: tulipLendingMarketAccount,
                userFarmObligation: userFarmObligation1Address,
                lendingMarketAuthority: tulipDerivedLendingMarketAuthority,
                lendingProgram: tulipLendingProgramId,
                tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId
            },
        }).catch(error => console.log("deposit fails on localnet for some reason"));
    }));
    it("withdraws raydium vault", () => __awaiter(void 0, void 0, void 0, function* () {
        program.rpc.withdrawRaydiumVaultClose(new anchor.BN(0), new anchor.BN(50), new anchor.BN(0), new anchor.BN(0), {
            accounts: {
                authority: provider.wallet.publicKey,
                userFarm: userFarmAddress,
                obligationVaultAddress: userFarmObligation1VaultAddress,
                leveragedFarm: tulipRayUsdcLevFarmAccount,
                vaultProgram: v1RaydiumVaultsProgram,
                authorityTokenAccount: userFarmObligationVault1LpTokenAccount,
                vaultPdaAccount: v1RayUsdcVaultPda,
                vault: v1RayUsdcVaultAccount,
                userBalanceAccount: vaultBalanceAccount,
                stakeProgramId: raydiumStakeProgramId,
                poolId: rayUsdcPoolId,
                poolAuthority: v1RayUsdcPoolAuthority,
                poolLpTokenAccount: v1RayUsdcPoolLpTokenAccount,
                userRewardATokenAccount: v1RayUsdcVaultRewardATokenAccount,
                poolRewardATokenAccount: v1RayUsdcPoolRewardATokenAccount,
                userRewardBTokenAccount: v1RayUsdcVaultRewardBTokenAccount,
                poolRewardBTokenAccount: v1RayUsdcPoolRewardBTokenAccount,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                tokenProgramId: splToken.TOKEN_PROGRAM_ID,
                tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId,
                userInfoAccount: v1RayUsdcVaultInfoAccount,
                userBalanceMeta: vaultBalanceMetadataAccount,
                userLpTokenAccount: userFarmObligationVault1LpTokenAccount,
            },
            remainingAccounts: [
                {
                    pubkey: tulipLendingMarketAccount,
                    isWritable: false,
                    isSigner: false,
                },
                {
                    pubkey: userFarmObligation1Address,
                    isWritable: true,
                    isSigner: false,
                },
                {
                    pubkey: tulipDerivedLendingMarketAuthority,
                    isWritable: false,
                    isSigner: false,
                },
                {
                    pubkey: tulipLendingProgramId,
                    isWritable: false,
                    isSigner: false,
                },
                {
                    pubkey: positionInfoAccount,
                    isWritable: true,
                    isSigner: false,
                },
                {
                    pubkey: anchor.web3.SystemProgram.programId,
                    isWritable: false,
                    isSigner: false,
                },
                {
                    pubkey: anchor.web3.SYSVAR_RENT_PUBKEY,
                    isWritable: false,
                    isSigner: false,
                }
            ]
        }).catch(() => {
            console.log("raydium vault withdrawal fails in localnet testing");
        }); /*.then(() => {
          console.log("should fail in localnet testing")
          process.exit(1);
        })*/
    }));
});
describe("tests leverage farm instructions via orca-usdc non double dip", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Examples;
    const programId = program.programId;
    let userFarmAddress;
    let userFarmObligation1VaultAddress;
    let userFarmObligation1Address;
    let userFarmObligationVault1LpTokenAccount;
    it("creates user farm", () => __awaiter(void 0, void 0, void 0, function* () {
        let [_userFarm, _userFarmNonce] = yield (0, utils_1.findUserFarmAddress)(provider.wallet.publicKey, tulipLeveragedFarmProgramId, new anchor.BN(0), new anchor.BN(17));
        userFarmAddress = _userFarm;
        let [_userFarmObligationVault, _userFarmObligationVaultNonce] = yield (0, utils_1.findUserFArmObligationVaultAddress)(userFarmAddress, new anchor.BN(0), tulipLeveragedFarmProgramId);
        userFarmObligation1VaultAddress = _userFarmObligationVault;
        let [_userFarmObligation, _userFarmObligationNonce] = yield (0, utils_1.findUserFarmObligationAddress)(provider.wallet.publicKey, userFarmAddress, tulipLeveragedFarmProgramId, new anchor.BN(0));
        userFarmObligation1Address = _userFarmObligation;
        userFarmObligationVault1LpTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, userFarmObligation1VaultAddress, orcaUsdcLpTokenMint);
        console.log("sending user farm create tx");
        const tx = yield program.rpc.createUserFarm(new anchor.BN(17), {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                userFarm: userFarmAddress,
                userFarmObligation: userFarmObligation1Address,
                lendingMarket: tulipLendingMarketAccount,
                global: tulipLevFarmGlobalAccount,
                leveragedFarm: orcaUsdcLevFarmAccount,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                lendingProgram: tulipLendingProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                obligationVaultAddress: userFarmObligation1VaultAddress,
                userFarmLpTokenAccount: yield serumAssoToken.getAssociatedTokenAddress(userFarmAddress, orcaUsdcLpTokenMint),
                userFarmBaseTokenAccount: yield serumAssoToken.getAssociatedTokenAddress(userFarmAddress, orcaTokenMint),
                userFarmQuoteTokenAccount: yield serumAssoToken.getAssociatedTokenAddress(userFarmAddress, usdcTokenMint),
                lpTokenMint: orcaUsdcLpTokenMint,
                baseTokenMint: orcaTokenMint,
                quoteTokenMint: usdcTokenMint,
                associatedTokenProgram: spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID,
                tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId,
            }
        });
        console.log("sent create user farm token account tx ", tx);
    }));
    let userFarmObligation2VaultAddress;
    let userFarmObligation2Address;
    let yourOrcaTokenAccount;
    it("create user farm obligation", () => __awaiter(void 0, void 0, void 0, function* () {
        yourOrcaTokenAccount = yield (0, spl_token_1.getAssociatedTokenAddress)(provider.wallet.publicKey, orcaTokenMint);
        let [_userFarmObligationVault, _userFarmObligationVaultNonce] = yield (0, utils_1.findUserFArmObligationVaultAddress)(userFarmAddress, new anchor.BN(1), tulipLeveragedFarmProgramId);
        userFarmObligation2VaultAddress = _userFarmObligationVault;
        let [_userFarmObligation, _userFarmObligationNonce] = yield (0, utils_1.findUserFarmObligationAddress)(provider.wallet.publicKey, userFarmAddress, tulipLeveragedFarmProgramId, new anchor.BN(1));
        userFarmObligation2Address = _userFarmObligation;
        const tx = yield program.rpc.createUserFarmObligation(new anchor.BN(17), new anchor.BN(1), {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                userFarm: userFarmAddress,
                userFarmObligation: userFarmObligation2Address,
                lendingMarket: tulipLendingMarketAccount,
                leveragedFarm: orcaUsdcLevFarmAccount,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                lendingProgram: tulipLendingProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                obligationVaultAddress: userFarmObligation2VaultAddress,
                tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId,
            }
        });
        console.log("sent create user farm token account tx ", tx);
    }));
});
describe("tests ray-usdc auto vaults", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Examples;
    const programId = program.programId;
    let depositTrackingAccount;
    let depositTrackingPda;
    let depositTrackingQueueAccount;
    let depositTrackingHoldAccount;
    let yourUnderlyingTokenAccount = yourRayUsdcLpTokenAccount;
    let yourSharesTokenAccount;
    it("registers deposit tracking account", () => __awaiter(void 0, void 0, void 0, function* () {
        (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, orcaTokenMint);
        console.log("progrmaId ", programId);
        console.log("usdcv1 vault ", rayUsdcV2Vault);
        console.log("provider", provider.wallet.publicKey);
        yourUnderlyingTokenAccount = yourRayUsdcLpTokenAccount;
        let [_depositTrackingAccount, _trackingNonce] = yield (0, utils_1.deriveTrackingAddress)(v2VaultsProgramId, rayUsdcV2Vault, provider.wallet.publicKey);
        depositTrackingAccount = _depositTrackingAccount;
        let [_depositTrackingPda, _depositTrackingPdaNonce] = yield (0, utils_1.deriveTrackingPdaAddress)(v2VaultsProgramId, depositTrackingAccount);
        depositTrackingPda = _depositTrackingPda;
        let [_depositTrackingQueueAccount, _queueNonce] = yield (0, utils_1.deriveTrackingQueueAddress)(v2VaultsProgramId, depositTrackingPda);
        depositTrackingQueueAccount = _depositTrackingQueueAccount;
        depositTrackingHoldAccount = yield serumAssoToken.getAssociatedTokenAddress(depositTrackingPda, rayUsdcV2VaultSharesMint);
        console.log("deposit tracking queue", depositTrackingQueueAccount.toString());
        console.log("deposit tracking hold", depositTrackingHoldAccount.toString());
        console.log("deposit tracking pda", depositTrackingPda.toString());
        console.log("deposit tracking", depositTrackingAccount.toString());
        console.log("sending register deposit tracking account tx");
        let tx = yield program.rpc.registerDepositTrackingAccount([new anchor.BN(0), new anchor.BN(9)], {
            options: {
                skipPreflight: false,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                vault: rayUsdcV2Vault,
                depositTrackingAccount,
                depositTrackingQueueAccount,
                depositTrackingHoldAccount,
                sharesMint: rayUsdcV2VaultSharesMint,
                underlyingMint: rayUsdcLpTokenMint,
                depositTrackingPda,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultProgram: v2VaultsProgramId,
            },
        });
        console.log("sent register deposit tracking account tx ", tx);
    }));
    let one = new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(6)));
    it("issues shares", () => __awaiter(void 0, void 0, void 0, function* () {
        yourUnderlyingTokenAccount = yield serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, rayUsdcLpTokenMint);
        yourSharesTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, rayUsdcV2VaultSharesMint);
        let tx = yield program.rpc.issueShares(one, [new anchor.BN(0), new anchor.BN(9)], {
            options: {
                skipPreflight: false,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                vault: rayUsdcV2Vault,
                depositTrackingAccount,
                depositTrackingPda,
                vaultPda: rayUsdcV2VaultPda,
                sharesMint: rayUsdcV2VaultSharesMint,
                receivingSharesAccount: depositTrackingHoldAccount,
                depositingUnderlyingAccount: yourUnderlyingTokenAccount,
                vaultUnderlyingAccount: rayUsdcV2VaultDepositQueue,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultProgram: v2VaultsProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
            },
        });
        console.log("sent issue shares tx");
    }));
    it("withdraws from deposit tracking account", () => __awaiter(void 0, void 0, void 0, function* () {
        console.log("wait 3 seconds");
        freeze(3000);
        console.log("in production this would fail, as 15 minutes need to pass before lockup is expired");
        let tx = yield program.rpc.withdrawDepositTracking(
        // fixed amount we get for 10 USDC based on the program state which has been dumped to disk
        new anchor.BN(877610), [new anchor.BN(0), new anchor.BN(9)], {
            options: { skipPreflight: true },
            accounts: {
                authority: provider.wallet.publicKey,
                depositTrackingAccount,
                depositTrackingPda,
                depositTrackingHoldAccount,
                receivingSharesAccount: yourSharesTokenAccount,
                sharesMint: rayUsdcV2VaultSharesMint,
                vault: rayUsdcV2Vault,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                vaultProgram: v2VaultsProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
            },
        });
        console.log("sent withdraw deposit tracking tx ", tx);
    }));
    const rayUsdcPoolId = new anchor.web3.PublicKey("CHYrUBX2RKX8iBg7gYTkccoGNBzP44LdaazMHCLcdEgS");
    const rayUsdcPoolLpTokenAccount = new anchor.web3.PublicKey("BNnXLFGva3K8ACruAc1gaP49NCbLkyE6xWhGV4G2HLrs");
    const rayUsdcPoolRewardATokenAccount = new anchor.web3.PublicKey("DpRueBHHhrQNvrjZX7CwGitJDJ8eZc3AHcyFMG4LqCQR");
    const rayUsdcPoolRewardBTokenAccount = new anchor.web3.PublicKey("DpRueBHHhrQNvrjZX7CwGitJDJ8eZc3AHcyFMG4LqCQR");
    const rayUsdcPoolAuthority = new anchor.web3.PublicKey("5KQFnDd33J5NaMC9hQ64P5XzaaSz8Pt7NBCkZFYn1po");
    it("withdraws from raydium vault", () => __awaiter(void 0, void 0, void 0, function* () {
        yield program.rpc.withdrawRaydiumVault(new anchor.BN(877610), {
            options: {
                skipPreflight: true
            },
            accounts: {
                authority: provider.wallet.publicKey,
                vault: rayUsdcV2Vault,
                vaultPda: rayUsdcV2VaultPda,
                associatedStakeInfoAccount: rayUsdcV2VaultAssociatedStakeInfo,
                poolId: rayUsdcPoolId,
                poolAuthority: rayUsdcPoolAuthority,
                underlyingWithdrawQueue: rayUsdcV2VaultWithdrawQueue,
                poolLpTokenAccount: rayUsdcPoolLpTokenAccount,
                vaultRewardATokenAccount: rayUsdcV2VaultRewardATokenAccount,
                vaultRewardBTokenAccount: rayUsdcV2VaultRewardATokenAccount,
                poolRewardATokenAccount: rayUsdcPoolRewardATokenAccount,
                poolRewardBTokenAccount: rayUsdcPoolRewardBTokenAccount,
                burningSharesTokenAccount: yourSharesTokenAccount,
                receivingUnderlyingTokenAccount: yourUnderlyingTokenAccount,
                sharesMint: rayUsdcV2VaultSharesMint,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                raydiumStakeProgram: new anchor.web3.PublicKey("EhhTKczWMGQt46ynNeRX1WfeagwwJd7ufHvCDjRxjo5Q"),
                vaultProgram: v2VaultsProgramId,
                feeCollectorRewardATokenAccount: new anchor.web3.PublicKey("EPPhqysp2Vxmap2p3qvUBH6uqfoGUR3E2pvWr4esxaRN"),
            }
        }).then(() => {
            console.log("transaction should not succeed in localnet environments");
            process.exit(1);
        }).catch(() => {
            console.log("error is expected to happen in localnet environments");
        });
    }));
});
describe("tests orca-usdc auto vaults", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Examples;
    const programId = program.programId;
    const orcaUsdcV2Vault = new anchor.web3.PublicKey("7nbcWTUnvELLmLjJtMRrbg9qH9zabZ9VowJSfwB2j8y7");
    const orcaUsdcV2VaultPda = new anchor.web3.PublicKey("Hum9fSBpV26PRBpCj53nYyBy6KSyRfKLsZcFTfLMUamf");
    const orcaUsdcV2VaultSharesMint = new anchor.web3.PublicKey("5Dzz5aB1x4DMkRYHuEA1tmzxm6jfpWGzn4ScNBUACGbd");
    const orcaUsdcV2VaultCompoundQueue = new anchor.web3.PublicKey("Dzpjfr3uEwpn7Vo538rEy9XV4pAjUDvxDPD3cePzH48G");
    const orcaUsdcV2VaultDepositQueue = new anchor.web3.PublicKey("DwZSxDX447QXLfzHUi5f2t59MhyGgfr6dBqwXsHhxwd");
    const orcaUsdcV2VaultWithdrawQueue = new anchor.web3.PublicKey("Bdv8bRJSNGhb285P2ZRXCxc6RieFz1vd8wcypMp3NvEB");
    const orcaUsdcLpTokenMint = new anchor.web3.PublicKey("n8Mpu28RjeYD7oUX3LG1tPxzhRZh3YYLRSHcHRdS3Zx");
    const orcaUsdcPoolTokenA = new anchor.web3.PublicKey("9vYWHBPz817wJdQpE8u3h8UoY3sZ16ZXdCcvLB7jY4Dj");
    const orcaUsdcPoolTokenB = new anchor.web3.PublicKey("6UczejMUv1tzdvUzKpULKHxrK9sqLm8edR1v9jinVWm9");
    const orcaUsdcPoolFeeAaccount = new anchor.web3.PublicKey('7CXZED4jfRp3qdHB9Py3up6v1C4UhHofFvfT6RXbJLRN');
    const orcaUsdcPoolSwapAccount = new anchor.web3.PublicKey("2p7nYbtPBgtmY69NsE8DAW6szpRJn7tQvDnqvoEWQvjY");
    const orcaUsdcPoolSwapAuthority = new anchor.web3.PublicKey("3fr1AhdiAmWLeNrS24CMoAu9pPgbzVhwLtJ6QUPmw2ob");
    const orcaSwapProgram = new anchor.web3.PublicKey("9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP");
    let depositTrackingAccount;
    let depositTrackingPda;
    let depositTrackingQueueAccount;
    let depositTrackingHoldAccount;
    const usdcTokenMint = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    let yourUnderlyingTokenAccount;
    let yourSharesTokenAccount;
    let yourOrcaUsdcLpTokenAccount;
    it("prepares test data", () => __awaiter(void 0, void 0, void 0, function* () {
        (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, orcaTokenMint).catch(() => __awaiter(void 0, void 0, void 0, function* () {
            // orca token account already exists
            yourOrcaTokenAccount = yield serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, orcaTokenMint);
        })).then((key) => __awaiter(void 0, void 0, void 0, function* () {
            yourOrcaTokenAccount = yield serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, orcaTokenMint);
        }));
        yourUsdcTokenAccount = yield serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, usdcTokenMint);
        yourOrcaUsdcLpTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, orcaUsdcLpTokenMint);
    }));
    it("registers deposit tracking account", () => __awaiter(void 0, void 0, void 0, function* () {
        console.log("progrmaId ", programId);
        console.log("usdcv1 vault ", orcaUsdcV2Vault);
        console.log("provider", provider.wallet.publicKey);
        let [_depositTrackingAccount, _trackingNonce] = yield (0, utils_1.deriveTrackingAddress)(v2VaultsProgramId, orcaUsdcV2Vault, provider.wallet.publicKey);
        depositTrackingAccount = _depositTrackingAccount;
        let [_depositTrackingPda, _depositTrackingPdaNonce] = yield (0, utils_1.deriveTrackingPdaAddress)(v2VaultsProgramId, depositTrackingAccount);
        depositTrackingPda = _depositTrackingPda;
        let [_depositTrackingQueueAccount, _queueNonce] = yield (0, utils_1.deriveTrackingQueueAddress)(v2VaultsProgramId, depositTrackingPda);
        depositTrackingQueueAccount = _depositTrackingQueueAccount;
        depositTrackingHoldAccount = yield serumAssoToken.getAssociatedTokenAddress(depositTrackingPda, orcaUsdcV2VaultSharesMint);
        console.log("deposit tracking queue", depositTrackingQueueAccount.toString());
        yourSharesTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, orcaUsdcV2VaultSharesMint);
        console.log("deposit tracking hold", depositTrackingHoldAccount.toString());
        console.log("deposit tracking pda", depositTrackingPda.toString());
        console.log("deposit tracking", depositTrackingAccount.toString());
        console.log("sending register deposit tracking account tx");
        let tx = yield program.rpc.registerDepositTrackingAccount([new anchor.BN(2), new anchor.BN(4)], {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                vault: orcaUsdcV2Vault,
                depositTrackingAccount,
                depositTrackingQueueAccount,
                depositTrackingHoldAccount,
                sharesMint: orcaUsdcV2VaultSharesMint,
                underlyingMint: orcaUsdcLpTokenMint,
                depositTrackingPda,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultProgram: v2VaultsProgramId,
            },
        });
        console.log("sent register deposit tracking account tx ", tx);
    }));
    let one = new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(6)));
    it("adds liquidity and issues shares", () => __awaiter(void 0, void 0, void 0, function* () {
        const tx = yield program.rpc.orcaAddLiqIssueShares(one, one, [new anchor.BN(2), new anchor.BN(4)], {
            options: {
                skipPreflight: true
            },
            accounts: {
                issueShares: {
                    authority: provider.wallet.publicKey,
                    vault: orcaUsdcV2Vault,
                    depositTrackingAccount,
                    depositTrackingPda,
                    vaultPda: orcaUsdcV2VaultPda,
                    sharesMint: orcaUsdcV2VaultSharesMint,
                    receivingSharesAccount: depositTrackingHoldAccount,
                    depositingUnderlyingAccount: yourOrcaUsdcLpTokenAccount,
                    vaultUnderlyingAccount: orcaUsdcV2VaultDepositQueue,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    vaultProgram: v2VaultsProgramId,
                    tokenProgram: splToken.TOKEN_PROGRAM_ID,
                },
                aquaFarmProgram: orcaAquaFarmProgram,
                addLiq: {
                    fundingTokenAccountA: yourOrcaTokenAccount,
                    fundingTokenAccountB: yourUsdcTokenAccount,
                    poolTokenA: orcaUsdcPoolTokenA,
                    poolTokenB: orcaUsdcPoolTokenB,
                    swapProgram: orcaSwapProgram,
                    swapAccount: orcaUsdcPoolSwapAccount,
                    swapAuthority: orcaUsdcPoolSwapAuthority,
                    swapPoolTokenMint: orcaUsdcLpTokenMint,
                }
            }
        });
        console.log("sent orca add liq issue shares tx ", tx);
    }));
    it("withdraws from deposit tracking account", () => __awaiter(void 0, void 0, void 0, function* () {
        console.log("wait 3 seconds");
        freeze(3000);
        console.log("in production this would fail, as 15 minutes need to pass before lockup is expired");
        let tx = yield program.rpc.withdrawDepositTracking(new anchor.BN(877610), [new anchor.BN(2), new anchor.BN(4)], {
            options: { skipPreflight: true },
            accounts: {
                authority: provider.wallet.publicKey,
                depositTrackingAccount,
                depositTrackingPda,
                depositTrackingHoldAccount,
                receivingSharesAccount: yourSharesTokenAccount,
                sharesMint: orcaUsdcV2VaultSharesMint,
                vault: orcaUsdcV2Vault,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                vaultProgram: v2VaultsProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
            },
        });
        console.log("sent withdraw deposit tracking tx ", tx);
    }));
});
describe("tests orca atlas-usdc double dip auto vaults", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Examples;
    const programId = program.programId;
    const orcaAtlasUsdcV2Vault = new anchor.web3.PublicKey("CjwvvwuacJAJm8w54VcNDgpbnyde6k65mvdRpEFK2Dqm");
    const orcaAtlasUsdcV2VaultPda = new anchor.web3.PublicKey("ESeMrWPgGsj6ZFpHkCDVfbcVan9hGjG2eNG9JnvCVWFi");
    const orcaAtlasUsdcV2VaultSharesMint = new anchor.web3.PublicKey("3r3whdyR2kGaWHV4h52gmSgmpUoMJZwva85x9TY11EmG");
    const orcaAtlasUsdcV2VaultCompoundQueue = new anchor.web3.PublicKey("8ALTUG1vc1JwPP92EoR1jt3gZNjG4d5Q67AF9woU1SXC");
    const orcaAtlasUsdcV2VaultDepositQueue = new anchor.web3.PublicKey("o74d2CniNsA4qN1wa1SHvtth3NTps4FLETAoC8h6UdK");
    const orcaAtlasUsdcV2VaultWithdrawQueue = new anchor.web3.PublicKey("2a4Y4S1yWotDa6pnuaWtftiz2yauHhjc3sNZd33L1Q7M");
    const orcaAtlasUsdcV2VaultDdUserFarm = new anchor.web3.PublicKey("BYzXWdynXxwGd42ki792ppHYXGeG6pTMJmLfQSqWoQhx");
    const orcaAtlasUsdcV2VaultDdGlobalBaseTokenVault = new anchor.web3.PublicKey("Bu3epZQvoSmUJtzAJWH8v91HFwbc9bRN6B9hrjGojFUW");
    const orcaAtlasUSdcV2VaultDdGlobalRewardTokenVault = new anchor.web3.PublicKey("H6xDcxgbV4W9FhiR2VQECSxavSzJHnRnmPzoDWtTc2Qt");
    const orcaAtlasUsdcLpTokenMint = new anchor.web3.PublicKey("FZ8x1LCRSPDeHBDoAc3Gc6Y7ETCynuHEr5q5YWV7uRCJ");
    const orcaAtlasUsdcPoolTokenA = new anchor.web3.PublicKey("xotXsNCx4tBhnwhrajGTaVgKq1sfuMkeYHc77ZegCqE");
    const orcaAtlasUsdcPoolTokenB = new anchor.web3.PublicKey("8YswVYsTi66umBF2Bnkh4LB2VWMKPssDpe54VAgiuJZQ");
    const orcaAtlasUsdcPoolFeeAaccount = new anchor.web3.PublicKey("CFN4DQ2p3qroX92pPNy3mov3Dw1aCNGLrU5AXHpHxbko");
    const orcaAtlasUsdcPoolSwapAccount = new anchor.web3.PublicKey("3V5sjXj1mrWjjB1Xt6Xwp554QwHE5fppGSxbk4GzAtEW");
    const orcaAtlasUsdcPoolSwapAuthority = new anchor.web3.PublicKey("8UYN675AJn5htWydDs724xqintBZ4XzsCWqMozUSDU8m");
    const orcaAtlasUsdcFarmTokenMint = new anchor.web3.PublicKey("HFmY1ggCsCky1zJ1sfdkNR4zb3u5n38YNRdf4vsGu17t");
    const orcaSwapProgram = new anchor.web3.PublicKey("9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP");
    const usdcTokenMint = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    let yourUnderlyingTokenAccount;
    let yourOrcaUsdcLpTokenAccount;
    let providerSharesAccount;
    let depositTrackingAccount;
    let depositTrackingPda;
    let depositTrackingQueueAccount;
    let depositTrackingHoldAccount;
    let depositTrackingOrcaDDQueueAddress;
    let yourAtlasTokenAccount;
    it("prepares test data", () => __awaiter(void 0, void 0, void 0, function* () {
        yourOrcaUsdcLpTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, orcaAtlasUsdcLpTokenMint);
        (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, atlasTokenMint)
            .catch(() => __awaiter(void 0, void 0, void 0, function* () {
            yourAtlasTokenAccount = yield serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, atlasTokenMint);
        })).then(() => __awaiter(void 0, void 0, void 0, function* () {
            yourAtlasTokenAccount = yield serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, atlasTokenMint);
        }));
    }));
    it("registers deposit tracking account", () => __awaiter(void 0, void 0, void 0, function* () {
        providerSharesAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, orcaAtlasUsdcV2VaultSharesMint);
        console.log("provider shares account ", providerSharesAccount.toString());
        let [_depositTrackingAccount, _trackingNonce] = yield (0, utils_1.deriveTrackingAddress)(v2VaultsProgramId, orcaAtlasUsdcV2Vault, provider.wallet.publicKey);
        depositTrackingAccount = _depositTrackingAccount;
        let [_depositTrackingPda, _depositTrackingPdaNonce] = yield (0, utils_1.deriveTrackingPdaAddress)(v2VaultsProgramId, depositTrackingAccount);
        depositTrackingPda = _depositTrackingPda;
        let [_depositTrackingQueueAccount, _queueNonce] = yield (0, utils_1.deriveTrackingQueueAddress)(v2VaultsProgramId, depositTrackingPda);
        depositTrackingQueueAccount = _depositTrackingQueueAccount;
        depositTrackingHoldAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, depositTrackingPda, orcaAtlasUsdcV2VaultSharesMint);
        let [_deriveTrackingOrcaDDQueueAddress, _ddQueueNonce] = yield (0, utils_1.deriveTrackingOrcaDDQueueAddress)(v2VaultsProgramId, orcaAtlasUsdcV2Vault, depositTrackingPda);
        depositTrackingOrcaDDQueueAddress = _deriveTrackingOrcaDDQueueAddress;
        console.log("deposit tracking dd queue ", depositTrackingOrcaDDQueueAddress.toString());
        console.log("deposit tracking hold ", depositTrackingHoldAccount.toString());
        console.log("deposit tracking queue ", depositTrackingQueueAccount.toString());
        console.log("deposit tracking pda ", depositTrackingPda.toString());
        console.log("deposit tracking account ", depositTrackingAccount.toString());
        const tx = yield program.rpc.registerDepositTrackingAccount([new anchor.BN(2), new anchor.BN(0)], {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                vault: orcaAtlasUsdcV2Vault,
                depositTrackingAccount,
                depositTrackingQueueAccount,
                depositTrackingHoldAccount,
                depositTrackingPda,
                underlyingMint: orcaAtlasUsdcLpTokenMint,
                sharesMint: orcaAtlasUsdcV2VaultSharesMint,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
                vaultProgram: v2VaultsProgramId
            },
            remainingAccounts: [
                {
                    pubkey: depositTrackingOrcaDDQueueAddress,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: orcaAtlasUsdcFarmTokenMint,
                    isSigner: false,
                    isWritable: true,
                },
            ],
            preInstructions: [
                anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
                    units: 1400000,
                })
            ],
        });
        console.log("register deposit tracking tx ", tx);
    }));
    let one = new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(6)));
    it("adds liquidity and issues shares", () => __awaiter(void 0, void 0, void 0, function* () {
        const tx = yield program.rpc.orcaAddLiqIssueShares(one, one, [new anchor.BN(2), new anchor.BN(0)], {
            options: {
                skipPreflight: true
            },
            accounts: {
                issueShares: {
                    authority: provider.wallet.publicKey,
                    vault: orcaAtlasUsdcV2Vault,
                    depositTrackingAccount,
                    depositTrackingPda,
                    vaultPda: orcaAtlasUsdcV2VaultPda,
                    sharesMint: orcaAtlasUsdcV2VaultSharesMint,
                    receivingSharesAccount: depositTrackingHoldAccount,
                    depositingUnderlyingAccount: yourOrcaUsdcLpTokenAccount,
                    vaultUnderlyingAccount: orcaAtlasUsdcV2VaultDepositQueue,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    vaultProgram: v2VaultsProgramId,
                    tokenProgram: splToken.TOKEN_PROGRAM_ID,
                },
                aquaFarmProgram: orcaAquaFarmProgram,
                addLiq: {
                    fundingTokenAccountA: yourAtlasTokenAccount,
                    fundingTokenAccountB: yourUsdcTokenAccount,
                    poolTokenA: orcaAtlasUsdcPoolTokenA,
                    poolTokenB: orcaAtlasUsdcPoolTokenB,
                    swapProgram: orcaSwapProgram,
                    swapAccount: orcaAtlasUsdcPoolSwapAccount,
                    swapAuthority: orcaAtlasUsdcPoolSwapAuthority,
                    swapPoolTokenMint: orcaAtlasUsdcLpTokenMint,
                }
            }
        });
        console.log("sent orca add liq issue shares tx ", tx);
    }));
    it("withdraws from deposit tracking account", () => __awaiter(void 0, void 0, void 0, function* () {
        console.log("wait 3 seconds");
        freeze(3000);
        console.log("in production this would fail, as 15 minutes need to pass before lockup is expired");
        let tx = yield program.rpc.withdrawDepositTracking(new anchor.BN(1392), [new anchor.BN(2), new anchor.BN(0)], {
            options: { skipPreflight: true },
            accounts: {
                authority: provider.wallet.publicKey,
                depositTrackingAccount,
                depositTrackingPda,
                depositTrackingHoldAccount,
                receivingSharesAccount: providerSharesAccount,
                sharesMint: orcaAtlasUsdcV2VaultSharesMint,
                vault: orcaAtlasUsdcV2Vault,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                vaultProgram: v2VaultsProgramId,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
            },
        });
        console.log("sent withdraw deposit tracking tx ", tx);
    }));
});
describe("tests atrix usdr-usdc auto vaults", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Examples;
    const programId = program.programId;
    const atrixUsdrUSdcV2Vault = new anchor.web3.PublicKey("DpiKJPdADQStkLoYsiS7rXspEUKBNJwXjuiXs4zvwovw");
    const atrixUsdrUSdcV2VaultPda = new anchor.web3.PublicKey("3rvmzbENBgDKFLEscS1CfYssVDBgsCwNoDmcx4dckz84");
    const atrixUsdrUSdcV2VaultSharesMint = new anchor.web3.PublicKey("7ui67CSJFK8PJQRZ5VySK4jAMJmWivyddqJ7h3CoJvty");
    const atrixUsdrUSdcV2VaultCompoundQueue = new anchor.web3.PublicKey("3hSx6q2v5zjunN7Y5wquZSVtJoVy6KA99Pk7uTsoMab2");
    const atrixUsdrUSdcV2VaultDepositQueue = new anchor.web3.PublicKey("GEHQTUXsyCEtC5ur2EPEcLE6wWscmnCSzPsiCz7cbcGC");
    const atrixUsdrUSdcV2VaultWithdrawQueue = new anchor.web3.PublicKey("5FknEi4RuZWYzJnhKRGQGhEj5kXRnu4FtHRVLAVAs7Ah");
    const atrixUsdrUSdcV2VaultSingleRewardTokenAccount = new anchor.web3.PublicKey("E6mtrtopGCseNroB1NxZv8EGJjUazxgCbhf1vCx91iXx");
    const atrixUsdrUsdcV2VaultStakerAccount = new anchor.web3.PublicKey("AY8oGnqwM9o7taXbSgWieQrEgn7NWDhSWkwqsaU8gdR7");
    const atrixUsdrUsdcV2VaultHarvesterAccount = new anchor.web3.PublicKey("48HoiUzAEPr5cvwMWxzsYoKfH53ZTkfy9oHzTBawi3gN");
    const atrixUsdrUsdcCropAccount = new anchor.web3.PublicKey("AP9GTHoqi51Ljf95HJW5Cp7rpU3AQMXyCeT3nNJxeVdZ");
    const atrixUsdrUsdcCropRewardTokenAccount = new anchor.web3.PublicKey("Efved2EZwUnWGeUayDDzYrRQ5NJSbF5dAKpJ52sXuA4J");
    const atrixUsdrUsdcFarmStakeTokenAccount = new anchor.web3.PublicKey("Djc2QekhrQ8AmhTf57gXc1TTtTXXfrpRjMqb77MHT7rd");
    const atrixUsdrUsdcFarmId = new anchor.web3.PublicKey("AXiRvQs6fyXFJSumZadrnMK1k5G2f2oEPDG573CrfQR5");
    const atrixUsdrUsdcPoolId = new anchor.web3.PublicKey("7V7igBALu1xyu4ZkXfuS6LnfkZ1aA6DKBx55ouA6Jhbm");
    const atrixUsdrUSdcLpTokenMint = new anchor.web3.PublicKey("HKx72DDVoUa1QPb7oqRs4b361TEYoiENxmJJm91Agq8W");
    const atrixUsdrUSdcPoolCoinTokenAccount = new anchor.web3.PublicKey("G23fX1ejHB68a4B8caYw3pgc5Fe545KPRs7pFM95PgEr");
    const atrixUsdrUSdcPoolPcTokenAccount = new anchor.web3.PublicKey("F573sZnEJEgXydbq3KBCG5rn5z6P9AnohVbokpHYcpmg");
    const atrixUsdrUSdcPoolLpTokenAccount = new anchor.web3.PublicKey("FzZPPqtj7T8FmSdsLaw17VHX3GMZa8gTxkzwJKRzc6Re");
    const atrixUsdrUSdcPoolOpenOrdersAccount = new anchor.web3.PublicKey("9BunfFitaX8B4bAJRRywR2b5Ad7pzKqw4LeBrAuKTDh4");
    const atrixUsdrUSdcSerumMarket = new anchor.web3.PublicKey("9vRkq3qWXvGiBMqu7J4mfWQrvppBbwQCTh2MfbqKCnZU");
    const atrixFarmProgramId = new anchor.web3.PublicKey("BLDDrex4ZSWBgPYaaH6CQCzkJXWfzCiiur9cSFJT8t3x");
    const atrixPoolProgramId = new anchor.web3.PublicKey("HvwYjjzPbXWpykgVZhqvvfeeaSraQVnTiQibofaFw9M7");
    const atrixProtocolAccount = new anchor.web3.PublicKey("3uTzTX5GBSfbW7eM9R9k95H7Txe32Qw3Z25MtyD2dzwC");
    let depositTrackingAccount;
    let depositTrackingPda;
    let depositTrackingQueueAccount;
    let depositTrackingHoldAccount;
    const usdcTokenMint = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    const usdrTokenMint = new anchor.web3.PublicKey("HNpdP2rL6FR6jM3bDxFX2Zo32D1YG2ZCztf9zzCrKMEX");
    let yourUnderlyingTokenAccount;
    let yourSharesTokenAccount;
    let yourOrcaUsdcLpTokenAccount;
    it("registers deposit tracking account", () => __awaiter(void 0, void 0, void 0, function* () {
        let [_depositTrackingAccount, _trackingNonce] = yield (0, utils_1.deriveTrackingAddress)(v2VaultsProgramId, atrixUsdrUSdcV2Vault, provider.wallet.publicKey);
        depositTrackingAccount = _depositTrackingAccount;
        let [_depositTrackingPda, _depositTrackingPdaNonce] = yield (0, utils_1.deriveTrackingPdaAddress)(v2VaultsProgramId, depositTrackingAccount);
        depositTrackingPda = _depositTrackingPda;
        let [_depositTrackingQueueAccount, _queueNonce] = yield (0, utils_1.deriveTrackingQueueAddress)(v2VaultsProgramId, depositTrackingPda);
        depositTrackingQueueAccount = _depositTrackingQueueAccount;
        depositTrackingHoldAccount = yield serumAssoToken.getAssociatedTokenAddress(depositTrackingPda, atrixUsdrUSdcV2VaultSharesMint);
        console.log("deposit tracking queue", depositTrackingQueueAccount.toString());
        yourSharesTokenAccount = yield (0, utils_1.createAssociatedTokenAccount)(provider, provider.wallet.publicKey, atrixUsdrUSdcV2VaultSharesMint);
        console.log("deposit tracking hold", depositTrackingHoldAccount.toString());
        console.log("deposit tracking pda", depositTrackingPda.toString());
        console.log("deposit tracking", depositTrackingAccount.toString());
        console.log("sending register atrix usdr-usdc deposit tracking account tx");
        let tx = yield program.rpc.registerDepositTrackingAccount([new anchor.BN(4), new anchor.BN(4)], {
            options: {
                skipPreflight: true,
            },
            accounts: {
                authority: provider.wallet.publicKey,
                vault: atrixUsdrUSdcV2Vault,
                depositTrackingAccount,
                depositTrackingQueueAccount,
                depositTrackingHoldAccount,
                sharesMint: atrixUsdrUSdcV2VaultSharesMint,
                underlyingMint: atrixUsdrUSdcLpTokenMint,
                depositTrackingPda,
                tokenProgram: splToken.TOKEN_PROGRAM_ID,
                associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
                vaultProgram: v2VaultsProgramId,
            },
        });
        console.log("sent register deposit tracking account tx ", tx);
    }));
});
describe("tests raydium v1 vault deposit", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Examples;
    const rayUsdcV1VaultAccount = new anchor.web3.PublicKey("HvNpbHuQUqGG748ZzgzcH5216wdQdTc283CEyFMc3RdG");
    const rayUsdcV1VaultPda = new anchor.web3.PublicKey("Gf38RxSF3FguiBVYfsB8AmpPyNkGCrNDE7LNvr6U8n7C");
    const rayUsdcV1LpTokenAccount = new anchor.web3.PublicKey("E8gJAEcHDB4be9sCKSytLUyBe3V5SEDHgn4192REJhaB");
    const rayUSdcV1OldInfoAccount = new anchor.web3.PublicKey("8vnMSWpzW2RVdAeMaqXKGbQ3r11ijf6vrCm28Ks1bXRA");
    let rayUsdcV1VaultUserRewardATokenAccount;
    let rayUsdcV1VaultUserRewardBTokenAccount;
    let userBalanceAccount;
    let userBalanceMetadataAccount;
    const programId = program.programId;
    it("prepares test data", () => __awaiter(void 0, void 0, void 0, function* () {
        let [_rayUsdcV1VaultUserRewardATokenAccount, _nonce1] = yield (0, utils_1.findVaultRewardAAccount)(rayUsdcV1VaultAccount, v1RaydiumVaultsProgram);
        let [_rayUsdcV1VaultUserRewardBTokenAccount, _nonce2] = yield (0, utils_1.findVaultRewardBAccount)(rayUsdcV1VaultAccount, v1RaydiumVaultsProgram);
        rayUsdcV1VaultUserRewardATokenAccount = _rayUsdcV1VaultUserRewardATokenAccount;
        rayUsdcV1VaultUserRewardBTokenAccount = _rayUsdcV1VaultUserRewardBTokenAccount;
        let [_userBalanceAccount, _nonce3] = yield (0, utils_1.findVaultBalanceAccount)(rayUSdcV1OldInfoAccount, provider.wallet.publicKey, v1RaydiumVaultsProgram);
        let [_userBalanceMetadata, _nonce4] = yield (0, utils_1.findVaultBalanceMetadataAccount)(_userBalanceAccount, provider.wallet.publicKey, v1RaydiumVaultsProgram);
        userBalanceAccount = _userBalanceAccount;
        userBalanceMetadataAccount = _userBalanceMetadata;
    }));
    it("deposits into ray-usdc vault", () => __awaiter(void 0, void 0, void 0, function* () {
        console.log("depositing into ray-usdc v1 raydium vault");
        const tx = program.rpc.depositRaydiumVaultV1(one, {
            accounts: {
                authority: provider.wallet.publicKey,
                authorityTokenAccount: yourRayUsdcLpTokenAccount,
                vaultPdaAccount: rayUsdcV1VaultPda,
                vault: rayUsdcV1VaultAccount,
                lpTokenAccount: rayUsdcV1LpTokenAccount,
                userBalanceAccount: userBalanceAccount,
                systemProgram: anchor.web3.SystemProgram.programId,
                stakeProgramId: raydiumStakeProgramId,
                poolId: rayUsdcPoolId,
                poolAuthority: v1RayUsdcPoolAuthority,
                userInfoAccount: rayUSdcV1OldInfoAccount,
                poolLpTokenAccount: v1RayUsdcPoolLpTokenAccount,
                userRewardATokenAccount: rayUsdcV1VaultUserRewardATokenAccount,
                poolRewardATokenAccount: v1RayUsdcPoolRewardATokenAccount,
                userRewardBTokenAccount: rayUsdcV1VaultUserRewardBTokenAccount,
                poolRewardBTokenAccount: v1RayUsdcPoolRewardBTokenAccount,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                tokenProgramId: splToken.TOKEN_PROGRAM_ID,
                userBalanceMetadata: userBalanceMetadataAccount,
            }
        });
        console.log("sent ray-usdc v1 deposit vault tx ", tx);
    }));
});
const timer = ms => new Promise(res => setTimeout(res, ms));
function freeze(time) {
    const stop = new Date().getTime() + time;
    while (new Date().getTime() < stop)
        ;
}
