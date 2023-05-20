use std::cmp::Ordering;

use crate::sortingutils::traits::{ComparatorType, VectorElement};

pub struct CycleSortIter<T>
    where T: VectorElement<T>
{
    pub(crate) vector: Vec<T>,
    pub(crate) comparator: ComparatorType<T>,
    pub(crate) index: usize
}

impl<T> CycleSortIter<T>
    where T: VectorElement<T>
{
    pub fn new(vector: Vec<T>, comparator: ComparatorType<T>) -> CycleSortIter<T> {
        Self {
            vector,
            comparator,
            index: 0,
        }
    }
}

impl<T> Iterator for CycleSortIter<T>
    where T: VectorElement<T>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.vector.get(self.index) {
            None => {
                None
            }
            Some(el) => {
                let cycle_start = self.index;
                let mut pos = cycle_start;
                let mut cycle_start_tmp = self.index;
                while let Some(el2) = self.vector.get(cycle_start_tmp+1){
                    if (self.comparator)(el, el2) == Ordering::Greater{
                        pos += 1;
                    }
                    cycle_start_tmp += 1;
                }
                if pos == cycle_start{
                    // Already in place
                    self.index += 1;
                    return Some(pos);
                }
                self.vector.swap(cycle_start, pos);
                Some(pos)
            }
        }
    }
}