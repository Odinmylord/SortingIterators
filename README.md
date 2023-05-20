# Rust Sorting Iterators
A rust library to sort vectors (containing any type of elements) with classes that are iterators.\
With this library it will be easy to understand how some algorithms work step-by-step.

# Algorithms
## Categories

There are three categories of algorithm. Each category is represented by an enum in the code:
- **SortingPosHighlight** (__CycleSort__, __PancakeSort__): Algorithms that return the index of the element that they are moving;
- **SortingPairPosHighlight** (__HeapSort__, __MergeSort__): Algorithms that return a pair of indexes that represents the range that they are working on;
- **SortingElemGiven** (__SleepSort__, __StalinSort__): Algorithms that return the element itself;

There also is __BogoSort__ which isn't part of the above-mentioned categories.
## How to add an algorithm

- Add the __algorithm.rs__ file to the __sortingalgorithms__ folder;
- (Optional) Add the respective class to one of the enums;
- Add a call to the macro that you think fits best at the end of the `mod sortingalgorithms` block;

# Usage
To use the algorithms simply implement the `VectorElement` trait for the type that you are going to use. \
After this using the `get_sorting_iterator(EnumElement, vector, comparator)` or by calling `new()` on the class you can get an istance.\
The next step is to call the `.next()` method and see what element the algorithm is working on.

# License
Shield: [![CC BY-SA 4.0][cc-by-sa-shield]][cc-by-sa]

This work is licensed under a
[Creative Commons Attribution-ShareAlike 4.0 International License][cc-by-sa].

[![CC BY-SA 4.0][cc-by-sa-image]][cc-by-sa]

[cc-by-sa]: http://creativecommons.org/licenses/by-sa/4.0/
[cc-by-sa-image]: https://licensebuttons.net/l/by-sa/4.0/88x31.png
[cc-by-sa-shield]: https://img.shields.io/badge/License-CC%20BY--SA%204.0-lightgrey.svg