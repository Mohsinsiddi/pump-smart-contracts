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
import { PumpGame } from "../target/types/pump_game";

describe("Pump Game", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.PumpGame as Program<PumpGame>;
  const pump_program = anchor.workspace.Pump as Program<Pump>;

  // Constants from our program
  const ACC_SEED = "game_account";

  const [acc_seed] = PublicKey.findProgramAddressSync(
    [Buffer.from(ACC_SEED), payer.payer.publicKey.toBuffer()],
    program.programId
  );

  // Constants from our program
  const ADMIN_CONFIG_SEED = "admin_authority";

  const [admin_config_seed] = PublicKey.findProgramAddressSync(
    [Buffer.from(ADMIN_CONFIG_SEED), payer.payer.publicKey.toBuffer()],
    program.programId
  );
  it("Create an Admin Config Account", async () => {
    if (false) {
      const context = {
        adminData: admin_config_seed,
        payer: payer.payer.publicKey,
        pumpProgram: pump_program.programId,
        systemProgram: SystemProgram.programId,
      };

      const transactionSignature = await program.methods
        .initAdminConfigAccount()
        .accounts(context)
        .rpc({ skipPreflight: true });
      console.log("Success!");
      console.log(`   Transaction Signature: ${transactionSignature}`);
    }
    const account = await program.account.adminConfig.fetch(admin_config_seed);
    console.log(account);
  });

  it("Update an Admin Config Account", async () => {
    if (false) {
      const context = {
        adminData: admin_config_seed,
        payer: payer.payer.publicKey,
        newPumpProgram: new PublicKey(
          "DJMMfpsEPB6JSpzakCK9CqBtAjzRhFx7AYNUAqVktmUE"
        ),
      };

      const transactionSignature = await program.methods
        .setAdminConfigData()
        .accounts(context)
        .rpc({ skipPreflight: true });
      console.log("Success!");
      console.log(`   Transaction Signature: ${transactionSignature}`);
    }
    const account = await program.account.adminConfig.fetch(admin_config_seed);
    console.log(account);
  });

  it("Create an Game Account", async () => {
    if (false) {
      const context = {
        payer: payer.payer.publicKey,
        gameData: acc_seed,
        systemProgram: SystemProgram.programId,
      };
      const chances = 0;
      const transactionSignature = await program.methods
        .initAccount(chances)
        .accounts(context)
        .rpc({ skipPreflight: true });
      console.log("Success!");
      console.log(`   Transaction Signature: ${transactionSignature}`);
    }
    const account = await program.account.gameData.fetch(acc_seed);
    console.log(account);
  });
  it("Fetch Game Account Data", async () => {
    const account = await program.account.gameData.fetch(acc_seed);
    console.log(account);
  });
});
