import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14 } from "../target/types/day14";
import { assert, expect } from "chai";

describe("day14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day14 as Program<Day14>;

  let myKeypair = anchor.web3.Keypair.generate();
  let hisKeypair = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      signer1: program.provider.publicKey,
      signer2: myKeypair.publicKey,
      signer3: hisKeypair.publicKey
    })
    .signers([myKeypair, hisKeypair])
    .rpc();
    console.log("the signer1 is: ", program.provider.publicKey.toBase58());
    console.log("the signer2 is: ", myKeypair.publicKey.toBase58());
    console.log("the signer3 is: ", hisKeypair.publicKey.toBase58());
  });

  it("Only owner test", async () => {
    const tx = await program.methods.accessControl().accounts({
      signerAccount: program.provider.publicKey
    })
    .rpc();
    console.log("Owner entered");
  })

//  it("Access control test attacker", async () => {
//
//     await expect(program.methods.accessControl().accounts({
//      signerAccount: hisKeypair.publicKey
//    })
//    .signers([hisKeypair])
//    .rpc()
//   ).throw
//  })
});
