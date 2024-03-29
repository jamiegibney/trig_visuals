#![allow(clippy::suboptimal_flops)]
use crate::{consts::*, labels::*, ITALIC_FONT, REGULAR_FONT};
use nannou::{
    prelude::*,
    text::{
        Font,
        Justify::{self, Center, Left},
        Layout,
    },
};
use std::f32::{
    consts::{PI, TAU},
    MAX as INF,
};
use FontStyle::{Italic, Regular};

#[derive(Clone, Copy)]
enum FontStyle {
    Regular,
    Italic,
}

impl FontStyle {
    pub const fn font_data(self) -> &'static [u8] {
        match self {
            Regular => REGULAR_FONT,
            Italic => ITALIC_FONT,
        }
    }
}

fn font_layout(
    font_size: u32,
    font_style: FontStyle,
    justify: Justify,
) -> Layout {
    Layout {
        justify,
        font_size,
        font: Font::from_bytes(font_style.font_data()).ok(),
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
        self.tan = self.tan.clamp(-INF, INF);
        self.cot = self.cot.clamp(-INF, INF);
        self.sec = self.sec.clamp(-INF, INF);
        self.csc = self.csc.clamp(-INF, INF);
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

    label_fading: Labels,
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

            label_fading: Labels::new(),
        }
    }

    // Update methods

    pub fn update(&mut self, delta_time: f32) {
        self.update_theta(delta_time);
        self.compute_trig_values();
        self.update_label_positions();
        self.label_fading.update(delta_time);
    }

    fn update_label_positions(&mut self) {
        // sin
        self.label_fading.sin_pos = vec2(
            self.trig_values_scaled.cos + 22.0,
            self.trig_values_scaled.sin * 0.5,
        );

        // cos
        self.label_fading.cos_pos = vec2(self.trig_values_scaled.cos * 0.5, 15.0);

        // tan
        self.label_fading.tan_pos =
            vec2(UNIT_RADIUS + 23.0, self.trig_values_scaled.tan * 0.5);

        // cot
        let cot_x_dir = if self.theta >= PI { -1.0 } else { 1.0 };
        self.label_fading.cot_pos = vec2(
            self.trig_values_scaled.cos * 0.5
                + (cot_x_dir * self.trig_values.cos * 20.0),
            (self.trig_values_scaled.sin + self.trig_values_scaled.csc) * 0.5
                + 12.0
                + (self.trig_values.sin.abs() * 8.0),
        );

        // sec
        self.label_fading.sec_pos = vec2(
            UNIT_RADIUS * 0.5 - (self.trig_values.tan * 7.0),
            self.trig_values_scaled.tan * 0.5 + 18.0,
        );

        // csc
        self.label_fading.csc_pos = vec2(-25.0, self.trig_values_scaled.csc * 0.5);

        // theta
        let (th_y, th_x) = (self.theta * 0.5).sin_cos();
        self.label_fading.theta_pos =
            vec2(th_x * UNIT_RADIUS * 0.93, th_y * UNIT_RADIUS * 0.93);

        // unit
        let (un_y, un_x) = (self.theta - PI * 0.5).sin_cos();
        self.label_fading.unit_pos = vec2(
            self.trig_values_scaled.cos * 0.5 + 15.0 * un_x,
            self.trig_values_scaled.sin * 0.5 + 15.0 * un_y,
        );
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
            draw.text("θ")
                .xy(self.label_fading.theta_pos)
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(Rgba::new(
                    1.0, 1.0, 1.0, self.label_fading.theta_label_opacity,
                ));
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
                .xy(self.label_fading.cos_pos)
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(Rgba::new(
                    COS_COLOR.red, COS_COLOR.green, COS_COLOR.blue,
                    self.label_fading.cos_label_opacity,
                ));
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
                .xy(self.label_fading.sin_pos)
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(Rgba::new(
                    SIN_COLOR.red, SIN_COLOR.green, SIN_COLOR.blue,
                    self.label_fading.sin_label_opacity,
                ));
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
                .xy(self.label_fading.tan_pos)
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
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
                .xy(self.label_fading.cot_pos)
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
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
                .xy(self.label_fading.sec_pos)
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
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
                .xy(self.label_fading.csc_pos)
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
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
            draw.text("1.0")
                .xy(self.label_fading.unit_pos)
                .layout(&font_layout(13, Regular, Center))
                .color(Rgba::new(
                    0.8, 0.8, 0.8, self.label_fading.unit_label_opacity,
                ));
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

#[allow(clippy::needless_pass_by_value)]
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
