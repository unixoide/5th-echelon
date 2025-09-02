#![allow(dead_code)]
//! Defines color constants for the UI.

use imgui::ImColor32;

/// Red color.
pub const RED: ImColor32 = ImColor32::from_rgb(255, 0, 0);
/// Green color.
pub const GREEN: ImColor32 = ImColor32::from_rgb(0, 255, 0);
/// Blue color.
pub const BLUE: ImColor32 = ImColor32::from_rgb(0, 0, 255);
/// Yellow color.
pub const YELLOW: ImColor32 = ImColor32::from_rgb(255, 255, 0);
/// Grey color.
pub const GREY: ImColor32 = ImColor32::from_rgb(128, 128, 128);
/// White color.
pub const WHITE: ImColor32 = ImColor32::from_rgb(255, 255, 255);
/// Orange color.
pub const ORANGE: ImColor32 = ImColor32::from_rgb(255, 165, 0);
