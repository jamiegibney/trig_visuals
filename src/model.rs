#![allow(clippy::suboptimal_flops)]
use crate::{ITALIC_FONT, REGULAR_FONT};
use nannou::{
    prelude::*,
    text::{
        Font,
        Justify::{self, Center, Left},
        Layout,
    },
};
use std::{
    f32::{
        consts::{PI, TAU},
        MAX as INF,
    },
    marker::PhantomData as PD,
};
use FontType::{Italic, Regular};

const DEFAULT_RATE: f32 = 0.25;
const RATE_INCREMENT: f32 = 0.08;
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
const CSC_COLOR: Rgb = Rgb { red: 0.0, green: 1.0, blue: 1.0, standard: PD };

#[derive(Clone, Copy)]
enum FontType {
    Regular,
    Italic,
}

impl FontType {
    pub const fn into_font(self) -> &'static [u8] {
        match self {
            Regular => REGULAR_FONT,
            Italic => ITALIC_FONT,
        }
    }
}

fn font_layout(
    font_size: u32,
    font_type: FontType,
    justify: Justify,
) -> Layout {
    Layout {
        justify,
        font_size,
        font: Font::from_bytes(font_type.into_font()).ok(),
        line_spacing: 3.0,
        ..Default::default()
    }
}

#[derive(Clone, Copy, Default, Debug)]
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

impl TrigValues {
    pub fn clamp_inf(&mut self) {
        let set_inf = |value: &mut f32| {
            if value.is_infinite() {
                *value = if value.is_sign_positive() { INF } else { -INF };
            }
        };

        // these can be infinite, so this prevents some drawing errors
        set_inf(&mut self.tan);
        set_inf(&mut self.cot);
        set_inf(&mut self.sec);
        set_inf(&mut self.csc);
    }
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

// --- *** --- //

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct Model {
    theta: f32,
    rate: f32,

    trig_values: TrigValues,
    trig_values_scaled: TrigValues,

    is_running: bool,
    draw_labels: bool,
    draw_values: bool,
    draw_theta: bool,
}

impl Model {
    pub fn new(app: &App) -> Self {
        _ = app
            .new_window()
            .size(800, 800)
            .view(view)
            .key_pressed(key_pressed)
            .build()
            .unwrap();

        Self {
            theta: 0.0,
            rate: DEFAULT_RATE,

            trig_values: TrigValues::default(),
            trig_values_scaled: TrigValues::default(),

            is_running: true,
            draw_labels: true,
            draw_values: true,
            draw_theta: true,
        }
    }

    // Update methods

    pub fn update(&mut self, delta_time: f32) {
        self.update_theta(delta_time);
        self.compute_trig_values();
    }

    fn update_theta(&mut self, delta_time: f32) {
        if !self.is_running {
            return;
        }

        self.theta += self.rate * delta_time;

        if self.theta >= TAU {
            self.theta -= TAU;
        }
    }

    fn compute_trig_values(&mut self) {
        let TrigValues { sin, cos, tan, cot, sec, csc } = &mut self.trig_values;

        *sin = self.theta.sin();
        *cos = self.theta.cos();
        *tan = self.theta.tan();
        *cot = tan.recip();
        *sec = cos.recip();
        *csc = sin.recip();

        self.trig_values_scaled = self.trig_values * UNIT_RADIUS;

        // some values can be inf, so this is needed to prevent a geometry error!
        self.trig_values.clamp_inf();
        self.trig_values_scaled.clamp_inf();
    }

    // Setting methods

    pub fn increment_rate(&mut self) {
        self.rate += RATE_INCREMENT;
    }

