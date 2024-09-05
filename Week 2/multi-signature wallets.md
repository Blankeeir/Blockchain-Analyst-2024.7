# Week 3 Report: Multi-Signature Wallet

**By Siyi Xu, Created on 01/09/2024**

## Introduction

Multi-signature (multi-sig) wallets are advanced digital wallets that require multiple private keys to authorize a transaction, unlike standard wallets that rely on a single private key. This approach adds an extra layer of security and is particularly beneficial for organizations and individuals aiming to safeguard their digital assets against unauthorized access and single points of failure. This report provides a comprehensive technical analysis of multi-signature wallets, exploring their structural components, security benefits, and potential challenges. The discussion also includes implementation details and optimization strategies to enhance wallet performance and usability.

## Main Problem and Focus

The core issue addressed by multi-signature wallets is the inherent vulnerability of single-signature wallets, where a single private key controls access to all funds. This creates a single point of failure: if the private key is lost, stolen, or otherwise compromised, all assets within the wallet can be accessed and potentially stolen without any means of recovery. 

In organizational contexts, single-signature wallets also lack mechanisms to enforce multi-party consent for transactions. This deficiency can lead to misuse of funds and a lack of transparency or accountability, as there is no enforced protocol to ensure that multiple stakeholders agree to any transaction before it is executed.

### Key Objectives of Multi-Signature Wallets:

1. **Mitigation of Single Point of Failure:** By requiring multiple signatures for authorization, multi-sig wallets reduce the risk of a single compromised key leading to a total loss of funds.
2. **Enforcement of Multi-Party Consent:** Multi-signature schemes ensure that multiple stakeholders must approve a transaction, thereby increasing security, trust, and accountability, especially in corporate or collaborative environments.

## Technical Structure of Multi-Signature Wallet

### 1. **Basic Architecture**

A multi-signature wallet is typically implemented using smart contracts on blockchain networks like Ethereum, Bitcoin, or others that support programmable scripts or contracts. The architecture of a multi-sig wallet involves several key components:

- **M-of-N Scheme:** The most common structure for multi-signature wallets is the M-of-N scheme, where **N** represents the total number of authorized signers, and **M** is the minimum number of signatures required to approve a transaction. For example, a 2-of-3 multi-sig wallet requires at least two of the three authorized parties to sign a transaction before it is executed.
  
- **Smart Contract Logic:** The smart contract acts as the wallet's underlying logic, storing the public keys of the authorized signers and enforcing the M-of-N signature requirement. It holds the digital assets and validates the signatures provided for each transaction request against the stored public keys.

- **Signature Aggregation:** When a transaction is initiated, it is sent to all authorized parties for signing. The signatures are aggregated and then submitted to the smart contract, which verifies them. Only if the minimum required number of valid signatures is provided does the contract execute the transaction.

### 2. **Implementation Details**

Multi-signature wallets are implemented differently depending on the blockchain network. Here is a technical overview of implementations on Ethereum and Bitcoin:

#### **Ethereum Multi-Signature Wallet**

On Ethereum, multi-signature wallets are often implemented using smart contracts written in **Solidity**. A popular example is the Gnosis Safe wallet, which allows for highly customizable multi-signature configurations.

**Key Elements:**

- **Smart Contract Code:** The Solidity contract stores the public keys of the signers and the M-of-N threshold. The `execTransaction` function checks if the number of valid signatures meets the threshold before executing any transaction.

- **Transaction Flow:**
  1. **Initiation:** A user proposes a transaction and sends it to the smart contract along with the required signatures.
  2. **Validation:** The contract validates the signatures against the stored public keys.
  3. **Execution:** If the required number of signatures is valid, the contract executes the transaction; otherwise, it rejects it.

**Example Solidity Code:**

```solidity
pragma solidity ^0.8.0;

contract MultiSigWallet {
    uint private constant MAX_OWNER_COUNT = 50;
    address[] public owners;
    uint public required;
    mapping(address => bool) public isOwner;
    mapping(bytes32 => mapping(address => bool)) public confirmations;

    // Modifier to ensure the sender is one of the owners
    modifier onlyOwner() {
        require(isOwner[msg.sender], "Not owner");
        _;
    }

    constructor(address[] memory _owners, uint _required) {
        require(_owners.length <= MAX_OWNER_COUNT, "Too many owners");
        for (uint i = 0; i < _owners.length; i++) {
            isOwner[_owners[i]] = true;
        }
        owners = _owners;
        required = _required;
    }

    function submitTransaction(address destination, uint value, bytes memory data) public onlyOwner {
        bytes32 txHash = keccak256(abi.encode(destination, value, data));
        confirmations[txHash][msg.sender] = true;
    }

    function confirmTransaction(bytes32 txHash) public onlyOwner {
        confirmations[txHash][msg.sender] = true;
        if (isConfirmed(txHash)) {
            executeTransaction(txHash);
        }
    }

    function executeTransaction(bytes32 txHash) internal {
        // Check if transaction has enough confirmations
        require(isConfirmed(txHash), "Not enough confirmations");
        (address destination, uint value, bytes memory data) = decodeTransaction(txHash);
        (bool success, ) = destination.call{value: value}(data);
        require(success, "Transaction failed");
    }

    function isConfirmed(bytes32 txHash) public view returns (bool) {
        uint count = 0;
        for (uint i = 0; i < owners.length; i++) {
            if (confirmations[txHash][owners[i]]) count += 1;
            if (count == required) return true;
        }
        return false;
    }

    // Helper function to decode transaction
    function decodeTransaction(bytes32 txHash) internal pure returns (address destination, uint value, bytes memory data) {
        // Decoding logic here
    }
}
```

