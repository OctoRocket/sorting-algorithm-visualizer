use super::SortingAlgorithm;

use rand::prelude::*;

#[derive(Clone)]
pub struct BogoSort {
    // Statics, defaulted by the sorting algorithm and kept that way
    name: &'static str,

    // Mutables, these change as the sorting algorithm works.
    current_list: Vec<Vec<usize>>,
    rng: rand::rngs::ThreadRng,
}

impl Default for BogoSort {
    fn default() -> Self {
        Self {
            name: "Bogo Sort",
            current_list: vec![(1..=4).collect()],
            rng: rand::thread_rng(),
        }
    }
}

impl SortingAlgorithm for BogoSort {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_list(&self) -> &Vec<Vec<usize>> {
        &self.current_list
    }

    fn set_list(&mut self, list: Vec<Vec<usize>>) {
        self.current_list = list;
    }

    fn step(&mut self) {
        self.current_list.get_mut(0).unwrap().shuffle(&mut self.rng);
    }
}
