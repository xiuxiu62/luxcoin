use std::fmt::{self, Display};

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use sha2::Digest;

use crate::core::{block::BlockHash, transaction::Transaction};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sha256([u8; 32]);

impl Sha256 {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for Sha256 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Serialize for Sha256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(as_hex(self.as_ref()).as_str())
    }
}

impl<'de> Deserialize<'de> for Sha256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(StringVisitor)
    }
}

impl Display for Sha256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", as_hex(&self.as_ref()[..]))
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = Sha256;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("test")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        from_hex(v).map_err(|e| E::custom(e))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleHash(Sha256);

impl MerkleHash {
    pub fn new(hash: Sha256) -> MerkleHash {
        Self(hash)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0.as_ref()[..]
    }
}

impl AsRef<Sha256> for MerkleHash {
    fn as_ref(&self) -> &Sha256 {
        &self.0
    }
}

impl Display for MerkleHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0.as_ref()))
    }
}

pub fn as_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn from_hex(s: &str) -> Result<Sha256, String> {
    match hex::decode(&s) {
        Ok(bytes) => {
            let mut sha = [0; 32];
            if bytes.len() == 32 {
                for i in 0..32 {
                    sha[i] = *bytes.get(i).unwrap();
                }
                Ok(Sha256::new(sha))
            } else {
                Err(format!(
                    "Invalid sha length. Expected: {} but got: {} in: {}",
                    32,
                    bytes.len(),
                    s
                ))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn hash(data: &[u8]) -> Sha256 {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);

    let result = hasher.finalize();
    let mut output = [0; 32];
    result
        .iter()
        .enumerate()
        .for_each(|(i, byte)| output[i] = *byte);

    Sha256::new(output)
}

pub fn target_hash(n_zero_bits: u32) -> BlockHash {
    let mut hash = [0xff; 32];

    let num_zero_bytes = (n_zero_bits / 8) as usize;
    for i in 0..num_zero_bytes {
        hash[i] = 0;
    }

    let remainder = 8 - (n_zero_bits % 8);
    if remainder == 8 {
        return BlockHash::new(Sha256::new(hash));
    }

    hash[num_zero_bytes] = (1 << remainder) - 1;
    BlockHash::new(Sha256::new(hash))
}

pub struct MerkleTree(MerkleHash);

impl MerkleTree {
    pub fn new(leaves: &Vec<&[u8]>) -> MerkleTree {
        let mut hashes: Vec<Sha256> = leaves.iter().map(|leaf| hash(*leaf)).collect();
        while hashes.len() != 1 {
            if hashes.len() % 2 == 1 {
                hashes.push(hashes.last().unwrap().clone());
            }

            let mut next_level_hashes = vec![];
            (0..hashes.len()).step_by(2).for_each(|i: usize| {
                let lhs = hashes.get(i).unwrap();
                let rhs = hashes.get(i + 1).unwrap();

                let mut concat: Vec<u8> = lhs.as_ref().iter().map(|byte: &u8| *byte).collect();
                concat.extend_from_slice(rhs.as_ref());
                next_level_hashes.push(hash(&concat));
            });

            hashes = next_level_hashes;
        }

        MerkleTree(MerkleHash::new(hashes.into_iter().next().unwrap()))
    }
}

impl AsRef<MerkleHash> for MerkleTree {
    fn as_ref(&self) -> &MerkleHash {
        &self.0
    }
}

impl From<&Vec<Transaction>> for MerkleTree {
    fn from(transactions: &Vec<Transaction>) -> Self {
        let leaves = transactions
            .iter()
            .map(|tx| tx.id().as_ref().as_slice())
            .collect::<Vec<&[u8]>>();
        MerkleTree::new(&leaves)
    }
}

#[cfg(test)]
mod tests {
    use super::hash;
    use crate::core::crypto::{as_hex, target_hash, MerkleTree};

    #[test]
    fn hash_works() {
        let data = b"hello world";
        assert_eq!(
            hex::encode(hash(data).as_ref()),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
    #[test]
    fn merkle_tree_even() {
        let root_node = MerkleTree::new(&vec![b"hello", b"world"]);
        assert_eq!(
            as_hex(root_node.as_ref().as_slice()),
            "7305db9b2abccd706c256db3d97e5ff48d677cfe4d3a5904afb7da0e3950e1e2"
        );
    }

    #[test]
    fn merkle_tree_odd() {
        let root_node = MerkleTree::new(&vec![
            b"programmed",
            b"to",
            b"work",
            b"and",
            b"not",
            b"to",
            b"feel",
        ]);
        assert_eq!(
            as_hex(root_node.as_ref().as_slice()),
            "4ba2b808c60bdee5df9da358021b50ae56f544682c7931fcc032d2ca323c13bb"
        )
    }

    #[test]
    fn merkle_tree_even_same_as_previous() {
        let root_node = MerkleTree::new(&vec![
            b"programmed",
            b"to",
            b"work",
            b"and",
            b"not",
            b"to",
            b"feel",
            b"feel",
        ]);
        assert_eq!(
            as_hex(root_node.as_ref().as_slice()),
            "4ba2b808c60bdee5df9da358021b50ae56f544682c7931fcc032d2ca323c13bb"
        )
    }

    #[test]
    fn target_hash_test() {
        assert_eq!(
            as_hex(target_hash(0).as_slice()),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
        assert_eq!(
            as_hex(target_hash(4).as_slice()),
            "0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
        assert_eq!(
            as_hex(target_hash(8).as_slice()),
            "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
        assert_eq!(
            as_hex(target_hash(12).as_slice()),
            "000fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
        assert_eq!(
            as_hex(target_hash(16).as_slice()),
            "0000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
        assert_eq!(
            as_hex(target_hash(20).as_slice()),
            "00000fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
    }
}
