use super::SortingAlgorithm;

use std::time;

#[derive(Clone)]
pub struct MergeSort {
    // Statics, defaulted by the sorting algorithm and kept that way
    name: &'static str,
    default_delay: time::Duration,

    // Mutables, these change as the sorting algorithm works.
    merge_tree: MergeTree,
}

#[derive(Clone, Debug)]
enum MergeTree {
    Branch(Box<MergeTree>, Box<MergeTree>),
    Leaf(Vec<usize>, bool), // The contents and whether they are sorted.
}

impl MergeTree {
    fn new(list: Vec<usize>) -> Self {
        Self::Leaf(list, false)
    }

    fn flatten(&self) -> Vec<Self> {
        match self {
            Self::Leaf(..) => vec![self.clone()],
            Self::Branch(left, right) => {
                [left.flatten(), right.flatten()].concat()
            },
        }
    }
}

impl Default for MergeSort {
    fn default() -> Self {
        Self {
            name: "Merge Sort",
            default_delay: time::Duration::from_millis(120),
            merge_tree: MergeTree::new((1..=16).collect()),
        }
    }
}

impl SortingAlgorithm for MergeSort {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_list(&self) -> Vec<Vec<usize>> {
        self.merge_tree
            .flatten()
            .into_iter()
            .map(|tree| if let MergeTree::Leaf(list, ..) = tree {
                list
            } else {
                unreachable!()
            }).collect()
    }

    fn set_list(&mut self, list: Vec<Vec<usize>>) {
        self.merge_tree = MergeTree::new(list.into_iter().flatten().collect());
    }

    fn get_delay(&self) -> time::Duration {
        self.default_delay
    }

    fn step(&mut self) {
        fn recurse_down(tree: &mut MergeTree) {
            match tree {
                MergeTree::Leaf(ref mut list, ref mut sorted) => {
                    if *sorted {
                        return;
                    }
                    if list.len() == 1 {
                        *sorted = true;
                        return;
                    }

                    let (left, right) = list.split_at(list.len() / 2);
                    let new_tree = MergeTree::Branch(
                        Box::new(MergeTree::Leaf(left.to_vec(), false)),
                        Box::new(MergeTree::Leaf(right.to_vec(), false)),
                    );
                    *tree = new_tree;
                },

                MergeTree::Branch(ref mut left, ref mut right) => {
                    let mut option_leaf = None;
                    if let MergeTree::Leaf(ref l_list, l_sorted) = **left {
                        if let MergeTree::Leaf(ref r_list, r_sorted) = **right {
                            if l_sorted && r_sorted {
                                option_leaf = Some(MergeTree::Leaf(merge(l_list, r_list), true));
                            }
                        }
                    }

                    if option_leaf.is_some() {
                        *tree = option_leaf.unwrap();
                    } else {
                        match **left {
                            MergeTree::Leaf(_, sorted) => {
                                if sorted {
                                    recurse_down(right);
                                } else {
                                    recurse_down(left);
                                }
                            },
                            MergeTree::Branch(..) => {
                                recurse_down(left);
                            },
                        }
                    }
                },
            }
        }

        recurse_down(&mut self.merge_tree);
    }
}

fn merge(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut new_list = vec![];

    let mut a_index = 0;
    let mut b_index = 0;

    loop {
        if a_index == a.len() {
            new_list.extend_from_slice(&b[b_index..]);
            break;
        } else if b_index == b.len() {
            new_list.extend_from_slice(&a[a_index..]);
            break;
        }

        if a[a_index] < b[b_index] {
            new_list.push(a[a_index]);
            a_index += 1;
        } else {
            new_list.push(b[b_index]);
            b_index += 1;
        }
    }

    new_list
}
