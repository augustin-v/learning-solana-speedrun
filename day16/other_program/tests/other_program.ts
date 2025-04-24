import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OtherProgram } from "../target/types/other_program";

describe("other_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.otherProgram as Program<OtherProgram>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [TrueOrFalse, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    console.log("address: ", program.programId.toBase58());

    
    //await program.methods.initialize().rpc();
    await program.methods.setbool(false).accounts({trueOrFalse: TrueOrFalse}).rpc();
  });
});
