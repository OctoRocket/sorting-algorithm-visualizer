pub mod bogo_sort;

use std::time;

use bogo_sort::BogoSort;

pub fn get_available_algorithms() -> Vec<Box<dyn SortingAlgorithm>> {
    vec![
        Box::new(BogoSort::default()),
    ]
}

pub trait SortingAlgorithm {
    /// Get the name of the sorting algorithm.
    fn get_name(&self) -> &str;

    /// Get the list state of the sorting algorithm.
    fn get_list(&self) -> &Vec<Vec<usize>>;

    /// Set the list state of the sorting algorithm.
    fn set_list(&mut self, list: Vec<Vec<usize>>);

    /// Get the default delay of the sorting algorithm running
    fn get_delay(&self) -> time::Duration;

    /// Do one step of the sorting algorithm.
    fn step(&mut self);
}
