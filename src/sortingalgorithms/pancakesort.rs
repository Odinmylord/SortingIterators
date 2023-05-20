use std::cmp::Ordering;

use crate::sortingutils::traits::{ComparatorType, VectorElement};

pub struct PancakeSortIter<T>
    where T: VectorElement<T>
{
    pub(crate) vector: Vec<T>,
    pub(crate) index: usize,
    pub(crate) comparator: ComparatorType<T>,
}

impl<T> PancakeSortIter<T>
    where T: VectorElement<T>
{
    pub fn new(v: Vec<T>, comparator: ComparatorType<T>) -> PancakeSortIter<T> {
        Self {
            index: v.len(),
            vector: v,
            comparator,
        }
    }

    fn flip(&mut self, k: usize) {
        let (left, _) = self.vector.split_at_mut(k + 1);
        left.reverse();
    }

    fn find_max(&self, n: usize) -> usize {
        let mut index = 0;
        for i in 0..n {
            if (self.comparator)(&self.vector[i], &self.vector[index]) == Ordering::Greater {
                index = i;
            }
        }
        index
    }
}

impl<T> Iterator for PancakeSortIter<T>
    where T: VectorElement<T>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= 1 {
            return None;
        }
        let max_pos = self.find_max(self.index);
        self.index -= 1;
        if max_pos != self.index {
            self.flip(max_pos);
            self.flip(self.index);
        }
        Some(max_pos)
    }
}
