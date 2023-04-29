#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use pink_extension as pink;

#[pink::contract(env=PinkEnvironment)]
mod query_indexer {
    use super::pink;
    use ink::prelude::{string::String, format};
    use ink::env::debug_println;
    use pink::{http_post, PinkEnvironment};
    use scale::{Decode, Encode};
    use serde::{Deserialize};
    use serde_json_core;

    /// storage
    #[ink(storage)]
    pub struct QueryIndexer {
        url: String,
    }

    /// DTO use for serializing and deserializing the json
    #[derive(Deserialize, Encode, Clone, Debug, PartialEq)]
    pub struct IndexerRewardResponse<'a> {
        #[serde(borrow)]
        data: IndexerRewardData<'a>,
    }

    #[derive(Deserialize, Encode, Clone, Debug, PartialEq)]
    #[allow(non_snake_case)]
    struct IndexerRewardData<'a> {
        #[serde(borrow)]
        developerRewards: DeveloperRewards<'a>,
    }

    #[derive(Deserialize, Encode, Clone, Debug, PartialEq)]
    struct DeveloperRewards<'a> {
        #[serde(borrow)]
        nodes: [DeveloperRewardNode<'a>; 1],
    }

    #[derive(Deserialize, Encode, Clone, Debug, PartialEq)]
    struct DeveloperRewardNode<'a> {
        amount: &'a str,
        era: &'a str,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        HttpRequestFailed,
        InvalidResponseBody,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl QueryIndexer {

        /// constructors
        #[ink(constructor)]
        pub fn new(url: String) -> Self {
            Self { url }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self { url: String::from("https://api.subquery.network/sq/GuiGou12358/lucky-shibuya-v0_1_0/") }
        }

        /// getter and setter
        #[ink(message)]
        pub fn get_url(&self) -> Result<String> {
            Ok(self.url.clone())
        }

        #[ink(message)]
        pub fn set_url(&mut self, url: String) {
            self.url = url;
        }


        /// query the indexer
        #[ink(message)]
        pub fn get_developer_rewards(&self, era: u16) -> Result<String> {

            let headers = alloc::vec![
                ("Content-Type".into(), "application/json".into()),
                ("Accept".into(), "application/json".into())
            ];

            let body = format!(
                r#"{{"query" : "{{developerRewards (filter: {{ era: {{ equalTo: \"{}\" }} }}){{nodes {{amount, era}}}}}}"}}"#,
                era
            );
            debug_println!("body: {}", body);

            let resp = http_post!(
                self.url.clone(),
                body,
                headers
            );
            debug_println!("status code {}", resp.status_code);

            if resp.status_code != 200 {
                return Err(Error::HttpRequestFailed);
            }
            let result: IndexerRewardResponse = serde_json_core::from_slice(resp.body.as_slice())
                .or(Err(Error::InvalidResponseBody))?
                .0;
            let rewards = String::from(result.data.developerRewards.nodes[0].amount);

            Ok(rewards)
        }


    }

    #[cfg(test)]
    mod tests {

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn get_developer_rewards() {
            // when your contracts is really deployed, the Phala Worker will do the HTTP requests
            // mock is needed for local test
            pink_extension_runtime::mock_ext::mock_all_ext();

            let url = "https://api.subquery.network/sq/GuiGou12358/lucky-shibuya-v0_1_0/";

            let query_indexer = QueryIndexer::new(url.to_string());
            let era = 2800;
            let res = query_indexer.get_developer_rewards(era);

            assert!(res.is_ok());

            let r = res.unwrap();

            // run with `cargo +nightly test -- --nocapture` to see the following output
            println!("Era {} Developer model {}", era, r);

        }
    }
}
