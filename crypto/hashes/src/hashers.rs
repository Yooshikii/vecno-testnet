use once_cell::sync::Lazy;

/// Base trait for hashers, allowing data updates.
pub trait HasherBase {
    fn update<A: AsRef<[u8]>>(&mut self, data: A) -> &mut Self;
}

/// Trait for hashers with finalize, reset, and hash functionality.
pub trait Hasher: HasherBase + Clone + Default {
    fn finalize(self) -> crate::Hash;
    fn reset(&mut self);
    #[inline(always)]
    fn hash<A: AsRef<[u8]>>(data: A) -> crate::Hash {
        let mut hasher = Self::default();
        hasher.update(data);
        hasher.finalize()
    }
}

pub use crate::pow_hashers::{VecnoHash, PowHash};

blake3_hasher! {
    struct TransactionHash => b"TransactionHash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    struct TransactionID => b"TransactionID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    struct TransactionSigningHash => b"TransactionSigningHash\0\0\0\0\0\0\0\0\0\0",
    struct BlockHash => b"BlockHash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    struct ProofOfWorkHash => b"ProofOfWorkHash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    struct MerkleBranchHash => b"MerkleBranchHash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    struct MuHashElementHash => b"MuHashElement\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    struct MuHashFinalizeHash => b"MuHashFinalize\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    struct PersonalMessageSigningHash => b"PersonalMessageSigningHash\0\0\0\0\0\0",
}

sha256_hasher! {
    struct TransactionSigningHashECDSA => "TransactionSigningHashECDSA",
}

macro_rules! sha256_hasher {
    ($(struct $name:ident => $domain_sep:literal),+ $(,)? ) => {$(
        #[derive(Clone)]
        pub struct $name(sha2::Sha256);

        impl $name {
            #[inline]
            pub fn new() -> Self {
                use sha2::{Sha256, Digest};
                // We use Lazy in order to avoid rehashing it
                // in the future we can replace this with the correct initial state.
                static HASHER: Lazy<$name> = Lazy::new(|| {
                    // SHA256 doesn't natively support domain separation, so we hash it to make it constant size.
                    let mut tmp_state = Sha256::new();
                    tmp_state.update($domain_sep);
                    let mut out = $name(Sha256::new());
                    out.write(tmp_state.finalize());

                    out
                });
                (*HASHER).clone()
            }

            pub fn write<A: AsRef<[u8]>>(&mut self, data: A) {
                sha2::Digest::update(&mut self.0, data.as_ref());
            }

            #[inline(always)]
            pub fn finalize(self) -> crate::Hash {
                let mut out = [0u8; 32];
                out.copy_from_slice(sha2::Digest::finalize(self.0).as_slice());
                crate::Hash(out)
            }
        }
    impl_hasher!{ struct $name }
    )*};
}

macro_rules! blake3_hasher {
    ($(struct $name:ident => $domain_sep:literal),+ $(,)? ) => {$(
        #[derive(Clone)]
        pub struct $name(blake3::Hasher);

        impl $name {
            #[inline(always)]
            pub fn new() -> Self {
                let hasher = blake3::Hasher::new_keyed($domain_sep);
                Self(hasher)
            }

            pub fn write<A: AsRef<[u8]>>(&mut self, data: A) {
                self.0.update(data.as_ref());
            }

            #[inline(always)]
            pub fn finalize(self) -> crate::Hash {
                let hash = self.0.finalize();
                let mut out = [0u8; 32];
                out.copy_from_slice(hash.as_bytes());
                crate::Hash(out)
            }
        }
    impl_hasher!{ struct $name }
    )*};
}
macro_rules! impl_hasher {
    (struct $name:ident) => {
        impl HasherBase for $name {
            #[inline(always)]
            fn update<A: AsRef<[u8]>>(&mut self, data: A) -> &mut Self {
                self.write(data);
                self
            }
        }
        impl Hasher for $name {
            #[inline(always)]
            fn finalize(self) -> crate::Hash {
                // Call the method
                $name::finalize(self)
            }
            #[inline(always)]
            fn reset(&mut self) {
                *self = Self::new();
            }
        }
        impl Default for $name {
            #[inline(always)]
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

use {blake3_hasher, impl_hasher, sha256_hasher};