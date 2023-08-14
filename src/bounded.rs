
pub enum BoundedOption<T> {
    /// A given item was found in `n` steps.
    Some(usize,T),
    /// No more items was confirmed in `n` steps.
    None(usize),
    /// The budget was used up searching for the next item.
    OutOfResource
}

impl<T> BoundedOption<T> {
    pub fn unwrap(&self) -> &T {
        match self {
            BoundedOption::Some(_,item) => item,
            _ => panic!("failed unwrapping")
        }
    }
}

// An iterator that limits the amount of work done looking for the
// next item.
pub trait BoundedIterator {
    /// Item returned by the iterator.
    type Item;

    /// Attempt to find the next item, expending at most `n` steps
    /// looking for it.  If the limit is reached, then `OutOfResource`
    /// is returned.
    fn next_for(&mut self, n: usize) -> BoundedOption<Self::Item>;
}
