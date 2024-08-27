#![cfg_attr(not(target_os = "solana"), allow(dead_code))]

use std::marker::PhantomData;
use std::ptr::NonNull;

use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;

#[repr(C)]
pub(crate) struct StableInstruction<'a> {
    accounts: StableVec<'a, AccountMeta>,
    data: StableVec<'a, u8>,
    program_id: Pubkey,
}

#[repr(C)]
pub(crate) struct StableVec<'a, T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    _marker: PhantomData<&'a [T]>,
}

impl<'a> StableInstruction<'a> {
    pub(crate) fn borrow(instruction: &'a Instruction) -> Self {
        Self {
            accounts: StableVec::borrow(&instruction.accounts),
            data: StableVec::borrow(&instruction.data),
            program_id: instruction.program_id,
        }
    }
}

impl<'a, T> StableVec<'a, T> {
    pub(crate) fn borrow(vec: &'a Vec<T>) -> Self {
        let ptr = vec.as_ptr();
        let cap = vec.capacity();
        let len = vec.len();

        // SAFETY: `Vec` guarantees that `ptr` is non-null.
        let ptr = unsafe { NonNull::new_unchecked(ptr.cast_mut()) };

        Self { ptr, cap, len, _marker: PhantomData }
    }
}

// Sanity checks about the layout of StableInstruction and StableVec.
const _: () = {
    use std::mem::{align_of, size_of};

    use solana_program::stable_layout::stable_instruction::StableInstruction as SolStableInstruction;
    use solana_program::stable_layout::stable_vec::StableVec as SolStableVec;

    #[cfg(not(no_offset_of))]
    use std::mem::offset_of;

    #[cfg(no_offset_of)]
    macro_rules! offset_of {
        ($Container:path, $field:tt $(,)?) => {{
            const __OFFSET: usize = {
                // Avoid deref-coercion.
                let $Container { $field: _, .. };

                let __uninit = ::core::mem::MaybeUninit::<$Container>::uninit();
                let __base = __uninit.as_ptr();

                let __field = unsafe { ::core::ptr::addr_of!((*__base).$field) };
                let __offset = unsafe { __field.cast::<u8>().offset_from(__base.cast()) };

                __offset as usize
            };
            __OFFSET
        }};
    }

    assert!(offset_of!(StableInstruction, accounts) == 0);
    assert!(offset_of!(StableInstruction, data) == 24);
    assert!(offset_of!(StableInstruction, program_id) == 48);
    assert!(align_of::<StableInstruction>() == 8);
    assert!(size_of::<StableInstruction>() == 24 + 24 + 32);

    assert!(offset_of!(SolStableInstruction, accounts) == 0);
    assert!(offset_of!(SolStableInstruction, data) == 24);
    assert!(offset_of!(SolStableInstruction, program_id) == 48);
    assert!(align_of::<SolStableInstruction>() == 8);
    assert!(size_of::<SolStableInstruction>() == 24 + 24 + 32);

    assert!(offset_of!(StableVec<AccountMeta>, ptr) == 0);
    assert!(offset_of!(StableVec<AccountMeta>, cap) == 8);
    assert!(offset_of!(StableVec<AccountMeta>, len) == 16);
    assert!(align_of::<StableVec<AccountMeta>>() == 8);
    assert!(size_of::<StableVec<AccountMeta>>() == 24);

    assert!(offset_of!(SolStableVec<AccountMeta>, ptr) == 0);
    assert!(offset_of!(SolStableVec<AccountMeta>, cap) == 8);
    assert!(offset_of!(SolStableVec<AccountMeta>, len) == 16);
    assert!(align_of::<SolStableVec<AccountMeta>>() == 8);
    assert!(size_of::<SolStableVec<AccountMeta>>() == 24);

    assert!(offset_of!(StableVec<u8>, ptr) == 0);
    assert!(offset_of!(StableVec<u8>, cap) == 8);
    assert!(offset_of!(StableVec<u8>, len) == 16);
    assert!(align_of::<StableVec<u8>>() == 8);
    assert!(size_of::<StableVec<u8>>() == 24);

    assert!(offset_of!(SolStableVec<u8>, ptr) == 0);
    assert!(offset_of!(SolStableVec<u8>, cap) == 8);
    assert!(offset_of!(SolStableVec<u8>, len) == 16);
    assert!(align_of::<SolStableVec<u8>>() == 8);
    assert!(size_of::<SolStableVec<u8>>() == 24);
};
