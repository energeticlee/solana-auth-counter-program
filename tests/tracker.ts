import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tracker } from "../target/types/tracker";
const assert = require("assert");
const { SystemProgram } = anchor.web3;

describe("token_project", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Tracker as Program<Tracker>;
  const tracker = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    await program.rpc.initialize({
      accounts: {
        tracker: tracker.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [tracker],
    });

    let account = await program.account.tracker.fetch(tracker.publicKey);
    assert.ok(account.counter.toString() == "0");
  });

  it("Counter + 1!", async () => {
    // Add your test here.
    let initialCount = await program.account.tracker.fetch(tracker.publicKey);
    await program.rpc.addTrackerCount({
      accounts: {
        tracker: tracker.publicKey,
        authority: provider.wallet.publicKey,
      },
    });

    let account = await program.account.tracker.fetch(tracker.publicKey);
    assert.ok(account.counter.toString() == "1");
  });

  it("Counter + 1, should fail", async () => {
    // Add your test here.
    try {
      await program.rpc.addTrackerCount({
        accounts: {
          tracker: tracker.publicKey,
          authority: tracker.publicKey,
        },
      });
      assert.ok(false);
    } catch (error) {
      assert.ok(error);
    }
  });
});
