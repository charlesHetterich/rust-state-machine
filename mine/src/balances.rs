use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Tokens: Zero + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountID, T::Tokens>,
}
impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }

    /// Set balance for given account `who`
    pub fn set_balance(&mut self, who: &T::AccountID, value: T::Tokens) {
        self.balances.insert(who.clone(), value);
    }

    /// Get balance of account `who` (defaults to 0)
    pub fn get_balance(&self, who: &T::AccountID) -> T::Tokens {
        *self.balances.get(who).unwrap_or(&T::Tokens::zero())
    }

    /// Move frunds from one account to another, only if
    /// requested transfer is valid
    pub fn transfer(
        &mut self,
        from: &T::AccountID,
        to: &T::AccountID,
        value: T::Tokens,
    ) -> Result<(), &'static str> {
        let from_balance = self.get_balance(from);
        let to_balance = self.get_balance(to);

        // safely calculate new balances
        let new_from_balance = from_balance
            .checked_sub(&value)
            .ok_or("Not enough funds.")?;
        let new_to_balance = to_balance.checked_add(&value).ok_or("Fund overflow.")?;

        // update balances if valid
        self.set_balance(from, new_from_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;
    impl crate::system::Config for TestConfig {
        type AccountID = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    impl Config for TestConfig {
        type Tokens = u128;
    }

    #[test]
    fn init_balances() {
        let mut pallet = Pallet::<TestConfig>::new();

        assert_eq!(pallet.get_balance(&"Alice".to_string()), 0);
        pallet.set_balance(&"Alice".to_string(), 100);
        assert_eq!(pallet.get_balance(&"Alice".to_string()), 100);
        assert_eq!(pallet.get_balance(&"Bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::<TestConfig>::new();
        assert_eq!(
            balances.transfer(&"alice".to_string(), &"bob".to_string(), 22),
            Err("Not enough funds.")
        );

        // balances.set_balance(&"alice".to_string(), 35);

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(
            balances.transfer(&"alice".to_string(), &"bob".to_string(), 22),
            Ok(())
        );
        assert_eq!(balances.get_balance(&"alice".to_string()), 78);
        assert_eq!(balances.get_balance(&"bob".to_string()), 22);
        assert_eq!(
            balances.transfer(&"alice".to_string(), &"bob".to_string(), 80),
            Err("Not enough funds.")
        );
    }
}
