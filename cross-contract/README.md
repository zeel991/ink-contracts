# **Caller Smart Contract**

The **Caller** smart contract is an **ink!** contract designed to interact with another contract, specifically the **Flipper** contract. This contract demonstrates **cross-contract calls** in **Substrate-based blockchains** using the **ink!** framework.

## **Features**
- Stores the **account ID** of the deployed **Flipper** contract.
- Provides a function to trigger the `flip` function of the **Flipper** contract via an **external call**.

## **Contract Structure**

### **Storage**
The contract maintains the **account ID** of the deployed **Flipper** contract. This allows it to send calls to modify the **Flipper** contract’s state.

### **Constructor**
When deploying the **Caller** contract, it requires the **account ID** of the **Flipper** contract. This **account ID** is stored in the contract’s state for future interactions.

### **Message Function**
The contract provides a function to invoke the `flip` function of the **Flipper** contract.  
It does this using an **external call**, passing the **function selector** corresponding to the `flip` function.

---

## **Deployment & Usage**

### **Steps to Deploy and Interact with a Locally Running Substrate Node**

#### **1. Build the contract**
```sh
cargo contract build
```
This command compiles the contract and generates the necessary artifacts for deployment.

#### **2. Deploy the contract**
Use the generated `.contract` file from:
```
target/ink/<Your-Contract>.contract
```

---

## **Deployment via Substrate UI**

### **Method 1: Using [Substrate UI](https://ui.use.ink/)**

1. **Go to the** [Substrate UI](https://ui.use.ink/) **website.**  
   
2. **Click on** `Upload a new contract`.  
   ![Substrate UI](/assets/Substrate_UI.png "Substrate UI")  

3. **Provide a name for your contract and upload the** `.contract` **file from**  
   ```
   target/ink/<Your-Contract>.contract
   ```
   Then click **Next**.  
   ![Upload UI](/assets/Upload_UI.png "Upload UI")  

4. **If the contract’s constructor requires arguments, provide them.**  
   ![Constructor UI](/assets/Constructor.png "Constructor UI")  

5. **Click** `Next` **and then** `Upload and Instantiate` **to deploy the contract.**

---

## **Method 2: Deploy via CLI**

You can deploy the contract using the **Polkadot JS API CLI**. Follow these steps:

#### **1. Set up your environment**
Ensure you have the necessary tools installed:
```sh
cargo install cargo-contract --force
```
Also, make sure you have a local **Substrate node** running.

#### **2. Upload the contract**
Run the following command to upload your contract to the chain:
```sh
cargo contract upload --suri //Alice --execute --url ws://127.0.0.1:9944
```
- `--suri //Alice`: Uses the `Alice` account for deployment.
- `--url ws://127.0.0.1:9944`: Specifies the WebSocket endpoint of the local node.

#### **3. Instantiate the contract**
After successful upload, instantiate it using:
```sh
cargo contract instantiate -suri //Alice --execute --url ws://127.0.0.1:9944 --constructor new <flipper_contract_address>
```
- Replace `<flipper_contract_address>` with the actual contract address of **Flipper**.

#### **4. Interact with the contract**
Once deployed, you can call functions like `flip` using:
```sh
cargo contract call --suri //Alice --execute --url ws://127.0.0.1:9944 --contract <caller_contract_address> --message call_flip
```
- Replace `<caller_contract_address>` with the **Caller** contract's address.



