import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicStorage } from "../target/types/basic_storage";

describe("basic_storage", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.basicStorage as Program<BasicStorage>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [myStorage, _bump] =
    anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    console.log("the storage account address is: ",myStorage.toBase58());

    await program.methods.printX().accounts({myStorage: myStorage}).rpc();
    // Add your test here.
    await program.methods.set(new anchor.BN(90)).rpc();
    await program.methods.printX().accounts({myStorage: myStorage}).rpc();
  });

});
