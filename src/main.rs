mod sorting_algorithms;

use rand::prelude::*;

use std::sync::Arc;
use eframe::egui::{self, epaint};
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
    list: Vec<Vec<T>>,
    sorted_list: Vec<T>,
    algorithm: Option<Box<dyn SortingAlgorithm>>,
    running: bool,
    sorted: bool,
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
            algorithm.set_list(self.list.clone())
        }

        self.sorted = false;
    }
}

impl Default for ProgramState<usize> {
    fn default() -> Self {
        Self {
            list: vec![],
            sorted_list: vec![],
            algorithm: None,
            running: false,
            sorted: false,
        }
    }
}

impl eframe::App for ProgramState<usize> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right(egui::Id::new("algorithm selection panel")).resizable(false).show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(8.0);
                ui.heading("Choose the Agorithm");
                ui.add_space(15.0);

                for algorithm in sorting_algorithms::get_available_algorithms() {
                    if ui.button(algorithm.get_name()).clicked() {
                        self.list = algorithm.get_list().to_vec();
                        self.algorithm = Some(algorithm);
                    }
                }
            });
        });

        egui::SidePanel::left(egui::Id::new("settings panel")).resizable(false).show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(8.0);
                ui.heading("Controls");
                ui.add_space(15.0);

                // Buttons
                ui.horizontal(|ui| {
                    let button_size = egui::vec2(ui.available_width() / 3.0 - ui.spacing().button_padding.x * 1.35, 0.0);
                    if ui.add(egui::Button::new("Play").min_size(button_size)).clicked() && !self.sorted {
                        self.running = true;
                    }
                    if ui.add(egui::Button::new("Step").min_size(button_size)).clicked() && !self.sorted {
                        self.running = false;

                        if let Some(ref mut algorithm) = &mut self.algorithm {
                            algorithm.step();
                        }
                    }
                    if ui.add(egui::Button::new("Pause").min_size(button_size)).clicked() && !self.sorted {
                        self.running = false;
                    }
                });
                if ui.button("Shuffle").clicked() {
                    self.shuffle();
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
                    let mut length = self.list.iter().flatten().count();
                    ui.label("List length: ");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                        ui.add(egui::DragValue::new(&mut length).speed(0.05));
                    });

                    if self.list.iter().flatten().count() != length {
                        self.list = vec![(1..=length).collect()];

                        if let Some(algorithm) = &mut self.algorithm {
                            algorithm.set_list(self.list.clone());
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(35.0);
                ui.heading("Sorting Algorithm Visualizer");
            });
            ui.centered_and_justified(draw_graph(self.list.clone()));
        });

        // Updating logic
        if let Some(algorithm) = &self.algorithm {
            self.list.clone_from(algorithm.get_list());
        }
        let mut flat_list = self.list.clone().into_iter().flatten().collect::<Vec<usize>>();
        if flat_list == self.sorted_list {
            self.sorted = true;
        } else if flat_list.len() != self.sorted_list.len() {
            flat_list.sort_unstable();
            self.sorted_list = flat_list;
        }
    }
}

fn draw_graph(list: Vec<Vec<usize>>) -> Box<dyn FnOnce(&mut egui::Ui)> {
    Box::new(move |ui| {
        ui.ctx().request_repaint();

        // x and y of the desired size of the frame is 1 times the width and 0.35
        // times the width respectively.
        let desired_size = ui.available_width() * egui::vec2(1.0, 0.35);
        let (_id, rect) = ui.allocate_space(desired_size);

        let bars = make_bars(rect, list, 10.0, 10.0);

        ui.painter().extend(bars);
    })
}

fn make_bars(rect: egui::Rect, list: Vec<Vec<usize>>, base_height: f32, base_spacing: f32) -> Vec<epaint::Shape> {
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
                epaint::pos2(rect.left() + slot.0 as f32 * bar_width, rect.bottom()),
                epaint::pos2(rect.left() + (slot.0 + 1) as f32 * bar_width, rect.bottom() - base_height),
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
    for number in list.join(&[min_value as usize][..]).into_iter().enumerate() {
        let bar_height = ((number.1 as f32 - min_value) / max_value) * max_height;

        let bar = epaint::Shape::rect_filled(
            egui::Rect::from_two_pos(
                epaint::pos2(rect.left() + bar_width * number.0 as f32, rect.bottom() - base_height - base_spacing),
                epaint::pos2(rect.left() + bar_width * (number.0 + 1) as f32, rect.bottom() - base_height - base_spacing - bar_height),
            ),
            epaint::Rounding::ZERO,
            epaint::Color32::WHITE,
        );
        bars.push(bar);
    }

    bars
}
