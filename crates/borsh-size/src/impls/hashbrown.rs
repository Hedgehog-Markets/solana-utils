use hashbrown::{HashMap, HashSet};

iter_impl! { [T] [S] HashSet<T, S> }
iter_impl! { [K, V] [S] HashMap<K, V, S> }
