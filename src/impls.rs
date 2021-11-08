use std::collections::HashMap;

use primitive_types::H256;

use crate::{ Database, DatabaseMut,  ValueType};

impl Database for HashMap<H256, Vec<u8>> {
    fn get(&self, key: H256) -> &[u8] {
        self.get(&key)
            .unwrap_or_else(|| panic!("Key {} not found", key))
    }
}

impl DatabaseMut for HashMap<H256, Vec<u8>> {
    fn set(&mut self, key: H256, value: ValueType) {
        if let ValueType::Added(value) = value {
            self.insert(key, value.to_vec());
        } else {
            self.remove(&key);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::convert::TryFrom;

    use quickcheck::{Arbitrary, Gen};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct K(usize);

    impl K {
        pub fn to_bytes(self) -> [u8; 8] {
            self.0.to_be_bytes()
        }

        #[allow(dead_code)]
        pub fn from_bytes(bytes: &[u8]) -> Self {
            Self(usize::from_be_bytes(
                <[u8; std::mem::size_of::<usize>()]>::try_from(bytes).unwrap(),
            ))
        }
    }

    impl Arbitrary for K {
        fn arbitrary(g: &mut Gen) -> Self {
            Self(usize::arbitrary(g))
        }
    }

    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
    pub struct Data(Vec<usize>);

    const AVG_DATA_SIZE: usize = 16;

    impl Arbitrary for Data {
        fn arbitrary(_: &mut Gen) -> Self {
            Self(Vec::arbitrary(&mut Gen::new(AVG_DATA_SIZE)))
        }
    }
}
