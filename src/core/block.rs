use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::core::{
    crypto::{self, MerkleHash, Sha256},
    transaction::Transaction,
};

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

impl BlockHeader {
    pub fn new(
        previous_block_hash: BlockHash,
        merkle_root: MerkleHash,
        timestamp: u32,
        difficulty: u32,
        nonce: u32,
    ) -> Self {
        Self {
            previous_block_hash,
            merkle_root,
            timestamp,
            difficulty,
            nonce,
        }
    }

    pub fn hash(&self) -> BlockHash {
        let data = format!(
            "{}{}{}{}{}",
            self.previous_block_hash, self.merkle_root, self.timestamp, self.difficulty, self.nonce
        );

        let hash = crypto::hash(data.as_bytes());
        BlockHash::new(hash)
    }

    pub fn previous_block_hash(&self) -> &BlockHash {
        &self.previous_block_hash
    }

    pub fn merkle_root(&self) -> &MerkleHash {
        &self.merkle_root
    }

    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn difficulty(&self) -> u32 {
        self.difficulty
    }

    pub fn nonce(&self) -> u32 {
        self.nonce
    }
}

pub struct Block {
    id: BlockHash,
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        let id = header.hash();
        Self {
            id,
            header,
            transactions,
        }
    }

    pub fn id(&self) -> &BlockHash {
        &self.id
    }

    pub fn header(&self) -> &BlockHeader {
        &self.header
    }

    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}
