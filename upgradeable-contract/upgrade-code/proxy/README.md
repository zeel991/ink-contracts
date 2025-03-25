# Upgradeable Smart Contract with Proxy Pattern(WIP)

## Overview

This project implements an upgradeable smart contract architecture using the **proxy pattern** in ink!, a smart contract framework for Substrate-based blockchains. Instead of deploying a new contract and migrating data, a **proxy contract** delegates all calls to an **implementation contract**, whose address can be updated to enable contract upgrades.

## Architecture

1. **Proxy Contract**:

   - Stores the address of the **implementation contract**.
   - Delegates function calls to the implementation using low-level `delegate_call`.
   - Allows updating the implementation contract’s address to enable upgrades.

2. **Implementation Contract**:

   - Contains the actual business logic.
   - Can be upgraded by deploying a new version and updating the proxy’s stored address.
   - Maintains the same storage layout as the previous version to avoid conflicts.

## How It Works

1. **Deploy Proxy Contract**: The proxy contract acts as the entry point for all interactions.
2. **Deploy Implementation Contract**: The first version of the contract logic is deployed separately.
3. **Set Implementation Address**: The proxy contract is initialized with the implementation contract’s address.
4. **Delegate Calls**: The proxy forwards user interactions to the implementation contract, ensuring all state modifications happen in the proxy’s storage.
5. **Upgrade Logic**: When an upgrade is needed:
   - Deploy a new implementation contract with improved logic.
   - Update the stored address in the proxy contract.
   - Users continue interacting with the proxy, but the logic is now updated.

## Key Considerations for ink! Implementation

- **Storage Layout**: Since ink! contracts use a different storage model than Solidity, it's crucial to ensure that the storage structure in the proxy contract remains unchanged across upgrades.
- **Delegate Calls**: ink! provides a way to perform low-level calls, which can be used to delegate calls to the implementation contract while keeping state in the proxy contract.
- **Access Control**: Only an authorized entity (e.g., the contract owner or governance mechanism) should be allowed to upgrade the implementation contract.
- **Testing & Security**: Before upgrading, thorough testing is required to ensure backward compatibility and avoid breaking changes.

## Benefits

- **Minimized Storage Overhead**: The proxy holds all state, preventing duplication across contract versions.
- **Continuous Upgrades**: New features and bug fixes can be introduced without requiring users to migrate to a new contract.
- **Efficient Gas Usage**: Since users interact with the proxy, only the implementation needs to be redeployed for upgrades.

