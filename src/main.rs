#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
)]

mod sorting_algorithms;
mod visualizations;

use std::{
    sync::Arc,
    time,
};
use eframe::egui::{
    self,
    epaint,
};
use rand::prelude::*;
use sorting_algorithms::SortingAlgorithm;
use visualizations::Visualizer;

fn main() -> eframe::Result {
    let viewport = egui::ViewportBuilder::default()
        .with_icon(Arc::new(get_icon()))
        .with_min_inner_size(egui::vec2(500.0, 300.0));
    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Sorting Algorithm Visualizer",
        options,
        Box::new(|cc| Ok(Box::new(ProgramState::new(cc)))))
}

struct ProgramState<T: Ord> {
    // Lists
    list: Vec<Vec<T>>,
    sorted_list: Vec<T>,
    highlights: Vec<(usize, usize)>,

    // The algorithm and related parameters
    algorithm: Option<Box<dyn SortingAlgorithm>>,
    delay: time::Duration,
    time_of_last_step: time::SystemTime,

    // Visualizers
    visualizer: visualizations::bar_graph::BarGraph,

    // State bools
    running: bool,
    show_hightlights: bool,
}

impl ProgramState<usize> {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    // TODO: Should this be a function of the program state?
    fn shuffle(&mut self) {
        let mut rng = rand::rng();

        let mut new_list = self.list
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<usize>>();
        new_list.shuffle(&mut rng);

        self.list = vec![new_list];

        if let Some(algorithm) = &mut self.algorithm {
            algorithm.set_list(self.list.clone());
        }

        self.visualizer.unsorted();
    }
}

impl Default for ProgramState<usize> {
    fn default() -> Self {
        Self {
            list: vec![],
            sorted_list: vec![],
            highlights: vec![],

            algorithm: None,
            delay: time::Duration::from_millis(100),
            time_of_last_step: time::UNIX_EPOCH,

            visualizer: visualizations::bar_graph::BarGraph::default(),
            running: false,
            show_hightlights: true,
        }
    }
}

impl eframe::App for ProgramState<usize> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        draw_algorithm_selection(self, ctx);

        draw_settings_panel(self, ctx);

        // Use the rest of the space in the middle to show the actual graph
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(25.0);
                ui.heading("Sorting Algorithm Visualizer");
            });
            ui.centered_and_justified(self.visualizer.draw_graph(&self.list, &self.highlights));
        });

        // Things that need to be updated every frame (e.g. checking if the list
        // is sorted, updating the program state's list to be in sync with the
        // algorithm's list, etc)
        frame_update(self, ctx);
    }
}

// Draw right panel
fn draw_algorithm_selection(state: &mut ProgramState<usize>, ctx: &egui::Context) {
    egui::SidePanel::right(egui::Id::new("algorithm selection panel")).resizable(false).show(ctx, |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(8.0);
            ui.heading("Choose the Agorithm");
            ui.add_space(15.0);

            for algorithm in sorting_algorithms::get_available_algorithms() {
                if ui.button(algorithm.get_name()).clicked() {
                    state.list = algorithm.get_list().0.into_iter().collect();
                    state.delay = algorithm.get_delay();
                    state.algorithm = Some(algorithm);
                }
            }
        });
    });
}

// Draw the left panel
fn draw_settings_panel(state: &mut ProgramState<usize>, ctx: &egui::Context) {
    egui::SidePanel::left(egui::Id::new("settings panel")).resizable(false).show(ctx, |ui| {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(8.0);
            ui.heading("Controls");
            ui.add_space(15.0);

            // Buttons
            ui.horizontal(|ui| {
                let button_size = egui::vec2(ui.spacing().button_padding.x.mul_add(-1.35, ui.available_width() / 3.0), 0.0);
                if ui.add(egui::Button::new("Play").min_size(button_size)).clicked() {
                    state.running = true;
                }
                if ui.add(egui::Button::new("Step").min_size(button_size)).clicked() {
                    state.running = false;

                    if let Some(ref mut algorithm) = &mut state.algorithm {
                        algorithm.step();
                    }
                }
                if ui.add(egui::Button::new("Pause").min_size(button_size)).clicked() {
                    state.running = false;
                }
            });
            if ui.button("Shuffle").clicked() {
                state.shuffle();
            }
            if ui.button("Toggle Highlights").clicked() {
                state.show_hightlights = !state.show_hightlights;
            }

            // Draw seperating bar
            ui.add_space(10.0);
            let bar_height = 1.0;
            let rect = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), bar_height),
                egui::Sense::hover(),
            ).0;
            ui.painter().add(epaint::Shape::rect_filled(rect, 0.0, epaint::Color32::DARK_GRAY));
            ui.add_space(10.0);

            ui.heading("Settings");
            ui.add_space(10.0);

            // Sliders
            ui.horizontal(|ui| {
                let mut length = state.list.iter().flatten().count();
                ui.label("List length: ");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.add(egui::DragValue::new(&mut length).speed(0.05));
                });

                if state.list.iter().flatten().count() != length {
                    state.list = vec![(1..=length).collect()];

                    if let Some(algorithm) = &mut state.algorithm {
                        algorithm.set_list(state.list.clone());
                    }
                }
            });
            ui.horizontal(|ui| {
                let mut delay = state.delay.as_millis() as u64;
                ui.label("Time between steps:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.add(egui::DragValue::new(&mut delay).speed(0.25));
                });

                if delay != state.delay.as_millis() as u64 {
                    state.delay = time::Duration::from_millis(delay);

                    if let Some(algorithm) = &mut state.algorithm {
                        algorithm.set_list(state.list.clone());
                    }
                }
            });
        });
    });
}

// Updating logic
fn frame_update(state: &mut ProgramState<usize>, ctx: &egui::Context) {
    if let Some(algorithm) = &state.algorithm {
        state.list = algorithm.get_list().0.into_iter().collect();
        state.highlights = if state.show_hightlights {
            algorithm.get_list().1
        } else {
            vec![]
        };
    }
    let mut flat_list = state.list.clone().into_iter().flatten().collect::<Vec<usize>>();
    if flat_list == state.sorted_list {
        state.running = false;
        state.visualizer.sorted(ctx);
    } else if flat_list.len() != state.sorted_list.len() {
        flat_list.sort_unstable();
        state.sorted_list = flat_list;
    }

    if let Some(algorithm) = &mut state.algorithm {
        if state.running && time::SystemTime::now().duration_since(state.time_of_last_step).unwrap() > state.delay {
            state.time_of_last_step = time::SystemTime::now();
            algorithm.step();
        }
    }
}

// Utility functions

// Get the program icon
// The icon isn't displayed on wayland for some reason.
fn get_icon() -> egui::IconData {
    let icon = include_bytes!("../icon.png");
    let image = image::load_from_memory(icon).unwrap().to_rgba8();

    let (width, height) = image.dimensions();
    egui::IconData {
        rgba: image.to_vec(),
        width,
        height,
    }
}
