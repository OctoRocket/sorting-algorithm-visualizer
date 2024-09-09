use super::SortingAlgorithm;
use std::{ops::Range, time};

pub struct MergeInPlace {
    // Statics
    name: String,
    delay: time::Duration,

    // Mutables
    list: Vec<usize>,
    slices: RecursiveSlices,
}

enum RecursiveSlices {
    Range((Range<usize>, bool)),
    Split(Box<RecursiveSlices>, Box<RecursiveSlices>),
}

impl Default for MergeInPlace {
    fn default() -> Self {
        let list = (1..=16).collect();
        Self {
            name: "Merge Sort In-Place".to_string(),
            delay: time::Duration::from_millis(120),
            list,
            slices: RecursiveSlices::Range((0..16, false)),
        }
    }
}

impl SortingAlgorithm for MergeInPlace {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_list(&self) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
        (vec![self.list.clone()], vec![])
    }

    fn set_list(&mut self, list: Vec<Vec<usize>>) {
        self.list = list.into_iter().flatten().collect();
        self.slices = RecursiveSlices::Range((0..self.list.len(), false));
    }

    fn get_delay(&self) -> time::Duration {
        self.delay
    }

    fn step(&mut self) {
        
    }
}
