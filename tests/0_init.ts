import {
  adminMock,
  authority,
  program,
  provider,
  user,
  waitTxn,
} from "./utils";
import { expect } from "chai";
import { adminPda } from "./utils/pda";

describe("init program", () => {
  before(async () => {
    const tx = await provider.connection.requestAirdrop(
      authority.publicKey,
      10000000
    );
    await waitTxn({ tx });
    const tx2 = await provider.connection.requestAirdrop(
      user.publicKey,
      10000000
    );
    await waitTxn({ tx: tx2 });
  });
  describe("happy path", () => {
    it("init program", async () => {
      await program.methods
        .init({
          signer: adminMock.signer,
          be: adminMock.be,
          feeWallet: adminMock.feeWallet,
        })
        .accounts({})
        .rpc();

      const admin = await program.account.admin.fetch(adminPda);
      expect(admin.signer.toString()).eq(adminMock.signer.toString());
      expect(admin.be).deep.eq(adminMock.be);
      expect(admin.feeWallet.toString()).eq(adminMock.feeWallet.toString());
    });
  });
});
