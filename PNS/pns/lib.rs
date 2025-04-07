#![cfg_attr(not(feature = "std"), no_std , no_main)]

#[ink::contract]
mod pns_resolver {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec; 

    #[ink(storage)]
    pub struct PnsResolver {
        name_to_owner: Mapping<Hash, AccountId>,
        name_to_cid: Mapping<Hash, Vec<u8>>,
    }

    impl PnsResolver {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                name_to_owner: Mapping::default(),
                name_to_cid: Mapping::default(),
            }
        }

        /// Register a new domain name with its IPFS CID
        #[ink(message)]
        pub fn register(&mut self, name: Hash, cid: Vec<u8>) -> bool {
            let caller = self.env().caller();
            if self.name_to_owner.get(name).is_some() {
                return false; // already registered
            }
            self.name_to_owner.insert(name, &caller);
            self.name_to_cid.insert(name, &cid);
            true
        }

        /// Update CID (only by current owner)
        #[ink(message)]
        pub fn update_cid(&mut self, name: Hash, new_cid: Vec<u8>) -> bool {
            let caller = self.env().caller();
            match self.name_to_owner.get(name) {
                Some(owner) if owner == caller => {
                    self.name_to_cid.insert(name, &new_cid);
                    true
                }
                _ => false,
            }
        }

        /// Get CID by domain name
        #[ink(message)]
        pub fn get_cid(&self, name: Hash) -> Vec<u8> {
            self.name_to_cid.get(name).unwrap_or_default()
        }

        /// Check if name is taken
        #[ink(message)]
        pub fn is_registered(&self, name: Hash) -> bool {
            self.name_to_owner.get(name).is_some()
        }

        /// Get owner of a name
        #[ink(message)]
        pub fn get_owner(&self, name: Hash) -> Option<AccountId> {
            self.name_to_owner.get(name)
        }
    }
}

