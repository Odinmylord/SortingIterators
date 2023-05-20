use std::cmp::Ordering;

use crate::sortingutils::traits::{ComparatorType, VectorElement};
use crate::sortingutils::utils;

pub struct MergeSortIter< T>
    where T: VectorElement<T>
{
    pub(crate) vector: Vec<T>,
    pub(crate) comparator: ComparatorType<T>,
    pub(crate) indexes: Vec<(usize, usize)>,
    pub(crate) index: usize,
    merge_l: bool,
    merge_r: bool,
}

impl< T> MergeSortIter< T>
    where T: VectorElement<T>
{
    pub fn new(v: Vec<T>, comparator: ComparatorType<T>) -> MergeSortIter< T> {
        let len = v.len();
        let mut indexes = vec![(0, len - 1)];
        indexes.resize(len+2, (0, 0));
        let upper_mid = len / 2 + len % 2;
        indexes[1] = (0, upper_mid - 1);
        indexes[2] = (upper_mid, len - 1);
        Self {
            vector: v,
            comparator,
            indexes,
            // this index is the index of the tree that I am currently looking
            index: 1,
            merge_l: false,
            merge_r: false,
        }
    }
    fn merge(&mut self) -> (usize, usize){
        let father_pair = self.indexes[self.index];
        let left_pair = self.indexes[self.index * 2 + 1];
        let right_pair = self.indexes[self.index * 2 + 2];
        let mut left_offset = 0;
        let mut right_offset = 0;
        let mut tmp_vec = vec![];
        tmp_vec.resize_with(utils::length_from_pair(father_pair), || T::get_default());
        for i in 0..utils::length_from_pair(father_pair) {
            if right_pair.0 + right_offset <= father_pair.1 && (self.comparator)(&self.vector[left_pair.0 + left_offset],
                                                                                 &self.vector[right_pair.0 + right_offset]) == Ordering::Greater
            {
                tmp_vec[i] = std::mem::replace(&mut self.vector[right_pair.0 + right_offset], T::get_default());
                right_offset += 1;
            } else if left_pair.0 + left_offset <= father_pair.1 {
                tmp_vec[i] = std::mem::replace(&mut self.vector[left_pair.0 + left_offset], T::get_default());
                left_offset += 1;
            }
        }
        for i in 0..utils::length_from_pair(father_pair) {
            if (self.comparator)(&tmp_vec[i], &T::get_default()) != Ordering::Equal {
                self.vector[i + father_pair.0] = std::mem::replace(&mut tmp_vec[i], T::get_default());
            }
        }
        // go to his right brother
        if self.index % 2 != 0 && self.index != 0 {
            self.index += 1;
            self.merge_l = true;
        } else if self.index == 2 {
            self.index = 0;
            self.merge_r = true;
            self.merge_l = true;
        }
        father_pair
    }
}

impl< T> Iterator for MergeSortIter< T>
    where T: VectorElement<T>
{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // The idea is to use the indexes vector as if it is a binary tree and build the recursive calls on top of that
        if self.merge_r && self.merge_l {
            self.merge_l = false;
            self.merge_r = false;
            let was_zero = self.index == 0;
            // go back to father
            //self.index = (self.index - 1)  / 2;
            let return_val = self.merge();
            if was_zero {
                self.index = 0;
                return None;
            }
            return Some(return_val);
        }
        let values_pair = self.indexes[self.index];
        let len = utils::length_from_pair(values_pair);
        let upper_mid = len / 2 + len % 2;
        let left_pos = self.index * 2 + 1;
        let right_pos = left_pos + 1;
        let mut left_pair = (values_pair.0, upper_mid - 1);
        let mut right_pair = (upper_mid, values_pair.1);
        left_pair.1 += values_pair.0;
        right_pair.0 += values_pair.0;
        let left_len = utils::length_from_pair(left_pair);
        let right_len = utils::length_from_pair(right_pair);
        let mut set_to_left = true;
        if left_len <= 1 {
            self.merge_l = true;
            set_to_left = false;
        }
        if left_len <= 1 && right_len <= 1 {
            if self.index % 2 == 0 {
                self.merge_r = true;
            } else {
                self.merge_l = true;
            }
            // in this way if right was already ready to merge left is set to ready
            if (self.comparator)(&self.vector[values_pair.0], &self.vector[values_pair.1]) == Ordering::Greater {
                self.vector.swap(values_pair.0, values_pair.1)
            }
            if !self.merge_r {
                self.index += 1;
            } else {
                self.index = (self.index - 1) / 2;
            }
            return Some(values_pair);
        }
        self.indexes[left_pos] = left_pair;
        self.indexes[right_pos] = right_pair;
        if set_to_left {
            self.index = left_pos;
        } else {
            self.index = right_pos;
        }
        Some(values_pair)
    }
}