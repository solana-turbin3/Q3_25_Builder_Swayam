import {Keypair , Connection, Commitment, PublicKey } from '@solana/web3.js'
import wallet from '../turbin3-wallet.json'
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const commitment : Commitment = 'confirmed';
const connection = new Connection('https://api.devnet.solana.com', commitment);

const mint = new PublicKey('GVX1GbKU6aiZ5dStWHsDmQ7X5Hd8mLJUT1vWkfmrzMQh');
const decimals = 1_000_000;

(async ()=> {
    try{
        // making a associated token account , from the user's public key and mint
        const ata = await getOrCreateAssociatedTokenAccount(connection,keypair,mint,keypair.publicKey)
        console.log(`associated token account is ${ata.address.toBase58()}`)

        //Mint to ATA 
        const mintTx = await mintTo(connection , keypair , mint , ata.address , keypair.publicKey , decimals)
        console.log(`Your txid for mint ${mintTx}`)
    }
    catch (e){
        console.log("error" ,e);
    }
})();