import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../turbin3-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);
const metadataURI = "https://gateway.irys.xyz/J8onQjBmM3PA4Z7ry8gVbgmNDHGKuwLTxdDUHNDqY1Um";
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(
        umi , 
        {
        mint ,
        name : "TURBIN3 RUG MINT",
        symbol: "NK",
        uri : metadataURI,
        sellerFeeBasisPoints : percentAmount(1),
        isMutable : true,
        collectionDetails : null
    })
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();

// https://explorer.solana.com/tx/2JweRpqyi67yCnSiuiMARHfFtHFyyBsTVdrLdZe7yFFf7coJfRtozJvVYuJzJmCEg5ouikvbsdcxFkjGR3uYGQix?cluster=devnet
// Mint Address:  FqeQDeHLmvchEUdXn8hgZbM6qTfwKVzTdKED47xrKJHN