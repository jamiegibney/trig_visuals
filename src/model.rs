use crate::{view::view, ITALIC_FONT, REGULAR_FONT};
use nannou::{
    prelude::*,
    text::{
        Font,
        Justify::{self, Center, Left},
        Layout,
    },
};
use std::{f32::consts::TAU, marker::PhantomData as PD};
use FontType::{Italic, Regular};

const DEFAULT_RATE: f32 = 0.007;
const STROKE_WEIGHT: f32 = 3.0;
const UNIT_RADIUS: f32 = 200.0;

const SIN_LABEL: &str = "sin θ";
const COS_LABEL: &str = "cos θ";
const TAN_LABEL: &str = "tan θ";
const COT_LABEL: &str = "cot θ";
const SEC_LABEL: &str = "sec θ";
const CSC_LABEL: &str = "csc θ";

const SIN_COLOR: Rgb = Rgb { red: 1.0, green: 0.0, blue: 0.0, standard: PD };
const COS_COLOR: Rgb = Rgb { red: 0.0, green: 1.0, blue: 0.0, standard: PD };
const TAN_COLOR: Rgb = Rgb { red: 1.0, green: 1.0, blue: 0.0, standard: PD };
const COT_COLOR: Rgb = Rgb { red: 1.0, green: 0.5, blue: 0.0, standard: PD };
const SEC_COLOR: Rgb = Rgb { red: 1.0, green: 0.0, blue: 1.0, standard: PD };
const CSC_COLOR: Rgb = Rgb { red: 0.0, green: 0.5, blue: 1.0, standard: PD };

enum FontType {
    Regular,
    Italic,
}

fn label_layout(
    font_size: u32,
    font_type: FontType,
    justify: Justify,
) -> Layout {
    Layout {
        justify,
        font_size,
        font: Some(
            Font::from_bytes(match font_type {
                Regular => REGULAR_FONT,
                Italic => ITALIC_FONT,
            })
            .expect("failed to load font"),
        ),
        ..Default::default()
    }
}

#[derive(Clone, Copy, Default)]
struct TrigValues {
    /// Sine function
    pub sin: f32,
    /// Cosine function
    pub cos: f32,
    /// Tangent function
    pub tan: f32,
    /// Cotangent function
    pub cot: f32,
    /// Secant function
    pub sec: f32,
    /// Cosecant function
    pub csc: f32,
}

impl std::ops::Mul<f32> for TrigValues {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            sin: self.sin * rhs,
            cos: self.cos * rhs,
            tan: self.tan * rhs,
            cot: self.cot * rhs,
            sec: self.sec * rhs,
            csc: self.csc * rhs,
        }
    }
}

pub struct Model {
    _window: window::Id,
    win_rect: Rect,

    theta: f32,
    rate: f32,

    trig_values: TrigValues,
    trig_values_scaled: TrigValues,

    is_running: bool,
    draw_labels: bool,
    draw_values: bool,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let _window = app
            .new_window()
            .size(800, 800)
            .view(view)
            .key_pressed(key_pressed)
            .build()
            .unwrap();

