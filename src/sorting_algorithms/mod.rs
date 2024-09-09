pub mod bogo_sort;
pub mod merge_sort;
pub mod merge_in_place;

use std::time;

pub fn get_available_algorithms() -> Vec<Box<dyn SortingAlgorithm>> {
    vec![
        Box::new(bogo_sort::BogoSort::default()),
        Box::new(merge_sort::MergeSort::default()),
        Box::new(merge_in_place::MergeInPlace::default()),
    ]
}

pub trait SortingAlgorithm {
    /// Get the name of the sorting algorithm.
    fn get_name(&self) -> &str;

    /// Get the list state of the sorting algorithm and a list of indexes that
    /// should be highlighted.
    fn get_list(&self) -> (Vec<Vec<usize>>, Vec<(usize, usize)>);

    /// Set the list state of the sorting algorithm.
    fn set_list(&mut self, list: Vec<Vec<usize>>);

    /// Get the default delay of the sorting algorithm running
    fn get_delay(&self) -> time::Duration;

    /// Do one step of the sorting algorithm.
    fn step(&mut self);
}
