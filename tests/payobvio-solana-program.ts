import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

import { PayobvioSolanaProgram } from "../target/types/payobvio_solana_program";

describe("payobvio-solana-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .PayobvioSolanaProgram as Program<PayobvioSolanaProgram>;
  const user = provider.wallet.publicKey;

  it("Initializes the account", async () => {
    const [dataAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("data_account"), user.toBuffer()],
      program.programId
    );

    const message = "Hello, Solana!";

    try {
      const tx = await program.methods
        .initialize(message)
        .accounts({
          dataAccount: dataAccount,
          user: user,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

      console.log("Your transaction signature", tx);

      const account = await program.account.dataAccount.fetch(dataAccount);
      expect(account.message).to.equal(message);
    } catch (err) {
      if (err instanceof AnchorError) {
        console.error("Error Code:", err.error.errorCode.code);
        console.error("Error Message:", err.error.errorMessage);
      } else {
        console.error("Unexpected error:", err);
      }
      throw err;
    }
  });
});