    pub fn decrement_rate(&mut self) {
        self.rate = f32::max(0.0, self.rate - RATE_INCREMENT);
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

    pub fn toggle_theta(&mut self) {
        self.draw_theta = !self.draw_theta;
    }

    pub fn reset_theta(&mut self) {
        self.theta = 0.0;
    }

    pub fn reset_rate(&mut self) {
        self.rate = DEFAULT_RATE;
    }

    // Draw methods

    pub fn draw_bg_lines(&self, draw: &Draw) {
        draw.line()
            .stroke_weight(STROKE_WEIGHT - 1.0)
            .start(vec2(-1000.0, 0.0))
            .end(vec2(1000.0, 0.0))
            .color(GREY);

        draw.line()
            .stroke_weight(STROKE_WEIGHT - 1.0)
            .start(vec2(0.0, 1000.0))
            .end(vec2(0.0, -1000.0))
            .color(GREY);

        self.draw_unit_line(draw);
    }

    pub fn draw_unit_circle(&self, draw: &Draw) {
        draw.ellipse()
            .no_fill()
            .radius(UNIT_RADIUS)
            .stroke_weight(STROKE_WEIGHT - 0.3)
            .stroke(GREY)
            .xy(Vec2::ZERO);

        if self.draw_theta {
            self.draw_theta_circle(draw);
        }
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

        // some values can be infinite (clamped to f32::MAX), so this
        // handles very large values in such a case
        let fmt_large = |val: f32| {
            if val > 1.0e9 {
                String::from("inf")
            }
            else if val < -1.0e9 {
                String::from("-inf")
            }
            else {
                format!("{val:.2}")
            }
        };

        // sin
        draw.text(&format!("{} = {:.2}", SIN_LABEL, self.trig_values.sin))
            .xy(vec2(430.0, 150.0))
            .layout(&font_layout(18, Italic, Left))
            .color(SIN_COLOR);
        // cos
        draw.text(&format!("{} = {:.2}", COS_LABEL, self.trig_values.cos))
            .xy(vec2(430.0, 100.0))
            .layout(&font_layout(18, Italic, Left))
            .color(COS_COLOR);
        // tan
        draw.text(&format!(
            "{} = {}",
            TAN_LABEL,
            fmt_large(self.trig_values.tan)
        ))
        .xy(vec2(430.0, 50.0))
        .layout(&font_layout(18, Italic, Left))
        .color(TAN_COLOR);
        // cot
        draw.text(&format!(
            "{} = {}",
            COT_LABEL,
            fmt_large(self.trig_values.cot)
        ))
        .xy(vec2(430.0, -50.0))
        .layout(&font_layout(18, Italic, Left))
        .color(COT_COLOR);
        // sec
        draw.text(&format!(
            "{} = {}",
            SEC_LABEL,
            fmt_large(self.trig_values.sec)
        ))
        .xy(vec2(430.0, -100.0))
        .layout(&font_layout(18, Italic, Left))
        .color(SEC_COLOR);
        // csc
        draw.text(&format!(
            "{} = {}",
            CSC_LABEL,
            fmt_large(self.trig_values.csc)
        ))
        .xy(vec2(430.0, -150.0))
        .layout(&font_layout(18, Italic, Left))
        .color(CSC_COLOR);

        // theta
        if self.draw_theta {
            draw.text(&format!(
                "θ = {:.2} ({:.0}º)",
                self.theta,
                self.theta.to_degrees()
            ))
            .xy(vec2(430.0, 200.0))
            .layout(&font_layout(18, Italic, Left))
            .color(WHITE);
        }

        let rate = if self.is_running { self.rate } else { 0.0 };

        // rate
        draw.text(&format!(
            // TODO come on...
            "rate = {:.2} rad/s\n           ({:.0} deg/s)",
            rate,
            rate.to_degrees()
        ))
        .xy(vec2(430.0, -200.0))
        .layout(&font_layout(18, Italic, Left))
        .color(GREY);
    }

    // Private draw methods

    fn draw_theta_circle(&self, draw: &Draw) {
        const THETA_POINTS: usize = 128;

        let progress = self.theta / TAU;
        let num_points = (THETA_POINTS as f32 * progress).ceil() as usize;

        if self.draw_labels {
            let (y, x) = (self.theta * 0.5).sin_cos();
            draw.text("θ")
                .xy(vec2(x * UNIT_RADIUS * 0.93, y * UNIT_RADIUS * 0.93))
                .layout(&font_layout(13, Regular, Center))
                .color(WHITE);
        }

        // needed to prevent nan error
        if num_points == 0 {
            return;
        }

        draw.path()
            .stroke()
            .weight(STROKE_WEIGHT)
            .points_colored((0..=num_points).map(|i| {
                let t = i as f32 / num_points as f32;
                let (y, x) = (self.theta * t).sin_cos();

                (vec2(x * UNIT_RADIUS, y * UNIT_RADIUS), WHITE)
            }))
            .finish();
    }

    fn draw_cos_line(&self, draw: &Draw) {
        draw.line()
            .start(Vec2::ZERO)
            .end(vec2(self.trig_values_scaled.cos, 0.0))
            .color(COS_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(COS_LABEL)
                .xy(vec2(self.trig_values_scaled.cos * 0.5, 15.0))
                .layout(&font_layout(13, Regular, Center))
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
                    self.trig_values_scaled.cos + 22.0,
                    self.trig_values_scaled.sin * 0.5,
                ))
                .layout(&font_layout(13, Regular, Center))
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
                .xy(vec2(UNIT_RADIUS + 23.0, self.trig_values_scaled.tan * 0.5))
                .layout(&font_layout(13, Regular, Center))
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
            let x_dir = if self.theta >= PI { -1.0 } else { 1.0 };
            draw.text(COT_LABEL)
                .xy(vec2(
                    self.trig_values_scaled.cos * 0.5
                        + (x_dir * self.trig_values.cos * 20.0),
                    (self.trig_values_scaled.sin + self.trig_values_scaled.csc)
                        * 0.5
                        + 12.0
                        + (self.trig_values.sin.abs() * 8.0),
                ))
                .layout(&font_layout(13, Regular, Center))
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
                    UNIT_RADIUS * 0.5 - (self.trig_values.tan * 7.0),
                    self.trig_values_scaled.tan * 0.5 + 18.0,
                ))
                .layout(&font_layout(13, Regular, Center))
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
                .xy(vec2(-25.0, self.trig_values_scaled.csc * 0.5))
                .layout(&font_layout(13, Regular, Center))
                .color(CSC_COLOR);
        }
    }

    fn draw_unit_line(&self, draw: &Draw) {
        draw.line()
            .start(Vec2::ZERO)
            .end(vec2(self.trig_values_scaled.cos, self.trig_values_scaled.sin))
            .color(GREY)
            .stroke_weight(STROKE_WEIGHT - 0.8);

        if self.draw_labels {
            let (y, x) = (self.theta - PI * 0.5).sin_cos();

            draw.text("1.0")
                .xy(vec2(
                    self.trig_values_scaled.cos * 0.5 + 15.0 * x,
                    self.trig_values_scaled.sin * 0.5 + 15.0 * y,
                ))
                .layout(&font_layout(13, Regular, Center))
                .color(LIGHTGREY);
        }
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.toggle_running(),
        Key::L => model.toggle_labels(),
        Key::V => model.toggle_values(),
        Key::T => model.toggle_theta(),
        Key::Up => model.increment_rate(),
        Key::Down => model.decrement_rate(),
        Key::R => model.reset_theta(),
        Key::S => model.reset_rate(),
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = &app.draw().translate(vec3(-120.0, 0.0, 0.0));
    draw.background().color(BLACK);

    model.draw_bg_lines(draw);
    model.draw_unit_circle(draw);
    model.draw_trig_lines(draw);
    model.draw_node(draw);
    model.draw_values(draw);

    draw.to_frame(app, &frame).unwrap();
}
