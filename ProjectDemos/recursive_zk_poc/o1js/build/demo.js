"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
// demo.ts
const o1js_1 = require("o1js");
const recursive_zk_wallet_1 = require("./recursive-zk-wallet");
async function main() {
    console.log('Starting Recursive ZK Wallet Demo...');
    console.log('------------------------------------');
    // Initialize the local blockchain
    const Local = await o1js_1.Mina.LocalBlockchain();
    o1js_1.Mina.setActiveInstance(Local);
    // Generate test accounts
    const deployerAccount = Local.testAccounts[0];
    const senderAccount = o1js_1.PrivateKey.random();
    const receiverAccount = o1js_1.PrivateKey.random();
    console.log('Compiling circuits...');
    await recursive_zk_wallet_1.BaseWalletCircuit.compile();
    await recursive_zk_wallet_1.RecursiveWalletCircuit.compile();
    // Initialize wallet
    const wallet = new recursive_zk_wallet_1.RecursiveZKWallet();
    console.log('\nGenerating test transactions...');
    // Create first transaction
    const amount1 = (0, o1js_1.Field)(100);
    const nonce1 = (0, o1js_1.Field)(1);
    // Create signature for first transaction
    const tx1Fields = [
        amount1,
        nonce1,
        o1js_1.Poseidon.hash(receiverAccount.toPublicKey().toFields()) // Hash PublicKey to Field
    ];
    const tx1Signature = o1js_1.Signature.create(senderAccount, tx1Fields);
    console.log('\nCreating first transaction:');
    console.log('Amount:', amount1.toString());
    console.log('From:', senderAccount.toPublicKey().toBase58());
    console.log('To:', receiverAccount.toPublicKey().toBase58());
    const tx1Proof = await wallet.createTransaction(senderAccount.toPublicKey(), receiverAccount.toPublicKey(), amount1, nonce1, tx1Signature);
    // Verify first transaction
    const tx1Valid = await wallet.verifyTransaction(tx1Proof);
    console.log('\nFirst transaction verification:', tx1Valid ? 'SUCCESS' : 'FAILED');
    console.log('Balance after first transaction:', wallet.getBalance(tx1Proof).toString());
    // Create second transaction
    const amount2 = (0, o1js_1.Field)(50);
    const nonce2 = (0, o1js_1.Field)(2);
    const tx2Fields = [
        amount2,
        nonce2,
        o1js_1.Poseidon.hash(receiverAccount.toPublicKey().toFields()) // Hash PublicKey to Field
    ];
    const tx2Signature = o1js_1.Signature.create(senderAccount, tx2Fields);
    const tx2 = new recursive_zk_wallet_1.SignedTransaction({
        transaction: new recursive_zk_wallet_1.WalletTransaction({
            sender: senderAccount.toPublicKey(),
            receiver: receiverAccount.toPublicKey(),
            amount: amount2,
            nonce: nonce2
        }),
        signature: tx2Signature
    });
    console.log('\nCreating second (chained) transaction:');
    console.log('Amount:', amount2.toString());
    console.log('From:', senderAccount.toPublicKey().toBase58());
    console.log('To:', receiverAccount.toPublicKey().toBase58());
    // Process chained transaction
    const chainedProof = await wallet.processChainedTransaction(tx2, tx1Proof);
    // Verify chained transaction
    const chainedValid = await wallet.verifyTransaction(chainedProof);
    console.log('\nChained transaction verification:', chainedValid ? 'SUCCESS' : 'FAILED');
    console.log('Balance after chained transaction:', wallet.getBalance(chainedProof).toString());
    console.log('\nDemo completed successfully!');
}
// Run the demo
main()
    .then(() => process.exit(0))
    .catch((err) => {
    console.error(err);
    process.exit(1);
});
