use super::SortingAlgorithm;

use std::time;

pub struct BubbleSort {
    // Static
    name: &'static str,
    default_delay: time::Duration,

    // Mutable
    list: Vec<usize>,

    bubble_index: usize,
    sorted_index: usize,
}

impl Default for BubbleSort {
    fn default() -> Self {
        let list: Vec<usize> = (1..=16).collect();
        Self {
            name: "Bubble Sort",
            default_delay: time::Duration::from_millis(120),

            bubble_index: 0,
            sorted_index: list.len() - 1,
            list,
        }
    }
}

impl SortingAlgorithm for BubbleSort {
    fn get_delay(&self) -> std::time::Duration {
        self.default_delay
    }

    fn get_list(&self) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
        let mut hightlights = vec![
            (0, self.bubble_index),
            // (0, self.bubble_index + 1),
        ];
        if self.sorted_index < self.list.len() {
            hightlights.push((0, self.sorted_index));
        }

        (vec![self.list.clone()], hightlights)
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn set_list(&mut self, list: Vec<Vec<usize>>) {
        self.list.clone_from(&list[0]);
        self.sorted_index = list[0].len() - 1;
        self.bubble_index = 0;
    }

    fn step(&mut self) {
        if self.list[self.bubble_index] > self.list[self.bubble_index + 1] {
            self.list.swap(self.bubble_index, self.bubble_index + 1);
        }

        self.bubble_index += 1;

        if self.bubble_index == self.sorted_index {
            self.bubble_index = 0;
            self.sorted_index -= 1;
        }
    }
}
