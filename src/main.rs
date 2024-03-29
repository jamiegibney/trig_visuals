use nannou::prelude::*;

mod consts;
mod labels;
mod model;
use model::Model;

pub const REGULAR_FONT: &[u8] = include_bytes!("../fonts/Times New Roman.ttf");
pub const ITALIC_FONT: &[u8] =
    include_bytes!("../fonts/Times New Roman Italic.ttf");

fn main() {
    nannou::app(Model::new).update(update).run();
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.update(
        update.since_last.as_secs_f32(),
        app.mouse.position(),
        app.mouse.buttons.left().is_down(),
    );
}
