import * as anchor from "@project-serum/anchor";
import * as serumAssoToken from "@project-serum/associated-token";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

export async function deriveManagementAddress(
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [programId.toBuffer(), Buffer.from("management")],
    programId
  );
}

export async function deriveVaultAddress(
  programId: anchor.web3.PublicKey,
  farmPartOne: anchor.BN,
  farmPartTwo: anchor.BN,
  tag: Uint8Array
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("v1"),
      farmPartOne.toArrayLike(Buffer, "le", 8),
      farmPartTwo.toArrayLike(Buffer, "le", 8),
      tag,
    ],
    programId
  );
}

export async function deriveVaultPdaAddress(
  programId: anchor.web3.PublicKey,
  vault: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [vault.toBuffer()],
    programId
  );
}

export async function deriveSharesMintAddress(
  programId: anchor.web3.PublicKey,
  vault: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [vault.toBuffer(), mint.toBuffer()],
    programId
  );
}

/// derives the address of a raydium vault user stake info address
export async function deriveRaydiumUserStakeInfoAddress(
  programId: anchor.web3.PublicKey,
  vaultPda: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("info"), vaultPda.toBuffer()],
    programId
  );
}

/// derives the address of a vault withdraw queue
export async function deriveWithdrawQueueAddress(
  programId: anchor.web3.PublicKey,
  vault: anchor.web3.PublicKey,
  underlyingMint: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("withdrawqueue"), vault.toBuffer(), underlyingMint.toBuffer()],
    programId
  );
}

/// derives the address of a vault compound queue
export async function deriveCompoundQueueAddress(
  programId: anchor.web3.PublicKey,
  vault: anchor.web3.PublicKey,
  underlyingMint: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("compoundqueue"), vault.toBuffer(), underlyingMint.toBuffer()],
    programId
  );
}

export async function deriveSerumTradeAccount(
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("srm")],
    programId
  );
}

export async function deriveSerumTradePdaAddress(
  programId: anchor.web3.PublicKey,
  tradeAccount: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [tradeAccount.toBuffer()],
    programId
  );
}

export async function deriveSerumTradeOpenOrdersAddress(
  programId: anchor.web3.PublicKey,
  tradeAccount: anchor.web3.PublicKey,
  serumMarket: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [tradeAccount.toBuffer(), serumMarket.toBuffer()],
    programId
  );
}

export async function deriveSerumFeeRecipientAddress(
  programId: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey,
  tradePda: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("serumfee"), tradePda.toBuffer(), mint.toBuffer()],
    programId
  );
}

export async function deriveTrackingAddress(
  programId: anchor.web3.PublicKey,
  vault: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("tracking"), vault.toBuffer(), owner.toBuffer()],
    programId
  );
}

export async function deriveTrackingPdaAddress(
  programId: anchor.web3.PublicKey,
  trackingAddress: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [trackingAddress.toBuffer()],
    programId
  );
}

export async function deriveTrackingQueueAddress(
  programId: anchor.web3.PublicKey,
  trackingPdaAddress: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("queue"), trackingPdaAddress.toBuffer()],
    programId
  );
}

export async function createAssociatedTokenAccount(
  provider: anchor.AnchorProvider, // payer
  owner: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey
): Promise<anchor.web3.PublicKey> {
  let tx = new anchor.web3.Transaction();
  tx.add(
    await serumAssoToken.createAssociatedTokenAccount(
      provider.wallet.publicKey,
      owner,
      mint
    )
  );
  await provider.sendAll([{ tx }]);
  let acct = await serumAssoToken.getAssociatedTokenAddress(owner, mint);
  return acct;
}

export async function findAssociatedStakeInfoAddress(
  poolId: anchor.web3.PublicKey,
  walletAddress: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<anchor.web3.PublicKey> {
  let [_associatedStakerSeed, _associatedStakerSeedNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        poolId.toBuffer(),
        walletAddress.toBuffer(),
        Buffer.from("staker_info_v2_associated_seed"),
      ],
      programId
    );
  return _associatedStakerSeed;
}

export async function deriveLendingPlatformAccountAddress(
  vaultPda: anchor.web3.PublicKey,
  lendingMarket: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [vaultPda.toBuffer(), lendingMarket.toBuffer()],
    programId
  );
}

export async function deriveLndingPlatformInformationAccountAddress(
  vault: anchor.web3.PublicKey,
  index: anchor.BN,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [vault.toBuffer(), index.toArrayLike(Buffer, "le", 8)],
    programId
  );
}

export async function deriveLendingPlatformConfigDataAddress(
  platformAddress: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("v1"), platformAddress.toBuffer()],
    programId
  );
}

export async function deriveMangoAccountAddress(
  vault: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [vault.toBuffer(), Buffer.from("mango")],
    programId
  );
}

export async function deriveOrcaUserFarmAddress(
  globalFarm: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey,
  aquaFarmProgramId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [globalFarm.toBuffer(), owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer()],
    aquaFarmProgramId
  );
}

/// derives the address of an orca double dip vault compound queue account
export async function deriveOrcaDDCompoundQueueAddress(
  programId: anchor.web3.PublicKey,
  vault: anchor.web3.PublicKey,
  ddFarmTokenMint: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("ddcompoundqueue"),
      vault.toBuffer(),
      ddFarmTokenMint.toBuffer(),
    ],
    programId
  );
}

export async function deriveTrackingOrcaDDQueueAddress(
  programId: anchor.web3.PublicKey,
  vault: anchor.web3.PublicKey,
  trackingPda: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("ddwithdrawqueue"), trackingPda.toBuffer(), vault.toBuffer()],
    programId
  );
}

export async function deriveMultiDepositStateTransitionAddress(
  vault: anchor.web3.PublicKey,
  progrmaId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("statetransition"), vault.toBuffer()],
    progrmaId
  );
}

export async function deriveEphemeralTrackingAddress(
  vault: anchor.web3.PublicKey,
  owner: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("ephemeraltracking"), vault.toBuffer(), owner.toBuffer()],
    programId
  );
}

export async function deriveOrcaDDWithdrawQueueAddress(
  vault: anchor.web3.PublicKey,
  farmTokenMint: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("ddwithdrawqueue"),
      vault.toBuffer(),
      farmTokenMint.toBuffer(),
    ],
    programId
  );
}

export async function deriveQuarryMinerAddress(
  quarry: anchor.web3.PublicKey,
  authority: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("Miner"), quarry.toBuffer(), authority.toBuffer()],
    programId
  );
}

export async function deriveQuarryVaultRewardTokenAccount(
  vault: anchor.web3.PublicKey,
  // the reward token mint
  mint: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("reward"), vault.toBuffer(), mint.toBuffer()],
    programId
  );
}

export async function deriveQuarryVaultConfigDataAddress(
  vault: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("config"), vault.toBuffer()],
    programId
  );
}

export async function deriveSunnyVaultAddress(
  quarry: anchor.web3.PublicKey,
  vaultPda: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey
): Promise<[anchor.web3.PublicKey, number]> {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("SunnyQuarryVault"), quarry.toBuffer(), vaultPda.toBuffer()],
    programId
  );
}

export async function sleep(ms) {
  return new Promise( res => setTimeout(res, ms))
}


