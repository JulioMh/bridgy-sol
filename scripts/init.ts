require("dotenv").config();
import * as anchor from "@coral-xyz/anchor";

import idl from "../target/idl/dogstar_bridge.json";
import { bs58, hex } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

const authority = anchor.web3.Keypair.fromSecretKey(
  bs58.decode(process.env.AUTHORITY_PRIV_KEY)
);

async function deployAndInitialize() {
  const be: number[] = [...hex.decode(process.env.BE_PUB)];
  const keypair = anchor.web3.Keypair.fromSecretKey(
    bs58.decode(process.env.ADMIN_PRIV_KEY)
  );
  const wallet = new anchor.Wallet(keypair);
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection("http://127.0.0.1:8899"),
    wallet
  );
  const program = new anchor.Program(idl as anchor.Idl, provider);

  const tx = await program.methods
    .init({
      signer: keypair.publicKey,
      be,
      feeWallet: new anchor.web3.PublicKey(process.env.FEE_WALLET),
    })
    .accounts({})
    .rpc();

  waitTxn({ tx, showLogs: true, program });

  console.log("Programa desplegado e inicializado con éxito");
}

deployAndInitialize().catch((err) => {
  console.error("Error desplegando e inicializando el programa:", err);
});

const waitTxn = async ({ tx, showLogs = false, program }) => {
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
