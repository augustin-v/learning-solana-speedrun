import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day13 } from "../target/types/day13";

describe("day13", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day13 as Program<Day13>;

  it("Is initialized!", async () => {
    // Add your test here.
    const listenerMyEvent = program.addEventListener('myEvent', (event, slot) => {
      console.log(`slot ${slot} event value ${event.value}`);
    });

    const listenerMySecondEvent = program.addEventListener('mySecondEvent', (event, slot) => {
      console.log(`slot ${slot}, event value ${event.value}, event message ${event.msg}`);
    });

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);

    await new Promise((resolve) => setTimeout(resolve, 5000));

    program.removeEventListener(listenerMyEvent);
    program.removeEventListener(listenerMySecondEvent);
  });
});
