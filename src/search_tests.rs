// Copyright 2026-present Adam Burucs. MIT license.

use crate::model::{ColorStat, ColorType};
use crate::search::{
    search_hex3, search_hex4, search_hex6, search_hex8, search_named_colors, search_rgb,
    search_rgba,
};

fn stat(color_type: ColorType, value: &str, count: u64) -> ColorStat {
    ColorStat {
        color_type,
        value: value.to_string(),
        count,
    }
}

fn assert_stat(stats: &[ColorStat], color_type: ColorType, value: &str, count: u64) {
    let matching = stats.iter().find(|candidate| {
        std::mem::discriminant(&candidate.color_type) == std::mem::discriminant(&color_type)
            && candidate.value == value
    });

    assert!(matching.is_some_and(|candidate| candidate.count == count));
}

#[test]
fn search_named_colors_counts_case_insensitively_and_merges_existing_stats() {
    let mut stats = vec![stat(ColorType::NamedColor, "red", 1)];

    search_named_colors("Red red BLUE redwood", &["red", "blue"], &mut stats);

    assert_eq!(stats.len(), 2);
    assert_stat(&stats, ColorType::NamedColor, "red", 3);
    assert_stat(&stats, ColorType::NamedColor, "blue", 1);
}

#[test]
fn search_hex3_counts_normalized_values() {
    let mut stats = Vec::new();

    search_hex3("#AbC #abc #abcd", &mut stats);

    assert_eq!(stats.len(), 1);
    assert_stat(&stats, ColorType::Hex3, "#abc", 2);
}

#[test]
fn search_hex4_counts_normalized_values() {
    let mut stats = Vec::new();

    search_hex4("#AbC1 #abc1 #abc12", &mut stats);

    assert_eq!(stats.len(), 1);
    assert_stat(&stats, ColorType::Hex4, "#abc1", 2);
}

#[test]
fn search_hex6_counts_normalized_values() {
    let mut stats = Vec::new();

    search_hex6("#A1B2C3 #a1b2c3 #a1b2c34", &mut stats);

    assert_eq!(stats.len(), 1);
    assert_stat(&stats, ColorType::Hex6, "#a1b2c3", 2);
}

#[test]
fn search_hex8_counts_normalized_values() {
    let mut stats = Vec::new();

    search_hex8("#A1B2C3D4 #a1b2c3d4 #a1b2c3d45", &mut stats);

    assert_eq!(stats.len(), 1);
    assert_stat(&stats, ColorType::Hex8, "#a1b2c3d4", 2);
}

#[test]
fn search_rgb_counts_exact_matches_and_merges_existing_stats() {
    let mut stats = vec![stat(ColorType::Rgb, "rgb(1, 2, 3)", 1)];

    search_rgb("rgb(1, 2, 3) rgb(1, 2, 3) rgb(4,5,6)", &mut stats);

    assert_eq!(stats.len(), 2);
    assert_stat(&stats, ColorType::Rgb, "rgb(1, 2, 3)", 3);
    assert_stat(&stats, ColorType::Rgb, "rgb(4,5,6)", 1);
}

#[test]
fn search_rgba_counts_valid_alpha_values_and_merges_existing_stats() {
    let mut stats = vec![stat(ColorType::Rgba, "rgba(1, 2, 3, 0.5)", 1)];

    search_rgba(
        "rgba(1, 2, 3, 0.5) rgba(1, 2, 3, 0.5) rgba(4,5,6,1) rgba(7,8,9,1.5)",
        &mut stats,
    );

    assert_eq!(stats.len(), 2);
    assert_stat(&stats, ColorType::Rgba, "rgba(1, 2, 3, 0.5)", 3);
    assert_stat(&stats, ColorType::Rgba, "rgba(4,5,6,1)", 1);
}
