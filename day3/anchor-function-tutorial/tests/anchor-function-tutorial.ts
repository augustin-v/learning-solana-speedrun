import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorFunctionTutorial } from "../target/types/anchor_function_tutorial";

describe("anchor-function-tutorial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorFunctionTutorial as Program<AnchorFunctionTutorial>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.boatyMcBoatface(new anchor.BN(24)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Adding...", async () => {
    const tx = await program.methods.add(new anchor.BN(5), new anchor.BN(8)).rpc();
    console.log("TX sign: ", tx);
  })
});
