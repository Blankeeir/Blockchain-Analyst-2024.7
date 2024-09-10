### Incident Summary

- **August 11, 2024, 23:55:51:** The attacker, identified as **NSPyuNudQ7XcckAm5cCMMkXMbUQKLyBPd2**, deployed the attack contract. [Contract details](https://explorer.onegate.space/contractinfo/0x3efdfcb9db2c944dd0e967eea2152e8e93556ccb).
  
- **August 12, 2024, 01:22:14:** The first attack occurred, with each reentrancy involving 0.1 fWETH. [Transaction details](https://explorer.onegate.space/transactionInfo/0x8cc6a68a58f70c1e9ffcdc3147322c32220c7dcbbf30f6107521f55a6e2118d1).

- Afterward, the attack amounts increased, and the attacker attempted to steal various tokens, including fUSDT. The stolen tokens were transferred to **Flamingo Wrapper**, unwrapped, and transferred cross-chain. bNEO was unwrapped into NEO (e.g., [example](https://explorer.onegate.space/transactionInfo/0x0897bc958256b6447be6ea22c6e9946b76098a7412ee36b472a93901829623b2)). The NEO tokens were then transferred to other parties, such as in [this transaction](https://explorer.onegate.space/transactionInfo/0x8135f10df565ad039eba0617fe8e55d8c7e16e2dd86a4a14ebfc07f148c0ae01), ultimately reaching the account [here](https://explorer.onegate.space/accountprofile/0xe94f7e0926b85eeab2d681d9c45a978d2c7906a0).

- **August 12, 2024, 05:56:31:** The final token theft occurred. [Transaction details](https://explorer.onegate.space/transactionInfo/0x83204033475cf0d626074af4715650797688f5ce8e5ad9e46d39b35cd6660e53).

- The attacker made some faulty transactions, such as attempting to exchange the stolen tokens on Flamingo. [Example of a faulty transaction](https://explorer.onegate.space/transactionInfo/0xd3a66e536f07e97e49864d42f577e403225e67e77107fb2b70e3f53d0de8e845).

- Judging by the speed of operations, the attacker likely carried out the entire attack manually.