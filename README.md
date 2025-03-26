# ink! Smart Contracts Tutorial Repository  

Welcome to the **ink! Smart Contracts Tutorial Repository**! 🚀  
This repository provides practical examples of ink! smart contracts, focusing on **cross-contract calls** and **contract upgradeability** techniques in Substrate-based blockchains.  

## 📌 What is ink!?  
[ink!](https://use.ink/) is a Rust-based smart contract framework for Substrate, allowing developers to write WebAssembly-based smart contracts efficiently.  

## 📖 Topics Covered  
Currently, this repository contains:  

1. **Cross-Contract Calls** – Learn how to interact with other ink! smart contracts.  
2. **Contract Upgradeability**  
   - **`set_code_hash` Method** – Upgrade a contract by updating its Wasm code.  
   - **Proxy Pattern** – Implement an upgradeable contract using a proxy contract.  

## 🔧 Getting Started  
To use and deploy these contracts, ensure you have the following installed:  

- Rust & Cargo  
- [`cargo-contract`](https://github.com/paritytech/cargo-contract)  
- A Substrate node (local or testnet)  

### Installation  
```sh
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install --force --locked cargo-contract
```

## 🤝 Contributing  
Contributions are welcome! If you have examples to add or improvements to suggest, feel free to open a pull request.  


---
Happy coding! 🚀  
