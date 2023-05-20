use strum::EnumIter;

#[derive(EnumIter)]
pub enum SortingPosHighlight {
    CycleSort,
    PancakeSort,
}

#[derive(EnumIter)]
pub enum SortingPairPosHighlight {
    HeapSort,
    MergeSort,
}

#[derive(EnumIter)]
pub enum SortingElemGiven {
    SleepSort,
    StalinSort,
}