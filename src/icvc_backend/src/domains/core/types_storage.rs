use candid::{CandidType, Deserialize};
use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

const COMPOSITE_KEY_MAX_SIZE: u32 = 64000;

pub trait KeyTuple {
    fn to_key_string(&self) -> String;
    fn from_key_string(s: &str) -> Self
    where
        Self: Sized;
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeKey {
    key: String,
}

impl CompositeKey {
    pub fn construct_key<T: KeyTuple>(tuple: &T) -> Self {
        Self {
            key: tuple.to_key_string(),
        }
    }
    pub fn deconstruct_key<T: KeyTuple>(&self) -> T {
        T::from_key_string(&self.key)
    }
}

impl Storable for CompositeKey {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: COMPOSITE_KEY_MAX_SIZE,
        is_fixed_size: false,
    };
}

impl KeyTuple for (u64,) {
    fn to_key_string(&self) -> String {
        format!("{}", self.0)
    }

    fn from_key_string(s: &str) -> Self {
        let part = s.parse().unwrap_or(0);
        (part,)
    }
}

impl KeyTuple for (u64, u64) {
    fn to_key_string(&self) -> String {
        format!("{}_{}", self.0, self.1)
    }

    fn from_key_string(s: &str) -> Self {
        let parts: Vec<u64> = s.split('_').map(|p| p.parse().unwrap_or(0)).collect();
        (parts[0], parts[1])
    }
}

impl KeyTuple for (u64, u64, u64) {
    fn to_key_string(&self) -> String {
        format!("{}_{}_{}", self.0, self.1, self.2)
    }

    fn from_key_string(s: &str) -> Self {
        let parts: Vec<u64> = s.split('_').map(|p| p.parse().unwrap_or(0)).collect();
        (parts[0], parts[1], parts[2])
    }
}

impl KeyTuple for (u64, u64, u64, u64) {
    fn to_key_string(&self) -> String {
        format!("{}_{}_{}_{}", self.0, self.1, self.2, self.3)
    }

    fn from_key_string(s: &str) -> Self {
        let parts: Vec<u64> = s.split('_').map(|p| p.parse().unwrap_or(0)).collect();
        (parts[0], parts[1], parts[2], parts[3])
    }
}
