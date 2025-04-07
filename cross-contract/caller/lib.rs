#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod caller {

    #[ink(storage)]
    pub struct Caller {

        flipper_contract: AccountId,
    }

    impl Caller {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(flipper_contract: AccountId) -> Self {
            Self { flipper_contract }
        }
    
        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn call_flip(&mut self ) {
            use ink::env::call::{ExecutionInput, Selector};

            let _result = ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                .call(self.flipper_contract)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!("flip"))))
                .returns::<()> () // No return value
                .invoke();

        }
    }
}
