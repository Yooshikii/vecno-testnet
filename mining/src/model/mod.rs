use std::collections::HashSet;
use vecno_consensus_core::tx::TransactionId;

pub(crate) mod candidate_tx;
pub mod owner_txs;
pub mod topological_index;
pub mod topological_sort;
pub mod tx_query;

/// A set of unique transaction ids
pub type TransactionIdSet = HashSet<TransactionId>;
