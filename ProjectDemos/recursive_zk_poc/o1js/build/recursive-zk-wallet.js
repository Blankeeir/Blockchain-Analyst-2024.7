"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RecursiveZKWallet = exports.RecursiveWalletProof = exports.RecursiveWalletCircuit = exports.BaseWalletProof = exports.BaseWalletCircuit = exports.SignedTransaction = exports.WalletTransaction = void 0;
const o1js_1 = require("o1js");
// Basic wallet transaction structure
class WalletTransaction extends (0, o1js_1.Struct)({
    sender: o1js_1.PublicKey,
    receiver: o1js_1.PublicKey,
    amount: o1js_1.Field,
    nonce: o1js_1.Field,
}) {
}
exports.WalletTransaction = WalletTransaction;
// Transaction with signature
class SignedTransaction extends (0, o1js_1.Struct)({
    transaction: WalletTransaction,
    signature: o1js_1.Signature,
}) {
}
exports.SignedTransaction = SignedTransaction;
// Base wallet circuit for single transactions
const BaseWalletCircuit = (0, o1js_1.ZkProgram)({
    name: "Base Wallet Circuit",
    publicInput: SignedTransaction,
    publicOutput: o1js_1.Field,
    methods: {
        processTransaction: {
            privateInputs: [],
            async method(transaction) {
                // Convert PublicKey to Field by hashing
                const receiverField = o1js_1.Poseidon.hash(transaction.transaction.receiver.toFields());
                // Verify signature
                const valid = transaction.signature.verify(transaction.transaction.sender, [
                    transaction.transaction.amount,
                    transaction.transaction.nonce,
                    receiverField
                ]);
                valid.assertTrue("Invalid signature");
                // Return amount as public output for recursive verification
                return { publicOutput: transaction.transaction.amount };
            }
        }
    }
});
exports.BaseWalletCircuit = BaseWalletCircuit;
// Proof type for base wallet circuit
class BaseWalletProof extends o1js_1.ZkProgram.Proof(BaseWalletCircuit) {
}
exports.BaseWalletProof = BaseWalletProof;
// Recursive circuit for chaining transactions
const RecursiveWalletCircuit = (0, o1js_1.ZkProgram)({
    name: "Recursive Wallet Circuit",
    publicInput: SignedTransaction,
    publicOutput: o1js_1.Field,
    methods: {
        processChainedTransaction: {
            privateInputs: [BaseWalletProof],
            async method(currentTx, previousTxProof) {
                // Verify the previous transaction proof
                previousTxProof.verify();
                // Get the amount from previous transaction
                const previousAmount = previousTxProof.publicOutput;
                // Convert PublicKey to Field by hashing
                const receiverField = o1js_1.Poseidon.hash(currentTx.transaction.receiver.toFields());
                // Verify current transaction signature
                const valid = currentTx.signature.verify(currentTx.transaction.sender, [
                    currentTx.transaction.amount,
                    currentTx.transaction.nonce,
                    receiverField
                ]);
                valid.assertTrue("Invalid signature");
                // Ensure transaction chain validity
                const hasEnoughBalance = previousAmount.greaterThanOrEqual(currentTx.transaction.amount);
                hasEnoughBalance.assertTrue("Insufficient balance");
                // Return remaining balance
                return { publicOutput: previousAmount.sub(currentTx.transaction.amount) };
            }
        }
    }
});
exports.RecursiveWalletCircuit = RecursiveWalletCircuit;
// Proof type for recursive wallet circuit
class RecursiveWalletProof extends o1js_1.ZkProgram.Proof(RecursiveWalletCircuit) {
}
exports.RecursiveWalletProof = RecursiveWalletProof;
// Wallet management class
class RecursiveZKWallet {
    constructor(baseCircuit = BaseWalletCircuit, recursiveCircuit = RecursiveWalletCircuit) {
        this.baseCircuit = baseCircuit;
        this.recursiveCircuit = recursiveCircuit;
        this.previousProof = null;
    }
    async createTransaction(sender, receiver, amount, nonce, signature) {
        const tx = new WalletTransaction({
            sender,
            receiver,
            amount,
            nonce
        });
        const signedTx = new SignedTransaction({
            transaction: tx,
            signature
        });
        const result = await this.baseCircuit.processTransaction(signedTx);
        return result.proof;
    }
    async processChainedTransaction(currentTx, previousProof) {
        this.previousProof = previousProof;
        const result = await this.recursiveCircuit.processChainedTransaction(currentTx, previousProof);
        return result.proof;
    }
    async verifyTransaction(proof) {
        try {
            await proof.verify();
            return true;
        }
        catch {
            return false;
        }
    }
    getBalance(proof) {
        return proof.publicOutput;
    }
}
exports.RecursiveZKWallet = RecursiveZKWallet;
