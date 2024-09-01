require("dotenv").config();
import * as anchor from "@coral-xyz/anchor";
import { DogstarBridge } from "../../target/types/dogstar_bridge";

export const program = anchor.workspace
  .DogstarBridge as anchor.Program<DogstarBridge>;
export const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

export const waitTxn = async ({ tx, showLogs = false }) => {
  const connection = program.provider.connection;

  const latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction(
    {
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    },
    "confirmed"
  );

  const txDetails = await program.provider.connection.getTransaction(tx, {
    maxSupportedTransactionVersion: 0,
    commitment: "confirmed",
  });
  const logs = txDetails?.meta?.logMessages || null;
  if (showLogs) {
    console.log(logs);
  }
  return logs;
};
