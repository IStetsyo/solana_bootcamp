import { airdropIfRequired } from "@solana-developers/helpers";
import { connection, publicKey } from "./check-balance";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

airdropIfRequired(
  connection,
  publicKey,
  1 * LAMPORTS_PER_SOL,
  0.5 * LAMPORTS_PER_SOL,
);
