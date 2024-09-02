use core::fmt::Debug;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct EdgeData {
    pub length: f32,
    pub color: Color
}

impl EdgeData {
    pub fn new(length: f32) -> EdgeData {
        let color = if length >  50.0 {
            Color::PURPLE
        } else {
            Color::WHITE
        };

        return EdgeData {
            length,
            color
        };
    }

    pub fn with_color(length: f32, color: Color) -> EdgeData {
        return EdgeData {
            length,
            color
        };
    }
}