use nannou::prelude::Rgb;
use std::marker::PhantomData as PD;

pub const DEFAULT_RATE: f32 = 0.25;
pub const RATE_INCREMENT: f32 = 0.08;
pub const STROKE_WEIGHT: f32 = 3.0;
pub const LABEL_FONT_SIZE: u32 = 15;
pub const UNIT_RADIUS: f32 = 200.0;

pub const FADE_TIME_SECS: f32 = 0.3;
pub const FADE_INTENSITY: f32 = 0.8;

pub const SIN_LABEL: &str = "sin θ";
pub const COS_LABEL: &str = "cos θ";
pub const TAN_LABEL: &str = "tan θ";
pub const COT_LABEL: &str = "cot θ";
pub const SEC_LABEL: &str = "sec θ";
pub const CSC_LABEL: &str = "csc θ";

pub const SIN_COLOR: Rgb = Rgb { red: 1.0, green: 0.0, blue: 0.0, standard: PD };
pub const COS_COLOR: Rgb = Rgb { red: 1.0, green: 1.0, blue: 0.0, standard: PD };
pub const TAN_COLOR: Rgb = Rgb { red: 0.0, green: 1.0, blue: 0.0, standard: PD };
pub const COT_COLOR: Rgb = Rgb { red: 0.0, green: 1.0, blue: 1.0, standard: PD };
pub const SEC_COLOR: Rgb = Rgb { red: 0.0, green: 0.0, blue: 1.0, standard: PD };
pub const CSC_COLOR: Rgb = Rgb { red: 1.0, green: 0.0, blue: 1.0, standard: PD };
