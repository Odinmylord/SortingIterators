use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use sorting_iterators;
use sorting_iterators::sortingalgorithms::mergesort::MergeSortIter;
use sorting_iterators::sortingalgorithms::{SortingIterator, VectorElement};
use sorting_iterators::sortingutils::traits::{HasDefault, Quantifiable};

// Iterators require vector elements to implement a trait so to use non user-defined types a wrapper
// is needed.
struct I32Container{
    val: i32
}
impl I32Container{
    pub fn new(x:i32) -> Self{
        I32Container{
            val: x,
        }
    }
}

impl Quantifiable for I32Container {
    fn to_num(&self) -> u32 {
        self.val as u32
    }
}

impl HasDefault<I32Container> for I32Container {
    fn get_default() -> I32Container {
        I32Container::new(-1)
    }
}

impl Debug for I32Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.val.to_string().as_str())
    }
}

fn compare_i32_container(x:&I32Container, y:&I32Container) -> Ordering {
    x.val.cmp(&y.val)
}

impl VectorElement<I32Container> for I32Container{}
fn main() {
    let vector = vec![I32Container::new(2),I32Container::new(5),I32Container::new(4),I32Container::new(3)];
    let mut var = sorting_iterators::get_sorting_iterator!(MergeSortIter, vector, compare_i32_container);
    let mut res = var.next();
    while res.is_some(){
        res = var.next();
    }
    dbg!(var.take_vector());
}
