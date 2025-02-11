use eframe::egui::{Context, Ui};

pub mod bar_graph;

pub trait Visualizer {
    fn draw_graph<'a>(
        &'a mut self,
        list: &'a [Vec<usize>],
        highlights: &'a [(usize, usize)],
    ) -> Box<dyn FnMut(&mut Ui) + 'a>;

    /// Triggers to indicate that the list is now sorted for any verification
    /// effects the `Visualizer` implements.
    fn sorted(&mut self, ctx: &Context);

    fn unsorted(&mut self);
}
