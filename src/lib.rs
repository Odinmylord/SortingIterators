mod sortingutils{
    pub mod enums;
    pub mod utils;
    pub mod traits;
}

pub mod sortingalgorithms {
    pub use strum::EnumIter;
    pub use strum::IntoEnumIterator;

    use crate::sortingalgorithms::bogosort::BogoSortIter;
    use crate::sortingalgorithms::cyclesort::CycleSortIter;
    use crate::sortingalgorithms::heapsort::HeapSortIter;
    use crate::sortingalgorithms::mergesort::MergeSortIter;
    use crate::sortingalgorithms::pancakesort::PancakeSortIter;
    use crate::sortingalgorithms::sleepsort::SleepSortIter;
    use crate::sortingalgorithms::stalinsort::StalinSortIter;
    pub use crate::sortingutils::traits::{ComparatorType, SortingIterator, VectorElement};

    pub mod bogosort;
    pub mod cyclesort;
    pub mod heapsort;
    pub mod mergesort;
    pub mod pancakesort;
    pub mod sleepsort;
    pub mod stalinsort;

    macro_rules! impl_sorting_iterator {
        ($iterator:ident<$t:ident>) =>{
            impl<$t> $crate::sortingutils::traits::SortingIterator<$t> for $iterator<$t>
                where $t: $crate::sortingutils::traits::VectorElement<$t> {
                fn get_vector(&self) -> &Vec<$t> {
                    &self.vector
                }
                fn take_vector(self: Box<Self>) -> Vec<$t> {
                    self.vector
                }
                fn get_indexes(&self) -> Option<&Vec<(usize, usize)>> {
                    None
                }
                fn get_index(&self) -> usize{
                    self.index as usize
                }
                fn get_name(&self) -> &str{
                    stringify!($iterator)
                }
                fn get_comparator(&self) -> &ComparatorType<$t>{
                    &self.comparator
                }
                fn take_vector_and_comparator(self: Box<Self>) -> (Vec<$t>, fn(&$t, &$t) -> std::cmp::Ordering) {
                    (self.vector, *self.comparator)
                }
            }
        }
    }
    macro_rules! impl_sorting_iterator_with_indexes {
        ($iterator:ident<$t:ident>) =>{
            impl<$t> $crate::sortingutils::traits::SortingIterator<$t> for $iterator<$t>
                where $t: $crate::sortingutils::traits::VectorElement<$t> {
                fn get_vector(&self) -> &Vec<$t> {
                    &self.vector
                }
                fn take_vector(self: Box<Self>) -> Vec<$t> {
                    self.vector
                }
                fn get_indexes(&self) -> Option<&Vec<(usize, usize)>> {
                    Some(&self.indexes)
                }
                fn get_index(&self) -> usize{
                    self.index as usize
                }
                fn get_name(&self) -> &str{
                    stringify!($iterator)
                }
                fn get_comparator(&self) -> &ComparatorType<$t>{
                    &self.comparator
                }
                fn take_vector_and_comparator(self: Box<Self>) -> (Vec<$t>, fn(&$t, &$t) -> std::cmp::Ordering) {
                    (self.vector, *self.comparator)
                }
            }
        }
    }
    #[macro_export]
    macro_rules! get_sorting_iterator {
        ($iterator:ident, $vector: expr, $comparator:expr) => {
                Box::new($iterator::new($vector, Box::new($comparator)))
            };
        ($iterator:ident, $vector: expr) => {
                Box::new($iterator::new($vector))
            }
        }


    impl_sorting_iterator!(BogoSortIter<T>);
    impl_sorting_iterator!(CycleSortIter<T>);
    impl_sorting_iterator!(HeapSortIter<T>);
    impl_sorting_iterator_with_indexes!(MergeSortIter<T>);
    impl_sorting_iterator!(PancakeSortIter<T>);
    impl_sorting_iterator!(SleepSortIter<T>);
    impl_sorting_iterator!(StalinSortIter<T>);
}

#[cfg(test)]
mod sorting_tests {
    use rand::prelude::SliceRandom;
    use rand::thread_rng;
    use strum::IntoEnumIterator;
    use crate::sortingalgorithms::bogosort::BogoSortIter;
    use crate::sortingalgorithms::cyclesort::CycleSortIter;
    use crate::sortingalgorithms::heapsort::HeapSortIter;
    use crate::sortingalgorithms::mergesort::MergeSortIter;
    use crate::sortingalgorithms::pancakesort::PancakeSortIter;
    pub use crate::sortingutils::traits::{ComparatorType, SortingIterator, VectorElement};
    use crate::sortingutils::enums::{SortingPairPosHighlight, SortingPosHighlight};
    use crate::sortingutils::traits::{HasDefault, Quantifiable};
    use super::*;

