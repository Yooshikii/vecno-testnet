// public for benchmarks
#[doc(hidden)]
pub mod mem_hash;
#[cfg(feature = "wasm32-sdk")]
pub mod wasm;
#[doc(hidden)]

use std::cmp::max;
use crate::mem_hash::mem_hash;
use vecno_consensus_core::{hashing, header::Header, BlockLevel};
use vecno_hashes::{Hash, PowHash};
use vecno_math::Uint256;
use sha3::{Digest, Sha3_256};
use blake3;

/// Computes SHA3-256 hash of a 32-byte input
fn sha3_hash(input: [u8; 32]) -> [u8; 32] {
    let mut sha3_hasher = Sha3_256::new();
    sha3_hasher.update(input);
    sha3_hasher.finalize().as_slice().try_into().expect("SHA-3 output length mismatch")
}

/// Computes Blake3 hash of a 32-byte input
fn blake3_hash(input: [u8; 32]) -> [u8; 32] {
    *blake3::hash(&input).as_bytes() // Safe: Blake3 outputs 32 bytes
}

/// Calculates the number of hash rounds (1–4) based on the first 4 bytes
fn calculate_rounds(input: [u8; 32]) -> usize {
    (u32::from_le_bytes(input[0..4].try_into().unwrap_or_default()) % 4 + 1) as usize
}

/// Performs XOR manipulations on adjacent bytes in 4-byte chunks
fn bit_manipulations(data: &mut [u8; 32]) {
    for i in (0..32).step_by(4) {
        data[i] ^= data[i + 1];
    }
}

/// Combines SHA3-256 and Blake3 hashes with byte-wise XOR
fn byte_mixing(sha3_hash: &[u8; 32], b3_hash: &[u8; 32]) -> [u8; 32] {
    let mut temp_buf = [0u8; 32];
    for i in 0..32 {
        temp_buf[i] = sha3_hash[i] ^ b3_hash[i];
    }
    temp_buf
}

/// State for PoW computation, holding the difficulty target and hasher
pub struct State {
    pub(crate) target: Uint256,
    // PRE_POW_HASH || TIME || 32 zero byte padding; without NONCE
    pub(crate) hasher: PowHash,
}

impl State {
    /// Initializes the PoW state with a block header
    #[inline]
    pub fn new(header: &Header) -> Self {
        let target = Uint256::from_compact_target_bits(header.bits);
        // Zero out the time and nonce.
        let pre_pow_hash = hashing::header::hash_override_nonce_time(header, 0, 0);
        // PRE_POW_HASH || TIME || 32 zero byte padding || NONCE
        let hasher = PowHash::new(pre_pow_hash, header.timestamp);
        Self { target, hasher }
    }

    /// Computes the PoW hash for a given nonce using Blake3, SHA3-256, and mem_hash
    #[inline]
    #[must_use]
        /// PRE_POW_HASH || TIME || 32 zero byte padding || NONCE
    pub fn calculate_pow(&self, nonce: u64) -> Uint256 {
        // Hasher already contains PRE_POW_HASH || TIME || 32 zero byte padding; so only the NONCE is missing
        // TODO: Parallelize nonce iteration by cloning State for multiple threads
        let hash = self.hasher.clone().finalize_with_nonce(nonce);
        let mut hash_bytes: [u8; 32] = hash.as_bytes().try_into().expect("Hash output length mismatch");
        let rounds = calculate_rounds(hash_bytes);
        let b3_hash: [u8; 32];

        for _ in 0..rounds {
            hash_bytes = blake3_hash(hash_bytes);
            bit_manipulations(&mut hash_bytes);
        }
        b3_hash = hash_bytes;

        for _ in 0..rounds {
            hash_bytes = sha3_hash(hash_bytes);
            bit_manipulations(&mut hash_bytes);
        }

        let m_hash = byte_mixing(&hash_bytes, &b3_hash);
        let final_hash = mem_hash(Hash::from_bytes(m_hash));
        Uint256::from_le_bytes(final_hash.as_bytes())
    }

    /// Verifies if the PoW hash meets the difficulty target
    #[inline]
    #[must_use]
    pub fn check_pow(&self, nonce: u64) -> (bool, Uint256) {
        let pow = self.calculate_pow(nonce);
        (pow <= self.target, pow)
    }
}

/// Calculates the block level based on the PoW hash
pub fn calc_block_level(header: &Header, max_block_level: BlockLevel) -> BlockLevel {
    let (block_level, _) = calc_block_level_check_pow(header, max_block_level);
    block_level
}

/// Calculates the block level and verifies the PoW
pub fn calc_block_level_check_pow(header: &Header, max_block_level: BlockLevel) -> (BlockLevel, bool) {
    if header.parents_by_level.is_empty() {
        return (max_block_level, true); // Genesis block
    }
    let state = State::new(header);
    let (passed, pow) = state.check_pow(header.nonce);
    let block_level = calc_level_from_pow(pow, max_block_level);
    (block_level, passed)
}

/// Converts a PoW hash to a block level
pub fn calc_level_from_pow(pow: Uint256, max_block_level: BlockLevel) -> BlockLevel {
    let signed_block_level = max_block_level as i64 - pow.bits() as i64;
    max(signed_block_level, 0) as BlockLevel
}