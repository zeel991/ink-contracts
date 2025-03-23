# Flipper Smart Contract

The **Flipper** smart contract is a simple ink! contract that stores a boolean value and allows toggling it. This contract is designed for Substrate-based blockchains using the ink! framework.

## Features
- Stores a boolean value that can be flipped.
- Provides a function to toggle the stored value.
- Allows querying the current stored value.
- Supports interaction with external contracts.

## Prerequisites
To deploy and interact with this contract, ensure you have:
- Rust and Cargo installed.
- The ink! CLI set up for smart contract development.
- A Substrate-based blockchain running locally.

## Contract Structure

### Storage
The contract maintains a single boolean value that represents its current state. 

### Constructor
The contract provides two ways to initialize the stored value:
- A constructor that accepts an initial boolean value.
- A default constructor that sets the value to `false`.

### Message Functions
The contract provides two main functions:
- A function to toggle the stored boolean value.
- A function to retrieve the current stored value.

## Deployment & Usage

1. Deploy the **Flipper** contract on a Substrate-based blockchain.
2. Use the provided functions to toggle and retrieve the boolean value.
3. Other contracts, such as the **Caller** contract, can interact with **Flipper** using cross-contract calls.
