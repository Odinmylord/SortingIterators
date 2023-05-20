use std::fmt::Debug;
use std::cmp::Ordering;

pub trait Quantifiable {
    fn to_num(&self) -> u32;
}


pub trait HasDefault<T>{
    fn get_default() -> T;
}

pub trait VectorElement<T>: Quantifiable + HasDefault<T> + Debug {}

pub trait SortingIterator<T>: Iterator {
    fn get_vector(&self) -> &Vec<T>;
    fn take_vector(self: Box<Self>) -> Vec<T>;
    fn get_name(&self) -> &str;
    fn get_indexes(&self) -> Option<&Vec<(usize, usize)>>;
    fn get_index(&self) -> usize;
    fn get_comparator(&self) -> &ComparatorType<T>;
    fn take_vector_and_comparator(self: Box<Self>) -> (Vec<T>, fn(&T, &T) -> Ordering);
}

pub type ComparatorType<T> = Box<fn(&T, &T) -> Ordering>;

