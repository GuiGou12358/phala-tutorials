#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

// pink_extension is short for Phala ink! extension
use pink_extension as pink;

#[pink::contract(env=PinkEnvironment)]
mod phat_hello {
    use super::pink;
    use alloc::{format, string::String};
    use pink::{http_get, PinkEnvironment};
    use scale::{Decode, Encode};
    use serde::Deserialize;
    // you have to use crates with `no_std` support in contract.
    use serde_json_core;
    use ink::env::debug_println;

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidEthAddress,
        HttpRequestFailed,
        InvalidResponseBody,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Defines the storage of your contract.
    /// All the fields will be encrypted and stored on-chain.
    /// In this stateless example, we just add a useless field for demo.
    #[ink(storage)]
    pub struct PhatHello {
        demo_field: bool,
    }

    #[derive(Deserialize, Encode, Clone, Debug, PartialEq)]
    pub struct EtherscanResponse<'a> {
        status: &'a str,
        message: &'a str,
        result: &'a str,
    }

    impl PhatHello {
        /// Constructor to initializes your contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { demo_field: true }
        }

        #[ink(message)]
        pub fn get_eth_balance(&self, acc: String) -> Result<String> {

            let account = if acc.starts_with("0x") {
                acc
            } else {
                format!("0x{}", acc)
            };

            debug_println!("account: {}", account);

            if !account.starts_with("0x") && account.len() != 42 {
                return Err(Error::InvalidEthAddress);
            }

            // get account ETH balance with HTTP requests to Etherscan
            // you can send any HTTP requests in Query handler
            let url = format!(
                "https://api.etherscan.io/api?module=account&action=balance&address={}",
                account
            );

            let resp = http_get!(url);

            if resp.status_code != 200 {
                return Err(Error::HttpRequestFailed);
            }

            let result: EtherscanResponse = serde_json_core::from_slice(&resp.body)
                .or(Err(Error::InvalidResponseBody))?
                .0;
            Ok(String::from(result.result))
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn it_works() {
            // when your contract is really deployed, the Phala Worker will do the HTTP requests
            // mock is needed for local test
            pink_extension_runtime::mock_ext::mock_all_ext();
            let phat_hello = PhatHello::new();
            let account = String::from("0x690B9A9E9aa1C9dB991C7721a92d351Db4FaC990");
            let res = phat_hello.get_eth_balance(account.clone());
            assert!(res.is_ok());
            println!("Account {} gets {} Wei", account, res.unwrap());

            let account = String::from("690B9A9E9aa1C9dB991C7721a92d351Db4FaC990");
            let res = phat_hello.get_eth_balance(account.clone());
            assert!(res.is_ok());
            println!("Account {} gets {} Wei", account, res.unwrap());

        }
    }
}
