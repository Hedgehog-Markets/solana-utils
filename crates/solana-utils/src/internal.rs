//! Internal utilities used by the crate.

/// Panics if the `target_os` is not `solana`.
macro_rules! assert_is_solana {
    ($($label:expr)?) => {
        if cfg!(not(target_os = "solana")) {
            panic!(concat!($($label, ": ",)? "not supported when target_os != \"solana\""));
        }
    };
}
