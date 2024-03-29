#![allow(unused)]
use super::*;
use crate::consts::*;
use std::collections::HashMap;

fn label_bounds() -> Vec2 {
    vec2(30.0, 25.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Label {
    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Csc,
    Theta,
    Unit,
}

impl Label {
    pub const fn should_fade(self, other: Self) -> bool {
        match self {
            Self::Sin => matches!(other, Self::Tan | Self::Csc),
            Self::Cos => matches!(other, Self::Sec),
            Self::Theta => matches!(other, Self::Sin),
            Self::Unit => matches!(other, Self::Cos | Self::Sin | Self::Csc),
            Self::Tan | Self::Cot | Self::Sec | Self::Csc => false,
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug)]
pub struct Labels {
    label_map: HashMap<Label, Rect>,

    pub sin_pos: Vec2,
    pub cos_pos: Vec2,
    pub tan_pos: Vec2,
    pub cot_pos: Vec2,
    pub sec_pos: Vec2,
    pub csc_pos: Vec2,
    pub theta_pos: Vec2,
    pub unit_pos: Vec2,

    sin_should_fade: bool,
    cos_should_fade: bool,
    theta_should_fade: bool,
    unit_should_fade: bool,

    pub sin_label_opacity: f32,
    pub cos_label_opacity: f32,
    pub theta_label_opacity: f32,
    pub unit_label_opacity: f32,

    fade_out_secs: f32,
    fade_in_secs: f32,
    fade_intensity: f32,
}

impl Labels {
    pub fn new() -> Self {
        let label_map = [
            (Label::Sin, Rect::from_xy_wh(Vec2::ZERO, label_bounds())),
            (Label::Cos, Rect::from_xy_wh(Vec2::ZERO, label_bounds())),
            (Label::Tan, Rect::from_xy_wh(Vec2::ZERO, label_bounds())),
            (Label::Cot, Rect::from_xy_wh(Vec2::ZERO, label_bounds())),
            (Label::Sec, Rect::from_xy_wh(Vec2::ZERO, label_bounds())),
            (Label::Csc, Rect::from_xy_wh(Vec2::ZERO, label_bounds())),
            (Label::Theta, Rect::from_xy_wh(Vec2::ZERO, label_bounds())),
            (Label::Unit, Rect::from_xy_wh(Vec2::ZERO, label_bounds())),
        ]
        .into_iter()
        .collect();

        Self {
            label_map,

            sin_should_fade: false,
            cos_should_fade: false,
            theta_should_fade: false,
            unit_should_fade: false,

            sin_pos: Vec2::ZERO,
            cos_pos: Vec2::ZERO,
            tan_pos: Vec2::ZERO,
            cot_pos: Vec2::ZERO,
            sec_pos: Vec2::ZERO,
            csc_pos: Vec2::ZERO,
            theta_pos: Vec2::ZERO,
            unit_pos: Vec2::ZERO,

            sin_label_opacity: 1.0,
            cos_label_opacity: 1.0,
            theta_label_opacity: 1.0,
            unit_label_opacity: 1.0,

            fade_in_secs: FADE_TIME_SECS,
            fade_out_secs: FADE_TIME_SECS,
            fade_intensity: FADE_INTENSITY,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.update_positions();
        self.update_intersecting();
        self.update_fade(delta_time);
    }

    fn update_positions(&mut self) {
        self.label_map.entry(Label::Sin).and_modify(|x| {
            *x = Rect::from_xy_wh(self.sin_pos, label_bounds());
        });
        self.label_map.entry(Label::Cos).and_modify(|x| {
            *x = Rect::from_xy_wh(self.cos_pos, label_bounds());
        });
        self.label_map.entry(Label::Tan).and_modify(|x| {
            *x = Rect::from_xy_wh(self.tan_pos, label_bounds());
        });
        self.label_map.entry(Label::Cot).and_modify(|x| {
            *x = Rect::from_xy_wh(self.cot_pos, label_bounds());
        });
        self.label_map.entry(Label::Sec).and_modify(|x| {
            *x = Rect::from_xy_wh(self.sec_pos, label_bounds());
        });
        self.label_map.entry(Label::Csc).and_modify(|x| {
            *x = Rect::from_xy_wh(self.csc_pos, label_bounds());
        });
        self.label_map.entry(Label::Theta).and_modify(|x| {
            *x = Rect::from_xy_wh(self.theta_pos, label_bounds());
        });
        self.label_map.entry(Label::Unit).and_modify(|x| {
            *x = Rect::from_xy_wh(self.unit_pos, label_bounds());
        });
    }

    fn update_intersecting(&mut self) {
        'outer: for (curr, curr_rect) in &self.label_map {
            if !matches!(
                curr,
                Label::Sin | Label::Cos | Label::Theta | Label::Unit
            ) {
                continue;
            }

            for (other, other_rect) in &self.label_map {
                if curr == other {
                    continue;
                }

                if curr.should_fade(*other)
                    && curr_rect.overlap(*other_rect).is_some()
                {
                    if matches!(curr, Label::Sin) {
                        self.sin_should_fade = true;
                    }
                    else if matches!(curr, Label::Cos) {
                        self.cos_should_fade = true;
                    }
                    else if matches!(curr, Label::Theta) {
                        self.theta_should_fade = true;
                    }
                    else if matches!(curr, Label::Unit) {
                        self.unit_should_fade = true;
                    }
                    continue 'outer;
                }
            }

            if matches!(curr, Label::Sin) {
                self.sin_should_fade = false;
            }
            else if matches!(curr, Label::Cos) {
                self.cos_should_fade = false;
            }
            else if matches!(curr, Label::Theta) {
                self.theta_should_fade = false;
            }
            else if matches!(curr, Label::Unit) {
                self.unit_should_fade = false;
            }
        }
    }

    fn update_fade(&mut self, dt: f32) {
        self.sin_label_opacity = (self.sin_label_opacity
            + if self.sin_should_fade {
                -self.fade_out_secs.recip() * dt
            }
            else {
                self.fade_in_secs.recip() * dt
            })
        .clamp(1.0 - FADE_INTENSITY, 1.0);
        self.cos_label_opacity = (self.cos_label_opacity
            + if self.cos_should_fade {
                -self.fade_out_secs.recip() * dt
            }
            else {
                self.fade_in_secs.recip() * dt
            })
        .clamp(1.0 - FADE_INTENSITY, 1.0);
        self.theta_label_opacity = (self.theta_label_opacity
            + if self.theta_should_fade {
                -self.fade_out_secs.recip() * dt
            }
            else {
                self.fade_in_secs.recip() * dt
            })
        .clamp(1.0 - FADE_INTENSITY, 1.0);
        self.unit_label_opacity = (self.unit_label_opacity
            + if self.unit_should_fade {
                -self.fade_out_secs.recip() * dt
            }
            else {
                self.fade_in_secs.recip() * dt
            })
        .clamp(1.0 - FADE_INTENSITY, 1.0);
    }
}
