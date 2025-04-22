import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DeployTutorial } from "../target/types/deploy_tutorial";

describe("deploy_tutorial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const programID = "3o3zvpMrVivaQRNyhsGGfi5Cm8Q6NJcFJcwoJNBf8Rkq";
  const program = anchor.workspace.deployTutorial as Program<DeployTutorial>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
