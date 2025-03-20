#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod proxy {

    #[ink(storage)]
    pub struct Proxy {
        value: u32,
        owner: AccountId,
    }

    impl Proxy {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller(); 
            Self { owner: caller ,
                   value : 0, 
                }
        }
        fn only_owner(&self) {
            assert_eq!(self.env().caller(), self.owner, "Only owner can call this function");
        }

        #[ink(message)]
        pub fn set_value(&mut self, val: u32) {
            self.value = val;
        }

        #[ink(message)]
        pub fn get_value(&self) -> u32 {
            self.value
        }
        // #[ink(message)]
        // pub fn double(&mut self) -> u32{
        //     let mut value = self.get_value();
        //     value = value.checked_add(value).expect("Overflow detected");
        //     self.value = value;
        //     self.value
        // }

        #[ink(message)]
        pub fn set_code(&mut self, code_hash: Hash) {
            
        self.only_owner();
        self.env().set_code_hash(&code_hash).unwrap_or_else(|err| {
            panic!(
                "Failed to `set_code_hash` to {:?} due to {:?}",
                code_hash, err
            )
        });
        ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        }  
    }
}
