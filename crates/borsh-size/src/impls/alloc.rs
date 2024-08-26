#![allow(clippy::module_inception)]

use core::borrow::Borrow;

use alloc::borrow::ToOwned;

use crate::BorshSize;

#[cfg(feature = "std")]
mod alloc {
    pub use std::{borrow, boxed, collections, rc, string, sync, vec};
}
#[cfg(not(feature = "std"))]
mod alloc {
    pub use alloc::{borrow, boxed, collections, rc, string, sync, vec};
}

deref_impl! { [T] alloc::boxed::Box<T> => T }
deref_impl! { [T] alloc::rc::Rc<T> => T }
deref_impl! { [T] alloc::sync::Arc<T> => T }

deref_impl! { [T] alloc::vec::Vec<T> => [T] }

deref_impl! { [] alloc::string::String => str }

impl<T> BorshSize for alloc::borrow::Cow<'_, T>
where
    T: ToOwned + BorshSize,
{
    const MIN_SIZE: usize = T::MIN_SIZE;
    const MAX_SIZE: Option<usize> = T::MAX_SIZE;

    fn borsh_size(&self) -> usize {
        T::borsh_size(self.borrow())
    }
}

iter_impl! { [T] [] alloc::collections::VecDeque<T> }
iter_impl! { [T] [] alloc::collections::LinkedList<T> }
iter_impl! { [T] [] alloc::collections::BinaryHeap<T> }

iter_impl! { [T] [] alloc::collections::BTreeSet<T> }
iter_impl! { [K, V] [] alloc::collections::BTreeMap<K, V> }

#[cfg(feature = "std")]
mod std {
    iter_impl! { [T] [S] std::collections::HashSet<T, S> }
    iter_impl! { [K, V] [S] std::collections::HashMap<K, V, S> }
}
