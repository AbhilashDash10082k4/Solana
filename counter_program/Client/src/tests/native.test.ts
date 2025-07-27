import * as web3 from "@solana/web3.js";
import * as borsh from "borsh";
import assert from "assert";
class CounterAccount {
  count = 0;
  constructor(fields: { count: number } | undefined = undefined) {
    if (fields) {
      this.count = fields.count;
    }
  }
}
const CounterAccountSchema = new Map([
  [CounterAccount, { kind: "struct", fields: [["count", "u32"]] }],
]);
const GREETING_SIZE = borsh.serialize(
  CounterAccountSchema,
  new CounterAccount()
).length;
const pg = {
  connection: new web3.Connection(web3.clusterApiUrl("devnet")),
  wallet: {
    // Replace with your actual keypair and publicKey
    keypair: web3.Keypair.generate(),
    publicKey: web3.PublicKey as any, // Placeholder for publicKey
  },
  PROGRAM_ID: new web3.PublicKey("HKUS3cDNXVfkafZY1rJquBs1ZGwakxvFtbbjb8wiD5am"),
};
pg.wallet.publicKey = pg.wallet.keypair.publicKey;

describe("Test", () => {
  it("greet", async () => {
    //Create keypair
    const counterAccountKp = new web3.Keypair();
    console.log(`counterAccountKp.publickey : ${counterAccountKp.publicKey}`)
    const lamports = await pg.connection.getMinimumBalanceForRentExemption(
      GREETING_SIZE
    );

    // Create instructions to generate corresponding data accounts
    const createGreetingAccountIx = web3.SystemProgram.createAccount({
      fromPubkey: pg.wallet.publicKey,
      lamports,
      newAccountPubkey: counterAccountKp.publicKey,
      programId: pg.PROGRAM_ID,
      space: GREETING_SIZE,
    });


    // Call the program and the counter accumulates
    const greetIx = new web3.TransactionInstruction({
      keys: [
        {
          pubkey: counterAccountKp.publicKey,
          isSigner: false,
          isWritable: true,
        },
      ],
      programId: pg.PROGRAM_ID,
    });

    //Create a transaction, including the above 2 instructions
    const tx = new web3.Transaction();
    tx.add(createGreetingAccountIx, greetIx);

    //Initiate a transaction and obtain the transaction hash
    const txHash = await web3.sendAndConfirmTransaction(pg.connection, tx, [
      pg.wallet.keypair,
      counterAccountKp,
    ]);
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Get the information of the specified data account
    const counterAccountOnSolana = await pg.connection.getAccountInfo(
      counterAccountKp.publicKey
    );
    if (!counterAccountOnSolana) {
      throw new Error("Account not found on Solana blockchain.");
    }
    //Deserialize
    const deserializedAccountData = borsh.deserialize(
      CounterAccountSchema,
      CounterAccount,
      counterAccountOnSolana.data
    );

    // Determine whether the current counter is accumulating
    assert.equal(deserializedAccountData.count, 1);
  });
});