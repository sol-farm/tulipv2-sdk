import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Examples } from "../target/types/examples";
import * as BufferLayout from "buffer-layout";
import * as serumAssoToken from "@project-serum/associated-token";
import * as splToken from "@solana/spl-token";
import * as assert from "assert";

import {
  createAssociatedTokenAccount,
  deriveMultiDepositStateTransitionAddress,
  deriveOrcaDDCompoundQueueAddress,
  deriveTrackingOrcaDDQueueAddress,
  deriveOrcaDDWithdrawQueueAddress,
  deriveManagementAddress,
  deriveRaydiumUserStakeInfoAddress,
  deriveSharesMintAddress,
  deriveVaultAddress,
  deriveVaultPdaAddress,
  deriveWithdrawQueueAddress,
  deriveCompoundQueueAddress,
  deriveSerumTradeAccount,
  deriveSerumTradePdaAddress,
  deriveSerumTradeOpenOrdersAddress,
  deriveSerumFeeRecipientAddress,
  deriveTrackingAddress,
  deriveTrackingPdaAddress,
  deriveTrackingQueueAddress,
  findAssociatedStakeInfoAddress,
  deriveLendingPlatformAccountAddress,
  deriveLndingPlatformInformationAccountAddress,
  deriveLendingPlatformConfigDataAddress,
  deriveMangoAccountAddress,
  deriveOrcaUserFarmAddress,
  deriveEphemeralTrackingAddress,
  deriveQuarryVaultConfigDataAddress,
  deriveSunnyVaultAddress,
  sleep,
  findUserFarmAddress,
  findUserFArmObligationVaultAddress,
  findUserFarmObligationAddress,
} from "./utils";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";

const usdcv1Vault = new anchor.web3.PublicKey(
  "3wPiV9inTGexMZjp6x5Amqwp2sRNtpSheG8Hbv2rgq8W"
);
const usdcv1VaultPda = new anchor.web3.PublicKey(
  "14fdy6YXbhDgnVQz4VcgSGgUcZ35eE48SKDrfqF87NUP"
);
const usdcv1SharesMint = new anchor.web3.PublicKey(
  "Cvvh8nsKZet59nsDDo3orMa3rZnPWQhpgrMCVcRDRgip"
);
const usdcv1DepositQueue = new anchor.web3.PublicKey(
  "36KtHLHxcGnrfEb2GLwPcbN9nHUkeoi3gd6rMQj8wwVj"
);
const usdcv1MangoVaultSharesAccount = new anchor.web3.PublicKey(
  "A9kM8NKf3v29F3DgRQ5Rw7TJoadFZZDfBGLRBGNzASrr"
);
const usdcv1TulipVaultSharesAccount = new anchor.web3.PublicKey(
  "M7akLS7xYVhp68LnMkBBCemvqkGcCycQ3qp7e3SsKR1"
);
const usdcv1SolendVaultSharesAccount = new anchor.web3.PublicKey(
  "UxVNZzzx3xRxKFAuGjRQgbDaa7mirSkFEAu7qiYQ1h4"
);
const usdcv1WithdrawQueue = new anchor.web3.PublicKey(
  "HLVcpKPkBJJJGTHTSaZcAixDppy4R65x1is3k8Q7qZpj"
);

const usdcTokenMint = new anchor.web3.PublicKey(
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
);
const associatedTokenProgramId = new anchor.web3.PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
);
const v2VaultsProgramId = new anchor.web3.PublicKey(
  "TLPv2tuSVvn3fSk8RgW3yPddkp5oFivzZV3rA9hQxtX"
);

