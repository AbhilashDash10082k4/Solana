import {PublicKey, Keypair } from "@solana/web3.js";
//PDA
const programAddress = new PublicKey("11111111111111111111111111111111"); //address of the System Program
const seed = [Buffer.from("adsince2k4")];
const [pda, bump ] = PublicKey.findProgramAddressSync(seed, programAddress);
//gen a keypair
const keypair = Keypair.generate();
console.log("PDA: ",pda);
console.log("SEED: ",seed);
console.log("programAddress: ",programAddress);
console.log("BUMP: ",bump);
console.log(`Public Key: ${keypair.publicKey}`);
console.log(`Secret Key: ${keypair.secretKey}`);