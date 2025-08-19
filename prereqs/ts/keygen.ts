import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58'
import promptSync from 'prompt-sync';
const prompt = promptSync();
let kp = Keypair.generate()
console.log(`You've generated a new Solana wallet:${kp.publicKey.toBase58()}`)
console.log(`[${kp.secretKey}]`)
