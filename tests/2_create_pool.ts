import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import {
  adminAta,
  mintPda,
  poolPayload,
  poolAta,
  poolPda,
  program,
  provider,
  token,
  authority,
  xtar,
  waitTxn,
} from "./utils";

describe("create pool", () => {
  describe("happy path", () => {
    it("create pool", async () => {
      await program.methods
        .createPool(poolPayload)
        .accounts({ mint: mintPda, poolAta, userAta: adminAta })
        .rpc();

      const pool = await program.account.pool.fetch(poolPda);
      const poolBalance = await provider.connection.getTokenAccountBalance(
        poolAta
      );
      const adminBalance = await provider.connection.getTokenAccountBalance(
        adminAta
      );

      expect(pool.fee.toNumber()).eq(poolPayload.fee.toNumber());
      expect(pool.splitFees.toNumber()).eq(poolPayload.splitFee.toNumber());
      expect(pool.authority.signer.toString()).eq(
        authority.publicKey.toString()
      );
      expect(pool.authority.feeWallet.toString()).eq(
        authority.publicKey.toString()
      );
      expect(pool.ata.toString()).eq(poolAta.toString());
      expect(pool.token.toString()).eq(mintPda.toString());
      expect(pool.isPublic).eq(poolPayload.isPublic);
      expect(pool.otherChainTokenAddress).deep.eq(xtar);
      expect(pool.tokenSymbol).eq(token.symbol);

      expect(adminBalance.value.uiAmount).eq(0);
      expect(poolBalance.value.uiAmount).eq(token.totalSupply.toNumber());
    });
  });
  // describe("errors", () => {
  //   it("only admin can create pools", async () => {
  //     try {
  //       await createAssociatedTokenAccount(
  //         provider.connection,
  //         authority,
  //         otherMintPda,
  //         authority.publicKey
  //       );

  //       await program.methods
  //         .createPool(poolPayload)
  //         .accounts({
  //           mint: otherMintPda,
  //           poolAta: otherPoolPda,
  //           userAta: otherAuthorityAta,
  //           payer: authority.publicKey,
  //         })
  //         .signers([authority])
  //         .rpc();
  //       expect(true).eq(false);
  //     } catch (_err) {
  //       expect(_err instanceof anchor.AnchorError);
  //       const err: anchor.AnchorError = _err;

  //       expect(err.message).eq(
  //         "AnchorError caused by account: payer. Error Code: ConstraintRaw. Error Number: 2003. Error Message: A raw constraint was violated."
  //       );
  //     }
  //   });
  // });
});