const mangoVault = new anchor.web3.PublicKey(
  "ZH9GWNBtwxcWeU9kHk77DSciwQnoJcSm8VVvYfmHXfe"
);
const mangoVaultPda = new anchor.web3.PublicKey(
  "Dhry4TVd862yzcAuxFZgiuh6juS4R6FesfRZkWG3pCe7"
);
const mangoVaultSharesMint = new anchor.web3.PublicKey(
  "5u6jxB7En2LF5aroeq8U5JUbnHa5WSYB5JTemh3gYaMj"
);
const mangoVaultPlatformInformation = new anchor.web3.PublicKey(
  "GDzbqzebKTxJaQVfxfqejkqU4HLcBmgNBwun3rADvm8J"
);
const mangoVaultPlatformConfigDataAccount = new anchor.web3.PublicKey(
  "EecDX1xHrjKXQWbE5WtwU7fAiEkKxAv7w196oe8jaoqY"
);
const mangoVaultDepositQueue = new anchor.web3.PublicKey(
  "3CnAyCjpA9mcxayed12cwJNGue7YbmyuBjpT9KN6meVT"
);
const mangoVaultMangoAccount = new anchor.web3.PublicKey(
  "3cZkd5eVyZhMhE8nJcR3rA7GgVQ6gCJt2qofr2GQd8ca"
);
const mangoCache = new anchor.web3.PublicKey(
  "EBDRoayCDDUvDgCimta45ajQeXbexv7aKqJubruqpyvu"
);
const mangoRootBank = new anchor.web3.PublicKey(
  "AMzanZxMirPCgGcBoH9kw4Jzi9LFMomyUCXbpzDeL2T8"
);
const mangoNodeBank = new anchor.web3.PublicKey(
  "BGcwkj1WudQwUUjFk78hAjwd1uAm8trh1N4CJSa51euh"
);
const mangoGroupAccount = new anchor.web3.PublicKey(
  "98pjRuQjK3qA6gXts96PqZT4Ze5QmnCmt3QYjhbUSPue"
);
const mangoTokenAccount = new anchor.web3.PublicKey(
  "8Vw25ZackDzaJzzBBqcgcpDsCsDfRSkMGgwFQ3gbReWF"
);
const mangoGroupSigner = new anchor.web3.PublicKey(
  "9BVcYqEQxyccuwznvxXqDkSJFavvTyheiTYk231T1A8S"
);
const mangoProgramId = new anchor.web3.PublicKey(
  "mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68"
);

const solendVault = new anchor.web3.PublicKey(
  "85JXjDiyianDpvz8y8efkRyFsxpnSJJpmyxrJ7bncKHM"
);
const solendVaultPda = new anchor.web3.PublicKey(
  "963HGaUjwGqvqwwqwJZayUXvCC21AAqZU5SLw1kU4gVc"
);
const solendVaultSharesMint = new anchor.web3.PublicKey(
  "CS8cJicaSpphTTboMJD1UeNpU7vpkZc86vKrtqzRVnG5"
);
const solendVaultPlatformInformation = new anchor.web3.PublicKey(
  "GRL5rtnvzCfQRdKJXkG2A8LvDSNXkbxENEnF4SwJ3pTF"
);
const solendVaultPlatformConfigDataAccount = new anchor.web3.PublicKey(
  "6AzcPNmNWomkdMRgcZJPVAs4zF1jev9wqxzzzxVzDDsi"
);
const solendVaultDepositQueue = new anchor.web3.PublicKey(
  "2Li9Q5Vx9BEnFTGJTLWc5pVqerYGjhgyGYzSAA2WhYKQ"
);
const solendReserveAccount = new anchor.web3.PublicKey(
  "BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw"
);
const solendReserveLiquiditySupply = new anchor.web3.PublicKey(
  "8SheGtsopRUDzdiD6v6BR9a6bqZ9QwywYQY99Fp5meNf"
);
const solendReserveCollateralMint = new anchor.web3.PublicKey(
  "993dVFL2uXWYeoXuEBFXR4BijeXdTv4s6BzsCjJZuwqk"
);
const solendLendingMarketAccount = new anchor.web3.PublicKey(
  "4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY"
);
const solendDerivedLendingMarketAuthority = new anchor.web3.PublicKey(
  "DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby"
);
const solendReservePythPriceAccount = new anchor.web3.PublicKey(
  "Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD"
);
const solendReserveSwitchboardPriceAccount = new anchor.web3.PublicKey(
  "CZx29wKMUxaJDq6aLVQTdViPL754tTR64NAgQBUGxxHb"
);
const solendProgramId = new anchor.web3.PublicKey(
  "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo"
);

