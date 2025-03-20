# Caller Smart Contract

The **Caller** smart contract is an ink! contract designed to interact with another contract, specifically the **Flipper** contract. This contract demonstrates cross-contract calls in Substrate-based blockchains using the ink! framework.

## Features
- Stores the account ID of the **Flipper** contract.
- Provides a function to trigger the `flip` function of the **Flipper** contract.

## Prerequisites
To deploy and interact with this contract, ensure you have:
- Rust and Cargo installed.
- The ink! CLI set up for smart contract development.
- A Substrate-based blockchain running locally.
- The **Flipper** contract already deployed on-chain.

## Contract Structure

### Storage
The contract maintains the account ID of the deployed **Flipper** contract. This allows it to send calls to modify its state.

### Constructor
When deploying the **Caller** contract, it requires the account ID of the **Flipper** contract. This account ID is stored in the contract’s state for future interactions.

### Message Function
The contract provides a function to invoke the `flip` function of the **Flipper** contract. It does this using an external call, passing the function selector corresponding to the `flip` function.

## Deployment & Usage

1. Deploy the **Flipper** contract first and note its account ID.
2. Deploy the **Caller** contract, providing the account ID of the deployed **Flipper** contract.
3. Call the function in **Caller** to interact with the **Flipper** contract.

