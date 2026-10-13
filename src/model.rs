// Copyright 2026-present Adam Burucs. MIT license.

use tabled::Tabled;

#[derive(Tabled, Debug)]
pub enum ColorType {
    Hex3,
    Hex4,
    Hex6,
    Hex8,
    Rgb,
    Rgba,
    NamedColor,
}

impl std::fmt::Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Hex3 => "Hex3",
            Self::Hex4 => "Hex4",
            Self::Hex6 => "Hex6",
            Self::Hex8 => "Hex8",
            Self::Rgb => "Rgb",
            Self::Rgba => "Rgba",
            Self::NamedColor => "NamedColor",
        })
    }
}

#[derive(Tabled, Debug)]
pub struct ColorStat {
    pub color_type: ColorType,
    pub value: String,
    pub count: u64,
}
