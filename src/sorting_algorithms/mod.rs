pub mod merge_sort;

use merge_sort::MergeSort;

pub fn get_available_algorithms() -> Vec<Box<dyn SortingAlgorithm>> {
  vec![
    Box::new(MergeSort::default()),
  ]
}

pub trait SortingAlgorithm {
  fn get_name(&self) -> &str;

  fn get_list(&self) -> &[usize];
}
