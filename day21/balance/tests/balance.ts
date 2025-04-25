import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Balance } from "../target/types/balance";

describe("balance", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.balance as Program<Balance>;
  let pubKey = new anchor.web3.PublicKey("BJ1k9XXJF9cSoU8xrQ5PsCjR7gT7wgk7jq53fyu89UcR");

  it("Is initialized!", async () => {
    const balance = await program.methods.readBalance().accounts({account: pubKey}).rpc();

  });
});
