import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sysvar } from "../target/types/sysvar";

describe("sysvar", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.sysvar as Program<Sysvar>;

  const StakeHistory_PublicKey = new anchor.web3.PublicKey(
    "SysvarStakeHistory1111111111111111111111111"
  );

  const lastRestartSlot_PublicKey = new anchor.web3.PublicKey("SysvarLastRestartS1ot1111111111111111111111");

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({stakeHistory: StakeHistory_PublicKey, lastRestartSlot: lastRestartSlot_PublicKey}).rpc();
    console.log("Your transaction signature", tx);
  });
});
