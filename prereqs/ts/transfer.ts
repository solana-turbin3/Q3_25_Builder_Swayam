import { Transaction, SystemProgram, Connection, Keypair, 
LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js"
import wallet from "./dev-wallet.json"
const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("7cmZpYn6dvF5WAMVZS1ZgRc7F8h4huhSg4PeTt65UqbN");
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try{
        const balance = await connection.getBalance(from.publicKey)
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey : from.publicKey , 
                toPubkey : to, 
                lamports : balance 
            })
        )
        transaction.recentBlockhash  = ( await connection.getLatestBlockhash('confirmed')).blockhash;
        transaction.feePayer = from.publicKey;
        const fee  = (await connection.getFeeForMessage(transaction.compileMessage(),'confirmed')).value
        transaction.instructions.pop()
        if(fee)
        transaction.add(
            SystemProgram.transfer(
               { fromPubkey : from.publicKey ,
                toPubkey : to ,
                lamports : balance - fee
               }

            )
        )
        transaction.recentBlockhash  = ( await connection.getLatestBlockhash('confirmed')).blockhash;
        transaction.feePayer = from.publicKey;
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
            );
        console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${signature}?cluster=devnet`);    
    }
    catch(e){
        console.log("error",e);
    }
}) ();
