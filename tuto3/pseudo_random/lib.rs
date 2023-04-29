#![cfg_attr(not(feature = "std"), no_std)]

use pink_extension as pink;

#[pink::contract(env=PinkEnvironment)]
mod pseudo_random {
    use super::pink;
    use pink::PinkEnvironment;
    use ink::env::hash::{Keccak256, HashOutput};
    use ink::prelude::vec::Vec;
    use ink::env::debug_println;

    #[ink(storage)]
    pub struct PseudoRandom {
    }

    #[derive(Debug, Eq, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PseudoRandomError {
        DivByZero,
        MulOverFlow,
        AddOverFlow,
        SubOverFlow,
    }

    impl PseudoRandom {

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_pseudo_random(&self, salt: Vec<u8>, min: u128, max: u128) -> Result<u128, PseudoRandomError> {
            let seed = self.env().block_timestamp();
            let mut input: Vec<u8> = Vec::new();
            input.extend_from_slice(&seed.to_be_bytes());
            input.extend_from_slice(salt.as_slice());
            let mut output = <Keccak256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Keccak256>(&input, &mut output);

            let a = output[0] as u128;

            //(a  as u32) * (max - min) / (u32::MAX) + min
            let b = max.checked_sub(min).ok_or(PseudoRandomError::SubOverFlow)?;
            let c = a.checked_mul(b).ok_or(PseudoRandomError::MulOverFlow)?;
            let d = c.checked_div(u8::MAX as u128).ok_or(PseudoRandomError::DivByZero)?;
            let e = d.checked_add(min).ok_or(PseudoRandomError::AddOverFlow)?;

            debug_println!("random {}", e);

            Ok(e)
        }

    }

    #[cfg(test)]
    mod tests {

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn test_pseudo_random() {

            let pseudo_random = PseudoRandom::default();

            let mut salt:Vec<u8> = Vec::new();
            salt.extend_from_slice(b"some salt");

            match pseudo_random.get_pseudo_random(salt, 1, 10){
                Ok(r) => assert!(r >= 1 && r <= 10),
                Err(_) => panic!("Error when generate the random number!")
            }

            let mut salt:Vec<u8> = Vec::new();
            salt.extend_from_slice(b"other salt");
            match pseudo_random.get_pseudo_random(salt, 1, 10){
                Ok(r) => assert!(r >= 1 && r <= 10),
                Err(_) => panic!("Error when generate the random number!")
            }

        }
    }

}
