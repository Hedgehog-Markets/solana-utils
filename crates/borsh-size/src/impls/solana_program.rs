use solana_program::nonce::state::DurableNonce;
use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};
use solana_program::{blake3, hash, keccak};

fixed_size_impl! { Pubkey = PUBKEY_BYTES }

fixed_size_impl! { hash::Hash = hash::HASH_BYTES }
fixed_size_impl! { blake3::Hash = blake3::HASH_BYTES }
fixed_size_impl! { keccak::Hash = keccak::HASH_BYTES }

fixed_size_impl! { DurableNonce = hash::HASH_BYTES }
