# Week 2: Fully Homomorphic Encryption (FHE)

## Introduction and Background

[IBM Definition](https://www.ibm.com/topics/homomorphic-encryption#:~:text=Fully%20homomorphic%20encryption%20(FHE)%20is,various%20security%20and%20privacy%20risks): Fully homomorphic encryption (FHE) achieves zero trust by unlocking the value of data on untrusted domains without needing to decrypt it.

### Repositories

- [Microsoft SEAL](https://github.com/microsoft/SEAL): Microsoft SEAL is an easy-to-use open-source (MIT licensed) homomorphic encryption library developed by the Cryptography and Privacy Research Group at Microsoft.
- [Google FHE Compiler for C++](https://github.com/google/fully-homomorphic-encryption.git): Open-source libraries and tools to perform fully homomorphic encryption (FHE) operations on an encrypted data set.

## Main Considerations

- **Security and Privacy**  
  FHE ensures that sensitive data remains encrypted even during computation, effectively eliminating exposure to unauthorized entities. However, the complexity and security of the FHE scheme depend on the underlying mathematical hardness assumptions, such as Ring Learning with Errors (RLWE).

- **Performance Overheads**  
  The primary challenge with FHE is its computational overhead. FHE operations are significantly slower than standard cryptographic operations due to the complexity of handling encrypted data and the need to manage noise growth in ciphertexts. Optimization techniques such as bootstrapping, batching, and key-switching are crucial to making FHE practical for real-world applications.

- **Implementation Complexity**  
  Implementing FHE requires a deep understanding of lattice-based cryptography and numerical methods. Developers must manage parameters like plaintext modulus, noise budget, and encryption keys to ensure both security and efficiency.

**Key Generation Formula:**  
\[
(pk, sk) \leftarrow \text{Gen}(\lambda, n, q)
\]

## Detailed Breakdown of Technical Structure

```cpp
#include "seal/seal.h"

using namespace seal;

int main() {
    // Set encryption parameters
    EncryptionParameters parms(scheme_type::BFV);
    size_t poly_modulus_degree = 8192;
    parms.set_poly_modulus_degree(poly_modulus_degree);
    parms.set_coeff_modulus(CoeffModulus::BFVDefault(poly_modulus_degree));
    parms.set_plain_modulus(PlainModulus::Batching(poly_modulus_degree, 20));

    // Create SEALContext
    SEALContext context(parms);

    // Key generation
    KeyGenerator keygen(context);
    PublicKey public_key = keygen.public_key();
    SecretKey secret_key = keygen.secret_key();
    Encryptor encryptor(context, public_key);
    Decryptor decryptor(context, secret_key);
    Evaluator evaluator(context);

    // Encode and encrypt
    Plaintext plain1("6");
    Plaintext plain2("7");
    Ciphertext encrypted1, encrypted2;
    encryptor.encrypt(plain1, encrypted1);
    encryptor.encrypt(plain2, encrypted2);

    // Homomorphic addition
    Ciphertext encrypted_sum;
    evaluator.add(encrypted1, encrypted2, encrypted_sum);

    // Decrypt result
    Plaintext plain_result;
    decryptor.decrypt(encrypted_sum, plain_result);

    std::cout << "Decrypted sum: " << plain_result.to_string() << std::endl;
    return 0;
}
```

## Optimization and Extension

## Conclusion
