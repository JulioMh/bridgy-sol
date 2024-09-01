require("dotenv").config();
import * as anchor from "@coral-xyz/anchor";
import { keccak256, toBuffer } from "ethereumjs-utils";
import { provider } from "./provider";
import { bs58, hex } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Wallet } from "ethers";

const be: number[] = [...hex.decode(process.env.BE_PUB)];

export const beWallet: Wallet = new Wallet(process.env.BE_PRIV);

export const badBeWallet: Wallet = new Wallet(process.env.BAD_BE_PRIV);

export const stellarWallet: string = process.env.OTHER_CHAIN_USER_ADDRESS;

export const xtar: string =
  "GAORYJ3KBDGIM7FFSKVUJHJ5NEFWIRDIAGGBJBJS7TY6ECZS53257IG4";

export const adminMock: Admin = {
  signer: provider.wallet.publicKey,
  be,
  feeWallet: new anchor.web3.PublicKey(process.env.FEE_WALLET),
};

export const authority = anchor.web3.Keypair.fromSecretKey(
  bs58.decode(process.env.AUTHORITY_PRIV_KEY)
);

export const user = anchor.web3.Keypair.fromSecretKey(
  bs58.decode(process.env.USER_PRIV_KEY)
);

export const token: Token = {
  timestamp: new anchor.BN(0),
  name: "test",
  uri: "",
  symbol: "test",
  decimals: 8,
  totalSupply: new anchor.BN(1000000),
  revokeAuthority: true,
  mintSupply: true,
  authority: authority.publicKey,
};

export const poolPayload: Pool = {
  timestamp: new anchor.BN(1234567),
  amount: token.totalSupply,
  fee: new anchor.BN(10),
  splitFee: new anchor.BN(40),
  isPublic: false,
  authority: {
    signer: authority.publicKey,
    feeWallet: authority.publicKey,
  },
  tokenAddress: xtar,
};

// TYPES

export interface Pool {
  timestamp: anchor.BN;
  amount: anchor.BN;
  fee: anchor.BN;
  splitFee: anchor.BN;
  tokenAddress: string;
  authority: Authority;
  isPublic: boolean;
}
export interface Authority {
  signer: anchor.web3.PublicKey;
  feeWallet: anchor.web3.PublicKey;
}
export interface Admin {
  signer: anchor.web3.PublicKey;
  be: number[];
  feeWallet: anchor.web3.PublicKey;
}
export interface Token {
  timestamp: anchor.BN;
  name: string;
  uri: string;
  symbol: string;
  decimals: number;
  totalSupply: anchor.BN;
  revokeAuthority: boolean;
  mintSupply: boolean;
  authority: anchor.web3.PublicKey;
}

export interface Release {
  amount: anchor.BN;
  timestamp: anchor.BN;
  mint: anchor.web3.PublicKey;
  to: anchor.web3.PublicKey;
}