        Self {
            _window,
            theta: 0.0,
            rate: DEFAULT_RATE,
            trig_values: TrigValues::default(),
            trig_values_scaled: TrigValues::default(),
            win_rect: app.main_window().rect(),
            is_running: true,
            draw_labels: true,
            draw_values: true,
        }
    }

    pub fn update_theta(&mut self) {
        if !self.is_running {
            return;
        }

        self.theta += self.rate;

        if self.theta >= TAU {
            self.theta -= TAU;
        }
    }

    pub fn compute_trig_values(&mut self) {
        let TrigValues { sin, cos, tan, cot, sec, csc } = &mut self.trig_values;

        *sin = self.theta.sin();
        *cos = self.theta.cos();
        *tan = self.theta.tan();
        *cot = tan.recip();
        *sec = cos.recip();
        *csc = sin.recip();

        self.trig_values_scaled = self.trig_values * UNIT_RADIUS;
    }

    pub fn increment_rate(&mut self) {
        self.rate += 0.001;
    }

    pub fn decrement_rate(&mut self) {
        self.rate = f32::max(0.0, self.rate - 0.001);
    }

    pub fn toggle_running(&mut self) {
        self.is_running = !self.is_running;
    }

    pub fn toggle_labels(&mut self) {
        self.draw_labels = !self.draw_labels;
    }

    pub fn toggle_values(&mut self) {
        self.draw_values = !self.draw_values;
    }

    pub fn draw_bg_lines(&self, draw: &Draw) {
        let ml = self.win_rect.mid_left();
        let mr = self.win_rect.mid_right();

        draw.line()
            .stroke_weight(STROKE_WEIGHT - 0.3)
            .start(ml)
            .end(vec2(1000.0, 0.0))
            .color(GREY);

        let top = self.win_rect.mid_top();
        let bot = self.win_rect.mid_bottom();

        draw.line()
            .stroke_weight(STROKE_WEIGHT - 0.3)
            .start(top)
            .end(bot)
            .color(GREY);

        self.draw_unit_line(draw);
    }

    pub fn draw_unit_circle(&self, draw: &Draw) {
        draw.ellipse()
            .no_fill()
            .radius(UNIT_RADIUS)
            .stroke_weight(STROKE_WEIGHT - 0.3)
            .stroke(WHITE)
            .xy(Vec2::ZERO);
    }

    pub fn draw_node(&self, draw: &Draw) {
        let pt = Vec2::new(
            self.trig_values.cos * UNIT_RADIUS,
            self.trig_values.sin * UNIT_RADIUS,
        );

        draw.ellipse().radius(8.0).color(WHITE).xy(pt);
    }

    pub fn draw_trig_lines(&self, draw: &Draw) {
        self.draw_sin_line(draw);
        self.draw_cos_line(draw);
        self.draw_tan_line(draw);
        self.draw_cot_line(draw);
        self.draw_sec_line(draw);
        self.draw_csc_line(draw);
    }

    pub fn draw_values(&self, draw: &Draw) {
        if !self.draw_values {
            return;
        }

        // sin
        draw.text(&format!("{} = {:.3}", SIN_LABEL, self.trig_values.sin))
            .xy(vec2(430.0, 150.0))
            .layout(&label_layout(18, Italic, Left))
            .color(SIN_COLOR);
        // cos
        draw.text(&format!("{} = {:.3}", COS_LABEL, self.trig_values.cos))
            .xy(vec2(430.0, 100.0))
            .layout(&label_layout(18, Italic, Left))
            .color(COS_COLOR);
        // tan
        draw.text(&format!("{} = {:.3}", TAN_LABEL, self.trig_values.tan))
            .xy(vec2(430.0, 50.0))
            .layout(&label_layout(18, Italic, Left))
            .color(TAN_COLOR);
        // cot
        draw.text(&format!("{} = {:.3}", COT_LABEL, self.trig_values.cot))
            .xy(vec2(430.0, -50.0))
            .layout(&label_layout(18, Italic, Left))
            .color(COT_COLOR);
        // sec
        draw.text(&format!("{} = {:.3}", SEC_LABEL, self.trig_values.sec))
            .xy(vec2(430.0, -100.0))
            .layout(&label_layout(18, Italic, Left))
            .color(SEC_COLOR);
        // csc
        draw.text(&format!("{} = {:.3}", CSC_LABEL, self.trig_values.csc))
            .xy(vec2(430.0, -150.0))
            .layout(&label_layout(18, Italic, Left))
            .color(CSC_COLOR);
    }

    fn draw_cos_line(&self, draw: &Draw) {
        draw.line()
            .start(Vec2::ZERO)
            .end(vec2(self.trig_values_scaled.cos, 0.0))
            .color(COS_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(COS_LABEL)
                .xy(vec2(self.trig_values_scaled.cos * 0.5, 12.0))
                .layout(&label_layout(13, Regular, Center))
                .color(COS_COLOR);
        }
    }

    fn draw_sin_line(&self, draw: &Draw) {
        draw.line()
            .start(vec2(self.trig_values_scaled.cos, 0.0))
            .end(vec2(self.trig_values_scaled.cos, self.trig_values_scaled.sin))
            .color(SIN_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(SIN_LABEL)
                .xy(vec2(
                    self.trig_values_scaled.cos + 20.0,
                    self.trig_values_scaled.sin * 0.5,
                ))
                .layout(&label_layout(13, Regular, Center))
                .color(SIN_COLOR);
        }
    }

    fn draw_tan_line(&self, draw: &Draw) {
        draw.line()
            .start(vec2(UNIT_RADIUS, 0.0))
            .end(vec2(UNIT_RADIUS, self.trig_values_scaled.tan))
            .color(TAN_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(TAN_LABEL)
                .xy(vec2(UNIT_RADIUS + 20.0, self.trig_values_scaled.tan * 0.5))
                .layout(&label_layout(13, Regular, Center))
                .color(TAN_COLOR);
        }
    }

    fn draw_cot_line(&self, draw: &Draw) {
        draw.line()
            .start(vec2(
                self.trig_values_scaled.cos, self.trig_values_scaled.sin,
            ))
            .end(vec2(0.0, self.trig_values_scaled.csc))
            .color(COT_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(COT_LABEL)
                .xy(vec2(
                    self.trig_values_scaled.cos * 0.5 + 12.0,
                    (self.trig_values_scaled.sin + self.trig_values_scaled.csc)
                        * 0.5
                        + 12.0,
                ))
                .layout(&label_layout(13, Regular, Center))
                .color(COT_COLOR);
        }
    }

    fn draw_sec_line(&self, draw: &Draw) {
        draw.line()
            .start(Vec2::ZERO)
            .end(vec2(UNIT_RADIUS, self.trig_values_scaled.tan))
            .color(SEC_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(SEC_LABEL)
                .xy(vec2(
                    UNIT_RADIUS * 0.5 + 12.0,
                    self.trig_values_scaled.tan * 0.5 + 12.0,
                ))
                .layout(&label_layout(13, Regular, Center))
                .color(SEC_COLOR);
        }
    }

    fn draw_csc_line(&self, draw: &Draw) {
        draw.line()
            .start(Vec2::ZERO)
            .end(vec2(0.0, self.trig_values_scaled.csc))
            .color(CSC_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(CSC_LABEL)
                .xy(vec2(-20.0, self.trig_values_scaled.csc * 0.5))
                .layout(&label_layout(13, Regular, Center))
                .color(CSC_COLOR);
        }
    }

    fn draw_unit_line(&self, draw: &Draw) {
        draw.line()
            .start(Vec2::ZERO)
            .end(vec2(self.trig_values_scaled.cos, self.trig_values_scaled.sin))
            .color(GREY)
            .stroke_weight(STROKE_WEIGHT - 0.8);

        // if self.draw_labels {
        //     draw.text("1.0")
        //         .xy(vec2(
        //             self.trig_values_scaled.cos * 0.5 + 10.0,
        //             self.trig_values_scaled.sin * 0.5 - 10.0,
        //         ))
        //         .layout(&label_layout())
        //         .color(WHITE);
        // }
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.toggle_running(),
        Key::L => model.toggle_labels(),
        Key::V => model.toggle_values(),
        Key::Up => model.increment_rate(),
        Key::Down => model.decrement_rate(),
        _ => {}
    }
}
