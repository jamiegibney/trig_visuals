use nannou::prelude::*;

mod view;

mod model;
use model::Model;

fn main() {
    nannou::app(Model::new).update(update).run();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update_theta();
    model.compute_trig_values();
}
