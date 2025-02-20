use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    /// The type which represents the content that can be claimed using this pallet.
    /// Could be the content directly as bytes, or better yet the hash of that content.
    /// We leave that decision to the runtime developer.
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only have one owner.
    claims: BTreeMap<T::Content, T::AccountID>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    /// Get the owner (if any) of a claim.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountID> {
        self.claims.get(claim)
    }

    /// Create a new claim on behalf of the `caller`.
    /// This function will return an error if someone already has claimed that content.
    pub fn create_claim(&mut self, caller: T::AccountID, claim: T::Content) -> DispatchResult {
        // check claim available
        if self.claims.contains_key(&claim) {
            return Err(&"this content is already claimed");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    /// Revoke an existing claim on some content.
    /// This function should only succeed if the caller is the owner of an existing claim.
    /// It will return an error if the claim does not exist, or if the caller is not the owner.
    pub fn revoke_claim(&mut self, caller: T::AccountID, claim: T::Content) -> DispatchResult {
        let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
        if owner != &caller {
            return Err("caller is not the owner of the claim");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

pub enum Call<T: Config> {
    CreateClaim(T::Content),
    RevokeClaim(T::Content),
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountID;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim(claim) => self.create_claim(caller, claim),
            Call::RevokeClaim(claim) => self.revoke_claim(caller, claim),
        }
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountID = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        /*
            TODO:
            Create an end to end test verifying the basic functionality of this pallet.
                - Check the initial state is as you expect.
                - Check that all functions work successfully.
                - Check that all error conditions error as expected.
        */
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::<TestConfig>::new();
        assert_eq!(pallet.get_claim(&"hello"), None);
        assert_eq!(pallet.create_claim(&"Alice", "hello"), Ok(()));
        assert_eq!(pallet.get_claim(&"hello"), Some(&"Alice"));
        assert_eq!(
            pallet.create_claim(&"Bob", "hello"),
            Err("this content is already claimed")
        );
        assert_eq!(
            pallet.revoke_claim(&"Bob", "hello"),
            Err("caller is not the owner of the claim")
        );
        assert_eq!(pallet.revoke_claim(&"Alice", "hello"), Ok(()));
        assert_eq!(pallet.get_claim(&"hello"), None);
    }
}
