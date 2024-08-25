use solana_program::pubkey::Pubkey;

/// Checks two pubkeys for equality in a computationally cheap way using `sol_memcmp`.
#[inline]
pub fn pubkeys_eq(a: &Pubkey, b: &Pubkey) -> bool {
    #[cfg(target_os = "solana")]
    {
        use solana_program::pubkey::PUBKEY_BYTES;

        let a: &[u8] = a.as_ref();
        let b: &[u8] = b.as_ref();

        // SAFETY: `a` and `b` are valid for reads of `PUBKEY_BYTES` bytes.
        unsafe { crate::syscalls::memcmp(a.as_ptr(), b.as_ptr(), PUBKEY_BYTES) == 0 }
    }
    #[cfg(not(target_os = "solana"))]
    {
        // For non-solana targets let the compiler optimize the equality check.
        a == b
    }
}
