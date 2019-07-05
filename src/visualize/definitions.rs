use crate::math::functions::Function;
use coffee::graphics::Color;

pub const BACKGROUND_COLOR: Color = Color::BLACK;

// #CDD24E
pub const PATH_COLOR: Color = Color {
    r: 205.0 / 255.0,
    g: 210.0 / 255.0,
    b: 78.0 / 255.0,
    a: 1.0,
};

pub const VECTOR_COLOR: Color = Color::WHITE;

// #807F6E
pub const VECTOR_CIRCLE_COLOR: Color = Color {
    r: 41.0 / 255.0,
    g: 171.0 / 255.0,
    b: 202.0 / 255.0,
    a: 0.1,
};
pub const VECTOR_CIRCLE_COLOR_STDDEV: f32 = 0.1;

pub const DEFAULT_N: isize = 16;
pub const N_CHANGE: isize = 2;

pub const UPS: u16 = 60;
pub const DEFAULT_SPEED: f64 = 1.0 / 10.0 / 60.0;

pub const DEFAULT_FN: Function = Function::Tent;

pub const PATH_WIDTH: f32 = 4.0;
pub const VECTOR_WIDTH: f32 = 2.0;
pub const VECTOR_CIRCLE_WIDTH: f32 = 1.5;
pub const VECTOR_HEAD_ANGLE: f64 = 0.5;
pub const VECTOR_HEAD_LENGTH_FACTOR: f64 = 0.1;
