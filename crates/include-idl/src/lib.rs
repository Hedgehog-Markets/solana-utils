#[cfg(feature = "shrink")]
mod shrink;

#[cfg(feature = "parse")]
pub mod parse;

#[cfg(feature = "shrink")]
pub use crate::shrink::compress_idl;

/// Includes compressed Anchor IDL inside your program binary.
///
/// # Example
///
/// ```
/// #[cfg(not(feature = "no-entrypoint"))]
/// # const _: &str = stringify! {
/// include_idl::include_idl(concat!(env!("OUT_DIR"), "/solana.idl.zip"));
/// # };
/// ```
#[macro_export]
macro_rules! include_idl {
    ($file:expr $(,)?) => {
        #[allow(unexpected_cfgs)]
        const _: () = {
            #[allow(dead_code, non_upper_case_globals)]
            #[no_mangle]
            #[cfg_attr(target_os = "solana", link_section = ".solana.idl")]
            static solana_idl: &[u8] = include_bytes!($file);
        };
    };
}

/// Includes compressed Kinobi IDL inside your program binary.
///
/// # Example
///
/// ```
/// #[cfg(not(feature = "no-entrypoint"))]
/// # const _: &str = stringify! {
/// include_idl::include_kinobi_idl(concat!(env!("OUT_DIR"), "/kinobi.idl.zip"));
/// # };
/// ```
#[macro_export]
macro_rules! include_kinobi_idl {
    ($file:expr $(,)?) => {
        #[allow(unexpected_cfgs)]
        const _: () = {
            #[allow(dead_code, non_upper_case_globals)]
            #[no_mangle]
            #[cfg_attr(target_os = "solana", link_section = ".kinobi.idl")]
            static kinobi_idl: &[u8] = include_bytes!($file);
        };
    };
}
