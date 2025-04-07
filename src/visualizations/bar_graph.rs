use super::Visualizer;

use eframe::{egui, epaint};

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

const SORTED_ANIMATION_LENGTH: f64 = 0.25;

pub struct BarGraph {
    // State vars
    sorted: bool,

    // Timers
    sorted_animation_time: f64,
}

impl BarGraph {
    /// TODO: Make these consts instead of parameters
    fn make_bars(
        &self,
        rect: egui::Rect,
        list: &[Vec<usize>],
        highlights: &[(usize, usize)],
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
        let highlight_indices = highlights
            .iter()
            // Make the returned highlights usable and ensure they are all valid
            .filter_map(|(first_index, second_index)| {
                Some(*list_directory.get(*first_index)?.get(*second_index)?)
            })
            .collect::<Vec<usize>>();

        for number in list.join(&[min_value as usize][..]).into_iter().enumerate() {
            let bar_height = ((number.1 as f32 - min_value) / max_value) * max_height;
            let color = if ctx.input(|i| i.time) - self.sorted_animation_time < SORTED_ANIMATION_LENGTH {
                epaint::Color32::LIGHT_GREEN
            } else if highlight_indices.contains(&number.0) && !self.sorted {
                epaint::Color32::LIGHT_RED
            } else {
                epaint::Color32::WHITE
            };

            let bar = epaint::Shape::rect_filled(
                egui::Rect::from_two_pos(
                    epaint::pos2(bar_width.mul_add(number.0 as f32, rect.left()), rect.bottom() - base_height - base_spacing),
                    epaint::pos2(bar_width.mul_add((number.0 + 1) as f32, rect.left()), rect.bottom() - base_height - base_spacing - bar_height),
                ),
                epaint::CornerRadius::ZERO,
                color,
            );
            bars.push(bar);
        }

        bars
    }
}

impl Visualizer for BarGraph {
    fn draw_graph<'a>(&'a mut self, list: &'a [Vec<usize>], highlights: &'a [(usize, usize)]) -> Box<dyn FnMut(&mut egui::Ui) + 'a> {
        Box::new(move |ui| {
            ui.ctx().request_repaint(); // Not sure if this is needed or not.

            // x and y of the desired size of the frame is 1 times the width and 0.35
            // times the width respectively.
            let desired_size = ui.available_width() * egui::vec2(1.0, 0.35);
            let (_, rect) = ui.allocate_space(desired_size);

            let bars = self.make_bars(rect, list, highlights, 10.0, 10.0, ui.ctx());

            ui.painter().extend(bars);
        })
    }

    fn sorted(&mut self, ctx: &egui::Context) {
        if !self.sorted {
            self.sorted = true;
            self.sorted_animation_time = ctx.input(|i| i.time);
        }
    }

    fn unsorted(&mut self) {
        self.sorted = false;
    }
}

impl Default for BarGraph {
    fn default() -> Self {
        Self {
            sorted: true,
            sorted_animation_time: 0.0,
        }
    }
}
