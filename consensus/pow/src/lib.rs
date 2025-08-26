use std::cmp::max;
use crate::mem_hash::mem_hash;
use vecno_consensus_core::{hashing, header::Header, BlockLevel};
use vecno_hashes::{PowHash};
use vecno_math::Uint256;

// public for benchmarks
#[doc(hidden)]
pub mod mem_hash;
#[cfg(feature = "wasm32-sdk")]
pub mod wasm;
#[doc(hidden)]

pub struct State {
    pub(crate) target: Uint256,
    pub(crate) hasher: PowHash,
    pub(crate) timestamp: u64,
}

impl State {
    #[inline]
    pub fn new(header: &Header) -> Self {
        let target = Uint256::from_compact_target_bits(header.bits);
        let hasher = PowHash::new(hashing::header::hash_override_nonce_time(header, 0, 0), header.timestamp);
        Self {
            target,
            hasher,
            timestamp: header.timestamp,
        }
    }

    #[inline]
    #[must_use]
    /// PRE_POW_HASH || TIME || 32 zero byte padding || NONCE
    pub fn calculate_pow(&self, nonce: u64) -> Uint256 {
        // Hasher contains PRE_POW_HASH || TIME || 32 zero byte padding; only NONCE is missing
        let block_hash = self.hasher.clone().finalize_with_nonce(nonce);
        let hash = mem_hash(block_hash, self.timestamp, nonce);
        Uint256::from_le_bytes(hash.as_bytes())
    }

    #[inline]
    #[must_use]
    pub fn check_pow(&self, nonce: u64) -> (bool, Uint256) {
        let pow = self.calculate_pow(nonce);
        (pow <= self.target, pow)
    }
}

pub fn calc_block_level(header: &Header, max_block_level: BlockLevel) -> BlockLevel {
    let (block_level, _) = calc_block_level_check_pow(header, max_block_level);
    block_level
}

pub fn calc_block_level_check_pow(header: &Header, max_block_level: BlockLevel) -> (BlockLevel, bool) {
    if header.parents_by_level.is_empty() {
        return (max_block_level, true); // Genesis block
    }
    let state = State::new(header);
    let (passed, pow) = state.check_pow(header.nonce);
    let block_level = calc_level_from_pow(pow, max_block_level);
    (block_level, passed)
}

pub fn calc_level_from_pow(pow: Uint256, max_block_level: BlockLevel) -> BlockLevel {
    let signed_block_level = max_block_level as i64 - pow.bits() as i64;
    max(signed_block_level, 0) as BlockLevel
}