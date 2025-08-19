import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("GVX1GbKU6aiZ5dStWHsDmQ7X5Hd8mLJUT1vWkfmrzMQh");

// Recipient address
const to = new PublicKey("7cmZpYn6dvF5WAMVZS1ZgRc7F8h4huhSg4PeTt65UqbN");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const sender_ata = await getOrCreateAssociatedTokenAccount(connection,keypair,mint,keypair.publicKey)

        // Get the token account of the toWallet address, and if it does not exist, create it
        const to_ata = await getOrCreateAssociatedTokenAccount(connection,keypair,mint,to);
        // Transfer the new token to the "toTokenAccount" we just created
        const tx = await transfer(connection , keypair , sender_ata.address, to_ata.address ,keypair,10 )
        console.log(tx)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();