#### **Bitcoin Multi-Signature Wallet**

On Bitcoin, multi-signature wallets are implemented using **P2SH (Pay-to-Script-Hash)** addresses and scripts written in Bitcoin's scripting language.

**Key Elements:**

- **P2SH Address:** The Bitcoin network supports a special type of address that references a script hash rather than a public key hash. A P2SH address allows the sender to require a specific script to be satisfied to spend funds. The script defines the M-of-N multi-signature requirement.
  
- **Script Execution:** When spending from a multi-signature address, the script provided must be matched with the script hash, and the required number of valid signatures must be provided.

**Example Bitcoin Script:**

```plaintext
OP_HASH160 <RedeemScriptHash> OP_EQUAL
```

Where the `<RedeemScriptHash>` is a hash of the redeem script that includes the multi-signature logic:

```plaintext
OP_2 <PubKey1> <PubKey2> <PubKey3> OP_3 OP_CHECKMULTISIG
```

This script specifies a 2-of-3 multi-signature setup.

### 3. **Security Advantages**

Multi-signature wallets provide several security benefits:

- **Mitigation of Single Points of Failure:** By requiring multiple signatures, multi-signature wallets significantly reduce the risk of losing access to funds due to a single compromised key.
  
- **Enhanced Fraud Prevention:** Multi-sig wallets prevent unauthorized transactions by enforcing multi-party consent, making them ideal for corporate use where multiple stakeholders need to authorize fund transfers.

- **Improved Accountability and Transparency:** Every transaction requires multiple signatures, ensuring that actions taken are transparent and can be audited by all stakeholders.

### 4. **Potential Challenges and Optimization Strategies**

Despite their advantages, multi-signature wallets face several challenges:

- **Complex Key Management:** The requirement for multiple keys introduces complexity in managing and storing these keys securely. Hardware wallets or secure key management systems are often necessary to mitigate risks.
  
- **Increased Transaction Size and Fees:** Multi-signature transactions are larger in size due to the additional signatures required, leading to higher transaction fees on blockchains like Bitcoin. Optimization techniques such as Schnorr signatures, which allow signature aggregation, can reduce transaction sizes and costs.

- **Smart Contract Vulnerabilities:** On smart contract-enabled blockchains like Ethereum, bugs in the contract code could lead to funds being locked or lost. Rigorous testing and formal verification methods are essential to mitigate these risks.

### Optimization Strategies

1. **Use of Schnorr Signatures:** Implementing Schnorr signatures for signature aggregation can reduce the size of multi-signature transactions, thereby lowering transaction fees and increasing efficiency.
2. **Threshold Signature Schemes:** Threshold signature schemes can be used to further optimize multi-signature operations, providing a cryptographic way to combine multiple signatures into a single signature, which reduces on-chain data storage requirements.
3. **Smart Contract Audits:** Regularly auditing smart contracts and using formal verification tools can prevent vulnerabilities and bugs that could compromise the security of a multi-signature wallet.

## Conclusion

Multi-signature wallets offer a robust solution for securing digital assets on blockchain networks by requiring multiple parties to authorize transactions. This approach mitigates the risks associated with single points of failure and provides greater control, security, and accountability, particularly in organizational settings. However, implementing multi-signature wallets requires careful consideration of key management, transaction costs, and potential vulnerabilities. By adopting best practices and optimizing strategies, multi-signature wallets can provide a secure and effective mechanism for managing digital assets in various applications.

### References

1. **Gnosis Safe Wallet Documentation**: [Gnosis Safe Docs](https://docs.gnosis-safe.io/)
2. **Bitcoin P2SH Documentation

**: [Bitcoin Developer Docs](https://developer.bitcoin.org/devguide/p2sh.html)
3. **Ethereum Smart Contract Security**: [Ethereum Smart Contract Best Practices](https://consensys.github.io/smart-contract-best-practices/)