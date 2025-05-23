use super::SortingAlgorithm;
use std::{ptr, time};

/// Represents a sublist with a start index (inclusive) and end index
/// (exclusive).
#[derive(Debug, Clone, Copy)]
struct Bound {
    start: usize,
    end: usize,
    /// If the sublist is sorted or unsorted.
    sorted: bool,
    /// How many times a subsection has been merged with another.
    times_merged: u32,
}

impl Bound {
    pub const fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            sorted: false,
            times_merged: 0,
        }
    }

    /// All indices contained in the bounds.
    pub fn indices(&self) -> Vec<usize> {
        (self.start..self.end).collect()
    }

    pub const fn split(self) -> (Self, Self) {
        (
            Self::new(self.start, usize::midpoint(self.start, self.end)),
            Self::new(usize::midpoint(self.start, self.end), self.end),
        )
    }

    pub const fn len(&self) -> usize {
        self.end.abs_diff(self.start)
    }
}

pub struct MergeInPlace {
    // Statics
    name: &'static str,
    default_delay: time::Duration,

    // Mutables
    list: Vec<usize>,
    sublists: Vec<Bound>,
    /// Points to the sublist currently being modified.
    sublist_index: usize,
}

impl MergeInPlace {
    fn set_list(&mut self, list: Vec<usize>) {
        self.list = list;
        self.sublists = vec![Bound::new(0, self.list.len())];
        self.sublist_index = 0;
    }

    /// Requires that the first sublist comes before the second sublist (and
    /// they are ordered consecutively).
    /// Returns a new bound that covers both sublists.
    fn merge_sublists(&mut self, first_sublist: Bound, second_sublist: Bound) -> Bound {
        let mut first_index = first_sublist.start;
        let mut second_index = second_sublist.start;

        while first_index < second_index && second_index < second_sublist.end {
            if self.list[first_index] > self.list[second_index] {
                shift_down(&mut self.list, second_index, first_index);
                second_index += 1;
            } else {
                first_index += 1;
            }
        }

        let mut new_bound = Bound::new(first_sublist.start, second_sublist.end);
        new_bound.sorted = true;
        new_bound.times_merged = first_sublist.times_merged + 1;
        new_bound
    }
}

impl Default for MergeInPlace {
    fn default() -> Self {
        let mut alg = Self {
            name: "Merge Sort (In place)",
            default_delay: time::Duration::from_millis(120),

            list: vec![],
            sublists: vec![],
            sublist_index: 0,
        };
        let list = (1..=16).collect();

        alg.set_list(list);

        alg
    }
}

impl SortingAlgorithm for MergeInPlace {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_list(&self) -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
        let highlights = self.sublists[self.sublist_index].indices()
            .into_iter()
            .map(|i| (0, i))
            .collect();

        (vec![self.list.clone()], highlights)
    }

    fn set_list(&mut self, list: Vec<Vec<usize>>) {
        self.set_list(list[0].clone());
    }

    fn get_delay(&self) -> time::Duration {
        self.default_delay
    }

    // Things that need to happen:
    // - If the section currently being worked on is 1 item long, make it
    // sorted.
    // - If the section being currently worked on is not sorted, split it.
    // - If the section is sorted and the section before it is sorted, check if
    // the section before it has been merged the same number of times as the
    // current section.
    //   - If yes: Merge this section with the previous section.
    //   - If no: Move on to the next section (Increment `sublist_index`).
    // - If we're at the end of the list and at a sorted sublist, merge
    // reguardless of sortedness and then move the `sublist_index` back one.`
    fn step(&mut self) {
        let working_sublist = &mut self.sublists[self.sublist_index];

        // If a sublist is one element long, it must be sorted.
        if working_sublist.len() == 1 {
            working_sublist.sorted = true;
        }

        let working_sublist = *working_sublist;
        let previous_sublist = if self.sublist_index == 0 {
            None
        } else {
            Some(self.sublists[self.sublist_index - 1])
        };
        // Check if we are at the end and everything is sorted; if we are,
        // merge.
        if working_sublist.sorted && self.sublist_index == self.sublists.len() - 1 {
            if let Some(previous) = previous_sublist {
                let new_sublist = self.merge_sublists(previous, working_sublist);
                self.sublist_index -= 1;
                self.sublists.remove(self.sublist_index);
                self.sublists.remove(self.sublist_index);
                self.sublists.insert(self.sublist_index, new_sublist);
            }

            return;
        }

        // Check if the current section is sorted; if not, split.
        if !working_sublist.sorted {
            let (first_half, second_half) = working_sublist.split();
            self.sublists.remove(self.sublist_index);
            self.sublists.insert(self.sublist_index, second_half);
            self.sublists.insert(self.sublist_index, first_half);

            return;
        }

        // If there is a previous sublist and it is sorted to the same degree as
        // us, merge with it.
        if let Some(previous) = previous_sublist {
            if working_sublist.sorted && previous.sorted && previous.times_merged == working_sublist.times_merged {
                let new_sublist = self.merge_sublists(previous, working_sublist);
                self.sublist_index -= 1;
                self.sublists.remove(self.sublist_index);
                self.sublists.remove(self.sublist_index);
                self.sublists.insert(self.sublist_index, new_sublist);

                return;
            }
        }

        // Else if none of that is true, just go on to the next segment.
        self.sublist_index += 1;
    }
}

fn shift_down<T>(list: &mut Vec<T>, from: usize, to: usize) {
    assert!((from < list.len()), "Tried to move from out of bounds index! (Index was {from} but list length is {})", list.len());
    assert!((to <= list.len()), "Tried to move to out of index! (Index was {to} but list length is {})", list.len());
    assert!((to <= from), "Tried to move from smaller index to larger index! (From index {from} to index {to})");

    unsafe {
        let from_ptr = list.as_mut_ptr().add(from);
        let to_ptr = list.as_mut_ptr().add(to);
        let moved = from_ptr.read();
        ptr::copy(to_ptr, to_ptr.add(1), from - to);
        ptr::write(to_ptr, moved);
    }
}
