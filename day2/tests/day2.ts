import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day2";

describe("day2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(777), new anchor.BN(888), "Bonjour").rpc();
    console.log("Your transaction signature", tx);
  });

  it("Array test", async () => {
    const tx = await program.methods.array([new anchor.BN(1), new anchor.BN(2), new anchor.BN(3)]).rpc();
    console.log("your tx signature", tx);
  });

  it("Underflow test", async () => {
    const tx = await program.methods.underflowExercise(new anchor.BN(0), new anchor.BN(1)).rpc();
    console.log("your tx signature", tx);
  })

  it("Calculator test", async () => {
    const tx = await program.methods.calculator(new anchor.BN(10), new anchor.BN(10), new anchor.BN(10), new anchor.BN(10), new anchor.BN(64), new anchor.BN(90.0)).rpc();
  })
});
