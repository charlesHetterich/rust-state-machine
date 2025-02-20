use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
    /// The current block number.
    block_number: u32,
    /// A map from an account to their nonce.
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    /// Get current block number
    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    /// Increment block number by one
    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    // Increment an account's nonce
    pub fn inc_nonce(&mut self, who: &String) {
        let nonce = self.nonce.entry(who.clone()).or_insert(0);
        *nonce += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_system() {
        let mut pallet = Pallet::new();

        assert_eq!(pallet.block_number(), 0);
        pallet.inc_block_number();
        assert_eq!(pallet.block_number(), 1);
    }

    #[test]
    fn init_nonce() {
        let mut pallet = Pallet::new();

        assert_eq!(pallet.nonce.get(&"Alice".to_string()), None);
        pallet.inc_nonce(&"Alice".to_string());
        assert_eq!(pallet.nonce.get(&"Alice".to_string()), Some(&1));
        pallet.inc_nonce(&"Alice".to_string());
        assert_eq!(pallet.nonce.get(&"Alice".to_string()), Some(&2));
    }
}
