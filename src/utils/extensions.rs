use std::collections::HashSet;

pub trait UniqueVec<T> {
    fn unique(&self) -> Vec<T>;
}

impl UniqueVec<String> for Vec<String> {
    fn unique(&self) -> Vec<String> {
        let set: HashSet<_> = self.iter().cloned().collect();
        set.into_iter().collect()
    }
}