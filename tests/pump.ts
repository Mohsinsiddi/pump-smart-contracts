import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pump } from "../target/types/pump";

import {
  PublicKey,
  Keypair,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
} from "@solana/web3.js";

import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  getMint,
} from "@solana/spl-token";

describe("pump", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.Pump as Program<Pump>;

  // Constants from our program
  const MINT_SEED = "mint";
  const METADATA_SEED = "metadata";

  const recipient = new PublicKey(
    "7Tm4rRNYcuQs5t8GzKm19CzjSdoQPnw9HUPbDbXxGu6L"
  );

  const [mint] = PublicKey.findProgramAddressSync(
    [Buffer.from(MINT_SEED)],
    program.programId
  );

  const MPL_TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const metadata = {
    name: "Just a Test Token",
    symbol: "TEST",
    uri: "https://5vfxc4tr6xoy23qefqbj4qx2adzkzapneebanhcalf7myvn5gzja.arweave.net/7UtxcnH13Y1uBCwCnkL6APKsge0hAgacQFl-zFW9NlI",
    decimals: 6,
  };

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

  it("Mint some tokens to your wallet!", async () => {
    // Derive the associated token address account for the mint and payer.
    const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
      mint,
      payer.payer.publicKey
    );

    // Amount of tokens to mint.
    const amount = new anchor.BN(1000000);

    // Mint the tokens to the associated token account.
    const transactionSignature = await program.methods
      .mintToken(amount)
      .accounts({
        payer: payer.payer.publicKey,
        mintAccount: mint,
        associatedTokenAccount: associatedTokenAccountAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        recipient: recipient,
      })
      .rpc({ skipPreflight: true });

    console.log("Success!");
    console.log(
      `   Associated Token Account Address: ${associatedTokenAccountAddress}`
    );
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
