use std::cmp::{Ordering};

use crate::sortingutils::traits::{ComparatorType, VectorElement};

pub struct HeapSortIter< T>
    where T: VectorElement<T>
{
    pub(crate) vector: Vec<T>,
    pub(crate) comparator: ComparatorType<T>,
    pub(crate) index: usize,
    is_heap: bool,
    call_heap: bool,
    max_pos: usize
}


impl< T> HeapSortIter< T>
    where T: VectorElement<T>
{
    pub fn new(v: Vec<T>, comparator: ComparatorType<T>) -> HeapSortIter<T> {
        let mut curr_pos = v.len() as f32;
        curr_pos /= 2.0;
        Self {
            call_heap: true,
            index: curr_pos.floor() as usize - 1,
            is_heap: false,
            max_pos: v.len(),
            vector: v,
            comparator,

        }
    }

    fn heapify(&mut self, max_pos: usize, i: usize) -> (bool, usize) {
        let mut largest = i;
        let left_child = 2 * i + 1;
        let right_child = 2 * i + 2;
        if left_child < max_pos && (self.comparator)(&self.vector[i], &self.vector[left_child]) == Ordering::Less {
            largest = left_child;
        }
        if right_child < max_pos && (self.comparator)(&self.vector[i], &self.vector[right_child]) == Ordering::Less {
            largest = right_child
        }
        if largest != i {
            self.vector.swap(largest, i);
            return (true, largest);
        }
        (false, largest)
    }
}

impl< T> Iterator for HeapSortIter< T>
    where T: VectorElement<T>
{
    // The combinations are same_value, same_value if nothing was changed, usize, usize if there was a swap
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.max_pos == 1 {
            return None;
        } else if self.call_heap {
            let res = self.heapify(self.max_pos, self.index);
            self.call_heap = res.0;
            if res.0 {
                let old_pos = self.index;
                self.index = res.1;
                return Some((old_pos, self.index));
            }
            if self.index == 0 {
                self.is_heap = true;
            } else {
                self.index -= 1;
                self.call_heap = true;
            }
            return Some((res.1, res.1));
        } else if self.is_heap {
            self.max_pos -= 1;
            self.vector.swap(0, self.max_pos);
            self.is_heap = false;
            self.call_heap = true;
            self.index = 0;
            return Some((0, self.max_pos+1));
        }
        None
    }
}