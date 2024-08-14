import "dotenv/config";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";

const PUBLIC_KEY = process.env.PUBLIC_KEY;

if (!PUBLIC_KEY) {
  throw new Error("PUBLIC_KEY must be specified in .env file!");
}

export const connection = new Connection(clusterApiUrl("devnet"));
console.log("Successfully connection to devnet cluster!");

export const publicKey = new PublicKey(PUBLIC_KEY);
connection.getBalance(publicKey).then((balance) => {
  const balanceInLamports = balance / LAMPORTS_PER_SOL;
  console.log(`Wallet: ${publicKey}. Balance: ${balanceInLamports}`);
});
