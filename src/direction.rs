//------------------------------------------------------------------------------
// Direction enum
//------------------------------------------------------------------------------

use bevy::math::Vec2;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn vector(self) -> Vec2 {
        match self {
            Self::Up => Vec2::new(0., 1.),
            Self::Down => Vec2::new(0., -1.),
            Self::Left => Vec2::new(-1., 0.),
            Self::Right => Vec2::new(1., 0.),
        }
    }
}
