import { expect } from "chai";
import * as anchor from "@coral-xyz/anchor";
import {
  mintPda,
  program,
  provider,
  token,
  metadata,
  decodeMintAccountData,
  authority,
  otherMetadata,
  otherMintPda,
  Token,
  errorMetadata,
  errorMintPda,
  tokenPda,
} from "./utils";
import { adminAta, otherAdminAta } from "./utils/ata";

describe("create token", () => {
  describe("happy path", () => {
    it("create token and revoke authority", async () => {
      await program.methods
        .createToken(token)
        .accounts({
          metadata,
          userAta: adminAta,
          authority: authority.publicKey,
        })
        .rpc();

      const mint = await provider.connection.getAccountInfo(mintPda);
      const programToken = await program.account.programToken.fetch(tokenPda);
      const mintInfo = decodeMintAccountData(mint.data);
      const balance = await provider.connection.getTokenAccountBalance(
        adminAta
      );
      expect(programToken.mint.toString()).eq(mintPda.toString());
      expect(programToken.symbol).eq(token.symbol);
      expect(mintInfo).deep.eq({
        mintAuthority: null,
        supply: "100000000000000",
        decimals: 8,
        isInitialized: true,
        freezeAuthority: null,
      });

      expect(balance.value.uiAmount).eq(1000000);
    });
    it("create token and without revoking authority", async () => {
      await program.methods
        .createToken({
          ...token,
          symbol: "test2",
          revokeAuthority: false,
          mintSupply: false,
        })
        .accounts({
          metadata: otherMetadata,
          userAta: otherAdminAta,
          authority: authority.publicKey,
        })
        .rpc();

      const mint = await provider.connection.getAccountInfo(otherMintPda);

      const mintInfo = decodeMintAccountData(mint.data);
      const balance = await provider.connection.getTokenAccountBalance(
        otherAdminAta
      );

      expect(mintInfo).deep.eq({
        mintAuthority: authority.publicKey.toString(),
        supply: "0",
        decimals: 8,
        isInitialized: true,
        freezeAuthority: null,
      });

      expect(balance.value.uiAmount).eq(0);
    });
  });
  describe("errors", () => {
    it("only admin can create the token", async () => {
      try {
        const tokenErr: Token = { ...token, symbol: "test3" };
        const authorityAta = anchor.utils.token.associatedAddress({
          mint: errorMintPda,
          owner: authority.publicKey,
        });
        await program.methods
          .createToken(tokenErr)
          .accounts({
            metadata: errorMetadata,
            payer: authority.publicKey,
            userAta: authorityAta,
            authority: authority.publicKey,
          })
          .signers([authority])
          .rpc();
        expect(true).eq(false);
      } catch (_err) {
        expect(_err instanceof anchor.AnchorError);
        const err: anchor.AnchorError = _err;
        expect(err.message).eq(
          "AnchorError caused by account: payer. Error Code: ConstraintRaw. Error Number: 2003. Error Message: A raw constraint was violated."
        );
      }
    });
  });
});
