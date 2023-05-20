use std::cmp::Ordering;

use crate::sortingutils::traits::{ComparatorType, VectorElement};

pub struct StalinSortIter<T>
    where T: VectorElement<T>
{
    pub(crate) vector: Vec<T>,
    pub(crate) comparator: ComparatorType<T>,
    pub(crate) index: usize,
}

impl<T> StalinSortIter<T>
    where T: VectorElement<T>
{
    pub fn new(vector: Vec<T>, comparator: ComparatorType<T>) -> StalinSortIter<T>
    {
        Self {
            vector,
            comparator,
            index: 0,
        }
    }
}

impl<T> Iterator for StalinSortIter<T>
    where
        T: VectorElement<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.vector.get(self.index) {
            None => {
                None
            }
            Some(el) => {
                match self.vector.get(self.index + 1) {
                    None => {
                        None
                    }
                    Some(el2) => {
                        if (self.comparator)(el, el2) == Ordering::Greater {
                            return Some(self.vector.remove(self.index + 1));
                        }
                        self.index += 1;
                        Some(T::get_default())
                    }
                }
            }
        }
    }
}