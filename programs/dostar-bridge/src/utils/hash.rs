use solana_program::{blake3::HASH_BYTES, keccak};

pub fn hash(value: &[u8]) -> [u8; HASH_BYTES] {
    let hash = {
        let mut hasher = keccak::Hasher::default();
        hasher.hash(value);
        hasher.result()
    };

    hash.0
}
