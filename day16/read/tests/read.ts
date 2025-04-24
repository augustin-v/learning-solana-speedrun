import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Read } from "../target/types/read";

describe("read", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const program = anchor.workspace.read as Program<Read>;

  it("Is initialized!", async () => {
    const otherProgramAddress = "CXRAVkuW5BFETzu6xX4MTVyG7VkGSg9LYhMPnSGUKCdJ";
    const otherProgramId = new anchor.web3.PublicKey(otherProgramAddress);

    const otherIdl = JSON.parse(
      require("fs").readFileSync("../other_program/target/idl/other_program.json", "utf8")
    );

    const otherProgram = new anchor.Program(otherIdl);

    const seeds = []

    const [trueOrFalseAcc, _bump] =
      anchor.web3.PublicKey.findProgramAddressSync(seeds, otherProgramId);
    
    let otherStorageStruct = await otherProgram.account.trueOrFalse.fetch(trueOrFalseAcc);
    console.log("The value of flag is:", otherStorageStruct.flag.toString());
  });
});