const tulipVault = new anchor.web3.PublicKey(
  "8KLrrsnUv3DjC9Q89xSQDVdiGLZHUEUuyPedfHrtuVRr"
);
const tulipVaultPda = new anchor.web3.PublicKey(
  "mrT9Q45iuC2HLYxpceaQFjzcAgd6Trks7bXAGbKYpwL"
);
const tulipVaultSharesMint = new anchor.web3.PublicKey(
  "D2PLcwGR1wsXUPhb1BHysSVEsHVVCQ129qGpgXXaTNR1"
);
const tulipVaultPlatformInformation = new anchor.web3.PublicKey(
  "4QVuedVSCMLA3eQ643czUy95uQFxXKAcCMJ1ChpkVA2B"
);
const tulipVaultPlatformConfigDataAccount = new anchor.web3.PublicKey(
  "7XTtoiHfkYzjLDxKDMoaVYncmdBW1yLfphmisSbBrnuZ"
);
const tulipVaultDepositQueue = new anchor.web3.PublicKey(
  "8eDHmS15CWd8d88uckg6oKxrfwijZVudZsDgdtgGqS49"
);
const tulipReserveAccount = new anchor.web3.PublicKey(
  "FTkSmGsJ3ZqDSHdcnY7ejN1pWV3Ej7i88MYpZyyaqgGt"
);
const tulipReserveLiquiditySupply = new anchor.web3.PublicKey(
  "64QJd6MYXUjCBvCaZKaqxiKmaMkPUdNonE1KuY1YoGGb"
);
const tulipReserveCollateralMint = new anchor.web3.PublicKey(
  "Amig8TisuLpzun8XyGfC5HJHHGUQEscjLgoTWsCCKihg"
);
const tulipLendingMarketAccount = new anchor.web3.PublicKey(
  "D1cqtVThyebK9KXKGXrCEuiqaNf5L4UfM1vHgCqiJxym"
);
const tulipDerivedLendingMarketAuthority = new anchor.web3.PublicKey(
  "8gEGZbUfVE1poBq71VHKX9LU7ca4x8wTUyZgcbyQe51s"
);
const tulipReservePythPriceAccount = new anchor.web3.PublicKey(
  "ExzpbWgczTgd8J58BrnESndmzBkRVfc6PhFjSGiQXgAB"
);
const tulipLendingProgramId = new anchor.web3.PublicKey(
  "4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM"
);
const solendCollateralTokenAccount = new anchor.web3.PublicKey(
  "6EaiG2gRVu9u7QzVmX59AWLSmiaEYvMrKWQfPMCgNxsZ"
);
const tulipCollateralTokenAccount = new anchor.web3.PublicKey(
  "2U6kk4iTVqeypBydVPKA8mLTLAQEBfWf4KYfmkcvomPE"
)
const tulipLeveragedFarmProgramId = new anchor.web3.PublicKey(
  "Bt2WPMmbwHPk36i4CRucNDyLcmoGdC7xEdrVuxgJaNE6"
)
const tulipLevFarmGlobalAccount = new anchor.web3.PublicKey(
  "HLuVf6p3SqgEKy8poYA6g26CDGuQddcbETmf8VdJKqjF"
)
const tulipRayUsdcLevFarmAccount = new anchor.web3.PublicKey(
  "84ayseJgpJavzfeESgRdyfMoDo2bs4J2YUBjMT4iTs66"
)
const rayUsdcLpTokenMint = new anchor.web3.PublicKey("FbC6K13MzHvN42bXrtGaWsvZY9fxrackRSZcBGfjPc7m")
const rayTokenMint = new anchor.web3.PublicKey("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R");

