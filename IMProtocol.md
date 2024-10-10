# Instant Messaging (IM) Protocols and Applications

Instant Messaging (IM) software and applications are pivotal in facilitating real-time communication across personal and professional spheres. For senior blockchain developers, understanding the underlying protocols that power these applications is essential, especially when considering decentralized and secure communication systems. This report provides an in-depth analysis of prominent IM protocols, their technical implementations, recent innovations, and their intersections with blockchain technology.

## Table of Contents

- [Instant Messaging (IM) Protocols and Applications](#instant-messaging-im-protocols-and-applications)
  - [Table of Contents](#table-of-contents)
  - [1. Extensible Messaging and Presence Protocol (XMPP)](#1-extensible-messaging-and-presence-protocol-xmpp)
    - [Overview](#overview)
    - [Features](#features)
    - [Applications](#applications)
    - [Technical Implementation](#technical-implementation)
      - [Sample Source Code](#sample-source-code)
        - [Installation](#installation)
        - [XMPP Chat Client](#xmpp-chat-client)
        - [Execution](#execution)
        - [Explanation](#explanation)
      - [Setting Up an XMPP Server](#setting-up-an-xmpp-server)
        - [Installation (Using Docker)](#installation-using-docker)
        - [Configuration](#configuration)
        - [Managing Users](#managing-users)
    - [Security Considerations](#security-considerations)
    - [Integration with Blockchain](#integration-with-blockchain)
      - [Example: Decentralized Authentication](#example-decentralized-authentication)
  - [2. Signal Protocol](#2-signal-protocol)
    - [Overview](#overview-1)
    - [Features](#features-1)
    - [Technical Implementation](#technical-implementation-1)
      - [Sample Source Code](#sample-source-code-1)
        - [Installation](#installation-1)
        - [Signal Protocol Example](#signal-protocol-example)
        - [Explanation](#explanation-1)
    - [Mathematical Foundations](#mathematical-foundations)
    - [Advanced Topics](#advanced-topics)
    - [Security Considerations](#security-considerations-1)
    - [Integration with Blockchain](#integration-with-blockchain-1)
      - [Example: Decentralized Prekey Distribution](#example-decentralized-prekey-distribution)
        - [Integration Steps](#integration-steps)
        - [Advantages](#advantages)
  - [3. MTProto (Telegram)](#3-mtproto-telegram)
    - [Overview](#overview-2)
    - [Features](#features-2)
    - [Technical Implementation](#technical-implementation-2)
      - [Sample Source Code](#sample-source-code-2)
        - [Installation](#installation-2)
        - [MTProto Client Example](#mtproto-client-example)
        - [Explanation](#explanation-2)
    - [Mathematical Foundations](#mathematical-foundations-1)
    - [Advanced Topics](#advanced-topics-1)
    - [Security Considerations](#security-considerations-2)
    - [Integration with Blockchain](#integration-with-blockchain-2)
      - [Example: Blockchain-Based Message Integrity](#example-blockchain-based-message-integrity)
        - [Advantages](#advantages-1)
  - [4. Rich Communication Services (RCS)](#4-rich-communication-services-rcs)
    - [Overview](#overview-3)
    - [Features](#features-3)
    - [Technical Implementation](#technical-implementation-3)
      - [Sample Source Code](#sample-source-code-3)
        - [Installation](#installation-3)
        - [RCS Messaging Example](#rcs-messaging-example)
        - [Explanation](#explanation-3)
    - [Mathematical Foundations](#mathematical-foundations-2)
    - [Advanced Topics](#advanced-topics-2)
    - [Security Considerations](#security-considerations-3)
    - [Integration with Blockchain](#integration-with-blockchain-3)
      - [Example: Immutable Audit Trails for RCS Messages](#example-immutable-audit-trails-for-rcs-messages)
        - [Integration Steps](#integration-steps-1)
        - [Advantages](#advantages-2)
  - [5. Web Real-Time Communication (WebRTC)](#5-web-real-time-communication-webrtc)
    - [Overview](#overview-4)
    - [Features](#features-4)
    - [Technical Implementation](#technical-implementation-4)
      - [Sample Source Code](#sample-source-code-4)
        - [Installation](#installation-4)
        - [Server-Side Code (`server.js`)](#server-side-code-serverjs)
        - [Client-Side Code (`public/index.html`)](#client-side-code-publicindexhtml)
    - [Explanation](#explanation-4)
      - [Server-Side](#server-side)
      - [Client-Side](#client-side)
    - [Mathematical Foundations](#mathematical-foundations-3)
    - [Security Considerations](#security-considerations-4)
    - [Integration with Blockchain](#integration-with-blockchain-4)
      - [Example: Decentralized Signaling with Blockchain](#example-decentralized-signaling-with-blockchain)
        - [Pseudo-code for Decentralized Signaling Using Blockchain](#pseudo-code-for-decentralized-signaling-using-blockchain)
    - [Advantages](#advantages-3)
  - [6. Secure Scuttlebutt (SSB)](#6-secure-scuttlebutt-ssb)
    - [Overview](#overview-5)
    - [Features](#features-5)
    - [Technical Implementation](#technical-implementation-5)
      - [Sample Source Code](#sample-source-code-5)
        - [Installation](#installation-5)
        - [SSB Client Example](#ssb-client-example)
    - [Explanation](#explanation-5)
    - [Mathematical Foundations](#mathematical-foundations-4)
    - [Advanced Topics](#advanced-topics-3)
    - [Security Considerations](#security-considerations-5)
    - [Integration with Blockchain](#integration-with-blockchain-5)
      - [Example: Blockchain-Verified Identity in SSB](#example-blockchain-verified-identity-in-ssb)
        - [Solidity Smart Contract for Identity Verification](#solidity-smart-contract-for-identity-verification)
        - [Integration Steps](#integration-steps-2)
    - [Advantages](#advantages-4)
  - [**2. Popular IM Applications and Their Protocols**](#2-popular-im-applications-and-their-protocols)
    - [**a. WhatsApp**](#a-whatsapp)
    - [**b. Telegram**](#b-telegram)
    - [**c. Signal**](#c-signal)
    - [**d. Slack**](#d-slack)
    - [**e. Microsoft Teams**](#e-microsoft-teams)
    - [**f. Discord**](#f-discord)
    - [**g. Element (formerly Riot.im)**](#g-element-formerly-riotim)
  - [**3. Latest Developments in IM Protocols**](#3-latest-developments-in-im-protocols)
    - [**a. Matrix Protocol Enhancements**](#a-matrix-protocol-enhancements)
    - [**b. RCS (Rich Communication Services)**](#b-rcs-rich-communication-services)
    - [**c. WebRTC (Web Real-Time Communication)**](#c-webrtc-web-real-time-communication)
    - [**d. Secure Scuttlebutt (SSB)**](#d-secure-scuttlebutt-ssb)
  - [**4. Overview of Currently Known IM Protocols**](#4-overview-of-currently-known-im-protocols)

---

## 1. Extensible Messaging and Presence Protocol (XMPP)

### Overview

The **Extensible Messaging and Presence Protocol (XMPP)** is an open-standard, XML-based protocol designed for real-time communication. Initially known as Jabber, XMPP facilitates decentralized, secure, and extensible messaging, making it suitable for various applications ranging from simple chat clients to complex multi-user systems.

### Features

- **Decentralized Architecture:** No central server authority; multiple servers interoperate to facilitate communication.
- **Extensibility (XMPP Extension Protocols - XEPs):** Modular extensions allow for adding new functionalities without altering the core protocol.
- **Presence Information:** Real-time status updates (e.g., online, away, do not disturb).
- **Multi-User Chat (MUC):** Supports group conversations in chat rooms.
- **Security:** Supports TLS for encrypted connections and SASL for authentication.
- **Federation:** Enables interoperability between different XMPP servers and clients.

### Applications

- **Jabber Clients:** Such as Pidgin, Conversations, and Gajim.
- **Legacy Services:** Google Talk (historically used XMPP).
- **Enterprise Solutions:** Custom communication platforms leveraging XMPP for internal messaging.

### Technical Implementation

#### Sample Source Code

A practical implementation involves creating an XMPP client using Python's `slixmpp` library. Below is a comprehensive example demonstrating connection, authentication, message sending, and reception.

##### Installation

```bash
pip install slixmpp
```

##### XMPP Chat Client

```python
import sys
import asyncio
import slixmpp
from slixmpp.exceptions import IqError, IqTimeout

class SimpleXMPPClient(slixmpp.ClientXMPP):
    def __init__(self, jid, password, recipient, message):
        super().__init__(jid, password) # slixmpp.client library initialized with a userID and password

        self.recipient = recipient
        self.message = message

        # Event handlers
        self.add_event_handler("session_start", self.start)
        self.add_event_handler("message", self.message_received)

    async def start(self, event):
        try:
            self.send_presence()
            await self.get_roster()

            # Send a message to the recipient
            self.send_message(mto=self.recipient,
                             mbody=self.message,
                             mtype='chat')
            print(f"Message sent to {self.recipient}: {self.message}")

            # Keep the connection alive to receive messages
            await asyncio.sleep(1)
            self.disconnect()
        except IqError as e:
            print(f"Error retrieving roster: {e.iq['error']['condition']}")
            self.disconnect()
        except IqTimeout:
            print("Server is taking too long to respond")
            self.disconnect()

    def message_received(self, msg):
        if msg['type'] in ('chat', 'normal'):
            print(f"New message from {msg['from']}: {msg['body']}")

if __name__ == '__main__':
    if len(sys.argv) != 5:
        print("Usage: python simple_xmpp_client.py <jid> <password> <recipient_jid> <message>")
        sys.exit(1)

    jid = sys.argv[1]
    password = sys.argv[2]
    recipient = sys.argv[3]
    message = sys.argv[4]

    # Create and connect the client
    xmpp = SimpleXMPPClient(jid, password, recipient, message)
    xmpp.connect()
    xmpp.process(forever=False)
```

##### Execution

Run the script with the required arguments:

```bash
python simple_xmpp_client.py user@jabber.org password recipient@jabber.org "Hello, this is a test message!"
```

##### Explanation

- **Initialization:** The `SimpleXMPPClient` class initializes the client with user credentials and sets up event handlers for session start and message reception.
- **Session Start Handler (`start`):**
  - Sends initial presence to inform the server and contacts of the client's availability.
  - Retrieves the roster (contact list) to ensure proper setup.
  - Sends a chat message to the specified recipient.
  - Disconnects after a short delay to allow message delivery.
- **Message Handler (`message_received`):** Listens for incoming messages and prints them to the console.

#### Setting Up an XMPP Server

Deploying a personal XMPP server provides control over messaging infrastructure. `ejabberd` is a robust, scalable XMPP server implementation.

##### Installation (Using Docker)

```bash
docker run -d --name ejabberd \
  -p 5222:5222 -p 5269:5269 -p 5280:5280 \
  -e EJABBERD_DOMAIN=your.domain.com \
  -e EJABBERD_HOST=0.0.0.0 \
  ejabberd/ecs:latest
```

##### Configuration

Edit the `ejabberd.yml` configuration file to set domain, admin users, and enable necessary modules like MUC.

##### Managing Users

```bash
docker exec -it ejabberd /bin/bash
ejabberdctl register username your.domain.com password
```

### Security Considerations

Implementing XMPP securely involves multiple layers:

- **Encryption:** Use TLS for data in transit and consider implementing end-to-end encryption (E2EE) using XEP-0384 (OMEMO) or XEP-0278 (HTTP File Upload with encryption).
- **Authentication:** Employ strong authentication mechanisms like SCRAM-SHA-1 for SASL.
- **Server Hardening:** Regularly update the server, disable unused modules, and monitor for vulnerabilities.
- **Privacy Controls:** Implement privacy lists and subscription management to control visibility and message reception.

### Integration with Blockchain

Integrating XMPP with blockchain can enhance decentralized communication systems. Potential integrations include:

- **Decentralized Identity (DID):** Utilize blockchain-based DIDs for authenticating XMPP users, enhancing trust without centralized authorities.
- **Immutable Messaging Logs:** Leverage blockchain to store hashes of messages for verifiable and tamper-proof communication logs.
- **Smart Contracts for Access Control:** Implement smart contracts to manage permissions and access to XMPP chat rooms or resources.
- **Token-Based Incentives:** Use blockchain tokens to incentivize participation or moderation within XMPP-based communities.

#### Example: Decentralized Authentication

Implement a custom SASL mechanism that verifies user identities against a blockchain ledger, ensuring that only verified users can authenticate and participate in communications.

```python
# Pseudo-code for integrating blockchain-based DID with XMPP SASL
class BlockchainSASLMechanism(slixmpp.SASLMechanism):
    name = 'BLOCKCHAIN_DID'
    priority = 50

    async def auth(self, authzid, authcid, password):
        # Retrieve DID from authcid
        did = authcid
        # Verify DID signature on blockchain
        if verify_did_signature(did, password):
            self.xmpp.authenticated = True
            return True
        return False
```

---

## 2. Signal Protocol

### Overview

The **Signal Protocol**, initially known as the Axolotl Protocol, is a state-of-the-art cryptographic protocol designed for secure, end-to-end encrypted communication. It underpins applications like Signal, WhatsApp (for Secret Chats), and Facebook Messenger (for Secret Conversations), providing robust security guarantees.

### Features

- **End-to-End Encryption (E2EE):** Ensures that only the communicating parties can read the messages.
- **Forward Secrecy:** Compromised keys do not affect the confidentiality of past communications.
- **Double Ratchet Algorithm:** Combines Diffie-Hellman (DH) key exchange with ratcheting for evolving encryption keys.
- **Prekeys:** Facilitates secure session initiation even when the recipient is offline.
- **Asynchronous Communication:** Supports message exchange without both parties needing to be online simultaneously.

### Technical Implementation

Implementing the Signal Protocol requires meticulous adherence to its cryptographic specifications. Utilizing existing libraries like `libsignal-protocol-java` ensures compliance and security.

#### Sample Source Code

Below is a Java example using the `libsignal-protocol-java` library to demonstrate key generation, session setup, message encryption, and decryption.

##### Installation

Add the library to your project using Maven or Gradle.

**Maven:**

```xml
<dependency>
    <groupId>org.signal</groupId>
    <artifactId>libsignal-protocol-java</artifactId>
    <version>2.8.2</version>
</dependency>
```

**Gradle:**

```groovy
implementation 'org.signal:libsignal-protocol-java:2.8.2'
```

##### Signal Protocol Example

```java
import org.signal.protocol.*;
import org.signal.protocol.state.InMemorySignalProtocolStore;
import org.signal.protocol.state.SignalProtocolAddress;

import java.util.HashMap;
import java.util.Map;

public class SignalProtocolExample {

    public static void main(String[] args) throws Exception {
        // Initialize identity keys for Alice
        IdentityKeyPair aliceIdentity = Curve.generateKeyPair();
        PreKeyBundle alicePreKey = new PreKeyBundle(
                1, // registrationId
                1, // deviceId
                1, // preKeyId
                alicePreKeyPair.getPublicKey(),
                aliceIdentity.getPublicKey(),
                aliceIdentity.getPublicKey().serialize()
        );

        // Initialize SignalProtocolStore for Bob
        SignalProtocolStore bobStore = new InMemorySignalProtocolStore(Curve.generateKeyPair());

        // Bob builds a session with Alice's prekey bundle
        SessionBuilder bobSessionBuilder = new SessionBuilder(bobStore, new SignalProtocolAddress("alice", 1));
        bobSessionBuilder.process(alicePreKey);

        // Bob encrypts a message to Alice
        SessionCipher bobCipher = new SessionCipher(bobStore, new SignalProtocolAddress("alice", 1));
        CiphertextMessage ciphertext = bobCipher.encrypt("Hello Alice!".getBytes());

        System.out.println("Encrypted Message: " + ciphertext.serialize());

        // Alice decrypts the message
        SignalProtocolStore aliceStore = new InMemorySignalProtocolStore(aliceIdentity);
        SessionCipher aliceCipher = new SessionCipher(aliceStore, new SignalProtocolAddress("bob", 1));
        byte[] decrypted = aliceCipher.decrypt(new PreKeySignalMessage(ciphertext.serialize()));

        System.out.println("Decrypted Message: " + new String(decrypted));
    }
}
```

##### Explanation

- **Key Generation:**
  - **Alice:** Generates her identity key pair and publishes a prekey bundle.
  - **Bob:** Initializes his `SignalProtocolStore` and establishes a session with Alice using her prekey bundle.
- **Session Setup:**
  - **Bob** processes Alice's prekey to establish a secure session.
- **Message Encryption:**
  - **Bob** uses `SessionCipher` to encrypt a plaintext message destined for Alice.
- **Message Decryption:**
  - **Alice** decrypts the received ciphertext using her `SessionCipher`.

### Mathematical Foundations

- **Elliptic Curve Diffie-Hellman (ECDH):** Facilitates the secure exchange of keys.
- **Double Ratchet Algorithm:** Ensures forward secrecy and post-compromise security by frequently updating encryption keys.
- **Hash Functions and HMAC:** Provide data integrity and authentication.

### Advanced Topics

- **Asynchronous Session Initiation:** Utilizing prekeys allows Bob to initiate sessions even if Alice is offline, leveraging stored prekeys on the server.
- **Ephemeral Keys:** Short-lived keys reduce the risk of key compromise affecting multiple messages.
- **Key Derivation Functions (KDF):** Derive session keys from shared secrets using secure KDFs like HKDF.

### Security Considerations

The Signal Protocol's security relies on:

- **Proper Implementation:** Utilize vetted libraries to prevent vulnerabilities.
- **Secure Storage:** Protect identity keys and session states from unauthorized access.
- **Regular Updates:** Keep cryptographic primitives and libraries up-to-date to mitigate emerging threats.
- **Mitigation of Side-Channel Attacks:** Implement constant-time algorithms to prevent timing attacks.

### Integration with Blockchain

Blockchain technology can enhance the Signal Protocol in several ways:

- **Decentralized Key Distribution:** Utilize blockchain for storing and verifying prekeys, eliminating reliance on centralized servers.
- **Immutable Audit Trails:** Record key exchanges and message metadata on a blockchain for transparency and accountability.
- **Smart Contract-Based Access Control:** Manage permissions and encryption parameters dynamically through smart contracts.
- **Token Incentives for Network Participation:** Encourage users to maintain availability of prekeys by rewarding participation with blockchain-based tokens.

#### Example: Decentralized Prekey Distribution

Implement a smart contract on Ethereum to store and retrieve users' prekeys securely.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract PrekeyStore {
    struct PrekeyBundle {
        uint256 registrationId;
        uint256 deviceId;
        uint256 preKeyId;
        bytes publicPreKey;
        bytes identityKey;
    }

    mapping(address => PrekeyBundle[]) private userPrekeys;

    event PrekeyAdded(address indexed user, uint256 preKeyId);

    function addPrekey(
        uint256 registrationId,
        uint256 deviceId,
        uint256 preKeyId,
        bytes memory publicPreKey,
        bytes memory identityKey
    ) public {
        PrekeyBundle memory bundle = PrekeyBundle({
            registrationId: registrationId,
            deviceId: deviceId,
            preKeyId: preKeyId,
            publicPreKey: publicPreKey,
            identityKey: identityKey
        });
        userPrekeys[msg.sender].push(bundle);
        emit PrekeyAdded(msg.sender, preKeyId);
    }

    function getPrekeys(address user) public view returns (PrekeyBundle[] memory) {
        return userPrekeys[user];
    }
}
```

##### Integration Steps

1. **Deployment:** Deploy the `PrekeyStore` smart contract on a blockchain platform like Ethereum.
2. **Prekey Registration:** Users submit their prekeys to the smart contract, storing them on-chain.
3. **Session Initiation:** When initiating a session, users retrieve the recipient's prekeys from the blockchain to establish a secure connection.

##### Advantages

- **Decentralization:** Removes the need for centralized servers to host prekeys.
- **Transparency:** Blockchain's immutable ledger ensures the integrity of prekeys.
- **Security:** Smart contracts enforce access controls and prevent unauthorized modifications.

---

## 3. MTProto (Telegram)

### Overview

**MTProto** is Telegram's proprietary protocol engineered for high-speed and secure communication. It optimizes data transmission for mobile environments and low-bandwidth scenarios, balancing efficiency with robust security mechanisms.

### Features

- **Efficiency:** Designed for minimal latency and high throughput.
- **Security:** Combines symmetric and asymmetric encryption to protect data.
- **Data Synchronization:** Ensures messages are consistently synchronized across multiple devices.
- **Multiplexing:** Supports multiple simultaneous requests over a single connection.
- **Proxy Support:** Facilitates connections through various proxy types to enhance privacy and bypass censorship.

### Technical Implementation

Implementing MTProto involves understanding its client-server architecture, encryption layers, and handling of different message types. Below is an example of building a basic MTProto client using the `@mtproto/core` library in Node.js.

#### Sample Source Code

##### Installation

```bash
npm install @mtproto/core
```

##### MTProto Client Example

```javascript
const MTProto = require('@mtproto/core').default;
const input = require('@mtproto/core/src/plugins/input-plugins/index.js').default;

// Initialize MTProto instance
const mtproto = new MTProto({
  api_id: YOUR_API_ID, // Obtain from https://my.telegram.org
  api_hash: 'YOUR_API_HASH', // Obtain from https://my.telegram.org
  test: false, // Set to true for testing
});

// Function to send authentication code
async function sendCode(phoneNumber) {
  const { phone_code_hash } = await mtproto.call('auth.sendCode', {
    phone_number: phoneNumber,
    settings: {
      _: 'codeSettings',
    },
  });
  return phone_code_hash;
}

// Function to sign in with received code
async function signIn(phoneNumber, phoneCodeHash, code) {
  try {
    const result = await mtproto.call('auth.signIn', {
      phone_number: phoneNumber,
      phone_code_hash: phoneCodeHash,
      phone_code: code,
    });
    console.log('Signed in:', result);
  } catch (error) {
    console.error('Sign-in error:', error);
  }
}

// Function to send a message
async function sendMessage(peerUserId, peerAccessHash, message) {
  const result = await mtproto.call('messages.sendMessage', {
    peer: {
      _: 'inputPeerUser',
      user_id: peerUserId,
      access_hash: peerAccessHash,
    },
    message: message,
    random_id: BigInt(Math.floor(Math.random() * 1e16)),
  });
  console.log('Message sent:', result);
}

// Example Usage
(async () => {
  const phoneNumber = '+1234567890';
  const phoneCodeHash = await sendCode(phoneNumber);
  console.log('Phone code hash:', phoneCodeHash);

  const code = '12345'; // Replace with the actual code received via SMS
  await signIn(phoneNumber, phoneCodeHash, code);

  // Replace with actual peer user ID and access hash
  const peerUserId = 123456789;
  const peerAccessHash = '0123456789abcdef';
  await sendMessage(peerUserId, peerAccessHash, 'Hello from MTProto client!');
})();
```

##### Explanation

- **Initialization:** Configure the MTProto instance with `api_id` and `api_hash` obtained from Telegram's developer portal.
- **Authentication:**
  - **sendCode:** Requests Telegram to send an authentication code to the specified phone number.
  - **signIn:** Uses the received code and `phone_code_hash` to authenticate the user.
- **Messaging:**
  - **sendMessage:** Sends a text message to a specified user identified by `user_id` and `access_hash`.

### Mathematical Foundations

- **RSA Encryption:** Utilized for initial key exchange during session establishment.
- **AES Encryption:** Symmetric encryption for message confidentiality.
- **SHA-1 and SHA-256:** Hash functions for data integrity and signature verification.
- **Diffie-Hellman Key Exchange:** Facilitates secure key agreement between parties.

### Advanced Topics

- **Proxy Integration:** Implement SOCKS5 or MTProxy proxies to obfuscate traffic and bypass network restrictions.
- **Handling Updates:** Manage incoming messages, updates, and server responses using MTProto's update mechanism.
- **Data Compression:** Utilize built-in compression to reduce bandwidth usage, especially beneficial for mobile networks.

### Security Considerations

MTProto's security is multifaceted:

- **Layered Encryption:** Combines symmetric and asymmetric encryption to secure data at different layers.
- **Session Encryption Keys:** Each session has unique encryption keys to prevent key reuse and enhance security.
- **Randomized Message IDs:** Protects against replay attacks by ensuring message uniqueness.
- **Code Verification:** Ensures that only users with the correct authentication code can access the account.

### Integration with Blockchain

Blockchain can augment MTProto-based systems through:

- **Decentralized Authentication:** Replace traditional phone-based authentication with blockchain-based identity verification.
- **Immutable Message Logs:** Store hashes of messages on a blockchain to ensure message integrity and non-repudiation.
- **Smart Contract-Based Permissions:** Manage access controls and user permissions via smart contracts.
- **Tokenized Incentives:** Reward users for maintaining network nodes or contributing to message relaying with blockchain tokens.

#### Example: Blockchain-Based Message Integrity

Store the hash of each sent message on a blockchain to verify integrity upon retrieval.

```javascript
const { ethers } = require('ethers');

// Smart contract interface
const contractABI = [
  "function storeMessageHash(bytes32 hash) public",
  "function verifyMessageHash(bytes32 hash) public view returns (bool)"
];
const contractAddress = '0xYourContractAddress';

// Initialize Ethereum provider and contract
const provider = new ethers.providers.InfuraProvider('mainnet', 'YOUR_INFURA_PROJECT_ID');
const wallet = new ethers.Wallet('YOUR_PRIVATE_KEY', provider);
const contract = new ethers.Contract(contractAddress, contractABI, wallet);

// Function to store message hash
async function storeMessageHash(message) {
  const hash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(message));
  const tx = await contract.storeMessageHash(hash);
  await tx.wait();
  console.log('Message hash stored:', hash);
}

// Function to verify message hash
async function verifyMessage(message) {
  const hash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes(message));
  const isValid = await contract.verifyMessageHash(hash);
  console.log(`Message integrity valid: ${isValid}`);
}

// Usage after sending a message
await storeMessageHash('Hello from MTProto client!');
await verifyMessage('Hello from MTProto client!');
```

##### Advantages

- **Integrity Verification:** Ensures that messages have not been tampered with.
- **Transparency:** Blockchain's public ledger allows for independent verification.
- **Non-Repudiation:** Users cannot deny sending a message once its hash is recorded on-chain.

---

## 4. Rich Communication Services (RCS)

### Overview

**Rich Communication Services (RCS)** is an advanced messaging protocol designed to replace traditional SMS, offering features comparable to modern IM applications. Developed by the GSM Association (GSMA), RCS aims to provide a standardized, carrier-backed messaging experience across different devices and networks.

### Features

- **Enhanced Messaging:** Supports multimedia messages, group chats, read receipts, typing indicators, and larger file transfers.
- **Business Messaging:** Facilitates interactions between businesses and customers through rich media, interactive buttons, and forms.
- **Integration with Native Messaging Apps:** Seamlessly integrates with the device's default messaging application.
- **End-to-End Encryption (E2EE):** Offers secure messaging, with availability varying by carrier and device.
- **IP-Based Communication:** Utilizes internet protocols for message delivery, reducing reliance on SMS infrastructure.

### Technical Implementation

Implementing RCS typically involves leveraging carrier services or third-party platforms that support the protocol. Google's Jibe RCS Platform provides APIs for developers to build RCS-enabled applications.

#### Sample Source Code

Below is an example of sending an RCS message using Google's Business Messages API with Python.

##### Installation

```bash
pip install google-api-python-client google-auth-httplib2 google-auth-oauthlib
```

##### RCS Messaging Example

```python
from google.oauth2 import service_account
from googleapiclient.discovery import build

# Path to your service account key file
SERVICE_ACCOUNT_FILE = 'path/to/service-account.json'

# Define the required scopes
SCOPES = ['https://www.googleapis.com/auth/businessmessages']

# Authenticate and construct service
credentials = service_account.Credentials.from_service_account_file(
    SERVICE_ACCOUNT_FILE, scopes=SCOPES)
service = build('businessmessages', 'v1', credentials=credentials)

# Define conversation and agent IDs
CONVERSATION_ID = 'conversation-id'
AGENT_ID = 'agent-id'

# Define the message content
message = {
    "messageId": "unique-message-id",
    "text": "Hello, this is an RCS message!",
    "suggestions": [
        {
            "reply": {
                "text": "Yes"
            }
        },
        {
            "reply": {
                "text": "No"
            }
        }
    ]
}

# Send the message
response = service.conversations().messages().create(
    parent=f'conversations/{CONVERSATION_ID}',
    body={
        "agent": f'agents/{AGENT_ID}',
        "message": message
    }
).execute()

print('Message sent:', response)
```

##### Explanation

- **Authentication:** Uses a service account with appropriate scopes to authenticate API requests securely.
- **Message Composition:** Constructs an RCS message with interactive suggestions (quick replies) to enhance user engagement.
- **Sending the Message:** Utilizes the `conversations.messages.create` method to dispatch the message within a specific conversation context.

### Mathematical Foundations

- **OAuth 2.0 Protocol:** Secures API access through token-based authentication.
- **JSON Web Tokens (JWT):** Facilitates secure data transmission between client and server.
- **Data Serialization:** Ensures structured and standardized message formats for interoperability.

### Advanced Topics

- **Interactive Rich Cards:** Implement dynamic content such as carousels, forms, and media cards to enrich user interactions.
- **Webhook Integrations:** Set up webhooks to handle incoming messages and events in real-time.
- **Analytics and Reporting:** Utilize APIs to gather insights on message delivery, user engagement, and interaction patterns.

### Security Considerations

Ensuring security in RCS implementations involves:

- **Encryption:** Enable E2EE where supported to protect message confidentiality.
- **Authentication:** Use robust authentication mechanisms to verify sender identities.
- **Data Privacy:** Comply with data protection regulations (e.g., GDPR) by managing user data responsibly.
- **Access Controls:** Implement role-based access to manage permissions and capabilities within messaging platforms.

### Integration with Blockchain

Blockchain can enhance RCS by introducing decentralized elements and immutable record-keeping:

- **Decentralized Messaging Infrastructure:** Utilize blockchain to host signaling servers, reducing reliance on centralized entities.
- **Immutable Audit Logs:** Record message metadata on-chain for verifiable auditing and compliance.
- **Smart Contract-Based Billing:** Automate billing processes for business messaging through smart contracts.
- **Tokenized Incentives for Service Providers:** Reward participants in the messaging ecosystem with blockchain tokens for maintaining infrastructure or providing services.

#### Example: Immutable Audit Trails for RCS Messages

Implement a smart contract to log hashes of RCS messages for audit purposes.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract RCSAuditTrail {
    event MessageLogged(bytes32 indexed messageHash, address sender, uint256 timestamp);

    mapping(bytes32 => bool) private loggedMessages;

    function logMessage(string memory message) public {
        bytes32 messageHash = keccak256(abi.encodePacked(message, msg.sender, block.timestamp));
        require(!loggedMessages[messageHash], "Message already logged");
        loggedMessages[messageHash] = true;
        emit MessageLogged(messageHash, msg.sender, block.timestamp);
    }

    function verifyMessage(string memory message, address sender, uint256 timestamp) public view returns (bool) {
        bytes32 messageHash = keccak256(abi.encodePacked(message, sender, timestamp));
        return loggedMessages[messageHash];
    }
}
```

##### Integration Steps

1. **Deployment:** Deploy the `RCSAuditTrail` contract on a blockchain platform.
2. **Message Logging:** After sending an RCS message, compute its hash and call `logMessage` to record it on-chain.
3. **Verification:** Use `verifyMessage` to confirm the existence and integrity of a message.

##### Advantages

- **Integrity Verification:** Ensures that messages have not been altered post-logging.
- **Transparency:** Blockchain's public ledger allows for independent verification of message logs.
- **Non-Repudiation:** Prevents senders from denying the sending of a particular message.
  

## 5. Web Real-Time Communication (WebRTC)

### Overview

Web Real-Time Communication (WebRTC) is an open-source project enabling real-time peer-to-peer communication directly between browsers and mobile applications. While not exclusively an IM protocol, WebRTC underpins modern communication applications by facilitating real-time voice, video, and data sharing without the need for plugins.

### Features

- **Peer-to-Peer Communication:** Establishes direct connections between clients for low-latency interactions.
- **Media Streaming:** Supports audio and video streaming with adaptive bitrate and codec negotiation.
- **Data Channels:** Enables arbitrary data transmission between peers for chat, file sharing, and more.
- **Security:** Enforces encryption (DTLS and SRTP) for all communications.
- **Cross-Platform Support:** Compatible with major browsers and mobile platforms.

### Technical Implementation

Implementing WebRTC involves handling signaling, establishing peer connections, and managing media streams or data channels. Below is an example of building a simple WebRTC data channel for text messaging using Node.js and Socket.io for signaling.

#### Sample Source Code

##### Installation

Initialize a new Node.js project and install the required dependencies.

```bash
mkdir webrtc-chat
cd webrtc-chat
npm init -y
npm install express socket.io
```

##### Server-Side Code (`server.js`)

```javascript
const express = require('express');
const http = require('http');
const socketIo = require('socket.io');

const app = express();
const server = http.createServer(app);
const io = socketIo(server);

app.use(express.static('public'));

io.on('connection', (socket) => {
    console.log('New client connected');

    // Relay signaling messages
    socket.on('signal', (data) => {
        io.to(data.target).emit('signal', {
            source: socket.id,
            signal: data.signal
        });
    });

    socket.on('join', () => {
        socket.join('chatroom');
        io.to(socket.id).emit('joined', socket.id);
    });

    socket.on('disconnect', () => {
        console.log('Client disconnected');
    });
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => console.log(`Server running on port ${PORT}`));
```

##### Client-Side Code (`public/index.html`)

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>WebRTC Chat</title>
    <style>
        body { font-family: Arial, sans-serif; }
        #chat { width: 300px; height: 200px; }
        #message { width: 200px; }
    </style>
</head>
<body>
    <h1>WebRTC Peer-to-Peer Chat</h1>
    <textarea id="chat" readonly></textarea><br>
    <input type="text" id="message" placeholder="Type a message">
    <button id="send">Send</button>

    <script src="/socket.io/socket.io.js"></script>
    <script>
        const socket = io();
        const chat = document.getElementById('chat');
        const messageInput = document.getElementById('message');
        const sendButton = document.getElementById('send');

        let peerConnection;
        let dataChannel;
        const config = {
            iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
        };

        socket.emit('join');

        socket.on('joined', (id) => {
            console.log('Joined with ID:', id);
            initiateConnection();
        });

        socket.on('signal', async (data) => {
            if (data.signal.type === 'offer') {
                await peerConnection.setRemoteDescription(new RTCSessionDescription(data.signal));
                const answer = await peerConnection.createAnswer();
                await peerConnection.setLocalDescription(answer);
                socket.emit('signal', { target: data.source, signal: peerConnection.localDescription });
            } else if (data.signal.type === 'answer') {
                await peerConnection.setRemoteDescription(new RTCSessionDescription(data.signal));
            } else if (data.signal.candidate) {
                try {
                    await peerConnection.addIceCandidate(new RTCIceCandidate(data.signal));
                } catch (e) {
                    console.error('Error adding received ice candidate', e);
                }
            }
        });

        function initiateConnection() {
            peerConnection = new RTCPeerConnection(config);

            peerConnection.onicecandidate = (event) => {
                if (event.candidate) {
                    socket.emit('signal', { target: 'all', signal: event.candidate });
                }
            };

            peerConnection.ondatachannel = (event) => {
                const receiveChannel = event.channel;
                receiveChannel.onmessage = (e) => {
                    chat.value += `Peer: ${e.data}\n`;
                };
            };

            dataChannel = peerConnection.createDataChannel('chat');
            dataChannel.onopen = () => console.log('Data channel open');
            dataChannel.onmessage = (e) => {
                chat.value += `Peer: ${e.data}\n`;
            };

            peerConnection.createOffer().then((offer) => {
                peerConnection.setLocalDescription(offer);
                socket.emit('signal', { target: 'all', signal: offer });
            }).catch((error) => {
                console.error('Error creating offer:', error);
            });
        }

        sendButton.onclick = () => {
            const message = messageInput.value;
            if (message && dataChannel && dataChannel.readyState === 'open') {
                chat.value += `You: ${message}\n`;
                dataChannel.send(message);
                messageInput.value = '';
            }
        };
    </script>
</body>
</html>
```

### Explanation

#### Server-Side

- **Serves Static Files:** Serves static files from the `public` directory.
- **Manages Socket Connections:** Handles signaling by relaying offer, answer, and ICE candidate messages between peers.

#### Client-Side

- **Connects to Signaling Server:** Uses Socket.io to connect to the signaling server.
- **Initiates WebRTC Peer Connection:** Configures STUN servers for NAT traversal.
- **Handles Signaling Messages:** Manages the creation and reception of offers, answers, and ICE candidates.
- **Establishes Data Channel:** Creates a data channel for text messaging.
- **Messaging Functionality:** Sends and receives messages over the data channel, displaying them in the chat area.

### Mathematical Foundations

- **ICE (Interactive Connectivity Establishment):** Facilitates NAT traversal using STUN/TURN servers.
- **DTLS (Datagram Transport Layer Security):** Ensures secure communication channels.
- **SRTP (Secure Real-Time Transport Protocol):** Protects media streams with encryption and authentication.

### Security Considerations

WebRTC incorporates several security measures:

- **Mandatory Encryption:** All data channels and media streams are encrypted using DTLS and SRTP.
- **Same-Origin Policy:** Restricts access to WebRTC APIs to scripts from the same origin unless explicitly permitted.
- **Permission Controls:** Browsers require user consent for accessing media devices (camera, microphone).
- **Peer Verification:** Implement mechanisms to verify peer identities to prevent man-in-the-middle attacks.

### Integration with Blockchain

Blockchain can enhance WebRTC by introducing decentralized signaling and identity verification:

- **Decentralized Signaling:** Utilize blockchain-based protocols to handle signaling without centralized servers, enhancing resilience and censorship resistance.
- **Immutable Identity Verification:** Leverage blockchain's immutable ledger to verify and authenticate peer identities securely.
- **Smart Contract-Based Session Management:** Manage session states and access controls through smart contracts, enabling programmable and automated communication workflows.
- **Tokenized Resource Allocation:** Implement token-based incentives for maintaining signaling nodes or providing TURN server resources.

#### Example: Decentralized Signaling with Blockchain

Implement a decentralized signaling mechanism using blockchain to record and retrieve signaling data.

##### Pseudo-code for Decentralized Signaling Using Blockchain

```javascript
// Pseudo-code for decentralized signaling using blockchain

// Smart contract functions
function registerPeer(address peer, string signalingData) public {
    // Store signaling data associated with the peer's address
}

function getSignalingData(address peer) public view returns (string memory) {
    // Retrieve signaling data for the peer
}

// Client-side interaction
async function registerSignalingData(signalingData) {
    await contract.registerPeer(userAddress, signalingData);
}

async function retrieveSignalingData(peerAddress) {
    return await contract.getSignalingData(peerAddress);
}

// During WebRTC connection setup
const signalingData = JSON.stringify(peerConnection.localDescription);
await registerSignalingData(signalingData);

// Retrieve and apply peer's signaling data
const peerSignalingData = await retrieveSignalingData(peerAddress);
const remoteDescription = new RTCSessionDescription(JSON.parse(peerSignalingData));
await peerConnection.setRemoteDescription(remoteDescription);
```

### Advantages

- **Decentralization:** Eliminates the need for centralized signaling servers.
- **Censorship Resistance:** Prevents single points of failure and control.
- **Enhanced Security:** Blockchain's immutability ensures the integrity of signaling data.



## 6. Secure Scuttlebutt (SSB)

### Overview

Secure Scuttlebutt (SSB) is a decentralized, peer-to-peer protocol emphasizing privacy and offline-first communication. Unlike traditional IM protocols, SSB does not rely on central servers, making it resilient against censorship and enhancing data ownership.

### Features

- **Decentralization:** No central servers; peers communicate directly or via trusted friends.
- **Cryptographic Identity Verification:** Utilizes public-key cryptography to verify peer identities.
- **Offline Synchronization:** Supports data synchronization when peers reconnect after being offline.
- **Immutable Append-Only Logs:** Each user maintains a log of messages that cannot be altered retroactively.
- **Replication and Gossip Protocols:** Ensures data consistency across distributed networks.

### Technical Implementation

Implementing SSB involves setting up peers, managing cryptographic keys, and handling message replication. Below is an example using the `ssb-client` library in Node.js.

#### Sample Source Code

##### Installation

```bash
npm install ssb-client
```

##### SSB Client Example

```javascript
const ssbClient = require('ssb-client');
const pull = require('pull-stream');

ssbClient((err, sbot) => {
    if (err) throw err;

    // Publish a message
    sbot.publish({
        type: 'post',
        text: 'Hello Secure Scuttlebutt!'
    }, (err, msg) => {
        if (err) throw err;
        console.log('Message published:', msg.key);
    });

    // Subscribe to your own feed
    sbot.createFeedStream({ id: sbot.id })
        .on('data', (msg) => {
            console.log('Received message:', msg.value.content.text);
        });

    // Replicate with a remote peer
    sbot.replicate.connect('net:remote-peer-address:port', (err) => {
        if (err) throw err;
        console.log('Connected to remote peer for replication');
    });
});
```

### Explanation

- **Client Initialization:** Connects to the local SSB server instance.
- **Publishing Messages:** Sends a new post to the user's feed.
- **Subscribing to Feeds:** Listens for new messages from the user's own feed.
- **Replication:** Establishes a connection with a remote peer for data synchronization.

### Mathematical Foundations

- **Public-Key Cryptography (Ed25519):** Secures identity verification and message signatures.
- **Hash Functions (SHA-256):** Ensures data integrity and facilitates content-addressable storage.
- **Gossip Protocols:** Enable efficient and reliable data dissemination across the network.

### Advanced Topics

- **GossipNet Protocol:** Enhances replication efficiency by optimizing peer discovery and data exchange.
- **Encrypted Messaging:** Implement end-to-end encryption for private communications within SSB.
- **Scalability Solutions:** Utilize sharding and distributed hash tables (DHTs) to manage large-scale networks.

### Security Considerations

SSB's security model is inherently robust due to its decentralized nature and cryptographic foundations:

- **Immutable Logs:** Prevents retroactive alterations of message history.
- **Authenticated Feeds:** Ensures that messages are genuinely authored by the purported sender.
- **Peer Verification:** Requires explicit trust relationships, reducing the risk of malicious actors.

### Integration with Blockchain

Blockchain can complement SSB by introducing decentralized governance and enhanced data verification:

- **Decentralized Identity Management:** Utilize blockchain-based DIDs to manage peer identities and trust relationships.
- **Immutable Meta-Data Storage:** Record hashes of SSB logs on a blockchain for external verification and auditing.
- **Smart Contract-Based Access Controls:** Manage permissions and access to SSB data through programmable contracts.
- **Tokenized Incentives for Network Participation:** Reward peers for maintaining network availability and data replication with blockchain tokens.

#### Example: Blockchain-Verified Identity in SSB

Integrate blockchain-based identity verification to enhance trust in SSB peer relationships.

##### Solidity Smart Contract for Identity Verification

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract IdentityRegistry {
    mapping(bytes32 => address) public didToAddress;

    event IdentityRegistered(bytes32 indexed did, address indexed user);

    function registerIdentity(bytes32 did) public {
        require(didToAddress[did] == address(0), "DID already registered");
        didToAddress[did] = msg.sender;
        emit IdentityRegistered(did, msg.sender);
    }

    function verifyIdentity(bytes32 did, address user) public view returns (bool) {
        return didToAddress[did] == user;
    }
}
```

##### Integration Steps

1. **Deployment:** Deploy the `IdentityRegistry` contract on a blockchain platform.
2. **Identity Registration:** Users register their Decentralized Identifiers (DIDs) with their blockchain addresses.
3. **Verification:** Peers verify each other's identities by cross-referencing DIDs with blockchain records.

### Advantages

- **Enhanced Trust:** Blockchain provides a verifiable and tamper-proof record of identities.
- **Decentralization:** Removes reliance on centralized identity authorities.
- **Interoperability:** Enables seamless integration with other blockchain-based systems and services.


## **2. Popular IM Applications and Their Protocols**

### **a. WhatsApp**
- **Protocol:** Utilizes the **Signal Protocol** for end-to-end encryption.
- **Features:** Text messaging, voice and video calls, multimedia sharing, and group chats.
- **Notes:** Proprietary application with strong encryption standards; not based on open protocols.

### **b. Telegram**
- **Protocol:** **MTProto** (Mobile Protocol).
- **Features:** Cloud-based messaging, support for large group chats, bots, file sharing, and secret chats with end-to-end encryption.
- **Notes:** Emphasizes speed and security; proprietary protocol.

### **c. Signal**
- **Protocol:** **Signal Protocol** (also known as the **Axolotl Protocol**).
- **Features:** End-to-end encrypted messaging, voice and video calls, and multimedia sharing.
- **Notes:** Open-source and highly regarded for its security features.

### **d. Slack**
- **Protocol:** Proprietary.
- **Features:** Team communication with channels, direct messaging, integrations with various productivity tools, and file sharing.
- **Notes:** Focused on professional and organizational use; not interoperable with other IM protocols.

### **e. Microsoft Teams**
- **Protocol:** Proprietary (built on Microsoft's communication infrastructure).
- **Features:** Team collaboration, chat, video conferencing, file sharing, and integration with Microsoft 365.
- **Notes:** Integrated within the Microsoft ecosystem; primarily aimed at businesses.

### **f. Discord**
- **Protocol:** Proprietary.
- **Features:** Text, voice, and video communication tailored for gaming communities, with support for servers, channels, and integrations.
- **Notes:** Popular among gamers and online communities; offers rich multimedia support.

### **g. Element (formerly Riot.im)**
- **Protocol:** **Matrix**.
- **Features:** Decentralized chat, end-to-end encryption, bridges to other IM services, and support for various media types.
- **Notes:** Open-source and focuses on interoperability and decentralization.

## **3. Latest Developments in IM Protocols**

### **a. Matrix Protocol Enhancements**
- **Description:** Matrix continues to evolve with improvements in scalability, security, and interoperability. Recent updates focus on enhancing federation capabilities, improving encryption mechanisms, and expanding integration options with other platforms.
- **Impact:** Strengthens Matrix's position as a versatile, open-standard protocol suitable for diverse real-time communication needs.

### **b. RCS (Rich Communication Services)**
- **Description:** RCS is an evolution of the traditional SMS protocol, aiming to provide richer features similar to those found in modern IM apps.
- **Features:** Enhanced messaging with multimedia support, read receipts, typing indicators, and integration with business services.
- **Impact:** Supported by major telecom carriers, RCS seeks to offer a standardized IM experience across different devices and networks.

### **c. WebRTC (Web Real-Time Communication)**
- **Description:** While not exclusively an IM protocol, WebRTC facilitates real-time communication directly between browsers and mobile applications.
- **Features:** Peer-to-peer audio, video, and data sharing without the need for plugins.
- **Impact:** Widely adopted for enabling real-time features in web-based IM applications and collaborative tools.

### **d. Secure Scuttlebutt (SSB)**
- **Description:** A decentralized protocol focused on privacy and offline-first communication.
- **Features:** Peer-to-peer messaging without central servers, cryptographic identity verification, and support for offline data synchronization.
- **Impact:** Appeals to communities prioritizing privacy, decentralization, and resilience against censorship.

## **4. Overview of Currently Known IM Protocols**

| **Protocol** | **Description** | **Key Features** | **Notable Applications** |
|--------------|-----------------|-------------------|--------------------------|
| **XMPP** | Open-standard, XML-based protocol for real-time messaging and presence. | Decentralized, extensible (XEPs), multi-user chat, presence. | Jabber clients, Pidgin, Conversations, Element (via bridging). |
| **IRC** | Traditional protocol for group communication in chat rooms. | Simplicity, channels, lightweight, extensible via IRCv3. | mIRC, HexChat, various open-source clients. |
| **SIMPLE** | SIP-based protocol for instant messaging and presence. | Integration with voice/video, presence information, SIP interoperability. | Enterprise communication systems, certain VoIP services. |
| **Matrix** | Decentralized protocol for real-time communication with interoperability. | Federation, end-to-end encryption, bridges to other protocols, rich media support. | Element, various Matrix clients and bridges. |
| **Signal Protocol** | Cryptographic protocol for secure end-to-end encrypted messaging. | Strong security, forward secrecy, asynchronous communication. | Signal app, WhatsApp (partially), Facebook Messenger (Secret Conversations). |
| **MTProto** | Telegram’s proprietary protocol designed for speed and security. | Fast data transmission, encryption, support for mobile devices. | Telegram. |
| **RCS** | Enhanced SMS protocol with rich features. | Multimedia messaging, read receipts, typing indicators, business messaging. | Supported by major Android devices and carriers. |
| **WebRTC** | Protocol suite for real-time communication in browsers and apps. | Peer-to-peer audio/video/data, low latency, no plugins needed. | Web-based chat applications, collaborative tools like Google Meet and Discord voice channels. |
| **MQTT** | Lightweight messaging protocol for IoT, adapted for certain IM use cases. | Publish-subscribe model, low bandwidth, reliable delivery. | IoT devices, smart home systems, some custom IM applications. |
| **Secure Scuttlebutt (SSB)** | Decentralized, peer-to-peer protocol focused on privacy and offline use. | No central servers, cryptographic identities, offline synchronization. | Various niche applications focused on privacy and decentralization. |
