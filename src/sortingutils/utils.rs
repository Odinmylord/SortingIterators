/// Takes a usize and gives back the value of its father at the 1st depth level of a heap vector tree.
/// It may be the value itself
pub(crate) fn get_first_depth_level(pos: usize) -> usize{
    if pos <= 2{
        return pos;
    }
    get_first_depth_level((pos-1)/2)
}

/// This function returns the length of a pair of usize elements
pub(crate) fn length_from_pair(pair: (usize, usize)) -> usize {
    if pair.0 > pair.1 {
        return 0;
    }
    pair.1 - pair.0 + 1
}