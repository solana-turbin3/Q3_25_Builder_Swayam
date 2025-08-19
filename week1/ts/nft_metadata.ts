import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://gateway.irys.xyz/FDutzyLGRXissGa5UMgynNzvbrUGgXQo2rBMTmPWA6Hy";
        const metadata = {
            name: "masti",
            symbol: "NK",
            description: "This is a rug nft",
            image: image,
            attributes: [
                {trait_type: '?', value: '?'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ],
                category : "image"
            },
            creators: [null]
        };
        // const myUri = ???
        const myUri = await umi.uploader.uploadJson(metadata)
        // console.log("Your metadata URI: ", myUri);
        console.log("metadata URI : " ,myUri)
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

// Metadata URI : https://gateway.irys.xyz/J8onQjBmM3PA4Z7ry8gVbgmNDHGKuwLTxdDUHNDqY1Um