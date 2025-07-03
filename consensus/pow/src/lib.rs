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
///
/// # Arguments
/// * `input` - A 32-byte input array to hash.
///
/// # Returns
/// A `Result` containing the 32-byte SHA3-256 hash or an error if the output length is invalid.
///
/// # Errors
/// Returns an error if the SHA3-256 output cannot be converted to a 32-byte array.
fn sha3_hash(input: [u8; 32]) -> Result<[u8; 32], &'static str> {
    let mut sha3_hasher = Sha3_256::new();
    sha3_hasher.update(input);
    sha3_hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(|_| "SHA3-256 output length mismatch")
}

/// Computes Blake3 hash of a 32-byte input
///
/// # Arguments
/// * `input` - A 32-byte input array to hash.
///
/// # Returns
/// A 32-byte Blake3 hash.
fn blake3_hash(input: [u8; 32]) -> [u8; 32] {
    *blake3::hash(&input).as_bytes() // Safe: Blake3 outputs 32 bytes
}

/// Calculates the number of hash rounds based on immutable block header fields
///
/// Determines a dynamic number of rounds (1–4) using the SHA3-256 hash of the pre-PoW hash
/// and the block timestamp. This prevents nonce selection attacks by ensuring the round count
/// is independent of the nonce.
///
/// # Arguments
/// * `pre_pow_hash` - A 32-byte hash of the block header (excluding nonce and timestamp).
/// * `timestamp` - The block timestamp as a u64.
///
/// # Returns
/// A `usize` representing the number of rounds (1–4).
fn calculate_rounds(pre_pow_hash: [u8; 32], timestamp: u64) -> usize {
    let mut hasher = Sha3_256::new();
    hasher.update(pre_pow_hash);
    hasher.update(timestamp.to_le_bytes());
    let hash = hasher.finalize();
    (u32::from_le_bytes(hash[0..4].try_into().unwrap_or_default()) % 4 + 1) as usize
}

/// Performs XOR manipulations on adjacent bytes in 4-byte chunks
///
/// Applies XOR operations to adjacent bytes in 4-byte chunks to enhance diffusion.
///
/// # Arguments
/// * `data` - A mutable 32-byte array to manipulate.
fn bit_manipulations(data: &mut [u8; 32]) {
    for i in (0..32).step_by(4) {
        data[i] ^= data[i + 1];
        data[i + 2] ^= data[i + 3]; // Enhanced mixing for better diffusion
    }
}

/// Combines SHA3-256 and Blake3 hashes with byte-wise XOR
///
/// # Arguments
/// * `sha3_hash` - A 32-byte SHA3-256 hash.
/// * `b3_hash` - A 32-byte Blake3 hash.
///
/// # Returns
/// A 32-byte array resulting from the byte-wise XOR of the inputs.
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
    pub(crate) hasher: PowHash,
    pub(crate) pre_pow_hash: [u8; 32], // Store pre-PoW hash for round calculation
    pub(crate) timestamp: u64, // Store timestamp for round calculation
    pub(crate) merkle_root: [u8; 32], // Store merkle_root for mem_hash
}

impl State {
    /// Initializes the PoW state with a block header
    ///
    /// Creates a new PoW state with the target difficulty derived from the header's bits,
    /// a pre-PoW hash computed with nonce and timestamp set to 0, the block's timestamp,
    /// and the merkle_root.
    ///
    /// # Arguments
    /// * `header` - The block header containing the bits, timestamp, hash_merkle_root, and other fields.
    ///
    /// # Returns
    /// A new `State` instance.
    #[inline]
    pub fn new(header: &Header) -> Self {
        let target = Uint256::from_compact_target_bits(header.bits);
        let pre_pow_hash = hashing::header::hash_override_nonce_time(header, 0, 0);
        let hasher = PowHash::new(pre_pow_hash, header.timestamp);
        let pre_pow_hash_bytes = pre_pow_hash
            .as_bytes()
            .try_into()
            .expect("Pre-PoW hash length mismatch");
        Self {
            target,
            hasher,
            pre_pow_hash: pre_pow_hash_bytes,
            timestamp: header.timestamp,
            merkle_root: header.hash_merkle_root.as_bytes().try_into().expect("Merkle root length mismatch"),
        }
    }

    /// Computes the PoW hash for a given nonce
    ///
    /// Combines Blake3, SHA3-256, and memory-hard hashing with a dynamic number of rounds
    /// based on the pre-PoW hash and timestamp to prevent nonce selection attacks.
    ///
    /// # Arguments
    /// * `nonce` - The nonce to include in the hash computation.
    ///
    /// # Returns
    /// A `Uint256` representing the PoW hash.
    #[inline]
    #[must_use]
    pub fn calculate_pow(&self, nonce: u64) -> Uint256 {
        let hash = self.hasher.clone().finalize_with_nonce(nonce);
        let mut hash_bytes: [u8; 32] = hash
            .as_bytes()
            .try_into()
            .expect("Hash output length mismatch");
        let rounds = calculate_rounds(self.pre_pow_hash, self.timestamp);
        let b3_hash: [u8; 32];

        for _ in 0..rounds {
            hash_bytes = blake3_hash(hash_bytes);
            bit_manipulations(&mut hash_bytes);
        }
        b3_hash = hash_bytes;

        for _ in 0..rounds {
            hash_bytes = sha3_hash(hash_bytes).expect("SHA3-256 failed");
            bit_manipulations(&mut hash_bytes);
        }

        let m_hash = byte_mixing(&hash_bytes, &b3_hash);
        let final_hash = mem_hash(
            Hash::from_bytes(m_hash),
            self.timestamp,
            nonce,
            self.merkle_root,
            &self.target,
        );
        Uint256::from_le_bytes(final_hash.as_bytes())
    }

    /// Verifies if the PoW hash meets the difficulty target
    ///
    /// # Arguments
    /// * `nonce` - The nonce to verify.
    ///
    /// # Returns
    /// A tuple containing a boolean indicating if the PoW hash meets the target and the computed `Uint256` hash.
    #[inline]
    #[must_use]
    pub fn check_pow(&self, nonce: u64) -> (bool, Uint256) {
        let pow = self.calculate_pow(nonce);
        (pow <= self.target, pow)
    }
}

/// Calculates the block level based on the PoW hash
///
/// # Arguments
/// * `header` - The block header containing the PoW data.
/// * `max_block_level` - The maximum block level allowed.
///
/// # Returns
/// The computed block level as a `BlockLevel`.
pub fn calc_block_level(header: &Header, max_block_level: BlockLevel) -> BlockLevel {
    let (block_level, _) = calc_block_level_check_pow(header, max_block_level);
    block_level
}

/// Calculates the block level and verifies the PoW
///
/// # Arguments
/// * `header` - The block header containing the PoW data.
/// * `max_block_level` - The maximum block level allowed.
///
/// # Returns
/// A tuple containing the computed block level and a boolean indicating if the PoW is valid.
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
///
/// # Arguments
/// * `pow` - The PoW hash as a `Uint256`.
/// * `max_block_level` - The maximum block level allowed.
///
/// # Returns
/// The computed block level as a `BlockLevel`.
pub fn calc_level_from_pow(pow: Uint256, max_block_level: BlockLevel) -> BlockLevel {
    let signed_block_level = max_block_level as i64 - pow.bits() as i64;
    max(signed_block_level, 0) as BlockLevel
}