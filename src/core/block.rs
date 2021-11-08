use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::core::crypto::{self, MerkleHash, Sha256};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockHash(Sha256);

impl BlockHash {
    pub fn new(hash: Sha256) -> Self {
        Self(hash)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0.as_ref()[..]
    }
}

impl AsRef<Sha256> for BlockHash {
    fn as_ref(&self) -> &Sha256 {
        &self.0
    }
}

impl Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", crypto::as_hex(self.as_slice()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    previous_block_hash: BlockHash,
    merkle_root: MerkleHash,
    timestamp: u32,
    difficulty: u32,
    nonce: u32,
}
