import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

import { PayobvioSolanaProgram } from "../target/types/payobvio_solana_program";

describe("payobvio-solana-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .PayobvioSolanaProgram as Program<PayobvioSolanaProgram>;
  const maintainer = provider.wallet.publicKey;

  const issueId = "issue_123";
  const bountyAmount = 1000;

  const [escrowAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), Buffer.from(issueId)],
    program.programId
  );

  const closeEscrowAccount = async () => {
    try {
      const tx = await program.methods
        .closeEscrow()
        .accounts({
          maintainer: maintainer,
          escrowAccount: escrowAccount,
          destination: maintainer,
        } as any)
        .rpc();

      console.log("Escrow account closed, transaction signature", tx);
    } catch (err) {
      console.error("Error closing escrow account:", err);
      throw err;
    }
  };

  it("Initializes the escrow account", async () => {
    try {
      await program.account.escrowAccount.fetch(escrowAccount);
      console.log("Escrow account already exists, closing it...");
      await closeEscrowAccount();
      return;
    } catch (err) {
      if (err.message.includes("Account does not exist")) {
        console.log(
          "Escrow account does not exist, creating a new escrow account..."
        );
      } else {
        console.error("Unexpected error while checking escrow account:", err);
        throw err;
      }
    }

    try {
      const tx = await program.methods
        .initializeEscrow(new BN(bountyAmount), issueId)
        .accounts({
          maintainer: maintainer,
          escrowAccount: escrowAccount,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();

      console.log("Your transaction signature", tx);
      const account = await program.account.escrowAccount.fetch(escrowAccount);
      console.log("Escrow account details:", account);
      expect(account.maintainer.toBase58()).to.equal(maintainer.toBase58());
      expect(account.amount.toNumber()).to.equal(bountyAmount);
      expect(account.issueId).to.equal(issueId);
    } catch (err) {
      console.error("Unexpected error:", err);
      throw err;
    }
  });

  it("Deposits funds into the escrow account", async () => {
    try {
      const tx = await program.methods
        .depositFunds(new BN(bountyAmount))
        .accounts({
          maintainer: maintainer,
          escrowAccount: escrowAccount,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();

      console.log("Your transaction signature", tx);
      const escrow = await program.account.escrowAccount.fetch(escrowAccount);
      console.log("Escrow account details after funding:", escrow);
      expect(escrow.amount.toNumber()).to.equal(bountyAmount);
      console.log("Funds deposited successfully");
    } catch (err) {
      console.error("Error depositing funds:", err);
      throw err;
    }
  });
});
