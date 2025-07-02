use crate::Hash;

#[derive(Clone)]
pub struct PowHash(blake3::Hasher);

#[derive(Clone)]
pub struct VecnoHash;

impl PowHash {
    #[inline]
    pub fn new(pre_pow_hash: Hash, timestamp: u64) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&pre_pow_hash.0);
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(&[0u8; 32]);
        Self(hasher)
    }

    #[inline(always)]
    pub fn finalize_with_nonce(self, nonce: u64) -> Hash {
        let mut hasher = self.0;
        hasher.update(&nonce.to_le_bytes());
        let mut hash_bytes = [0u8; 32];
        hasher.finalize_xof().fill(&mut hash_bytes);
        Hash::from_bytes(hash_bytes)
    }
}

impl VecnoHash {
    #[inline]
    pub fn hash(in_hash: Hash) -> Hash {
        let bytes: &[u8] = &in_hash.0;
        let mut hasher = blake3::Hasher::new();
        hasher.update(bytes);
        let mut hash = [0u8; 32];
        hasher.finalize_xof().fill(&mut hash);
        Hash(hash)
    }
}