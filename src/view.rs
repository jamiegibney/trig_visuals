use super::*;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = &app.draw();
    draw.background().color(BLACK);

    model.draw_bg_lines(draw);
    model.draw_unit_circle(draw);
    model.draw_trig_lines(draw);
    model.draw_node(draw);

    draw.to_frame(app, &frame).unwrap();
}
