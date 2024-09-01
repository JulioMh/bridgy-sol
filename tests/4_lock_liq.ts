import * as anchor from "@coral-xyz/anchor";
import {
  adminFeeAta,
  adminMock,
  authority,
  authorityAta,
  getBalance,
  mintPda,
  poolAta,
  program,
  stellarWallet,
  token,
  user,
  userAta,
  waitTxn,
  xtar,
} from "./utils";
import { expect } from "chai";

describe("lock liquidity", () => {
  const date = Date.now();
  describe("errors", () => {
    it("only admin can lock if pool is private", async () => {
      try {
        await program.methods
          .lockLiq({
            amount: new anchor.BN(500),
            to: stellarWallet,
            timestamp: new anchor.BN(date),
          })
          .accounts({
            mint: mintPda,
            user: user.publicKey,
            userAta: userAta,
            poolAta: poolAta,
            adminFeeAta: adminFeeAta,
            authorityFeeAta: authorityAta,
            authorityFee: authority.publicKey,
            adminFee: adminMock.feeWallet,
          })
          .signers([user])
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;

        expect(err.error.errorMessage).eq("Bridge is private");
      }
    });
  });
  describe("happy path", () => {
    it("lock liq", async () => {
      const tx = await program.methods
        .setIsPublic(true)
        .accounts({ payer: authority.publicKey, mint: mintPda })
        .signers([authority])
        .rpc();
      await waitTxn({ tx });
      const subscriptionId = program.addEventListener("lockEvent", (event) => {
        expect(event.amount.toNumber()).eq(450);
        expect(event.to.toString()).eq(stellarWallet);
        expect(event.tokenAddress).eq(mintPda.toString());
        expect(event.otherChainTokenAddress).eq(xtar);
      });

      const tx2 = await program.methods
        .lockLiq({
          amount: new anchor.BN(500),
          timestamp: new anchor.BN(date),
          to: stellarWallet,
        })
        .accounts({
          mint: mintPda,
          user: user.publicKey,
          userAta: userAta,
          poolAta: poolAta,
          adminFeeAta: adminFeeAta,
          authorityFeeAta: authorityAta,
          authorityFee: authority.publicKey,
          adminFee: adminMock.feeWallet,
        })
        .signers([user])
        .rpc();

      await new Promise((resolve) => setTimeout(resolve, 2000));
      program.removeEventListener(subscriptionId);

      const poolBalance = await getBalance(poolAta);
      const authorityBalance = await getBalance(authorityAta);
      const adminBalance = await getBalance(adminFeeAta);
      const userBalance = await getBalance(userAta);

      const authorityFee = 500 * 0.1 * 0.6;
      const adminFee = 500 * 0.1 * 0.4;
      expect(userBalance).eq(500);
      expect(authorityBalance).eq(authorityFee);
      expect(adminBalance).eq(adminFee);
      expect(poolBalance).eq(
        token.totalSupply
          .sub(new anchor.BN(1000))
          .add(new anchor.BN(500 - authorityFee - adminFee))
          .toNumber()
      );
    });
  });
});
