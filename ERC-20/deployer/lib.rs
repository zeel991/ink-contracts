#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod deployer_contract {
    use erc20::Erc20Ref;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::ToAccountId;

    #[ink(storage)]
    pub struct DeployerContract {
        deployed_contracts: Vec<AccountId>,
    }

    impl DeployerContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                deployed_contracts: Vec::new(),
            }
        }

        /// Deploys a new ERC20 token contract.
        #[ink(message)]
        pub fn deploy_erc20(
            &mut self,
            total_supply: u128,
            name: Option<String>,
            symbol: Option<String>,
            salt: u64, // Allows deploying multiple versions
        ) -> AccountId {
            let salt_bytes = salt.to_le_bytes(); // for contract address uniqueness
            let erc20_instance = Erc20Ref::new(total_supply, name, symbol)
                .endowment(0)
                .code_hash(self.erc20_code_hash())
                .salt_bytes(&salt_bytes)
                .instantiate();

            let contract_address = erc20_instance.to_account_id();
            self.deployed_contracts.push(contract_address);
            contract_address
        }

        /// Returns all deployed contract addresses.
        #[ink(message)]
        pub fn get_deployed_contracts(&self) -> Vec<AccountId> {
            self.deployed_contracts.clone()
        }

        fn erc20_code_hash(&self) -> Hash {
            let bytes: [u8; 32] = [
                0xf5, 0x7c, 0xc5, 0xb3, 0x4d, 0x13, 0xac, 0xd0,
                0x93, 0x59, 0xec, 0xd0, 0xa4, 0xbd, 0x0f, 0xf6,
                0x22, 0xea, 0x8d, 0x3b, 0x94, 0x1b, 0xda, 0x65,
                0xc1, 0x28, 0x7c, 0x8c, 0x7c, 0xba, 0x51, 0xf5,
            ];
            Hash::from(bytes)
        }
        
        
        
    }
}
