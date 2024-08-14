import "dotenv/config";
import { Keypair } from "@solana/web3.js";

const SECRET_KEY = process.env.SECRET_KEY;

if (!SECRET_KEY) {
  throw new Error("SECRET_KEY must be specified in .env file!");
}

const secretKey = Uint8Array.from(JSON.parse(SECRET_KEY));
const keypair = Keypair.fromSecretKey(secretKey);

console.log("Keypair successfully generated!");
console.log(`Generated public key: ${keypair.publicKey.toBase58()}`);
