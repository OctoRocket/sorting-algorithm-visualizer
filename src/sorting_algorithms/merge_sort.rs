#[derive(Clone)]
pub struct MergeSort<'a> {
  // Statics, defaulted by the sorting algorithm and kept that way
  name: &'a str,

  // Mutables, these change as the sorting algorithm works.
  current_list: Vec<Vec<usize>>,
}

impl<'a> Default for MergeSort<'a> {
  fn default() -> Self {
    let name = "Merge Sort";
    let current_list = vec![(1..=16).collect::<Vec<usize>>()];

    Self { name, current_list }
  }
}

impl<'a> super::SortingAlgorithm for MergeSort<'a> {
  fn get_name(&self) -> &str {
    self.name
  }

  fn get_list(&self) -> &Vec<Vec<usize>> {
      &self.current_list
  }

  fn set_list(&mut self, list: Vec<Vec<usize>>) {
      self.current_list = list;
  }
}
