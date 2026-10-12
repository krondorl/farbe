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
    color_type: ColorType,
    value: String,
    count: u64,
}
