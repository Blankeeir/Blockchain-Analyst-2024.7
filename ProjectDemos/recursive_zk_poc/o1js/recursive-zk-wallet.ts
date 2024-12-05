import { 
  Field, 
  ZkProgram, 
  Struct, 
  Signature, 
  PublicKey, 
  Bool,
  Poseidon 
} from 'o1js';

// Basic wallet transaction structure
class WalletTransaction extends Struct({
  sender: PublicKey,
  receiver: PublicKey,
  amount: Field,
  nonce: Field,
}) {}

// Transaction with signature
class SignedTransaction extends Struct({
  transaction: WalletTransaction,
  signature: Signature,
}) {}

// Base wallet circuit for single transactions
const BaseWalletCircuit = ZkProgram({
  name: "Base Wallet Circuit",
  publicInput: SignedTransaction,
  publicOutput: Field,

  methods: {
    processTransaction: {
      privateInputs: [],
      async method(transaction: SignedTransaction): Promise<{ publicOutput: Field }> {
        // Convert PublicKey to Field by hashing
        const receiverField = Poseidon.hash(
          transaction.transaction.receiver.toFields()
        );
    
        // Verify signature
        const valid = transaction.signature.verify(
          transaction.transaction.sender,
          [
            transaction.transaction.amount,
            transaction.transaction.nonce,
            receiverField
          ]
        );
        
        valid.assertTrue("Invalid signature");
    
        // Return amount as public output for recursive verification
        return { publicOutput: transaction.transaction.amount };
      }
    }
  }
});

// Proof type for base wallet circuit
class BaseWalletProof extends ZkProgram.Proof(BaseWalletCircuit) {}

// Recursive circuit for chaining transactions
const RecursiveWalletCircuit = ZkProgram({
  name: "Recursive Wallet Circuit",
  publicInput: SignedTransaction,
  publicOutput: Field,

  methods: {
    processChainedTransaction: {
      privateInputs: [BaseWalletProof],
      async method(
        currentTx: SignedTransaction,
        previousTxProof: BaseWalletProof
      ): Promise<{ publicOutput: Field }> {
        // Verify the previous transaction proof
        previousTxProof.verify();

        // Get the amount from previous transaction
        const previousAmount = previousTxProof.publicOutput;

        // Convert PublicKey to Field by hashing
        const receiverField = Poseidon.hash(
          currentTx.transaction.receiver.toFields()
        );

        // Verify current transaction signature
        const valid = currentTx.signature.verify(
          currentTx.transaction.sender,
          [
            currentTx.transaction.amount,
            currentTx.transaction.nonce,
            receiverField
          ]
        );

        valid.assertTrue("Invalid signature");

        // Ensure transaction chain validity
        const hasEnoughBalance = previousAmount.greaterThanOrEqual(
          currentTx.transaction.amount
        );
        hasEnoughBalance.assertTrue("Insufficient balance");

        // Return remaining balance
        return { publicOutput: previousAmount.sub(currentTx.transaction.amount) };
      }
    }
  }
});

// Proof type for recursive wallet circuit
class RecursiveWalletProof extends ZkProgram.Proof(RecursiveWalletCircuit) {}

// Wallet management class
class RecursiveZKWallet {
  private previousProof: BaseWalletProof | null = null;

  constructor(
    private readonly baseCircuit = BaseWalletCircuit,
    private readonly recursiveCircuit = RecursiveWalletCircuit
  ) {}

  async createTransaction(
    sender: PublicKey, 
    receiver: PublicKey,
    amount: Field,
    nonce: Field,
    signature: Signature
  ): Promise<BaseWalletProof> {
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

  async processChainedTransaction(
    currentTx: SignedTransaction,
    previousProof: BaseWalletProof
  ): Promise<RecursiveWalletProof> {
    this.previousProof = previousProof;
    const result = await this.recursiveCircuit.processChainedTransaction(
      currentTx,
      previousProof
    );
    return result.proof;
  }

  async verifyTransaction(proof: BaseWalletProof | RecursiveWalletProof): Promise<boolean> {
    try {
      await proof.verify();
      return true;
    } catch {
      return false;
    }
  }

  getBalance(proof: BaseWalletProof | RecursiveWalletProof): Field {
    return proof.publicOutput;
  }
}

export {
  WalletTransaction,
  SignedTransaction,
  BaseWalletCircuit,
  BaseWalletProof,
  RecursiveWalletCircuit,
  RecursiveWalletProof,
  RecursiveZKWallet
};