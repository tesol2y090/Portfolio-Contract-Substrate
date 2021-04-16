#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod portcontract {

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct Portcontract {
        messages: Vec<String>,
        is_set_message: StorageHashMap<AccountId, bool>,
    }

    impl Portcontract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                messages: vec![],
                is_set_message: Default::default(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                messages: vec![],
                is_set_message: Default::default(),
            }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn get_length(&self) -> u32 {
            self.messages.len() as u32
        }

        pub fn set_message(&mut self, _message: String) {
            let caller = self.env().caller();
            self.messages.push(_message);
            self.is_set_message.insert(caller, true);
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {}

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {}
    }
}
