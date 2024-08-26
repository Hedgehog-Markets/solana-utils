macro_rules! fixed_size_impl {
    ($ty:ty = $size:expr) => {
        impl $crate::BorshSize for $ty {
            const MIN_SIZE: usize = $size;
            const MAX_SIZE: Option<usize> = Some(<Self as $crate::BorshSize>::MIN_SIZE);

            #[inline(always)]
            fn borsh_size(&self) -> usize {
                <Self as $crate::BorshSize>::MIN_SIZE
            }
        }
    };
}

macro_rules! sized_impl {
    ($($ty:ty)*) => {
        $(
            impl $crate::BorshSize for $ty {
                const MIN_SIZE: usize = ::core::mem::size_of::<Self>();
                const MAX_SIZE: Option<usize> = Some(<Self as $crate::BorshSize>::MIN_SIZE);

                #[inline(always)]
                fn borsh_size(&self) -> usize {
                    <Self as $crate::BorshSize>::MIN_SIZE
                }
            }
        )*
    };
}

macro_rules! deref_impl {
    ([$($T:ident),*] $ty:ty => $deref:ty) => {
        impl<$($T),*> $crate::BorshSize for $ty
        where
            $($T: $crate::BorshSize,)*
        {
            const MIN_SIZE: usize = <$deref>::MIN_SIZE;
            const MAX_SIZE: Option<usize> = <$deref>::MAX_SIZE;

            #[inline]
            fn borsh_size(&self) -> usize {
                <$deref>::borsh_size(self)
            }
        }
    };
}

macro_rules! iter_impl {
    ([$($T1:ident),*] [$($T2:ident),*] $ty:ty) => {
        impl<$($T1),* $(, $T2)*> $crate::BorshSize for $ty
        where
            $($T1: $crate::BorshSize,)*
        {
            const MIN_SIZE: usize = u32::MIN_SIZE;
            const MAX_SIZE: Option<usize> = None;

            fn borsh_size(&self) -> usize {
                crate::utils::iter_size(u32::MIN_SIZE, self.len(), self.iter())
            }
        }
    };
}
