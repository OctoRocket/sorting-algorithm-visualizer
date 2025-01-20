use super::SortingAlgorithm;
use std::time;

// Stores a start index and an end index (exclusive)
#[derive(Debug, Clone, Copy)]
struct Bounds {
    start: usize,
    end: usize,
    times_sorted: u32,
}

impl Bounds {
    const fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            times_sorted: 0,
        }
    }

    const fn len(&self) -> usize {
        self.end.abs_diff(self.start)
    }

    const fn split(&self) -> (Self, Self) {
        (
            Self::new(self.start, self.end / 2),
            Self::new(self.end / 2, self.end),
        )
    }
}

pub struct MergeInPlace {
    // Statics
    name: String,
    delay: time::Duration,

    // Mutables
    list: Vec<usize>,
    /// Represents a list of slices of the main list
    slices: Vec<Bounds>,
    slice_index: usize,
    merging: bool,
    /// Increases by one when everything is sorted to a certain lower degree
    desired_sortedness: u32
}

impl MergeInPlace {
    fn set_list(&mut self, list: Vec<usize>) {
        self.list = list;
        self.slices = vec![Bounds::new(0, self.list.len())];
    }
}

impl Default for MergeInPlace {
    fn default() -> Self {
        let mut alg = Self {
            name: "Merge Sort In-Place".to_string(),
            delay: time::Duration::from_millis(120),

            list: vec![],
            slices: vec![],
            slice_index: 0,
            merging: false,
        };
        let list = (1..=16).collect();

        alg.set_list(list);

        alg
    }
}

impl SortingAlgorithm for MergeInPlace {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_list(&self) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
        (
            vec![self.list.clone()],
            // The first slice, as long as it isn't the whole list
            if self.slices.len() > 1 {
                (self.slices[self.slice_index].start..self.slices[self.slice_index].end)
                    .map(|x| (0, x))
                    .collect()
            } else {
                vec![]
            },
        )
    }

    fn set_list(&mut self, list: Vec<Vec<usize>>) {
        self.set_list(list.into_iter().flatten().collect());
    }

    fn get_delay(&self) -> time::Duration {
        self.delay
    }

    fn step(&mut self) {
        // If two sections can be merged
        if self.slices.len() > 1 && self.slices[0].times_sorted == self.slices[1].times_sorted {
            self.merging = true;
            // self.merge();
        }
        // Otherwise
        else {
            self.merging = false;
            // Split unsorted slices
            if !self.slices[self.slice_index].times_sorted && self.slices[self.slice_index].len() > 1 {
                let new_bounds = self.slices[self.slice_index].split();

                self.slices.remove(self.slice_index);
                self.slices.insert(self.slice_index, new_bounds.0);
                self.slices.insert(self.slice_index + 1, new_bounds.1);
            }
            // Else, mark length 1 slice unsorted
            else if !self.slices[self.slice_index].times_sorted && self.slices[self.slice_index].len() == 1 {
                self.slices[self.slice_index].times_sorted = true;
                self.slice_index += 1;
            }
        }
    }
}
