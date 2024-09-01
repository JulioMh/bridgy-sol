import * as anchor from "@coral-xyz/anchor";

export const decodeMintAccountData = (data: Buffer) => {
  const mintAuthorityOption = data.readUInt32LE(0);
  const mintAuthority = mintAuthorityOption
    ? new anchor.web3.PublicKey(data.slice(4, 36))
    : null;
  const supply = data.readBigUInt64LE(36);
  const decimals = data.readUInt8(44);
  const isInitialized = data.readUInt8(45) !== 0;
  const freezeAuthorityOption = data.readUInt32LE(46);
  const freezeAuthority = freezeAuthorityOption
    ? new anchor.web3.PublicKey(data.slice(50, 82))
    : null;

  return {
    mintAuthority: mintAuthority ? mintAuthority.toBase58() : null,
    supply: supply.toString(),
    decimals,
    isInitialized,
    freezeAuthority: freezeAuthority ? freezeAuthority.toBase58() : null,
  };
};
