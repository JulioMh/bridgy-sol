import * as anchor from "@coral-xyz/anchor";
import {
  badBeWallet,
  getBalance,
  mintPda,
  poolAta,
  program,
  Release,
  releasePda,
  token,
  user,
  userAta,
} from "./utils";
import { generateCoupon } from "./utils/coupons";
import { expect } from "chai";

describe("release liquidity", () => {
  const date = Date.now();
  const releasePayload: Release = {
    amount: new anchor.BN(1000),
    mint: mintPda,
    timestamp: new anchor.BN(date),
    to: user.publicKey,
  };
  describe("happy path", () => {
    it("release liq", async () => {
      const coupon = await generateCoupon(releasePayload);
      const subscriptionId = program.addEventListener(
        "releaseEvent",
        (event) => {
          expect(event.amount.toNumber()).eq(1000);
          expect(event.to.toString()).eq(user.publicKey.toString());
          expect(event.tokenAddress).eq(mintPda.toString());
        }
      );

      await program.methods
        .releaseLiq({
          release: releasePayload,
          coupon,
        })
        .accounts({
          mint: mintPda,
          user: user.publicKey,
          userAta: userAta,
          poolAta: poolAta,
        })
        .signers([user])
        .rpc();

      await new Promise((resolve) => setTimeout(resolve, 2000));
      program.removeEventListener(subscriptionId);
      const release = await program.account.release.fetch(releasePda);
      const poolBalance = await getBalance(poolAta);
      const userBalance = await getBalance(userAta);

      expect(release.lastClaim.toNumber()).eq(date);
      expect(release.totalClaimed.toNumber()).eq(1000);
      expect(userBalance).eq(1000);
      expect(poolBalance).eq(
        token.totalSupply.sub(new anchor.BN(1000)).toNumber()
      );
    });
  });
  describe("error", () => {
    it("using an used coupon", async () => {
      const coupon = await generateCoupon(releasePayload);
      try {
        await program.methods
          .releaseLiq({
            release: releasePayload,
            coupon,
          })
          .accounts({
            mint: mintPda,
            user: user.publicKey,
            userAta: userAta,
            poolAta: poolAta,
          })
          .signers([user])
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;

        expect(err.error.errorMessage).eq("This coupon already has been used");
      }
    });

    it("using bad arguments", async () => {
      const coupon = await generateCoupon(releasePayload);
      try {
        await program.methods
          .releaseLiq({
            release: {
              ...releasePayload,
              amount: new anchor.BN(100000),
            },
            coupon,
          })
          .accounts({
            mint: mintPda,
            user: user.publicKey,
            userAta: userAta,
            poolAta: poolAta,
          })
          .signers([user])
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;

        expect(err.error.errorMessage).eq("Invalid Coupon");
      }
    });

    it("signed by bad actor", async () => {
      const coupon = await generateCoupon(releasePayload, badBeWallet);
      try {
        await program.methods
          .releaseLiq({
            release: releasePayload,
            coupon,
          })
          .accounts({
            mint: mintPda,
            user: user.publicKey,
            userAta: userAta,
            poolAta: poolAta,
          })
          .signers([user])
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;

        expect(err.error.errorMessage).eq("Invalid Coupon");
      }
    });
  });
});
