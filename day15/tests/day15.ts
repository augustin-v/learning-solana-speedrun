import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day15 } from "../target/types/day15";

describe("day15", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const defaultKeyPair = new anchor.web3.PublicKey(
    "BJ1k9XXJF9cSoU8xrQ5PsCjR7gT7wgk7jq53fyu89UcR"
  );

  const program = anchor.workspace.day15 as Program<Day15>;

  it("Is initialized!", async () => {
    let bal_before = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("before: ", bal_before);

    const tx = await program.methods.initialize().rpc();

    let bal_after = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("after: ", bal_after);

    console.log(
      "diff: ",
      BigInt(bal_before.toString()) - BigInt(bal_after.toString())
    );
  });
});
