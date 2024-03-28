use nannou::prelude::*;

mod view;

mod model;
use model::Model;

pub const REGULAR_FONT: &[u8] = include_bytes!("../fonts/Times New Roman.ttf");
pub const ITALIC_FONT: &[u8] =
    include_bytes!("../fonts/Times New Roman Italic.ttf");

fn main() {
    nannou::app(Model::new).update(update).run();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update_theta();
    model.compute_trig_values();
}
