use core::marker::PhantomData;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use core::ops::Range;

use crate::utils::{iter_size, max, min, BorshSizeProperties};
use crate::BorshSize;

impl<T> BorshSize for &'_ T
where
    T: BorshSize,
{
    const MIN_SIZE: usize = T::MIN_SIZE;
    const MAX_SIZE: Option<usize> = T::MAX_SIZE;

    fn borsh_size(&self) -> usize {
        T::borsh_size(self)
    }
}

impl<T> BorshSize for PhantomData<T>
where
    T: ?Sized,
{
    const MIN_SIZE: usize = 0;
    const MAX_SIZE: Option<usize> = Some(0);

    fn borsh_size(&self) -> usize {
        Self::MIN_SIZE
    }
}

sized_impl! { u8 u16 u32 u64 u128 }
sized_impl! { i8 i16 i32 i64 i128 }

sized_impl! { NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 }
sized_impl! { NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 }

fixed_size_impl! { usize = u64::MIN_SIZE }
fixed_size_impl! { isize = i64::MIN_SIZE }

fixed_size_impl! { NonZeroUsize = usize::MIN_SIZE }
fixed_size_impl! { NonZeroIsize = isize::MIN_SIZE }

sized_impl! { f32 f64 }
sized_impl! { bool }

fixed_size_impl! { () = 0 }

impl<T> BorshSize for Option<T>
where
    T: BorshSize,
{
    const MIN_SIZE: usize = 1;
    const MAX_SIZE: Option<usize> = match T::MAX_SIZE {
        Some(size) => Some(1 + size),
        None => None,
    };

    fn borsh_size(&self) -> usize {
        match self {
            Some(v) => 1 + v.borsh_size(),
            None => 1,
        }
    }
}

impl<T, E> BorshSize for Result<T, E>
where
    T: BorshSize,
    E: BorshSize,
{
    const MIN_SIZE: usize = 1 + min(T::MIN_SIZE, E::MIN_SIZE);
    const MAX_SIZE: Option<usize> = match (T::MAX_SIZE, E::MAX_SIZE) {
        (Some(t_size), Some(e_size)) => Some(1 + max(t_size, e_size)),
        _ => None,
    };

    fn borsh_size(&self) -> usize {
        1 + match self {
            Ok(v) => v.borsh_size(),
            Err(err) => err.borsh_size(),
        }
    }
}

impl<const LEN: usize, T> BorshSize for [T; LEN]
where
    T: BorshSize,
{
    const MIN_SIZE: usize = LEN * T::MIN_SIZE;
    const MAX_SIZE: Option<usize> = match T::MAX_SIZE {
        Some(size) => Some(LEN * size),
        None => None,
    };

    fn borsh_size(&self) -> usize {
        iter_size(0, self.len(), self.iter())
    }
}

impl<T> BorshSize for [T]
where
    T: BorshSize,
{
    const MIN_SIZE: usize = u32::MIN_SIZE;
    const MAX_SIZE: Option<usize> = None;

    fn borsh_size(&self) -> usize {
        iter_size(u32::MIN_SIZE, self.len(), self.iter())
    }
}

impl BorshSize for str {
    const MIN_SIZE: usize = <[u8]>::MIN_SIZE;
    const MAX_SIZE: Option<usize> = <[u8]>::MAX_SIZE;

    fn borsh_size(&self) -> usize {
        self.as_bytes().borsh_size()
    }
}

impl<T> BorshSize for Range<T>
where
    T: BorshSize,
{
    const MIN_SIZE: usize = 2 * T::MIN_SIZE;
    const MAX_SIZE: Option<usize> = match T::MAX_SIZE {
        Some(size) => Some(2 * size),
        None => None,
    };

    fn borsh_size(&self) -> usize {
        if T::IS_FIXED_SIZE {
            Self::MIN_SIZE
        } else {
            self.start.borsh_size() + self.end.borsh_size()
        }
    }
}

macro_rules! tuple_impl {
    (
        impl = [{ $($idx:tt $ty:ident)* }]
    ) => {
        impl<$($ty),*> BorshSize for ($($ty,)*)
        where
            $($ty: BorshSize,)*
        {
            const MIN_SIZE: usize = 0 $(+ $ty::MIN_SIZE)*;
            const MAX_SIZE: Option<usize> = {
                let sum = Some(0);

                $(
                    let sum = match (sum, $ty::MAX_SIZE) {
                        (Some(sum), Some(size)) => Some(sum + size),
                        _ => None,
                    };
                )*

                sum
            };

            fn borsh_size(&self) -> usize {
                if Self::IS_FIXED_SIZE {
                    return Self::MIN_SIZE;
                }

                let mut size = 0;

                $(
                    size += self.$idx.borsh_size();
                )*

                size
            }
        }
    };
    (
        stack = [{ $($idx:tt $ty:ident)* }]
        tail = [{ }]
    ) => {};
    (
        stack = [{ $($stack:tt)* }]
        tail = [{ $idx:tt $head:ident $($tail:tt)* }]
    ) => {
        tuple_impl! {
            impl = [{ $($stack)* $idx $head }]
        }
        tuple_impl! {
            stack = [{ $($stack)* $idx $head }]
            tail = [{ $($tail)* }]
        }
    };

    // Entrypoint.
    ($($idx:tt $ty:ident)*) => {
        tuple_impl! {
            stack = [{ }]
            tail = [{ $($idx $ty)* }]
        }
    };
}

tuple_impl! {
     0  T0  1  T1  2  T2  3  T3  4  T4  5  T5  6  T6  7  T7  8  T8  9  T9
    10 T10 11 T11 12 T12 13 T13 14 T14 15 T15 16 T16 17 T17 18 T18 19 T19
    20 T20 21 T21 22 T22 23 T23 24 T24 25 T25 26 T26 27 T27 28 T28 29 T29
    30 T30 31 T31 32 T32
}
