// Copyright 2026-present Adam Burucs. MIT license.

#[derive(Debug)]
pub enum ColorType {
    Hex3,
    Hex4,
    Hex6,
    Hex8,
    Rgb,
    Rgba,
    NamedColor,
}

#[derive(Debug)]
pub struct ColorStat {
    pub color_type: ColorType,
    pub value: String,
    pub count: u64,
}
