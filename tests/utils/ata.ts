import * as anchor from "@coral-xyz/anchor";
import { mintPda, otherMintPda, poolPda } from "./pda";
import { adminMock, authority, user } from "./mock";
import { provider } from "./provider";

export const adminAta = anchor.utils.token.associatedAddress({
  mint: mintPda,
  owner: adminMock.signer,
});

export const otherAdminAta = anchor.utils.token.associatedAddress({
  mint: otherMintPda,
  owner: adminMock.signer,
});

export const otherAuthorityAta = anchor.utils.token.associatedAddress({
  mint: otherMintPda,
  owner: authority.publicKey,
});

export const poolAta = anchor.utils.token.associatedAddress({
  mint: mintPda,
  owner: poolPda,
});

export const userAta = anchor.utils.token.associatedAddress({
  mint: mintPda,
  owner: user.publicKey,
});

export const otherPoolAta = anchor.utils.token.associatedAddress({
  mint: otherMintPda,
  owner: poolPda,
});

export const adminFeeAta = anchor.utils.token.associatedAddress({
  mint: mintPda,
  owner: adminMock.feeWallet,
});

export const authorityAta = anchor.utils.token.associatedAddress({
  mint: mintPda,
  owner: authority.publicKey,
});

export const getBalance = async (
  ata: anchor.web3.PublicKey
): Promise<number> => {
  const balance = await provider.connection.getTokenAccountBalance(ata);
  return balance.value.uiAmount;
};
