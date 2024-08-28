#![cfg_attr(not(feature = "std"), no_std)]

// Allow use of `#[derive(BorshSize)]` within this crate.
extern crate self as borsh_size;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[macro_use]
mod internal_macros;

mod impls;
mod utils;

pub use borsh_size_macro::BorshSize;
pub use utils::BorshSizeProperties;

pub trait BorshSize {
    /// The minimum size in bytes when this type is borsh serialized.
    const MIN_SIZE: usize;
    /// The maximum size in bytes when this type is borsh serialized.
    ///
    /// `None` if there is no maximum size.
    const MAX_SIZE: Option<usize>;

    /// Returns the size in bytes when this value is borsh serialized.
    #[inline(always)]
    fn borsh_size(&self) -> usize {
        if Self::IS_FIXED_SIZE { Self::MIN_SIZE } else { unimplemented_borsh_size() }
    }
}

#[cold]
#[track_caller]
#[inline(never)]
fn unimplemented_borsh_size() -> ! {
    panic!("missing borsh_size implementation for dynamically sized type")
}
