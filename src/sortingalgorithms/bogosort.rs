use std::cmp::Ordering;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::sortingutils::traits::{ComparatorType, VectorElement};

pub struct BogoSortIter<T>
    where T: VectorElement<T>
{
    pub(crate) vector: Vec<T>,
    pub(crate) comparator: ComparatorType<T>,
    pub(crate) index: usize, // It counts how many sorts happened
}

impl<T> BogoSortIter<T>
    where T: VectorElement<T>
{
    pub fn new(vector: Vec<T>, comparator: ComparatorType<T>) -> BogoSortIter<T>
    {
        Self {
            vector,
            comparator,
            index: 0,
        }
    }

    fn is_vector_sorted(&self) -> bool {
        self.vector.windows(2).all(|el| (self.comparator)(&el[0], &el[1]) != Ordering::Greater)
    }
}


impl<T> Iterator for BogoSortIter<T>
    where
        T: VectorElement<T>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_vector_sorted() {
            return None;
        }
        let mut rng = thread_rng();
        self.vector.shuffle(&mut rng);
        self.index += 1;
        Some(self.index)
    }
}