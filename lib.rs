#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod whitelist_contract {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct WhitelistContract {
        registerlist: Mapping<String, String>,
    }

    impl WhitelistContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { registerlist : Mapping::default()}
        }

        #[ink(message)]
        pub fn get_registed(&self, serial : String) -> Option<String> {
            self.registerlist.get(serial)
        }

        #[ink(message)]
        pub fn register(&mut self, serial : String, accountid: String) {
            self.registerlist.insert(serial, &accountid);
        }

        #[ink(message)]
        pub fn unregister(&mut self, serial: String) {
            self.registerlist.remove(serial);
        }

        #[ink(message)]
        pub fn registed(&mut self, serial : String) -> bool {
            !self.registerlist.get(serial).is_none()
        }
    }
}
