use num::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    /// Address/pointer to on chain data
    type AccountId: Ord + Clone;

    /// Incremental
    type BlockNumber: Zero + One + AddAssign + Copy;

    /// Incremental
    type Nonce: Zero + One + AddAssign;
}

#[derive(Debug)]
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Pallet {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get current block number
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    /// Increment block number by one
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    // Increment an account's nonce
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = self.nonce.entry(who.clone()).or_insert(T::Nonce::zero());
        *nonce += T::Nonce::one();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut pallet = Pallet::<TestConfig>::new();

        assert_eq!(pallet.block_number(), 0);
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn init_nonce() {
        let mut pallet = Pallet::<TestConfig>::new();

        assert_eq!(pallet.nonce.get(&"Alice".to_string()), None);
        pallet.inc_nonce(&"Alice".to_string());
        assert_eq!(pallet.nonce.get(&"Alice".to_string()), Some(&1));
        pallet.inc_nonce(&"Alice".to_string());
        assert_eq!(pallet.nonce.get(&"Alice".to_string()), Some(&2));
    }
}
