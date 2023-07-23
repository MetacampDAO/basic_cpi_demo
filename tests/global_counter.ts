import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GlobalCounter } from "../target/types/global_counter";
import * as token from "@solana/spl-token";
import { assert } from "chai";
import { SYSVAR_RENT_PUBKEY } from "@solana/web3.js";

describe("global_counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GlobalCounter as Program<GlobalCounter>;

  it("Is initialized!", async () => {
    const kp1 = anchor.web3.Keypair.generate();
    const kp2 = anchor.web3.Keypair.generate();
    const mint = anchor.web3.Keypair.generate();

    const kp1Ata = await token.getAssociatedTokenAddress(
      mint.publicKey,
      kp1.publicKey
    );
    const kp2Ata = await token.getAssociatedTokenAddress(
      mint.publicKey,
      kp2.publicKey
    );

    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(kp1.publicKey, 1e9)
    );

    console.log("TEST 1");
    // Add your test here.
    try {
      const tx = await program.methods
        .createMintAndTransferTo(new anchor.BN(10_00), new anchor.BN(5_00))
        .accounts({
          initializer: kp1.publicKey,
          initializerAta: kp1Ata,
          receiver: kp2.publicKey,
          receiverAta: kp2Ata,
          mint: mint.publicKey,
          tokenProgram: token.TOKEN_PROGRAM_ID,
          associatedTokenProgram: token.ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([kp1, mint])
        .rpc();
    } catch (error) {
      console.log("ERROR", error);
    }

    const kp1AtaBalance =
      await program.provider.connection.getTokenAccountBalance(kp1Ata);

    const kp2AtaBalance =
      await program.provider.connection.getTokenAccountBalance(kp2Ata);

    assert.equal(kp1AtaBalance.value.uiAmount, 5);
    assert.equal(kp2AtaBalance.value.uiAmount, 5);
  });
});