    impl HasDefault<i32> for i32 {
        fn get_default() -> i32 {
            -999
        }
    }

    impl Quantifiable for i32 {
        fn to_num(&self) -> u32 {
            *self as u32
        }
    }

    impl VectorElement<i32> for i32 {}

    #[test]
    fn test_bogo_iterator() {
        let mut rng = thread_rng();
        let mut vector = vec![1, 2, 3];
        let final_vector = vec![1, 2, 3];
        vector.shuffle(&mut rng);
        let mut sorter = get_sorting_iterator!(BogoSortIter, vector, |x,y| i32::cmp(x,y));
        let mut res = sorter.next();
        while res.is_some() {
            res = sorter.next();
        }
        assert_eq!(sorter.get_vector(), &final_vector);
    }

    #[test]
    fn test_pos_highlight_iterators() {
        let mut rng = thread_rng();
        let mut vector = vec![2, 1, 5,4];
        let final_vector = vec![1, 2, 4, 5];
        // This initialization is needed so that the first take_vector can work
        let mut sorter: Box<dyn SortingIterator<i32, Item=usize>>;
        sorter = get_sorting_iterator!(CycleSortIter, vector, |x, y| i32::cmp(x, y));
        for sort_alg in SortingPosHighlight::iter() {
            vector = sorter.take_vector();
            vector.shuffle(&mut rng);
            sorter = match sort_alg {
                SortingPosHighlight::CycleSort => {
                    get_sorting_iterator!(CycleSortIter, vector, |x, y| i32::cmp(x, y))
                }
                SortingPosHighlight::PancakeSort => {
                    get_sorting_iterator!(PancakeSortIter, vector, |x, y| i32::cmp(x, y))
                }
            };
            let mut res = sorter.next();
            while res.is_some() {
                res = sorter.next();
            }
            assert_eq!(sorter.get_vector(), &final_vector);
        }
    }

    #[test]
    fn test_pair_pos_highlight_iterators() {
        let mut rng = thread_rng();
        let mut vector = vec![1, 2, 3];
        let final_vector = vec![1, 2, 3];
        // This initialization is needed so that the first take_vector can work
        let mut sorter: Box<dyn SortingIterator<i32, Item=(usize, usize)>>;
        sorter = get_sorting_iterator!(HeapSortIter, vector, |x, y| i32::cmp(x, y));
        for sort_alg in SortingPairPosHighlight::iter() {
            vector = sorter.take_vector();
            vector.shuffle(&mut rng);
            sorter = match sort_alg {
                SortingPairPosHighlight::HeapSort => {
                    get_sorting_iterator!(HeapSortIter, vector, |x, y| i32::cmp(x, y))
                }
                SortingPairPosHighlight::MergeSort => {
                    get_sorting_iterator!(MergeSortIter, vector, |x, y| i32::cmp(x, y))
                }
            };
            let mut res = sorter.next();
            while res.is_some() {
                res = sorter.next();
            }
            assert_eq!(sorter.get_vector(), &final_vector);
        }
    }
}

#[cfg(test)]
mod stalin_sort_test{
    use crate::get_sorting_iterator;
    use crate::sortingalgorithms::SortingIterator;
    use crate::sortingalgorithms::stalinsort::StalinSortIter;
    use crate::sortingutils::traits::HasDefault;

    #[test]
    fn test_results(){
        let vector = vec![2,1,5,4];
        let expected_vector = vec![2,5];
        let mut sorter = get_sorting_iterator!(StalinSortIter, vector, |x, y| i32::cmp(x, y));
        // The first element will never be removed
        assert_eq!(sorter.next(), Some(1));
        assert_eq!(sorter.next(), Some(i32::get_default()));
        assert_eq!(sorter.next(), Some(4));
        assert_eq!(sorter.take_vector(), expected_vector);
    }

    #[test]
    fn test_reverse(){
        let vector = vec![5,4,3,2];
        let expected_vector = vec![5];
        let mut sorter = get_sorting_iterator!(StalinSortIter, vector, |x, y| i32::cmp(x, y));
        assert_eq!(sorter.next(), Some(4));
        assert_eq!(sorter.next(), Some(3));
        assert_eq!(sorter.next(), Some(2));
        assert_eq!(sorter.take_vector(), expected_vector);
    }
}