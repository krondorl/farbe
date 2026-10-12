// Copyright 2026-present Adam Burucs. MIT license.

pub enum ColorType {
    Hex3,
    Hex4,
    Hex6,
    Hex8,
    Rgb,
    Rgba,
    NamedColor,
}

pub struct ColorStat {
    pub color_type: ColorType,
    pub value: String,
    pub count: u64,
}
