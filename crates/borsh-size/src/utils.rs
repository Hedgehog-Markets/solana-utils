use crate::BorshSize;

mod private {
    pub trait Sealed {}
}

pub trait BorshSizeProperties: BorshSize + private::Sealed {
    const FIXED_SIZE: usize = match Self::MAX_SIZE {
        Some(max) if Self::MIN_SIZE == max => Self::MIN_SIZE,
        _ => panic!("this type does not have a fixed size"),
    };

    const IS_FIXED_SIZE: bool = match Self::MAX_SIZE {
        Some(max) => Self::MIN_SIZE == max,
        None => false,
    };
}

impl<T> private::Sealed for T where T: ?Sized + BorshSize {}
impl<T> BorshSizeProperties for T where T: ?Sized + BorshSize + private::Sealed {}

pub(crate) const fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

pub(crate) const fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

#[inline]
pub(crate) fn iter_size<T, I>(prefix: usize, len: usize, iter: I) -> usize
where
    T: BorshSize,
    I: ExactSizeIterator<Item = T>,
{
    if T::IS_FIXED_SIZE {
        prefix + len * T::MIN_SIZE
    } else {
        let mut size = prefix;

        for element in iter {
            size += element.borsh_size();
        }

        size
    }
}
