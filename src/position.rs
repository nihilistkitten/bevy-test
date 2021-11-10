//! The position component.

/// A position component.
#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({x}, {y})", x = self.x, y = self.y)
    }
}
