use std::sync::Arc;
use vecno_consensus_core::tx::Transaction;

#[derive(Debug)]
pub struct TransactionInsertion {
    pub removed: Option<Arc<Transaction>>,
    pub accepted: Vec<Arc<Transaction>>,
}

impl TransactionInsertion {
    pub fn new(removed: Option<Arc<Transaction>>, accepted: Vec<Arc<Transaction>>) -> Self {
        Self { removed, accepted }
    }
}
