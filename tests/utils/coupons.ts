import { beWallet, Release } from "./mock";
import { keccak256, toBuffer, ecsign } from "ethereumjs-utils";
import { HDNodeWallet, Wallet } from "ethers";

class ReleaseClass {
  amount: import("bn.js");
  timestamp: import("bn.js");
  mint: Buffer;

  constructor({ amount, timestamp, mint }) {
    this.amount = amount;
    this.timestamp = timestamp;
    this.mint = mint.toBuffer();
  }
}

function serialize(release: Release) {
  const buffer = Buffer.alloc(8 + 8 + 32 + 32);

  buffer.writeBigUInt64LE(BigInt(release.amount.toNumber()), 0);

  buffer.writeBigUInt64LE(BigInt(release.timestamp.toNumber()), 8);

  const mintBuffer = release.mint.toBuffer();
  mintBuffer.copy(buffer, 16);

  const toBuffer = release.to.toBuffer();
  toBuffer.copy(buffer, 48);

  return buffer;
}
const signCoupon = async (hash: string, signer: Wallet | HDNodeWallet) => {
  const sig = ecsign(toBuffer(hash), toBuffer(signer.privateKey));

  return {
    signature: Buffer.concat([sig.r, sig.s]).toString("hex"),
    recoveryId: sig.v - 27,
  };
};

export const generateCoupon = (release: Release, wallet = beWallet) => {
  const payload = serialize(release);
  const hash = keccak256(Buffer.from(payload));
  return signCoupon(hash, wallet);
};
