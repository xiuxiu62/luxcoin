use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::core::{
    address::Address,
    crypto::{self, Sha256},
    error::LuxError,
    luxcoin::Luxcoin,
};

use super::error::LuxResult;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionId(Sha256);

impl TransactionId {
    pub fn new(data: Sha256) -> Self {
        Self(data)
    }
}

impl AsRef<Sha256> for TransactionId {
    fn as_ref(&self) -> &Sha256 {
        &self.0
    }
}

impl Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputIndex(i32);

impl OutputIndex {
    pub const fn new(index: i32) -> Self {
        Self(index)
    }
}

impl Display for OutputIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Set all bits to 0
const COINBASE_UTXO_ID: TransactionId = TransactionId(Sha256::new([0; 32]));
// Set all bits to 1
const COINBASE_OUTPUT_INDEX: OutputIndex = OutputIndex::new(-1);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionInput {
    utxo_id: TransactionId,
    output_index: OutputIndex,
}

impl TransactionInput {
    pub fn new(utxo_id: TransactionId, output_index: OutputIndex) -> Self {
        Self {
            utxo_id,
            output_index,
        }
    }

    pub fn utxo_id(&self) -> &TransactionId {
        &self.utxo_id
    }

    pub fn output_index(&self) -> &OutputIndex {
        &self.output_index
    }

    pub fn new_coinbase() -> Self {
        Self {
            utxo_id: COINBASE_UTXO_ID,
            output_index: COINBASE_OUTPUT_INDEX,
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.utxo_id == COINBASE_UTXO_ID && self.output_index == COINBASE_OUTPUT_INDEX
    }
}

impl Display for TransactionInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.utxo_id, self.output_index)
    }
}

pub struct TransactionOutput {
    to: Address,
    amount: Luxcoin,
}

impl<'a> TransactionOutput {
    pub fn new(to: Address, amount: Luxcoin) -> Self {
        Self { to, amount }
    }

    pub fn to(&self) -> &Address {
        &self.to
    }

    pub fn amount(&self) -> Luxcoin {
        self.amount
    }
}

impl Display for TransactionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.to, self.amount)
    }
}

pub struct Transaction {
    id: TransactionId,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    // Minimum block height this transaction can be included in
    // to avoid collisions with where transactions have the same io
    locktime: u32,
}

impl Transaction {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>,
        locktime: u32,
    ) -> LuxResult<Self> {
        let id = Self::hash_transaction_data(&inputs, &outputs);
        let transaction = Self {
            id,
            inputs,
            outputs,
            locktime,
        };
        transaction.validate_format()?;
        Ok(transaction)
    }

    pub fn id(&self) -> &TransactionId {
        &self.id
    }

    pub fn inputs(&self) -> &Vec<TransactionInput> {
        &self.inputs
    }

    pub fn outputs(&self) -> &Vec<TransactionOutput> {
        &self.outputs
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.get(0).unwrap().is_coinbase()
    }

    // Ensures transaction is valid under Coinbase standards
    fn validate_format(&self) -> LuxResult<()> {
        let contains_coinbase_inputs = self.inputs.iter().any(TransactionInput::is_coinbase);
        let coinbase_requirements_satisfied = self.inputs.len() == 1 && self.outputs.len() == 1;
        if contains_coinbase_inputs && !coinbase_requirements_satisfied {
            Err(LuxError::InvalidTransaction(format!("Transaction: {} has the coinbase input, but it doesn't satisfy all coinbase requirements.", self.id)))
        } else {
            Ok(())
        }
    }

    fn hash_transaction_data(
        inputs: &Vec<TransactionInput>,
        outputs: &Vec<TransactionOutput>,
    ) -> TransactionId {
        let data = format!(
            "{}{}",
            inputs
                .iter()
                .map(TransactionInput::to_string)
                .collect::<Vec<String>>()
                .join(""),
            outputs
                .iter()
                .map(TransactionOutput::to_string)
                .collect::<Vec<String>>()
                .join("")
        );
        TransactionId(crypto::hash(data.as_bytes()))
    }
}