describe("examples", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Examples as Program<Examples>;

  const programId = program.programId;

  let depositTrackingAccount: anchor.web3.PublicKey;
  let depositTrackingPda: anchor.web3.PublicKey;
  let depositTrackingQueueAccount: anchor.web3.PublicKey;
  let depositTrackingHoldAccount: anchor.web3.PublicKey;

  let yourUnderlyingTokenAccount: anchor.web3.PublicKey;
  let yourSharesTokenAccount: anchor.web3.PublicKey;

  let nine = new anchor.BN(9).mul(new anchor.BN(10).pow(new anchor.BN(6)));
  it("registers deposit tracking account", async () => {
    console.log("progrmaId ", programId);
    console.log("usdcv1 vault ", usdcv1Vault);
    console.log("provider", provider.wallet.publicKey);
    let [_depositTrackingAccount, _trackingNonce] = await deriveTrackingAddress(
      v2VaultsProgramId,
      usdcv1Vault,
      provider.wallet.publicKey
    );
    depositTrackingAccount = _depositTrackingAccount;
    let [_depositTrackingPda, _depositTrackingPdaNonce] =
      await deriveTrackingPdaAddress(v2VaultsProgramId, depositTrackingAccount);
    depositTrackingPda = _depositTrackingPda;
    let [_depositTrackingQueueAccount, _queueNonce] =
      await deriveTrackingQueueAddress(v2VaultsProgramId, depositTrackingPda);
    depositTrackingQueueAccount = _depositTrackingQueueAccount;
    depositTrackingHoldAccount = await serumAssoToken.getAssociatedTokenAddress(
      depositTrackingPda,
      usdcv1SharesMint
    );
    console.log(
      "deposit tracking queue",
      depositTrackingQueueAccount.toString()
    );
    console.log("deposit tracking hold", depositTrackingHoldAccount.toString());
    console.log("deposit tracking pda", depositTrackingPda.toString());
    console.log("deposit tracking", depositTrackingAccount.toString());
    const authority = provider.wallet;
    console.log("sending register deposit tracking account tx");
    let tx = await program.rpc.registerDepositTrackingAccount(
      [new anchor.BN(1), new anchor.BN(65537)],
      {
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
      }
    );
    console.log("sent register deposit tracking account tx ", tx);
  });
  it("issues shares", async () => {
    yourUnderlyingTokenAccount = await serumAssoToken.getAssociatedTokenAddress(
      provider.wallet.publicKey,
      usdcTokenMint
    );
    yourSharesTokenAccount = await createAssociatedTokenAccount(
      provider,
      provider.wallet.publicKey,
      usdcv1SharesMint
    );
    let tx = await program.rpc.issueShares(
      nine,
      [new anchor.BN(1), new anchor.BN(65537)],
      {
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
      }
    );
    console.log("sent issue shares tx")
  });
  it("withdraws from deposit tracking account", async () => {
    console.log("wait 3 seconds")
    freeze(3000);
    console.log("in production this would fail, as 15 minutes need to pass before lockup is expired")
    let tx = await program.rpc.withdrawDepositTracking(
      // fixed amount we get for 10 USDC based on the program state which has been dumped to disk
      new anchor.BN(8972072),
      [new anchor.BN(1), new anchor.BN(65537)],
      {
        options: {skipPreflight: true},
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
      }
    );
    console.log("sent withdraw deposit tracking tx");
  });
  it("withdraws multi deposit vault through tulip", async () => {
    let tx = await program.rpc.withdrawMultiDepositVaultThroughTulip(
      new anchor.BN(8972072),
      {
        options: {skipPreflight: true},
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
      }
    ).then(() => {
      console.log("this instruction should not pass in localnet testing")
      process.exit(255)
    }).catch(() => {
      console.log("this test is expected to fail in localnet testing")
    });
  });
  it("withdraws multi deposit vault through mango", async () => {
    program.rpc.withdrawMultiDepositVaultThroughMango(
      new anchor.BN(8972072),
      {
        options: {skipPreflight: true},
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
      }
    ).then(() => {
      console.log("test should not pass in localnet");
      process.exit(255);
    }).catch(() => {
      console.log("due to localnet restrictions, this test is expected to fail")
    });
  });
  it("withdraws multi deposit vault through solend", async () => {
   program.rpc.withdrawMultiDepositVaultThroughSolend(
      new anchor.BN(8972072),
      {
        options: {skipPreflight: true},
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
      }
    ).catch(() => {
      console.log("test failure is expected in localnet")
    })
    .then(() => {});
  });
});
describe("test lending instructions via usdc ", async () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Examples as Program<Examples>;

  const programId = program.programId;

  let nine = new anchor.BN(9).mul(new anchor.BN(10).pow(new anchor.BN(6)));
  let one = new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(6)));

  let yourCollateralTokenAccount: anchor.web3.PublicKey;
  let yourUnderlyingTokenAccount: anchor.web3.PublicKey;

  it("gets underlying token accounts", async() => {
    yourUnderlyingTokenAccount = await serumAssoToken.getAssociatedTokenAddress(
      provider.wallet.publicKey,
      usdcTokenMint
    );
  })

  it("deposits reserve liquidity", async () => {
    yourCollateralTokenAccount = await createAssociatedTokenAccount(
      provider,
      provider.wallet.publicKey,
      tulipReserveCollateralMint,
    )
    const tx = await program.rpc.depositReserveLiquidity(one, {
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
    })
    console.log("sent deposit reserve liquidity tx ", tx);
  })
  it("redeems reserve collateral", async() => {
    const bal = await provider.connection.getTokenAccountBalance(yourCollateralTokenAccount);
    const tx = await program.rpc.redeemReserveCollateral(
      new anchor.BN(bal.value.amount),
      {
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
      }
    )
  })
})

