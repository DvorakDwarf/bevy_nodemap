use core::fmt::Debug;
use bevy::prelude::*;
use bevy::color::palettes::css;

#[derive(Debug, Default, Clone, Copy)]
pub struct EdgeData {
    pub length: f32,
    pub color: Srgba
}

impl EdgeData {
    pub fn new(length: f32) -> EdgeData {
        let color = if length >  50.0 {
            css::PURPLE
        } else {
            css::WHITE
        };

        return EdgeData {
            length,
            color
        };
    }

    pub fn with_color(length: f32, color: Srgba) -> EdgeData {
        return EdgeData {
            length,
            color
        };
    }
}