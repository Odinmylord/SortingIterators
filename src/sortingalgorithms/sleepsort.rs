use std::cmp::Ordering;
use crate::sortingutils::traits::{VectorElement, ComparatorType};

pub struct SleepSortIter<T>
    where T: VectorElement<T>
{
    pub(crate) vector: Vec<T>,
    pub(crate) index: u32,
    // It is the time
    pub(crate) comparator: ComparatorType<T>,
}

/// This Iterator removes duplicates (elements with same int value)
impl<T> SleepSortIter<T>
    where T: VectorElement<T>
{
    pub fn new(v: Vec<T>) -> SleepSortIter<T> {
        Self {
            vector: v,
            index: 0,
            comparator: Box::new(|_x, _y| Ordering::Equal), // this is a dummy comparator
        }
    }
}

impl<T> Iterator for SleepSortIter<T>
    where T: VectorElement<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.vector.is_empty() {
            return None;
        }
        for i in 0..self.vector.len() {
            if self.vector[i].to_num() - self.index == 0 {
                return Some(self.vector.remove(i));
            }
        }
        Some(T::get_default())
    }
}