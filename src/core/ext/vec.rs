
pub trait VecExt<T> {
    fn last_index(&self) -> usize;
    fn index_of<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<usize>;
    fn try_remove(&mut self, index: usize) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {

    fn last_index(&self) -> usize {
        self.len() - 1
    }

    fn index_of<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<usize> {
        for (i, it) in self.iter().enumerate() {
            if predicate(it) {
                return Some(i);
            }
        }
        return None;
    }

    fn try_remove(&mut self, index: usize) -> Option<T> {
        match () {
            _ if index < self.len() => Some(self.remove(index)),
            _ => None,
        }
    }
}
