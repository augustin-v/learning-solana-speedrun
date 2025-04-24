import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Mappings } from "../target/types/mappings";

describe("mappings", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.mappings as Program<Mappings>;

  it("Is initialized!", async () => {
    const key = new anchor.BN(4);
    const value = new anchor.BN(1337);

    const seeds = [key.toArrayLike(Buffer, "le", 8)];

    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    await program.methods.initialize(key).accounts({val: valueAccount}).rpc();
    await program.methods.set(key, value).accounts({val: valueAccount}).rpc()

    let result = await program.account.val.fetch(valueAccount);

    console.log(`the value ${result.value} was stored in ${valueAccount.toBase58()}`);
  });
});
