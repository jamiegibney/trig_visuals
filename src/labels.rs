#![allow(unused)]
use super::*;
use crate::consts::*;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

fn label_bounds() -> Vec2 {
    vec2(40.0, 30.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Label {
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
            Self::Sec => matches!(other, Self::Cot),
            Self::Theta => matches!(other, Self::Sin),
            Self::Unit => matches!(other, Self::Cos | Self::Sin | Self::Csc),
            Self::Tan | Self::Cot | Self::Csc => false,
        }
    }
}

#[derive(Debug)]
struct LabelData {
    pub rect: Rect,
    pub should_fade: AtomicBool,
    pub opacity: f32,
}

impl Default for LabelData {
    fn default() -> Self {
        Self {
            rect: Rect::from_xy_wh(Vec2::ZERO, label_bounds()),
            should_fade: AtomicBool::new(false),
            opacity: 1.0,
        }
    }
}

impl Clone for LabelData {
    fn clone(&self) -> Self {
        Self {
            rect: self.rect,
            should_fade: AtomicBool::new(self.should_fade.load(Relaxed)),
            opacity: self.opacity,
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug)]
pub struct Labels {
    label_map: HashMap<Label, LabelData>,

    // pub sin_pos: Vec2,
    // pub cos_pos: Vec2,
    // pub tan_pos: Vec2,
    // pub cot_pos: Vec2,
    // pub sec_pos: Vec2,
    // pub csc_pos: Vec2,
    // pub theta_pos: Vec2,
    // pub unit_pos: Vec2,
    fade_out_secs: f32,
    fade_in_secs: f32,
    fade_intensity: f32,
}

impl Labels {
    pub fn new() -> Self {
        let label_map = [
            (Label::Sin, LabelData::default()),
            (Label::Cos, LabelData::default()),
            (Label::Tan, LabelData::default()),
            (Label::Cot, LabelData::default()),
            (Label::Sec, LabelData::default()),
            (Label::Csc, LabelData::default()),
            (Label::Theta, LabelData::default()),
            (Label::Unit, LabelData::default()),
        ]
        .into_iter()
        .collect();

        Self {
            label_map,

            // sin_pos: Vec2::ZERO,
            // cos_pos: Vec2::ZERO,
            // tan_pos: Vec2::ZERO,
            // cot_pos: Vec2::ZERO,
            // sec_pos: Vec2::ZERO,
            // csc_pos: Vec2::ZERO,
            // theta_pos: Vec2::ZERO,
            // unit_pos: Vec2::ZERO,
            fade_in_secs: FADE_TIME_SECS,
            fade_out_secs: FADE_TIME_SECS,
            fade_intensity: FADE_INTENSITY,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // self.update_positions();
        self.update_intersecting();
        self.update_fade(delta_time);
    }

    pub fn get_opacity(&self, label: Label) -> f32 {
        self.label_map.get(&label).map_or(1.0, |lbl| lbl.opacity)
    }

    pub fn update_position(&mut self, label: Label, pos: Vec2) {
        self.label_map.entry(label).and_modify(|data| {
            data.rect = Rect::from_xy_wh(pos, label_bounds());
        });
    }

    pub fn get_position(&self, label: Label) -> Vec2 {
        self.label_map
            .get(&label)
            .expect("failed to unwrap label from map")
            .rect
            .xy()
    }

    fn update_intersecting(&mut self) {
        'outer: for (&curr, curr_data) in &self.label_map {
            if !matches!(
                curr,
                Label::Sin
                    | Label::Cos
                    | Label::Theta
                    | Label::Unit
                    | Label::Sec
            ) {
                continue;
            }

            let mut should_fade = false;

            for (&other, other_data) in &self.label_map {
                if curr == other {
                    continue;
                }

                if curr.should_fade(other)
                    && curr_data.rect.overlap(other_data.rect).is_some()
                {
                    should_fade = true;
                    break;
                }
            }

            curr_data.should_fade.store(should_fade, Relaxed);
        }
    }

    fn update_fade(&mut self, dt: f32) {
        let mut set_opacity = |label| {
            if let Some(data) = self.label_map.get_mut(&label) {
                data.opacity = (data.opacity
                    + if data.should_fade.load(Relaxed) {
                        -self.fade_out_secs.recip() * dt
                    }
                    else {
                        self.fade_in_secs.recip() * dt
                    })
                .clamp(1.0 - FADE_INTENSITY, 1.0);
            }
        };

        set_opacity(Label::Sin);
        set_opacity(Label::Cos);
        set_opacity(Label::Sec);
        set_opacity(Label::Theta);
        set_opacity(Label::Unit);
    }
}
