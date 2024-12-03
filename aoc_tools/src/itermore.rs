use std::{collections::HashMap};

pub trait IterMoreTools: Iterator {
    fn try_collect_vec<T, E>(self) -> Result<Vec<T>, E>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
    {
        self.collect()
    }

    fn try_collect_map<K, V, E>(self) -> Result<HashMap<K, V>, E>
    where
        Self: Sized + Iterator<Item = Result<(K, V), E>>,
        K: std::hash::Hash + Eq,
    {
        self.collect()
    }

    fn stateful_map<B, F, S>(self, initial_state: S, f: F) -> StatefulMap<Self, F, S>
    where
        Self: Sized,
        F: FnMut(&mut S, Self::Item) -> B,
    {
        StatefulMap::new(self, f, initial_state)
    }

    fn aggregate<B, F>(self, aggregator: F) -> Option<B>
        where
            Self: Sized,
            F: Fn(B, Self::Item) -> B,
            B: From<Self::Item> {
        self.fold(None, |acc, x| {
            Some(match acc {
                Some(acc_val) => aggregator(acc_val, x),
                None => x.into(),
            })
        })
    }
}

pub struct StatefulMap<I, F, S> {
    iter: I,
    f: F,
    state: S,
}

impl<I, F, S> StatefulMap<I, F, S> {
    fn new(iter: I, f: F, state: S) -> StatefulMap<I, F, S> {
        StatefulMap { iter, f, state }
    }
}

impl<B, I: Iterator, F, S> Iterator for StatefulMap<I, F, S>
where
    F: FnMut(&mut S, I::Item) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        Some((self.f)(&mut self.state, self.iter.next()?))
    }
}

impl<T> IterMoreTools for T where T: Iterator + ?Sized {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stateful_map() {
        let source = vec![1, 2, 3, 4, 5];

        let dest: Vec<_> = source
            .into_iter()
            .stateful_map(0, |a, f| { *a += f; *a} )
            .collect();


        // 0+1=1, 1+2=3, 3+3=6, 6+4=10, 10+5=15
        assert_eq!(dest, vec![1, 3, 6, 10, 15]);
    }
}