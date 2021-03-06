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
} from "./utils";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
const v2VaultsProgramId = new anchor.web3.PublicKey(
  "TLPv2tuSVvn3fSk8RgW3yPddkp5oFivzZV3rA9hQxtX"
);
const rayUsdcLpTokenMint = new anchor.web3.PublicKey("FbC6K13MzHvN42bXrtGaWsvZY9fxrackRSZcBGfjPc7m")
const orcaTokenMint = new anchor.web3.PublicKey("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE");
const orcaAquaFarmProgram = new anchor.web3.PublicKey(
  "82yxjeMsvaURa4MbZZ7WZZHfobirZYkH1zF8fmeGtyaQ"
)
const atlasTokenMint = new anchor.web3.PublicKey("ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx");
let yourUsdcTokenAccount: anchor.web3.PublicKey;
let yourOrcaTokenAccount: anchor.web3.PublicKey;
describe("examples", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Examples as Program<Examples>;

  const programId = program.programId;
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
  const tulipProgramId = new anchor.web3.PublicKey(
    "4bcFeLv4nydFrsZqV5CgwCVrPhkQKsXtzfy2KyMz7ozM"
  );
  const solendCollateralTokenAccount = new anchor.web3.PublicKey(
    "6EaiG2gRVu9u7QzVmX59AWLSmiaEYvMrKWQfPMCgNxsZ"
  );
  const tulipCollateralTokenAccount = new anchor.web3.PublicKey(
    "2U6kk4iTVqeypBydVPKA8mLTLAQEBfWf4KYfmkcvomPE"
  )
  let depositTrackingAccount: anchor.web3.PublicKey;
  let depositTrackingPda: anchor.web3.PublicKey;
  let depositTrackingQueueAccount: anchor.web3.PublicKey;
  let depositTrackingHoldAccount: anchor.web3.PublicKey;

  let yourUnderlyingTokenAccount: anchor.web3.PublicKey;
  let yourSharesTokenAccount: anchor.web3.PublicKey;

  let nine = new anchor.BN(9).mul(new anchor.BN(10).pow(new anchor.BN(6)));
  let one = new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(6)));
  it("logs exchange rate", async () => {
    await program.rpc.logExchangeRate(
      [new anchor.BN(1), new anchor.BN(65537)],
      {
        accounts: {
          vault: usdcv1Vault,
          sharesMint: usdcv1SharesMint,
          vaultProgram: v2VaultsProgramId,
        }
      }
    )
  })
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
            lendingProgram: tulipProgramId,
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
  let yourCollateralTokenAccount: anchor.web3.PublicKey;
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
        lendingProgram: tulipProgramId,
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
          lendingProgram: tulipProgramId,
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
          pythPriceAccount: tulipReservePythPriceAccount,
        }
      }
    )
  })
});
describe("tests ray-usdc auto vaults", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Examples as Program<Examples>;

  const programId = program.programId;
  const rayUsdcV2Vault = new anchor.web3.PublicKey("6tkFEgE6zry2gGC4yqLrTghdqtqadyT5H3H2AJd4w5rz");
  const rayUsdcV2VaultPda = new anchor.web3.PublicKey("G6V4Kohr4PFpuLjhzNKGaTWB5p93dPS4cGe7pQ3MhiK1");
  const rayUsdcV2VaultAssociatedStakeInfo = new anchor.web3.PublicKey("HyXpbhK7aubL257mZYDbCzGbcLMjAFWiQp9XnrvrcnE8");
  const rayUsdcV2VaultSharesMint = new anchor.web3.PublicKey("9qLZgUPVe7r7YetwCWxBkY1uQAs8UNKuqQbGs3cdHYU8");
  const rayUsdcV2VaultCompoundQueue = new anchor.web3.PublicKey("GNgMgCtnYS26XSjpbu8GtS5CDjEfMcHHM3zrH6ieuvLB");
  const rayUsdcV2VaultDepositQueue = new anchor.web3.PublicKey("6iQTNqWi9EPs2ST8FqmbE1QZFy6tPLnNrnt7Z5zsrty8");
  const rayUsdcV2VaultWithdrawQueue = new anchor.web3.PublicKey("4vvCisFXys52FatAVd9aT8UafCd5zEBpkL5DTnNTVA5u");
  const rayUsdcV2VaultRewardATokenAccount = new anchor.web3.PublicKey("7DcoC6MGB6T4U5Tqwaq1qefv6JgrHuPmzeiACxyYjcYn");
  const rayUsdcV2VaultStakeInfo = new anchor.web3.PublicKey("8nUUtLjoRg1HZmLSB48keMX5Dc36HL2jJNrJ5xDxtv9g");
  let depositTrackingAccount: anchor.web3.PublicKey;
  let depositTrackingPda: anchor.web3.PublicKey;
  let depositTrackingQueueAccount: anchor.web3.PublicKey;
  let depositTrackingHoldAccount: anchor.web3.PublicKey;

  let yourUnderlyingTokenAccount: anchor.web3.PublicKey;
  let yourSharesTokenAccount: anchor.web3.PublicKey;
  it("registers deposit tracking account", async () => {
    createAssociatedTokenAccount(provider, provider.wallet.publicKey, orcaTokenMint)
    console.log("progrmaId ", programId);
    console.log("usdcv1 vault ", rayUsdcV2Vault);
    console.log("provider", provider.wallet.publicKey);
    let [_depositTrackingAccount, _trackingNonce] = await deriveTrackingAddress(
      v2VaultsProgramId,
      rayUsdcV2Vault,
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
      rayUsdcV2VaultSharesMint
    );
    console.log(
      "deposit tracking queue",
      depositTrackingQueueAccount.toString()
    );
    console.log("deposit tracking hold", depositTrackingHoldAccount.toString());
    console.log("deposit tracking pda", depositTrackingPda.toString());
    console.log("deposit tracking", depositTrackingAccount.toString());
    console.log("sending register deposit tracking account tx");
    let tx = await program.rpc.registerDepositTrackingAccount(
      [new anchor.BN(0), new anchor.BN(9)],
      {
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
      }
    );
    console.log("sent register deposit tracking account tx ", tx);
  });
  let one = new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(6)));
  it("issues shares", async () => {
    yourUnderlyingTokenAccount = await serumAssoToken.getAssociatedTokenAddress(
      provider.wallet.publicKey,
      rayUsdcLpTokenMint
    );
    yourSharesTokenAccount = await createAssociatedTokenAccount(
      provider,
      provider.wallet.publicKey,
      rayUsdcV2VaultSharesMint
    );
    let tx = await program.rpc.issueShares(
      one,
      [new anchor.BN(0), new anchor.BN(9)],
      {
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
      new anchor.BN(877610),
      [new anchor.BN(0), new anchor.BN(9)],
      {
        options: {skipPreflight: true},
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
      }
    );
    console.log("sent withdraw deposit tracking tx ", tx);
  });
  const rayUsdcPoolId = new anchor.web3.PublicKey("CHYrUBX2RKX8iBg7gYTkccoGNBzP44LdaazMHCLcdEgS");
  const rayUsdcPoolLpTokenAccount = new anchor.web3.PublicKey("BNnXLFGva3K8ACruAc1gaP49NCbLkyE6xWhGV4G2HLrs");
  const rayUsdcPoolRewardATokenAccount = new anchor.web3.PublicKey("DpRueBHHhrQNvrjZX7CwGitJDJ8eZc3AHcyFMG4LqCQR");
  const rayUsdcPoolRewardBTokenAccount = new anchor.web3.PublicKey("DpRueBHHhrQNvrjZX7CwGitJDJ8eZc3AHcyFMG4LqCQR");
  const rayUsdcPoolAuthority = new anchor.web3.PublicKey("5KQFnDd33J5NaMC9hQ64P5XzaaSz8Pt7NBCkZFYn1po");
  it("withdraws from raydium vault", async () => {
    await program.rpc.withdrawRaydiumVault(new anchor.BN(877610), {
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
      console.log("transaction should not succeed in localnet environments")
      process.exit(1)
    }).catch(() => {
      console.log("error is expected to happen in localnet environments")
    })
  })
})
describe("tests orca-usdc auto vaults", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Examples as Program<Examples>;

  const programId = program.programId;
  const orcaUsdcV2Vault = new anchor.web3.PublicKey("7nbcWTUnvELLmLjJtMRrbg9qH9zabZ9VowJSfwB2j8y7");
  const orcaUsdcV2VaultPda = new anchor.web3.PublicKey("Hum9fSBpV26PRBpCj53nYyBy6KSyRfKLsZcFTfLMUamf");
  const orcaUsdcV2VaultSharesMint = new anchor.web3.PublicKey("5Dzz5aB1x4DMkRYHuEA1tmzxm6jfpWGzn4ScNBUACGbd");
  const orcaUsdcV2VaultCompoundQueue = new anchor.web3.PublicKey("Dzpjfr3uEwpn7Vo538rEy9XV4pAjUDvxDPD3cePzH48G");
  const orcaUsdcV2VaultDepositQueue = new anchor.web3.PublicKey("DwZSxDX447QXLfzHUi5f2t59MhyGgfr6dBqwXsHhxwd");
  const orcaUsdcV2VaultWithdrawQueue = new anchor.web3.PublicKey("Bdv8bRJSNGhb285P2ZRXCxc6RieFz1vd8wcypMp3NvEB");
  const orcaUsdcLpTokenMint = new anchor.web3.PublicKey("n8Mpu28RjeYD7oUX3LG1tPxzhRZh3YYLRSHcHRdS3Zx");
  const orcaUsdcPoolTokenA = new anchor.web3.PublicKey("9vYWHBPz817wJdQpE8u3h8UoY3sZ16ZXdCcvLB7jY4Dj");
  const orcaUsdcPoolTokenB = new anchor.web3.PublicKey("6UczejMUv1tzdvUzKpULKHxrK9sqLm8edR1v9jinVWm9")
  const orcaUsdcPoolFeeAaccount = new anchor.web3.PublicKey('7CXZED4jfRp3qdHB9Py3up6v1C4UhHofFvfT6RXbJLRN');
  const orcaUsdcPoolSwapAccount = new anchor.web3.PublicKey("2p7nYbtPBgtmY69NsE8DAW6szpRJn7tQvDnqvoEWQvjY")
  const orcaUsdcPoolSwapAuthority = new anchor.web3.PublicKey("3fr1AhdiAmWLeNrS24CMoAu9pPgbzVhwLtJ6QUPmw2ob");
  const orcaSwapProgram = new anchor.web3.PublicKey("9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP");
  let depositTrackingAccount: anchor.web3.PublicKey;
  let depositTrackingPda: anchor.web3.PublicKey;
  let depositTrackingQueueAccount: anchor.web3.PublicKey;
  let depositTrackingHoldAccount: anchor.web3.PublicKey;
  const usdcTokenMint = new anchor.web3.PublicKey(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
  );
  let yourUnderlyingTokenAccount: anchor.web3.PublicKey;
  let yourSharesTokenAccount: anchor.web3.PublicKey;
  let yourOrcaUsdcLpTokenAccount: anchor.web3.PublicKey;
  it("prepares test data", async () => {
    createAssociatedTokenAccount(
      provider,
      provider.wallet.publicKey,
      orcaTokenMint
    ).catch(async () => {
      // orca token account already exists
      yourOrcaTokenAccount = await serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, orcaTokenMint);
      
    }).then(async (key) => {
      yourOrcaTokenAccount = await serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, orcaTokenMint);
    })
    yourUsdcTokenAccount = await serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, usdcTokenMint)
    yourOrcaUsdcLpTokenAccount = await createAssociatedTokenAccount(provider, provider.wallet.publicKey, orcaUsdcLpTokenMint)
  })
  it("registers deposit tracking account", async () => {
    console.log("progrmaId ", programId);
    console.log("usdcv1 vault ", orcaUsdcV2Vault);
    console.log("provider", provider.wallet.publicKey);
    let [_depositTrackingAccount, _trackingNonce] = await deriveTrackingAddress(
      v2VaultsProgramId,
      orcaUsdcV2Vault,
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
      orcaUsdcV2VaultSharesMint
    );
    console.log(
      "deposit tracking queue",
      depositTrackingQueueAccount.toString()
    );
    yourSharesTokenAccount = await createAssociatedTokenAccount(
      provider,
      provider.wallet.publicKey,
      orcaUsdcV2VaultSharesMint,
    )
    console.log("deposit tracking hold", depositTrackingHoldAccount.toString());
    console.log("deposit tracking pda", depositTrackingPda.toString());
    console.log("deposit tracking", depositTrackingAccount.toString());
    console.log("sending register deposit tracking account tx");
    let tx = await program.rpc.registerDepositTrackingAccount(
      [new anchor.BN(2), new anchor.BN(4)],
      {
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
      }
    );
    console.log("sent register deposit tracking account tx ", tx);
  });
  let one = new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(6)));
  it("adds liquidity and issues shares", async () => {
    const tx = await program.rpc.orcaAddLiqIssueShares(
      one, one,
      [new anchor.BN(2), new anchor.BN(4)],
      {
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
      }
    )
    console.log("sent orca add liq issue shares tx ", tx);
  });
  it("withdraws from deposit tracking account", async () => {
    console.log("wait 3 seconds")
    freeze(3000);
    console.log("in production this would fail, as 15 minutes need to pass before lockup is expired")
    let tx = await program.rpc.withdrawDepositTracking(
      new anchor.BN(877610),
      [new anchor.BN(2), new anchor.BN(4)],
      {
        options: {skipPreflight: true},
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
      }
    );
    console.log("sent withdraw deposit tracking tx ", tx);
  });

})
describe("tests orca atlas-usdc double dip auto vaults",() => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Examples as Program<Examples>;

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
  const orcaAtlasUsdcPoolTokenB = new anchor.web3.PublicKey("8YswVYsTi66umBF2Bnkh4LB2VWMKPssDpe54VAgiuJZQ")
  const orcaAtlasUsdcPoolFeeAaccount = new anchor.web3.PublicKey("CFN4DQ2p3qroX92pPNy3mov3Dw1aCNGLrU5AXHpHxbko");
  const orcaAtlasUsdcPoolSwapAccount = new anchor.web3.PublicKey("3V5sjXj1mrWjjB1Xt6Xwp554QwHE5fppGSxbk4GzAtEW")
  const orcaAtlasUsdcPoolSwapAuthority = new anchor.web3.PublicKey("8UYN675AJn5htWydDs724xqintBZ4XzsCWqMozUSDU8m");
  const orcaAtlasUsdcFarmTokenMint = new anchor.web3.PublicKey("HFmY1ggCsCky1zJ1sfdkNR4zb3u5n38YNRdf4vsGu17t");

  const orcaSwapProgram = new anchor.web3.PublicKey("9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP");
  const usdcTokenMint = new anchor.web3.PublicKey(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
  );
  let yourUnderlyingTokenAccount: anchor.web3.PublicKey;
  let yourOrcaUsdcLpTokenAccount: anchor.web3.PublicKey;
  let providerSharesAccount: anchor.web3.PublicKey;
  let depositTrackingAccount: anchor.web3.PublicKey;
  let depositTrackingPda: anchor.web3.PublicKey;
  let depositTrackingQueueAccount: anchor.web3.PublicKey;
  let depositTrackingHoldAccount: anchor.web3.PublicKey;
  let depositTrackingOrcaDDQueueAddress: anchor.web3.PublicKey;
  let yourAtlasTokenAccount: anchor.web3.PublicKey;
  it("prepares test data", async() => {
    yourOrcaUsdcLpTokenAccount = await createAssociatedTokenAccount(provider, provider.wallet.publicKey, orcaAtlasUsdcLpTokenMint)
    createAssociatedTokenAccount(provider, provider.wallet.publicKey, atlasTokenMint)
    .catch(async () => {
      yourAtlasTokenAccount = await serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, atlasTokenMint);
    }).then(async () => {
      yourAtlasTokenAccount = await serumAssoToken.getAssociatedTokenAddress(provider.wallet.publicKey, atlasTokenMint);
    })
  })
  it("registers deposit tracking account", async () => {
    providerSharesAccount = await createAssociatedTokenAccount(
      provider,
      provider.wallet.publicKey,
      orcaAtlasUsdcV2VaultSharesMint
    );
    console.log("provider shares account ", providerSharesAccount.toString());
    let [_depositTrackingAccount, _trackingNonce] = await deriveTrackingAddress(
      v2VaultsProgramId,
      orcaAtlasUsdcV2Vault,
      provider.wallet.publicKey
    );
    depositTrackingAccount = _depositTrackingAccount;
    let [_depositTrackingPda, _depositTrackingPdaNonce] =
      await deriveTrackingPdaAddress(v2VaultsProgramId, depositTrackingAccount);
    depositTrackingPda = _depositTrackingPda;
    let [_depositTrackingQueueAccount, _queueNonce] =
      await deriveTrackingQueueAddress(v2VaultsProgramId, depositTrackingPda);
    depositTrackingQueueAccount = _depositTrackingQueueAccount;
    depositTrackingHoldAccount = await createAssociatedTokenAccount(
      provider,
      depositTrackingPda,
      orcaAtlasUsdcV2VaultSharesMint
    );
    let [_deriveTrackingOrcaDDQueueAddress, _ddQueueNonce] =
      await deriveTrackingOrcaDDQueueAddress(
        v2VaultsProgramId,
        orcaAtlasUsdcV2Vault,
        depositTrackingPda
      );
    depositTrackingOrcaDDQueueAddress = _deriveTrackingOrcaDDQueueAddress;
    console.log("deposit tracking dd queue ", depositTrackingOrcaDDQueueAddress.toString())
    console.log("deposit tracking hold ", depositTrackingHoldAccount.toString())
    console.log("deposit tracking queue ", depositTrackingQueueAccount.toString())
    console.log("deposit tracking pda ", depositTrackingPda.toString());
    console.log("deposit tracking account ", depositTrackingAccount.toString());
    const tx = await program.rpc.registerDepositTrackingAccount(
      [new anchor.BN(2), new anchor.BN(0)],
      {
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
      }
    );
    console.log("register deposit tracking tx ", tx);
  }); 
  let one = new anchor.BN(1).mul(new anchor.BN(10).pow(new anchor.BN(6)));
  it("adds liquidity and issues shares", async () => {
    const tx = await program.rpc.orcaAddLiqIssueShares(
      one, one,
      [new anchor.BN(2), new anchor.BN(0)],
      {
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
      }
    )
    console.log("sent orca add liq issue shares tx ", tx);
  });
  it("withdraws from deposit tracking account", async () => {
    console.log("wait 3 seconds")
    freeze(3000);
    console.log("in production this would fail, as 15 minutes need to pass before lockup is expired")
    let tx = await program.rpc.withdrawDepositTracking(
      new anchor.BN(1392),
      [new anchor.BN(2), new anchor.BN(0)],
      {
        options: {skipPreflight: true},
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
      }
    );
    console.log("sent withdraw deposit tracking tx ", tx);
  });
})
describe("tests atrix usdr-usdc auto vaults", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Examples as Program<Examples>;

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
  const atrixUsdrUSdcLpTokenMint = new anchor.web3.PublicKey("HKx72DDVoUa1QPb7oqRs4b361TEYoiENxmJJm91Agq8W")

  const atrixUsdrUSdcPoolCoinTokenAccount = new anchor.web3.PublicKey("G23fX1ejHB68a4B8caYw3pgc5Fe545KPRs7pFM95PgEr");
  const atrixUsdrUSdcPoolPcTokenAccount = new anchor.web3.PublicKey("F573sZnEJEgXydbq3KBCG5rn5z6P9AnohVbokpHYcpmg")
  const atrixUsdrUSdcPoolLpTokenAccount = new anchor.web3.PublicKey("FzZPPqtj7T8FmSdsLaw17VHX3GMZa8gTxkzwJKRzc6Re");
  const atrixUsdrUSdcPoolOpenOrdersAccount = new anchor.web3.PublicKey("9BunfFitaX8B4bAJRRywR2b5Ad7pzKqw4LeBrAuKTDh4")
  const atrixUsdrUSdcSerumMarket = new anchor.web3.PublicKey("9vRkq3qWXvGiBMqu7J4mfWQrvppBbwQCTh2MfbqKCnZU");


  const atrixFarmProgramId = new anchor.web3.PublicKey("BLDDrex4ZSWBgPYaaH6CQCzkJXWfzCiiur9cSFJT8t3x");
  const atrixPoolProgramId = new anchor.web3.PublicKey("HvwYjjzPbXWpykgVZhqvvfeeaSraQVnTiQibofaFw9M7");
  const atrixProtocolAccount = new anchor.web3.PublicKey("3uTzTX5GBSfbW7eM9R9k95H7Txe32Qw3Z25MtyD2dzwC")

  let depositTrackingAccount: anchor.web3.PublicKey;
  let depositTrackingPda: anchor.web3.PublicKey;
  let depositTrackingQueueAccount: anchor.web3.PublicKey;
  let depositTrackingHoldAccount: anchor.web3.PublicKey;
  const usdcTokenMint = new anchor.web3.PublicKey(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
  );
  const usdrTokenMint = new anchor.web3.PublicKey(
    "HNpdP2rL6FR6jM3bDxFX2Zo32D1YG2ZCztf9zzCrKMEX"
  )
  let yourUnderlyingTokenAccount: anchor.web3.PublicKey;
  let yourSharesTokenAccount: anchor.web3.PublicKey;
  let yourOrcaUsdcLpTokenAccount: anchor.web3.PublicKey;
  it("registers deposit tracking account", async () => {
    let [_depositTrackingAccount, _trackingNonce] = await deriveTrackingAddress(
      v2VaultsProgramId,
      atrixUsdrUSdcV2Vault,
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
      atrixUsdrUSdcV2VaultSharesMint
    );
    console.log(
      "deposit tracking queue",
      depositTrackingQueueAccount.toString()
    );
    yourSharesTokenAccount = await createAssociatedTokenAccount(
      provider,
      provider.wallet.publicKey,
      atrixUsdrUSdcV2VaultSharesMint,
    )
    console.log("deposit tracking hold", depositTrackingHoldAccount.toString());
    console.log("deposit tracking pda", depositTrackingPda.toString());
    console.log("deposit tracking", depositTrackingAccount.toString());
    console.log("sending register atrix usdr-usdc deposit tracking account tx");
    let tx = await program.rpc.registerDepositTrackingAccount(
      [new anchor.BN(4), new anchor.BN(4)],
      {
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
      }
    );
    console.log("sent register deposit tracking account tx ", tx);
  });  
})
const timer = ms => new Promise( res => setTimeout(res, ms));

function freeze(time) {
  const stop = new Date().getTime() + time;
  while(new Date().getTime() < stop);       
}
