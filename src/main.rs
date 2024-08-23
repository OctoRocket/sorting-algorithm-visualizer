#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
)]

mod sorting_algorithms;

use std::{
    sync::Arc,
    time,
};
use eframe::egui::{self, epaint};
use rand::prelude::*;
use sorting_algorithms::SortingAlgorithm;

const BAR_COLORS: [epaint::Color32; 12] = [
    epaint::Color32::DARK_RED,
    epaint::Color32::RED,
    epaint::Color32::LIGHT_RED,
    epaint::Color32::BROWN,
    epaint::Color32::YELLOW,
    epaint::Color32::GOLD,
    epaint::Color32::DARK_GREEN,
    epaint::Color32::GREEN,
    epaint::Color32::LIGHT_GREEN,
    epaint::Color32::DARK_BLUE,
    epaint::Color32::BLUE,
    epaint::Color32::LIGHT_BLUE,
];

fn main() -> eframe::Result {
    let viewport = egui::ViewportBuilder::default()
        .with_icon(Arc::new(get_icon()))
        .with_min_inner_size(egui::vec2(500.0, 300.0));
    let options = eframe::NativeOptions {
        viewport,
        follow_system_theme: false,
        centered: true,
        ..Default::default()
    };

    eframe::run_native("Sorting Algorithm Visualizer", options, Box::new(|cc| Ok(Box::new(ProgramState::new(cc)))))
}

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

struct ProgramState<T: Ord> {
    // Lists
    list: Vec<Vec<T>>,
    sorted_list: Vec<T>,
    highlights: Vec<(usize, usize)>,

    // The algorithm and etc
    algorithm: Option<Box<dyn SortingAlgorithm>>,
    delay: time::Duration,

    // State bools
    running: bool,
    sorted: bool,

    // Timers
    time_of_last_step: time::SystemTime,
    sorted_animation_time: f64,
}

impl ProgramState<usize> {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();

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

        self.sorted = false;
    }

    fn draw_graph(&self) -> Box<dyn FnOnce(&mut egui::Ui) + '_> {
        Box::new(move |ui| {
            ui.ctx().request_repaint();

            // x and y of the desired size of the frame is 1 times the width and 0.35
            // times the width respectively.
            let desired_size = ui.available_width() * egui::vec2(1.0, 0.35);
            let (_, rect) = ui.allocate_space(desired_size);

            let bars = self.make_bars(rect, &self.list, 10.0, 10.0, ui.ctx());

            ui.painter().extend(bars);
        })
    }

    fn make_bars(
        &self,
        rect: egui::Rect,
        list: &[Vec<usize>],
        base_height: f32,
        base_spacing: f32,
        ctx: &egui::Context,
    ) -> Vec<epaint::Shape> {
        let mut bars = vec![];
        let max_height = rect.height() - base_height - 25.0;

        let filled_in = list.iter()
            .map(|l| vec![true; l.len()])
            .collect::<Vec<Vec<bool>>>()
            .join(&[false][..]);
        let bar_width = rect.width() / filled_in.len() as f32;
        let mut color_index = 0;
        for slot in filled_in.iter().enumerate() {
            if *slot.1 {
                let color = if list.len() == 1 {
                    epaint::Color32::DARK_GRAY
                } else {
                    BAR_COLORS[color_index % BAR_COLORS.len()]
                };

                let base = epaint::Shape::rect_filled(epaint::Rect::from_two_pos(
                    epaint::pos2((slot.0 as f32).mul_add(bar_width, rect.left()), rect.bottom()),
                    epaint::pos2(((slot.0 + 1) as f32).mul_add(bar_width, rect.left()), rect.bottom() - base_height),
                ), 0.0, color);

                bars.push(base);
            } else {
                color_index += 1;
            }
        }

        if list.is_empty() {
            return bars;
        }

        let max_value = *list.iter().flatten().max().unwrap_or(&0) as f32;
        let min_value = list.iter().flatten().min().unwrap_or(&0).saturating_sub(1) as f32;
        let list_directory = list
            .iter()
            .map(|v| 0..v.len())
            .scan(0, |len, list| {
                let new_list = list
                    .clone()
                    .map(|i| i + *len)
                    .collect::<Vec<_>>();
                *len += list.len() + 1; // Add one to account for the 0 values
                                        // in between
                Some(new_list)
            })
            .collect::<Vec<_>>();
        let highlight_indices = self.highlights
            .iter()
            // If there is an error here you probably set your highlights up
            // wrong
            .map(|(first_index, second_index)| list_directory[*first_index][*second_index])
            .collect::<Vec<_>>();

        for number in list.join(&[min_value as usize][..]).into_iter().enumerate() {
            let bar_height = ((number.1 as f32 - min_value) / max_value) * max_height;
            let color = if ctx.input(|i| i.time) - self.sorted_animation_time < 0.25 {
                epaint::Color32::LIGHT_GREEN
            } else if highlight_indices.contains(&number.0) {
                epaint::Color32::LIGHT_RED
            } else {
                epaint::Color32::WHITE
            };

            let bar = epaint::Shape::rect_filled(
                egui::Rect::from_two_pos(
                    epaint::pos2(bar_width.mul_add(number.0 as f32, rect.left()), rect.bottom() - base_height - base_spacing),
                    epaint::pos2(bar_width.mul_add((number.0 + 1) as f32, rect.left()), rect.bottom() - base_height - base_spacing - bar_height),
                ),
                epaint::Rounding::ZERO,
                color,
            );
            bars.push(bar);
        }

        bars
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

            running: false,
            sorted: false,

            time_of_last_step: time::UNIX_EPOCH,
            sorted_animation_time: -1000.0,
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
            ui.centered_and_justified(self.draw_graph());
        });

        // Things that need to be updated every frame (e.g. checking if the list
        // is sorted (maybe should be moved so the sorting algorithm says when
        // the list is sorted), updating the program state's list to be in sync
        // with the algorithm's list (maybe should just be an empty algorithm?),
        // etc)
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
                if ui.add(egui::Button::new("Play").min_size(button_size)).clicked() && !state.sorted {
                    state.running = true;
                }
                if ui.add(egui::Button::new("Step").min_size(button_size)).clicked() && !state.sorted {
                    state.running = false;

                    if let Some(ref mut algorithm) = &mut state.algorithm {
                        algorithm.step();
                    }
                }
                if ui.add(egui::Button::new("Pause").min_size(button_size)).clicked() && !state.sorted {
                    state.running = false;
                }
            });
            if ui.button("Shuffle").clicked() {
                state.shuffle();
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
        state.highlights = if state.running {
            algorithm.get_list().1
        } else {
            vec![]
        };
    }
    let mut flat_list = state.list.clone().into_iter().flatten().collect::<Vec<usize>>();
    if !state.sorted && flat_list == state.sorted_list {
        state.sorted = true;
        state.running = false;
        state.sorted_animation_time = ctx.input(|i| i.time);
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
