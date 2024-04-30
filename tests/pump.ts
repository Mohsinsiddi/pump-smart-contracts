import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pump } from "../target/types/pump";

import {
  PublicKey,
  Keypair,
  SYSVAR_RENT_PUBKEY,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  SystemProgram,
} from "@solana/web3.js";

import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  getMint,
} from "@solana/spl-token";
import { PumpGame } from "../target/types/pump_game";

describe("pump", () => {
  const MINIMUM_SOL_TO_LEAVE_IN_IT = "0.00089088 SOL";
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.Pump as Program<Pump>;
  const gameProgram = anchor.workspace.PumpGame as Program<PumpGame>;

  const metadata = {
    name: "Dev TEST Token",
    symbol: "DDTEST",
    uri: "https://5vfxc4tr6xoy23qefqbj4qx2adzkzapneebanhcalf7myvn5gzja.arweave.net/7UtxcnH13Y1uBCwCnkL6APKsge0hAgacQFl-zFW9NlI",
    decimals: 6,
  };

  // Constants from our program
  const MINT_SEED = "mmm";
  const METADATA_SEED = "metadata";

  const recipient = new PublicKey(
    "devjbkEUcKtEfw3h8nzScA4eS1tyWejcpTzNJmr46Xa"
  );

  const [mint] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(MINT_SEED),
      Buffer.from(metadata.symbol),
      payer.payer.publicKey.toBuffer(),
    ],
    program.programId
  );
  console.log("Mint", mint.toBase58());

  const TOKEN_VAULT_SEED = "token_vault";
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from(TOKEN_VAULT_SEED), mint.toBuffer()],
    program.programId
  );
  const ADMIN_CONFIG_SEED = "admin_authority";

  const [admin_config_seed] = PublicKey.findProgramAddressSync(
    [Buffer.from(ADMIN_CONFIG_SEED), payer.payer.publicKey.toBuffer()],
    gameProgram.programId
  );

  const MPL_TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const isMintAuthUpdated = true;
  const isFreeAuthRevoked = true;

  it("Create an SPL Token!", async () => {
    const tokenMinted = true;
    if (!tokenMinted) {
      // Derive the metadata account address.
      const [metadataAddress] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(METADATA_SEED),
          MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
        ],
        MPL_TOKEN_METADATA_PROGRAM_ID
      );

      const context = {
        payer: payer.payer.publicKey,
        mintAccount: mint,
        metadataAccount: metadataAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        recipient: recipient,
        pda: pda,
      };

      const transactionSignature = await program.methods
        .createToken(metadata)
        .accounts(context)
        .rpc({ skipPreflight: true });

      console.log("Success!");
      console.log(`   Mint Address: ${mint}`);
      console.log(`   Transaction Signature: ${transactionSignature}`);
    }
  });

  it("Buy Tokens!", async () => {
    if (false) {
      // Derive the associated token address account for the mint and payer.
      const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
        mint,
        payer.payer.publicKey
      );

      // Amount of tokens to mint.
      const sol_amount = new anchor.BN(10000000);

      // Constants from our program
      const ACC_SEED = "game_account";

      const [acc_seed] = PublicKey.findProgramAddressSync(
        [Buffer.from(ACC_SEED), payer.payer.publicKey.toBuffer()],
        gameProgram.programId
      );

      // const createTransactionSignature = await gameProgram.methods
      //   .initAccount(0)
      //   .accounts({
      //     systemProgram: SystemProgram.programId,
      //     payer: payer.payer.publicKey,
      //     gameData: acc_seed,
      //   })
      //   .rpc();

      // console.log("Success!");
      // console.log(
      //   `  Create Game Account Transaction Signature: ${createTransactionSignature}`
      // );

      console.log("mint", mint.toBase58());

      // Mint the tokens to the associated token account.
      const transactionSignature = await program.methods
        .buyTokens(metadata.symbol, sol_amount)
        .accounts({
          payer: payer.payer.publicKey,
          mintAccount: mint,
          associatedTokenAccount: associatedTokenAccountAddress,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          pda: pda,
          creatorAddress: payer.publicKey,
        })
        .rpc({ skipPreflight: true });

      console.log("Success!");
      console.log(
        `   Associated Token Account Address: ${associatedTokenAccountAddress}`
      );
      console.log(`   Transaction Signature: ${transactionSignature}`);
    }
  });

  it("Sell Tokens", async () => {
    if (true) {
      // Mint the tokens to the associated token account.
      // Amount of tokens to mint.
      // Derive the associated token address account for the mint and payer.
      const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
        mint,
        payer.payer.publicKey
      );

      console.log(associatedTokenAccountAddress.toBase58());

      const token_amount = new anchor.BN(9999996);
      const transactionSignature = await program.methods
        .sellTokens(metadata.symbol, token_amount)
        .accounts({
          signer: payer.payer.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          mintAccount: mint,
          from: associatedTokenAccountAddress,
          pda: pda,
          systemProgram: SystemProgram.programId,
          creatorAddress: payer.publicKey,
        })
        .signers([])
        .rpc({ skipPreflight: true });

      console.log("Success!");
      console.log(
        `   Associated Token Account Address: ${associatedTokenAccountAddress}`
      );
      console.log(`   Transaction Signature: ${transactionSignature}`);
    }
  });

  it("Update Mint Authority of Token", async () => {
    if (!isMintAuthUpdated) {
      // Mint the tokens to the associated token account.
      const transactionSignature = await program.methods
        .transferMintAuth()
        .accounts({
          payer: payer.payer.publicKey,
          mintAccount: mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          newMintAuth: new PublicKey(
            "8LMbjUogGPjKk5dr4T9mTDapQAdvGPxpcYrtXj64dmDw"
          ),
        })
        .rpc({ skipPreflight: false });

      console.log("Success!");
      console.log(`   Transaction Signature: ${transactionSignature}`);
    }
  });

  it("Revoke Freeze Authority of Token", async () => {
    if (!isFreeAuthRevoked) {
      // Mint the tokens to the associated token account.
      const transactionSignature = await program.methods
        .revokeFreezeAuth()
        .accounts({
          payer: payer.payer.publicKey,
          mintAccount: mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc({ skipPreflight: false });

      console.log("Success!");
      console.log(`   Transaction Signature: ${transactionSignature}`);
    }
  });
});
