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
exports.findVaultRewardBAccount = exports.findVaultRewardAAccount = exports.findVaultRewardAccount = exports.findVaultBalanceMetadataAccount = exports.findVaultBalanceAccount = exports.findUserPositionInfoAddress = exports.findUserFArmObligationVaultAddress = exports.findUserFarmAddress = exports.findUserFarmObligationAddress = exports.sleep = exports.deriveSunnyVaultAddress = exports.deriveQuarryVaultConfigDataAddress = exports.deriveQuarryVaultRewardTokenAccount = exports.deriveQuarryMinerAddress = exports.deriveOrcaDDWithdrawQueueAddress = exports.deriveEphemeralTrackingAddress = exports.deriveMultiDepositStateTransitionAddress = exports.deriveTrackingOrcaDDQueueAddress = exports.deriveOrcaDDCompoundQueueAddress = exports.deriveOrcaUserFarmAddress = exports.deriveMangoAccountAddress = exports.deriveLendingPlatformConfigDataAddress = exports.deriveLndingPlatformInformationAccountAddress = exports.deriveLendingPlatformAccountAddress = exports.findAssociatedStakeInfoAddress = exports.createAssociatedTokenAccount = exports.deriveTrackingQueueAddress = exports.deriveTrackingPdaAddress = exports.deriveTrackingAddress = exports.deriveSerumFeeRecipientAddress = exports.deriveSerumTradeOpenOrdersAddress = exports.deriveSerumTradePdaAddress = exports.deriveSerumTradeAccount = exports.deriveCompoundQueueAddress = exports.deriveWithdrawQueueAddress = exports.deriveRaydiumUserStakeInfoAddress = exports.deriveSharesMintAddress = exports.deriveVaultPdaAddress = exports.deriveVaultAddress = exports.deriveManagementAddress = void 0;
const anchor = __importStar(require("@project-serum/anchor"));
const serumAssoToken = __importStar(require("@project-serum/associated-token"));
const spl_token_1 = require("@solana/spl-token");
function deriveManagementAddress(programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([programId.toBuffer(), Buffer.from("management")], programId);
    });
}
exports.deriveManagementAddress = deriveManagementAddress;
function deriveVaultAddress(programId, farmPartOne, farmPartTwo, tag) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([
            Buffer.from("v1"),
            farmPartOne.toArrayLike(Buffer, "le", 8),
            farmPartTwo.toArrayLike(Buffer, "le", 8),
            tag,
        ], programId);
    });
}
exports.deriveVaultAddress = deriveVaultAddress;
function deriveVaultPdaAddress(programId, vault) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([vault.toBuffer()], programId);
    });
}
exports.deriveVaultPdaAddress = deriveVaultPdaAddress;
function deriveSharesMintAddress(programId, vault, mint) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([vault.toBuffer(), mint.toBuffer()], programId);
    });
}
exports.deriveSharesMintAddress = deriveSharesMintAddress;
/// derives the address of a raydium vault user stake info address
function deriveRaydiumUserStakeInfoAddress(programId, vaultPda) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("info"), vaultPda.toBuffer()], programId);
    });
}
exports.deriveRaydiumUserStakeInfoAddress = deriveRaydiumUserStakeInfoAddress;
/// derives the address of a vault withdraw queue
function deriveWithdrawQueueAddress(programId, vault, underlyingMint) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("withdrawqueue"), vault.toBuffer(), underlyingMint.toBuffer()], programId);
    });
}
exports.deriveWithdrawQueueAddress = deriveWithdrawQueueAddress;
/// derives the address of a vault compound queue
function deriveCompoundQueueAddress(programId, vault, underlyingMint) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("compoundqueue"), vault.toBuffer(), underlyingMint.toBuffer()], programId);
    });
}
exports.deriveCompoundQueueAddress = deriveCompoundQueueAddress;
function deriveSerumTradeAccount(programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("srm")], programId);
    });
}
exports.deriveSerumTradeAccount = deriveSerumTradeAccount;
function deriveSerumTradePdaAddress(programId, tradeAccount) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([tradeAccount.toBuffer()], programId);
    });
}
exports.deriveSerumTradePdaAddress = deriveSerumTradePdaAddress;
function deriveSerumTradeOpenOrdersAddress(programId, tradeAccount, serumMarket) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([tradeAccount.toBuffer(), serumMarket.toBuffer()], programId);
    });
}
exports.deriveSerumTradeOpenOrdersAddress = deriveSerumTradeOpenOrdersAddress;
function deriveSerumFeeRecipientAddress(programId, mint, tradePda) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("serumfee"), tradePda.toBuffer(), mint.toBuffer()], programId);
    });
}
exports.deriveSerumFeeRecipientAddress = deriveSerumFeeRecipientAddress;
function deriveTrackingAddress(programId, vault, owner) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("tracking"), vault.toBuffer(), owner.toBuffer()], programId);
    });
}
exports.deriveTrackingAddress = deriveTrackingAddress;
function deriveTrackingPdaAddress(programId, trackingAddress) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([trackingAddress.toBuffer()], programId);
    });
}
exports.deriveTrackingPdaAddress = deriveTrackingPdaAddress;
function deriveTrackingQueueAddress(programId, trackingPdaAddress) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("queue"), trackingPdaAddress.toBuffer()], programId);
    });
}
exports.deriveTrackingQueueAddress = deriveTrackingQueueAddress;
function createAssociatedTokenAccount(provider, // payer
owner, mint) {
    return __awaiter(this, void 0, void 0, function* () {
        let tx = new anchor.web3.Transaction();
        tx.add(yield serumAssoToken.createAssociatedTokenAccount(provider.wallet.publicKey, owner, mint));
        yield provider.sendAll([{ tx }]);
        let acct = yield serumAssoToken.getAssociatedTokenAddress(owner, mint);
        return acct;
    });
}
exports.createAssociatedTokenAccount = createAssociatedTokenAccount;
function findAssociatedStakeInfoAddress(poolId, walletAddress, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        let [_associatedStakerSeed, _associatedStakerSeedNonce] = yield anchor.web3.PublicKey.findProgramAddress([
            poolId.toBuffer(),
            walletAddress.toBuffer(),
            Buffer.from("staker_info_v2_associated_seed"),
        ], programId);
        return _associatedStakerSeed;
    });
}
exports.findAssociatedStakeInfoAddress = findAssociatedStakeInfoAddress;
function deriveLendingPlatformAccountAddress(vaultPda, lendingMarket, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([vaultPda.toBuffer(), lendingMarket.toBuffer()], programId);
    });
}
exports.deriveLendingPlatformAccountAddress = deriveLendingPlatformAccountAddress;
function deriveLndingPlatformInformationAccountAddress(vault, index, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([vault.toBuffer(), index.toArrayLike(Buffer, "le", 8)], programId);
    });
}
exports.deriveLndingPlatformInformationAccountAddress = deriveLndingPlatformInformationAccountAddress;
function deriveLendingPlatformConfigDataAddress(platformAddress, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("v1"), platformAddress.toBuffer()], programId);
    });
}
exports.deriveLendingPlatformConfigDataAddress = deriveLendingPlatformConfigDataAddress;
function deriveMangoAccountAddress(vault, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([vault.toBuffer(), Buffer.from("mango")], programId);
    });
}
exports.deriveMangoAccountAddress = deriveMangoAccountAddress;
function deriveOrcaUserFarmAddress(globalFarm, owner, aquaFarmProgramId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([globalFarm.toBuffer(), owner.toBuffer(), spl_token_1.TOKEN_PROGRAM_ID.toBuffer()], aquaFarmProgramId);
    });
}
exports.deriveOrcaUserFarmAddress = deriveOrcaUserFarmAddress;
/// derives the address of an orca double dip vault compound queue account
function deriveOrcaDDCompoundQueueAddress(programId, vault, ddFarmTokenMint) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([
            Buffer.from("ddcompoundqueue"),
            vault.toBuffer(),
            ddFarmTokenMint.toBuffer(),
        ], programId);
    });
}
exports.deriveOrcaDDCompoundQueueAddress = deriveOrcaDDCompoundQueueAddress;
function deriveTrackingOrcaDDQueueAddress(programId, vault, trackingPda) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("ddwithdrawqueue"), trackingPda.toBuffer(), vault.toBuffer()], programId);
    });
}
exports.deriveTrackingOrcaDDQueueAddress = deriveTrackingOrcaDDQueueAddress;
function deriveMultiDepositStateTransitionAddress(vault, progrmaId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("statetransition"), vault.toBuffer()], progrmaId);
    });
}
exports.deriveMultiDepositStateTransitionAddress = deriveMultiDepositStateTransitionAddress;
function deriveEphemeralTrackingAddress(vault, owner, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("ephemeraltracking"), vault.toBuffer(), owner.toBuffer()], programId);
    });
}
exports.deriveEphemeralTrackingAddress = deriveEphemeralTrackingAddress;
function deriveOrcaDDWithdrawQueueAddress(vault, farmTokenMint, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([
            Buffer.from("ddwithdrawqueue"),
            vault.toBuffer(),
            farmTokenMint.toBuffer(),
        ], programId);
    });
}
exports.deriveOrcaDDWithdrawQueueAddress = deriveOrcaDDWithdrawQueueAddress;
function deriveQuarryMinerAddress(quarry, authority, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("Miner"), quarry.toBuffer(), authority.toBuffer()], programId);
    });
}
exports.deriveQuarryMinerAddress = deriveQuarryMinerAddress;
function deriveQuarryVaultRewardTokenAccount(vault, 
// the reward token mint
mint, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("reward"), vault.toBuffer(), mint.toBuffer()], programId);
    });
}
exports.deriveQuarryVaultRewardTokenAccount = deriveQuarryVaultRewardTokenAccount;
function deriveQuarryVaultConfigDataAddress(vault, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("config"), vault.toBuffer()], programId);
    });
}
exports.deriveQuarryVaultConfigDataAddress = deriveQuarryVaultConfigDataAddress;
function deriveSunnyVaultAddress(quarry, vaultPda, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield anchor.web3.PublicKey.findProgramAddress([Buffer.from("SunnyQuarryVault"), quarry.toBuffer(), vaultPda.toBuffer()], programId);
    });
}
exports.deriveSunnyVaultAddress = deriveSunnyVaultAddress;
function sleep(ms) {
    return __awaiter(this, void 0, void 0, function* () {
        return new Promise(res => setTimeout(res, ms));
    });
}
exports.sleep = sleep;
function findUserFarmObligationAddress(authority, userFarmAddr, programId, obligationIndex) {
    return __awaiter(this, void 0, void 0, function* () {
        let seeds = [
            authority.toBuffer(),
            userFarmAddr.toBuffer(),
            obligationIndex.toArrayLike(Buffer, "le", 8),
        ];
        return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
    });
}
exports.findUserFarmObligationAddress = findUserFarmObligationAddress;
// finds a UserFarm account address 
function findUserFarmAddress(
// the user's main wallet account
authority, 
// the id of the lending program
programId, 
// the index of the account
// 0 = first account, 1 = second account, etc...
index, 
// the enum of the particular farm
// 0 = ray-usdc lp, 1 = ray-sol lp
farm) {
    return __awaiter(this, void 0, void 0, function* () {
        let seeds = [
            authority.toBuffer(),
            index.toArrayLike(Buffer, "le", 8),
            farm.toArrayLike(Buffer, "le", 8),
        ];
        return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
    });
}
exports.findUserFarmAddress = findUserFarmAddress;
function findUserFArmObligationVaultAddress(userFarmAccount, obligationIndex, farmProgramId) {
    return __awaiter(this, void 0, void 0, function* () {
        let seeds = [
            userFarmAccount.toBuffer(),
            obligationIndex.toArrayLike(Buffer, "le", 8),
        ];
        return anchor.web3.PublicKey.findProgramAddress(seeds, farmProgramId);
    });
}
exports.findUserFArmObligationVaultAddress = findUserFArmObligationVaultAddress;
function findUserPositionInfoAddress(userFarmAddress, programId, obligationIndex) {
    return __awaiter(this, void 0, void 0, function* () {
        let seeds = [
            Buffer.from("position_info"),
            userFarmAddress.toBuffer(),
            obligationIndex.toArrayLike(Buffer, "le", 8),
        ];
        return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
    });
}
exports.findUserPositionInfoAddress = findUserPositionInfoAddress;
function findVaultBalanceAccount(vaultInfoAccount, obligationVaultAccount, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        let seeds = [
            vaultInfoAccount.toBuffer(),
            obligationVaultAccount.toBuffer(),
        ];
        return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
    });
}
exports.findVaultBalanceAccount = findVaultBalanceAccount;
function findVaultBalanceMetadataAccount(vaultBalanceAccount, obligationVaultAccount, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        let seeds = [
            vaultBalanceAccount.toBuffer(),
            obligationVaultAccount.toBuffer(),
        ];
        return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
    });
}
exports.findVaultBalanceMetadataAccount = findVaultBalanceMetadataAccount;
function findVaultRewardAccount(vaultBalanceMetadataAccount, obligationVaultAccount, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        let seeds = [
            vaultBalanceMetadataAccount.toBuffer(),
            obligationVaultAccount.toBuffer(),
        ];
        return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
    });
}
exports.findVaultRewardAccount = findVaultRewardAccount;
function findVaultRewardAAccount(vault, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        let seeds = [
            vault.toBuffer(),
            Buffer.from("reward"),
        ];
        return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
    });
}
exports.findVaultRewardAAccount = findVaultRewardAAccount;
function findVaultRewardBAccount(vault, programId) {
    return __awaiter(this, void 0, void 0, function* () {
        let seeds = [
            vault.toBuffer(),
            Buffer.from("rewardb")
        ];
        return anchor.web3.PublicKey.findProgramAddress(seeds, programId);
    });
}
exports.findVaultRewardBAccount = findVaultRewardBAccount;
