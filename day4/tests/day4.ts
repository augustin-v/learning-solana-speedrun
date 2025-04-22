import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day4 } from "../target/types/day4";
import { assert } from "chai";
import { t } from "@starknet-react/core/dist/index-DOtHQdsr";

describe("day4", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day4 as Program<Day4>;

//  it("Limit range test", async () => {
//    // Add your test here.
//    try {
//      const tx = await program.methods.limitRange(new anchor.BN(9)).rpc();
//      console.log("Your tx signature", tx);
//    } catch (_err) {
//      assert.isTrue(_err instanceof anchor.AnchorError);
//      const err: anchor.AnchorError = _err;
//      const errMsg = "a is too small";
//      assert.strictEqual(err.error.errorMessage, errMsg);
//      console.log("Error number:", err.error.errorCode.number);
//    }
//
//    try {
//      const tx = await program.methods.limitRange(new anchor.BN(101)).rpc();
//      console.log("your tx signature", tx);
//    } catch (_err) {
//      assert.isTrue(_err instanceof anchor.AnchorError);
//      const err: anchor.AnchorError = _err;
//      const errMsg = "a is too big";
//      assert.strictEqual(err.error.errorMessage, errMsg);
//      console.log("Error number:", err.error.errorCode.number);
//    }
//  });

  it("Always error", async () => {
    try {
      const tx = await program.methods.func().rpc();
      console.log("Your tx signature", tx);
      assert.fail("function should fail")
    } catch (_err) {
      const err: anchor.AnchorError = _err;
      const errMsg = "Always errors";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("error code: ", err.error.errorCode.number);
      console.log("error msg: ", err.error.errorMessage);
    }
  }); 
});
