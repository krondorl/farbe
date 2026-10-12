// Copyright 2026-present Adam Burucs. MIT license.

use crate::model::ColorStat;
use crate::model::ColorType;
use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
#[allow(clippy::ptr_arg)]
pub fn search_named_colors(content: &str, names: &[&str], color_stats: &mut Vec<ColorStat>) {
    let mut counts = HashMap::<String, u64>::new();

    for &name in names {
        let pattern = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(name))).unwrap();

        let count = pattern.find_iter(content).count() as u64;

        if count > 0 {
            *counts.entry(name.to_ascii_lowercase()).or_default() += count;
        }
    }

    for (value, count) in counts {
        if let Some(cs) = color_stats.iter_mut().find(|cs| {
            matches!(cs.color_type, ColorType::NamedColor) && cs.value.eq_ignore_ascii_case(&value)
        }) {
            cs.count += count;
        } else {
            color_stats.push(ColorStat {
                color_type: ColorType::NamedColor,
                value,
                count,
            });
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::ptr_arg)]
pub fn search_hex3(content: &str, color_stats: &mut Vec<ColorStat>) {
    let pattern = Regex::new(r"#[0-9a-fA-F]{3}\b").unwrap();

    let mut counts = HashMap::<String, u64>::new();

    for mat in pattern.find_iter(content) {
        *counts.entry(mat.as_str().to_ascii_lowercase()).or_default() += 1;
    }

    for (value, count) in counts {
        if let Some(cs) = color_stats.iter_mut().find(|cs| {
            matches!(cs.color_type, ColorType::Hex3) && cs.value.eq_ignore_ascii_case(&value)
        }) {
            cs.count += count;
        } else {
            color_stats.push(ColorStat {
                color_type: ColorType::Hex3,
                value,
                count,
            });
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::ptr_arg)]
pub fn search_hex4(content: &str, color_stats: &mut Vec<ColorStat>) {
    let pattern = Regex::new(r"#[0-9a-fA-F]{4}\b").unwrap();

    let mut counts = HashMap::<String, u64>::new();

    for mat in pattern.find_iter(content) {
        *counts.entry(mat.as_str().to_ascii_lowercase()).or_default() += 1;
    }

    for (value, count) in counts {
        if let Some(cs) = color_stats.iter_mut().find(|cs| {
            matches!(cs.color_type, ColorType::Hex4) && cs.value.eq_ignore_ascii_case(&value)
        }) {
            cs.count += count;
        } else {
            color_stats.push(ColorStat {
                color_type: ColorType::Hex4,
                value,
                count,
            });
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::ptr_arg)]
pub fn search_hex6(content: &str, color_stats: &mut Vec<ColorStat>) {
    let pattern = Regex::new(r"#[0-9a-fA-F]{6}\b").unwrap();

    let mut counts = HashMap::<String, u64>::new();

    for mat in pattern.find_iter(content) {
        *counts.entry(mat.as_str().to_ascii_lowercase()).or_default() += 1;
    }

    for (value, count) in counts {
        if let Some(cs) = color_stats.iter_mut().find(|cs| {
            matches!(cs.color_type, ColorType::Hex6) && cs.value.eq_ignore_ascii_case(&value)
        }) {
            cs.count += count;
        } else {
            color_stats.push(ColorStat {
                color_type: ColorType::Hex6,
                value,
                count,
            });
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::ptr_arg)]
pub fn search_hex8(content: &str, color_stats: &mut Vec<ColorStat>) {
    let pattern = Regex::new(r"#[0-9a-fA-F]{8}\b").unwrap();

    let mut counts = HashMap::<String, u64>::new();

    for mat in pattern.find_iter(content) {
        *counts.entry(mat.as_str().to_ascii_lowercase()).or_default() += 1;
    }

    for (value, count) in counts {
        if let Some(cs) = color_stats.iter_mut().find(|cs| {
            matches!(cs.color_type, ColorType::Hex8) && cs.value.eq_ignore_ascii_case(&value)
        }) {
            cs.count += count;
        } else {
            color_stats.push(ColorStat {
                color_type: ColorType::Hex8,
                value,
                count,
            });
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::ptr_arg)]
pub fn search_rgb(content: &str, color_stats: &mut Vec<ColorStat>) {
    let pattern = Regex::new(r"rgb\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*\)").unwrap();

    let mut counts = HashMap::<&str, u64>::new();

    for mat in pattern.find_iter(content) {
        *counts.entry(mat.as_str()).or_default() += 1;
    }

    for (value, count) in counts {
        if let Some(cs) = color_stats
            .iter_mut()
            .find(|cs| matches!(cs.color_type, ColorType::Rgb) && cs.value == value)
        {
            cs.count += count;
        } else {
            color_stats.push(ColorStat {
                color_type: ColorType::Rgb,
                value: value.to_string(),
                count,
            });
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::ptr_arg)]
pub fn search_rgba(content: &str, color_stats: &mut Vec<ColorStat>) {
    let pattern =
        Regex::new(r"rgba\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*(?:0|1|0?\.\d+)\s*\)").unwrap();

    let mut counts = HashMap::<&str, u64>::new();

    for mat in pattern.find_iter(content) {
        *counts.entry(mat.as_str()).or_default() += 1;
    }

    for (value, count) in counts {
        if let Some(cs) = color_stats
            .iter_mut()
            .find(|cs| matches!(cs.color_type, ColorType::Rgba) && cs.value == value)
        {
            cs.count += count;
        } else {
            color_stats.push(ColorStat {
                color_type: ColorType::Rgba,
                value: value.to_string(),
                count,
            });
        }
    }
}