describe("tests leverage farm instructions via ray-usdc", async () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Examples as Program<Examples>;

  const programId = program.programId;
  let userFarmAddress: anchor.web3.PublicKey;
  let userFarmObligation1VaultAddress: anchor.web3.PublicKey;
  let userFarmObligation1Address: anchor.web3.PublicKey;
  it("creates user farm", async () => {
    let [_userFarm, _userFarmNonce] = await findUserFarmAddress(
      provider.wallet.publicKey,
      tulipLeveragedFarmProgramId,
      new anchor.BN(0),
      new anchor.BN(0),
    );
    userFarmAddress = _userFarm;
    let [_userFarmObligationVault, _userFarmObligationVaultNonce] = await findUserFArmObligationVaultAddress(
      userFarmAddress,
      new anchor.BN(0),
      tulipLeveragedFarmProgramId,
    );
    userFarmObligation1VaultAddress = _userFarmObligationVault;
    let [_userFarmObligation, _userFarmObligationNonce] = await findUserFarmObligationAddress(
      provider.wallet.publicKey,
      userFarmAddress,
      tulipLeveragedFarmProgramId,
      new anchor.BN(0),
    );
    userFarmObligation1Address = _userFarmObligation;
    const tx = await program.rpc.createUserFarm(new anchor.BN(0), {
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
        userFarmLpTokenAccount: await serumAssoToken.getAssociatedTokenAddress(
          userFarmAddress,
          rayUsdcLpTokenMint,
        ),
        userFarmBaseTokenAccount: await serumAssoToken.getAssociatedTokenAddress(
          userFarmAddress,
          rayTokenMint,
        ),
        userFarmQuoteTokenAccount: await serumAssoToken.getAssociatedTokenAddress(
          userFarmAddress,
          usdcTokenMint,
        ),
        lpTokenMint: rayUsdcLpTokenMint,
        baseTokenMint: rayTokenMint,
        quoteTokenMint: usdcTokenMint,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tulipLeveragedFarmProgram: tulipLeveragedFarmProgramId,
      }
    })
    console.log("sent create user farm token account tx ", tx)
  })
  let userFarmObligation2VaultAddress: anchor.web3.PublicKey;
  let userFarmObligation2Address: anchor.web3.PublicKey;
  it("create user farm obligation", async () => {

    let [_userFarmObligationVault, _userFarmObligationVaultNonce] = await findUserFArmObligationVaultAddress(
      userFarmAddress,
      new anchor.BN(0),
      tulipLeveragedFarmProgramId,
    );
    userFarmObligation2VaultAddress = _userFarmObligationVault;
    let [_userFarmObligation, _userFarmObligationNonce] = await findUserFarmObligationAddress(
      provider.wallet.publicKey,
      userFarmAddress,
      tulipLendingProgramId,
      new anchor.BN(0),
    );
    userFarmObligation2Address = _userFarmObligation;
  })
})

const timer = ms => new Promise( res => setTimeout(res, ms));

function freeze(time) {
  const stop = new Date().getTime() + time;
  while(new Date().getTime() < stop);       
}
