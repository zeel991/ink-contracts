#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::erc20::Erc20Ref;

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        /// Total token supply.
        total_supply: u128,
        /// Mapping from owner to number of owned token.
        balances: Mapping<AccountId, u128>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: Mapping<(AccountId, AccountId), u128>,
        /// Name of the token.
        name: Option<String>,
        /// Symbol of the token.
        symbol: Option<String>,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: u128,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: u128,
    }

    /// The ERC-20 error types.
    #[allow(clippy::cast_possible_truncation)]
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Creates a new ERC-20 contract with the specified initial supply, name, and symbol.
        #[ink(constructor)]
        pub fn new(total_supply: u128, name: Option<String>, symbol: Option<String>) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            Self {
                total_supply,
                balances,
                allowances: Default::default(),
                name,
                symbol,
            }
        }

        /// Creates a new ERC-20 contract with the specified initial supply but without name and symbol.
        #[ink(constructor)]
        pub fn new_nameless(total_supply: u128) -> Self {
            Self::new(total_supply, None, None)
        }

        /// Returns the name of the token.
        #[ink(message)]
        pub fn name(&self) -> Option<String> {
            self.name.clone()
        }

        /// Returns the symbol of the token.
        #[ink(message)]
        pub fn symbol(&self) -> Option<String> {
            self.symbol.clone()
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> u128 {
            self.total_supply
        }

        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u128 {
            self.balances.get(owner).unwrap_or_default()
        }

        /// Returns the amount which `spender` is still allowed to withdraw from `owner`.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: u128) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: u128) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        /// Transfers `value` tokens on the behalf of `from` to the account `to`.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u128,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowances.get((from, caller)).unwrap_or_default();
            
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            
            self.transfer_from_to(from, to, value)?;
            
            self.allowances.insert((from, caller), &(allowance.checked_sub(value).unwrap()));
            Ok(())
        }

        /// Internal function to transfer tokens from one account to another.
        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u128,
        ) -> Result<()> {
            let from_balance = self.balances.get(from).unwrap_or_default();
            
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            
            self.balances.insert(from, &(from_balance.checked_sub(value).unwrap()));
            let to_balance = self.balances.get(to).unwrap_or_default();
            
            // Safe from overflow because the sum of all balances is checked to be <= total_supply
            // and total_supply is checked to be <= u128::MAX / 2
            self.balances.insert(to, &(to_balance.checked_add(value).unwrap()));
            
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            
            Ok(())
        }
    }
}
