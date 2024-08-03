mod sorting_algorithms;

use rand::prelude::*;

use std::sync::Arc;
use eframe::egui::{self, epaint};
use sorting_algorithms::SortingAlgorithm;

const BAR_COLORS: [epaint::Color32; 4] = [
    epaint::Color32::RED,
    epaint::Color32::YELLOW,
    epaint::Color32::GREEN,
    epaint::Color32::BLUE,
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
    algorithm: Option<Box<dyn SortingAlgorithm>>,
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
    }
}

impl Default for ProgramState<usize> {
    fn default() -> Self {
        let algorithm = None;

        Self {
            list: vec![],
            algorithm,
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
                ui.heading("Settings");
                ui.add_space(15.0);
                if ui.button("Shuffle").clicked() {
                    self.shuffle();
                }
                if ui.button("TEST").clicked() {
                    self.list = vec![vec![0, 2], vec![1, 3]];
                }
                ui.horizontal(|ui| {
                    let mut length = self.list.iter().flatten().count();
                    ui.label("List length: ");
                    ui.add(egui::DragValue::new(&mut length));

                    if self.list.iter().flatten().count() != length {
                        self.list = vec![(1..=length).collect()];
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(35.0);
                ui.heading("Sorting Algorithm Visualizer");
            });
            ui.centered_and_justified(draw_graph(self.list.clone()));
        });
    }
}

fn draw_graph(list: Vec<Vec<usize>>) -> Box<dyn FnOnce(&mut egui::Ui)> {
    Box::new(move |ui| {
        ui.ctx().request_repaint();

        // x and y of the desired size of the frame is 1 times the width and 0.35
        // times the width respectively.
        let desired_size = ui.available_width() * egui::vec2(1.0, 0.35);
        let (_id, rect) = ui.allocate_space(desired_size);

        let bars = make_bars(rect, list, 10.0, 10.0, 10.0);

        ui.painter().extend(bars);
    })
}

fn make_bars(rect: egui::Rect, list: Vec<Vec<usize>>, base_height: f32, bar_spacing: f32, base_spacing: f32) -> Vec<epaint::Shape> {
    let mut bars = vec![];
    let max_height = rect.height() - base_height - 25.0;
    let bar_width = (rect.width() - bar_spacing) / list.iter().flatten().count() as f32 - bar_spacing;

    let filled_in = generate_filled_in(&list);
    let base_width = rect.width() / filled_in.len() as f32;
    let mut color_index = 0;
    for slot in filled_in.iter().enumerate() {
        if *slot.1 {
            let color = if list.len() == 1 {
                epaint::Color32::DARK_GRAY
            } else {
                BAR_COLORS[color_index % BAR_COLORS.len()]
            };

            let base = epaint::Shape::rect_filled(epaint::Rect::from_two_pos(
                epaint::pos2(rect.left() + slot.0 as f32 * base_width, rect.bottom()),
                epaint::pos2(rect.left() + (slot.0 + 1) as f32 * base_width, rect.bottom() - base_height),
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
    let min_value = *list.iter().flatten().min().unwrap_or(&0) as f32;
    for number in list.iter().flatten().enumerate() {
        let bar_height = (*number.1 as f32 - min_value + 1.0) / max_value * max_height;
        let bar_offset = (number.0 as f32 * (bar_width + bar_spacing)) + rect.left() + bar_spacing;

        let bar = epaint::Shape::rect_filled(
            egui::Rect::from_two_pos(
                epaint::pos2(bar_offset, rect.bottom() - base_height - base_spacing),
                epaint::pos2(bar_offset + bar_width, rect.bottom() - base_height - base_spacing - bar_height),
            ),
            epaint::Rounding::ZERO,
            epaint::Color32::WHITE,
        );
        bars.push(bar);
    }

    bars
}

fn generate_filled_in<T>(list: &[Vec<T>]) -> Vec<bool> {
    let mut result = list.iter().flat_map(|v| {
        let mut v = vec![true; v.len() * 4];
        v.push(false);
        v
    });
    result.next_back();
    result.collect()
}
