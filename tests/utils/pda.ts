import * as anchor from "@coral-xyz/anchor";
import { program } from "./provider";
import { authority, user } from "./mock";

export const [adminPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("ADMIN")],
  program.programId
);

export const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("token"), authority.publicKey.toBuffer(), Buffer.from("test")],
  program.programId
);

export const [otherMintPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("token"), authority.publicKey.toBuffer(), Buffer.from("test2")],
  program.programId
);

export const [errorMintPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("token"), authority.publicKey.toBuffer(), Buffer.from("test3")],
  program.programId
);

export const [tokenPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("token"), mintPda.toBuffer()],
  program.programId
);

export const [poolPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("pool"), mintPda.toBuffer()],
  program.programId
);

export const [otherPoolPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("pool"), otherMintPda.toBuffer()],
  program.programId
);

export const [releasePda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("release"), user.publicKey.toBuffer(), poolPda.toBuffer()],
  program.programId
);

const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
export const [metadata] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("metadata"),
    TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    mintPda.toBuffer(),
  ],
  TOKEN_METADATA_PROGRAM_ID
);
export const [otherMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("metadata"),
    TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    otherMintPda.toBuffer(),
  ],
  TOKEN_METADATA_PROGRAM_ID
);

export const [errorMetadata] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("metadata"),
    TOKEN_METADATA_PROGRAM_ID.toBuffer(),
    errorMintPda.toBuffer(),
  ],
  TOKEN_METADATA_PROGRAM_ID
);
