
# **Proxy Smart Contract**

The **Proxy** smart contract is an **ink!** contract that allows **dynamic contract upgrades** using the **set_code** function. This demonstrates **upgradable smart contracts** in **Substrate-based blockchains**.

## **Features**
- Stores a **value** and an **owner**.
- Provides functions to **set** and **get** the stored value.
- Initially, the **double** function is **commented out** (disabled).
- Allows the **owner** to update the contract’s logic using **set_code**.
- After an upgrade, the **double** function will be available.

---

## **Contract Structure**

### **Storage**
- `value (u32)`: Stores a numeric value.
- `owner (AccountId)`: Stores the contract owner’s account.

### **Constructor**
- Sets `owner` to the contract deployer (`caller`).
- Initializes `value` to `0`.

### **Message Functions**
- `set_value(val: u32)`: Updates the stored value.
- `get_value() -> u32`: Retrieves the stored value.
- `set_code(code_hash: Hash)`: Allows only the **owner** to change the contract’s code.

---

## **Deployment & Upgrade Process**

We will deploy the contract in **two phases**:
1. **Deploy the contract without the `double` function**.
2. **Upgrade the contract by deploying a new version with the `double` function**.

---

## **Step 1: Deploy the Initial Contract (Without `double` Function)**

### **1. Comment out the `double` function in your contract**
In `proxy.rs`, **comment out** the `double` function:
```rust
// #[ink(message)]
// pub fn double(&mut self) -> u32{
//     let mut value = self.get_value();
//     value = value.checked_add(value).expect("Overflow detected");
//     self.value = value;
//     self.value
// }
```

### **2. Build the contract**
```sh
cargo contract build
```

### **3. Upload and Deploy the Contract**
- **Via CLI**:
  ```sh
  cargo contract upload --suri //Alice --url ws://127.0.0.1:9944 --execute
  ```
- **Instantiate the contract**:
  ```sh
  cargo contract instantiate --suri //Alice --url ws://127.0.0.1:9944 --execute
  ```

- **Save the contract ID** (`<contract_address>`) from the output.
  ```sh
  cargo contract call --contract <contract_address> --message set_value --args 10 --execute  
  ```
- **This will set the value stored on chain as 10.**
---

## **Step 2: Deploy an Updated Version with `double` Function**

### **1. Uncomment the `double` function**
Uncomment the function in `proxy.rs`:
```rust
#[ink(message)]
pub fn double(&mut self) -> u32{
    let mut value = self.get_value();
    value = value.checked_add(value).expect("Overflow detected");
    self.value = value;
    self.value
}
```

### **2. Build the updated contract**
```sh
cargo contract build
```

- **Via CLI**:
  ```sh
  cargo contract upload --suri //Alice --url ws://127.0.0.1:9944 --execute
  ```
- **Instantiate the contract**:
  ```sh
  cargo contract instantiate --suri //Alice --url ws://127.0.0.1:9944 --execute
  ```

Copy the **new** `code_hash` from the output. You can also get the code hash from `target/ink/proxy.json`

---

## **Step 3: Upgrade the Contract**

### **1. Call `set_code` to upgrade the contract**
Replace `<contract_address>` with your deployed contract’s ID and `<new_code_hash>` with the hash from Step 2:
```sh
cargo contract call --suri //Alice --url ws://127.0.0.1:9944 --contract <previous_contract_address> --message set_code --args <new_code_hash> --execute
```

If successful, your contract now runs with the **updated logic**!

---

## **Step 4: Test the New `double` Function**



### **1. Retrieve the value**
```sh
cargo contract call --suri //Alice --url ws://127.0.0.1:9944 --contract <previous_contract_address> --message get_value
```
**Expected Output:** `10`

### **2. Call `double` function**
```sh
cargo contract call --suri //Alice --url ws://127.0.0.1:9944 --contract <previous_contract_address> --message double --execute
```
**Expected Output:** `20`

---

## **Conclusion**
- We **deployed** the contract without the `double` function.
- We **upgraded** the contract by uploading a new version with `double`.
- We **used** `set_code` to **replace** the old contract logic.
- We **verified** that the new function works!

