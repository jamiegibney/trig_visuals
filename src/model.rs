#![allow(clippy::suboptimal_flops)]
use crate::{consts::*, labels::*, ITALIC_FONT, REGULAR_FONT};
use nannou::{
    prelude::*,
    text::{
        Align::End,
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
        // TODO: this will fix the spacing issue with the "rate" value text, but
        // shifts everything upwards
        // y_align: End,
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

#[derive(Debug, Default, Clone, Copy)]
enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    #[allow(unreachable_patterns)]
    pub fn toggle_light_dark(&mut self) {
        match self {
            Self::Dark => *self = Self::Light,
            Self::Light => *self = Self::Dark,
            _ => {}
        }
    }

    pub const fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
struct Visible {
    sin: bool,
    cos: bool,
    tan: bool,
    cot: bool,
    sec: bool,
    csc: bool,
}

impl Default for Visible {
    fn default() -> Self {
        Self {
            sin: true,
            cos: true,
            tan: true,
            cot: true,
            sec: true,
            csc: true,
        }
    }
}

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

    theme: Theme,

    selected_label: Option<Label>,

    radius: f32,

    mouse_state: bool,
    value_rects: Vec<Rect>,
    visible: Visible,

    labels: Labels,
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

            theme: Theme::default(),

            selected_label: None,

            radius: UNIT_RADIUS,

            mouse_state: false,
            value_rects: (0..6)
                .map(|i| {
                    let size = vec2(140.0, 50.0);
                    let i = if i < 3 { i as f32 } else { i as f32 + 1.0 };
                    Rect::from_xy_wh(vec2(390.0, i * 50.0 - 150.0), size)
                })
                .collect(),
            visible: Visible::default(),

            labels: Labels::new(),
        }
    }

    // Update methods

    pub fn update(
        &mut self,
        delta_time: f32,
        mouse_pos: Vec2,
        mouse_down: bool,
    ) {
        self.update_theta(delta_time);
        self.compute_trig_values();
        self.update_label_positions();
        self.update_value_rects(mouse_pos, mouse_down);
        self.labels.update(delta_time);
    }

    fn update_value_rects(&mut self, mut mouse_pos: Vec2, mouse_down: bool) {
        // need to accommodate for translation
        mouse_pos.x += 120.0;
        if mouse_down && self.mouse_state {
            return;
        }
        else if !mouse_down {
            if self.mouse_state {
                self.mouse_state = false;
            }
            return;
        }

        let idx = self
            .value_rects
            .iter()
            .enumerate()
            .find_map(|(i, &rect)| rect.contains(mouse_pos).then_some(i));

        if let Some(i) = idx {
            match i {
                5 => self.visible.sin = !self.visible.sin,
                4 => self.visible.cos = !self.visible.cos,
                3 => self.visible.tan = !self.visible.tan,
                2 => self.visible.cot = !self.visible.cot,
                1 => self.visible.sec = !self.visible.sec,
                0 => self.visible.csc = !self.visible.csc,
                _ => {}
            }
        }

        self.mouse_state = true;
    }

    fn update_label_positions(&mut self) {
        if self.visible.sin {
            // sin
            self.labels.update_position(
                Label::Sin,
                vec2(
                    self.trig_values_scaled.cos + 22.0,
                    self.trig_values_scaled.sin * 0.5,
                ),
            );
        }
        else {
            self.labels
                .update_position(Label::Sin, vec2(1000.0, 1000.0));
        }

        // cos
        if self.visible.cos {
            self.labels.update_position(
                Label::Cos,
                vec2(self.trig_values_scaled.cos * 0.5, 15.0),
            );
        }
        else {
            self.labels
                .update_position(Label::Cos, vec2(1000.0, 1000.0));
        }

        // tan
        if self.visible.tan {
            self.labels.update_position(
                Label::Tan,
                vec2(self.radius + 23.0, self.trig_values_scaled.tan * 0.5),
            );
        }
        else {
            self.labels
                .update_position(Label::Tan, vec2(1000.0, 1000.0));
        }

        // cot
        if self.visible.cot {
            let cot_x_dir = if self.theta >= PI { -1.0 } else { 1.0 };
            self.labels.update_position(
                Label::Cot,
                vec2(
                    self.trig_values_scaled.cos * 0.5
                        + (cot_x_dir * self.trig_values.cos * 20.0),
                    (self.trig_values_scaled.sin + self.trig_values_scaled.csc)
                        * 0.5
                        + 12.0
                        + (self.trig_values.sin.abs() * 8.0),
                ),
            );
        }
        else {
            self.labels
                .update_position(Label::Cot, vec2(1000.0, 1000.0));
        }

        // sec
        if self.visible.sec {
            let sec_offset =
                self.trig_values.tan.signum() * self.trig_values.sin.abs();
            self.labels.update_position(
                Label::Sec,
                vec2(
                    self.radius * 0.5
                        - (self.trig_values.tan * 5.0)
                        - sec_offset * 10.0,
                    self.trig_values_scaled.tan * 0.5 + 18.0,
                ),
            );
        }
        else {
            self.labels
                .update_position(Label::Sec, vec2(1000.0, 1000.0));
        }

        // csc
        if self.visible.csc {
            self.labels.update_position(
                Label::Csc,
                vec2(-25.0, self.trig_values_scaled.csc * 0.5),
            );
        }
        else {
            self.labels
                .update_position(Label::Csc, vec2(1000.0, 1000.0));
        }

        // theta
        let (th_y, th_x) = (self.theta * 0.5).sin_cos();
        self.labels.update_position(
            Label::Theta,
            vec2(th_x * self.radius * 0.93, th_y * self.radius * 0.93),
        );

        // unit
        let (un_y, un_x) = (self.theta - PI * 0.5).sin_cos();
        self.labels.update_position(
            Label::Unit,
            vec2(
                self.trig_values_scaled.cos * 0.5 + 15.0 * un_x,
                self.trig_values_scaled.sin * 0.5 + 15.0 * un_y,
            ),
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

        self.trig_values_scaled = self.trig_values * self.radius;

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

    pub fn increase_scale(&mut self) {
        self.radius += 10.0;
    }

    pub fn decrease_scale(&mut self) {
        self.radius -= 10.0;
    }

    pub fn reset_scale(&mut self) {
        self.radius = UNIT_RADIUS;
    }

    pub fn toggle_theme(&mut self) {
        self.theme.toggle_light_dark();
    }

    pub fn bg_color(&self) -> Rgb {
        if self.theme.is_dark() {
            Rgb::new(0.0, 0.0, 0.0)
        }
        else {
            Rgb::new(0.9, 0.9, 0.9)
        }
    }

    // Draw methods

    pub fn draw_bg_lines(&self, draw: &Draw) {
        let color = if self.theme.is_dark() { 1.0 } else { 0.0 };

        draw.line()
            .stroke_weight(STROKE_WEIGHT - 1.0)
            .start(vec2(-1000.0, 0.0))
            .end(vec2(1000.0, 0.0))
            .color(Rgba::new(color, color, color, 0.1));

        draw.line()
            .stroke_weight(STROKE_WEIGHT - 1.0)
            .start(vec2(0.0, 1000.0))
            .end(vec2(0.0, -1000.0))
            .color(Rgba::new(color, color, color, 0.1));
    }

    pub fn draw_unit_circle(&self, draw: &Draw) {
        let color = if self.theme.is_dark() { 1.0 } else { 0.0 };

        draw.ellipse()
            .no_fill()
            .radius(self.radius)
            .stroke_weight(STROKE_WEIGHT - 0.3)
            .stroke(Rgba::new(color, color, color, 0.3))
            .xy(Vec2::ZERO);

        if self.draw_theta {
            self.draw_theta_circle(draw);
        }
    }

    pub fn draw_node(&self, draw: &Draw) {
        let pt = Vec2::new(
            self.trig_values.cos * self.radius,
            self.trig_values.sin * self.radius,
        );

        let color = if self.theme.is_dark() { 1.0 } else { 0.0 };

        draw.ellipse()
            .radius(8.0)
            .color(Rgba::new(color, color, color, 0.75))
            .xy(pt);
    }

    #[rustfmt::skip]
    pub fn draw_trig_lines(&self, draw: &Draw) {
        if self.visible.sin { self.draw_sin_line(draw); }
        if self.visible.cos { self.draw_cos_line(draw); }
        if self.visible.tan { self.draw_tan_line(draw); }
        if self.visible.cot { self.draw_cot_line(draw); }
        if self.visible.sec { self.draw_sec_line(draw); }
        if self.visible.csc { self.draw_csc_line(draw); }

        self.draw_unit_line(draw);
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
            .color(self.label_color(Label::Sin));
        // cos
        draw.text(&format!("{} = {:.2}", COS_LABEL, self.trig_values.cos))
            .xy(vec2(430.0, 100.0))
            .layout(&font_layout(18, Italic, Left))
            .color(self.label_color(Label::Cos));
        // tan
        draw.text(&format!(
            "{} = {}",
            TAN_LABEL,
            fmt_large(self.trig_values.tan)
        ))
        .xy(vec2(430.0, 50.0))
        .layout(&font_layout(18, Italic, Left))
        .color(self.label_color(Label::Tan));
        // cot
        draw.text(&format!(
            "{} = {}",
            COT_LABEL,
            fmt_large(self.trig_values.cot)
        ))
        .xy(vec2(430.0, -50.0))
        .layout(&font_layout(18, Italic, Left))
        .color(self.label_color(Label::Cot));
        // sec
        draw.text(&format!(
            "{} = {}",
            SEC_LABEL,
            fmt_large(self.trig_values.sec)
        ))
        .xy(vec2(430.0, -100.0))
        .layout(&font_layout(18, Italic, Left))
        .color(self.label_color(Label::Sec));
        // csc
        draw.text(&format!(
            "{} = {}",
            CSC_LABEL,
            fmt_large(self.trig_values.csc)
        ))
        .xy(vec2(430.0, -150.0))
        .layout(&font_layout(18, Italic, Left))
        .color(self.label_color(Label::Csc));

        // theta
        if self.draw_theta {
            draw.text(&format!(
                "θ = {:.2} ({:.0}º)",
                self.theta,
                self.theta.to_degrees()
            ))
            .xy(vec2(430.0, 200.0))
            .layout(&font_layout(18, Italic, Left))
            .color(if self.theme.is_dark() {
                WHITE
            }
            else {
                BLACK
            });
        }

        // rate
        let rate = if self.is_running { self.rate } else { 0.0 };
        let rate_color = if self.theme.is_dark() { 0.6 } else { 0.4 };

        draw.text(&format!(
            // TODO come on...
            "rate = {:.2} rad/s\n           ({:.0} deg/s)",
            rate,
            rate.to_degrees()
        ))
        .xy(vec2(430.0, -210.0))
        .layout(&font_layout(18, Italic, Left))
        .color(Rgb::new(rate_color, rate_color, rate_color));
    }

    fn label_color(&self, label: Label) -> Rgba {
        let dimmed = 0.2;
        match label {
            Label::Sin => Rgba::new(
                SIN_COLOR.red,
                SIN_COLOR.green,
                SIN_COLOR.blue,
                if self.visible.sin { 1.0 } else { dimmed },
            ),
            Label::Cos => Rgba::new(
                COS_COLOR.red,
                COS_COLOR.green,
                COS_COLOR.blue,
                if self.visible.cos { 1.0 } else { dimmed },
            ),
            Label::Tan => Rgba::new(
                TAN_COLOR.red,
                TAN_COLOR.green,
                TAN_COLOR.blue,
                if self.visible.tan { 1.0 } else { dimmed },
            ),
            Label::Cot => Rgba::new(
                COT_COLOR.red,
                COT_COLOR.green,
                COT_COLOR.blue,
                if self.visible.cot { 1.0 } else { dimmed },
            ),
            Label::Sec => Rgba::new(
                SEC_COLOR.red,
                SEC_COLOR.green,
                SEC_COLOR.blue,
                if self.visible.sec { 1.0 } else { dimmed },
            ),
            Label::Csc => Rgba::new(
                CSC_COLOR.red,
                CSC_COLOR.green,
                CSC_COLOR.blue,
                if self.visible.csc { 1.0 } else { dimmed },
            ),
            Label::Theta => Rgba::new(1.0, 1.0, 1.0, 1.0),
            Label::Unit => Rgba::new(0.5, 0.5, 0.5, 1.0),
        }
    }

    // Private draw methods

    fn draw_theta_circle(&self, draw: &Draw) {
        const THETA_POINTS: usize = 128;

        let theta_color = if self.theme.is_dark() { 1.0 } else { 0.0 };

        if self.draw_labels {
            draw.text("θ")
                .xy(self.labels.get_position(Label::Theta))
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(Rgba::new(
                    theta_color,
                    theta_color,
                    theta_color,
                    self.labels.get_opacity(Label::Theta),
                ));
        }

        let progress = self.theta / TAU;
        let num_points = (THETA_POINTS as f32 * progress).ceil() as usize;

        // needed to prevent nan error
        if num_points == 0 {
            return;
        }

        draw.polyline()
            .weight(STROKE_WEIGHT)
            .points_colored((0..=num_points).map(|i| {
                let t = i as f32 / num_points as f32;
                let (y, x) = (self.theta * t).sin_cos();

                (
                    vec2(x * self.radius, y * self.radius),
                    Rgb::new(theta_color, theta_color, theta_color),
                )
            }))
            .finish();
    }

    fn draw_sin_line(&self, draw: &Draw) {
        draw.line()
            .start(vec2(self.trig_values_scaled.cos, 0.0))
            .end(vec2(self.trig_values_scaled.cos, self.trig_values_scaled.sin))
            .color(SIN_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(SIN_LABEL)
                .xy(self.labels.get_position(Label::Sin))
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(Rgba::new(
                    SIN_COLOR.red,
                    SIN_COLOR.green,
                    SIN_COLOR.blue,
                    self.labels.get_opacity(Label::Sin),
                ));
        }
    }

    fn draw_cos_line(&self, draw: &Draw) {
        draw.line()
            .start(Vec2::ZERO)
            .end(vec2(self.trig_values_scaled.cos, 0.0))
            .color(COS_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(COS_LABEL)
                .xy(self.labels.get_position(Label::Cos))
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(Rgba::new(
                    COS_COLOR.red,
                    COS_COLOR.green,
                    COS_COLOR.blue,
                    self.labels.get_opacity(Label::Cos),
                ));
        }
    }

    fn draw_tan_line(&self, draw: &Draw) {
        draw.line()
            .start(vec2(self.radius, 0.0))
            .end(vec2(self.radius, self.trig_values_scaled.tan))
            .color(TAN_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(TAN_LABEL)
                .xy(self.labels.get_position(Label::Tan))
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
                .xy(self.labels.get_position(Label::Cot))
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(COT_COLOR);
        }
    }

    fn draw_sec_line(&self, draw: &Draw) {
        draw.line()
            .start(Vec2::ZERO)
            .end(vec2(self.radius, self.trig_values_scaled.tan))
            .color(SEC_COLOR)
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            draw.text(SEC_LABEL)
                .xy(self.labels.get_position(Label::Sec))
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(Rgba::new(
                    SEC_COLOR.red,
                    SEC_COLOR.green,
                    SEC_COLOR.blue,
                    self.labels.get_opacity(Label::Sec),
                ));
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
                .xy(self.labels.get_position(Label::Csc))
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(CSC_COLOR);
        }
    }

    fn draw_unit_line(&self, draw: &Draw) {
        draw.line()
            .start(Vec2::ZERO)
            .end(vec2(self.trig_values_scaled.cos, self.trig_values_scaled.sin))
            .color(Rgba::new(1.0, 1.0, 1.0, 0.2))
            .stroke_weight(STROKE_WEIGHT);

        if self.draw_labels {
            let unit_color = if self.theme.is_dark() { 0.8 } else { 0.2 };

            draw.text("1")
                .xy(self.labels.get_position(Label::Unit))
                .layout(&font_layout(LABEL_FONT_SIZE, Regular, Center))
                .color(Rgba::new(
                    unit_color,
                    unit_color,
                    unit_color,
                    self.labels.get_opacity(Label::Unit),
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
        // Key::H => model.toggle_theme(),
        Key::Up => model.increment_rate(),
        Key::Down => model.decrement_rate(),
        Key::R => model.reset_theta(),
        Key::S => model.reset_rate(),
        Key::Equals => model.increase_scale(),
        Key::Minus => model.decrease_scale(),
        Key::Key0 => model.reset_scale(),
        _ => {}
    }
}

#[allow(clippy::needless_pass_by_value)]
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = &app.draw().translate(vec3(-120.0, 0.0, 0.0));
    draw.background().color(model.bg_color());

    model.draw_bg_lines(draw);
    model.draw_unit_circle(draw);
    model.draw_trig_lines(draw);
    model.draw_node(draw);
    model.draw_values(draw);

    draw.to_frame(app, &frame).unwrap();
}
