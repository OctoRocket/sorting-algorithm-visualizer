use super::SortingAlgorithm;

use std::time;
use rand::prelude::*;

#[derive(Clone)]
pub struct BogoSort {
    // Statics, defaulted by the sorting algorithm and kept that way
    name: &'static str,
    default_delay: time::Duration,

    // Mutables, these change as the sorting algorithm works.
    current_list: Vec<Vec<usize>>,
    rng: rand::rngs::ThreadRng,
}

impl Default for BogoSort {
    fn default() -> Self {
        Self {
            name: "Bogo Sort",
            default_delay: time::Duration::from_millis(40),
            current_list: vec![(1..=4).collect()],
            rng: rand::thread_rng(),
        }
    }
}

impl SortingAlgorithm for BogoSort {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_list(&self) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
        (self.current_list.clone(), vec![])
    }

    fn set_list(&mut self, list: Vec<Vec<usize>>) {
        self.current_list = list;
    }

    fn step(&mut self) {
        self.current_list.get_mut(0).unwrap().shuffle(&mut self.rng);
    }

    fn get_delay(&self) -> std::time::Duration {
        self.default_delay
    }
}
