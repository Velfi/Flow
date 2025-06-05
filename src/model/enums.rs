use std::fmt::{self, Display};

// This enum only exists because I'm unsure of how to mutate the model during drawing or
// if that's even possible. This works around that by giving the draw phase a chance
// to respond to the redraw request before `update()` sets the state to Complete.
#[derive(PartialEq, Debug)]
pub enum RedrawBackground {
    Pending,
    InProgress,
    Complete,
}

impl RedrawBackground {
    pub fn next(&self) -> Self {
        match self {
            Self::Pending => Self::InProgress,
            Self::InProgress => Self::Complete,
            Self::Complete => Self::Complete,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Background {
    Black,
    White,
    Vectors,
}

impl Display for Background {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Black => "Black",
                Self::White => "White",
                Self::Vectors => "Vector Field",
            }
        )
    }
}

impl Default for Background {
    fn default() -> Self {
        Self::Vectors
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleShape {
    Circle,
    Square,
    Triangle,
    Star,
    Diamond,
}

impl std::fmt::Display for ParticleShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Circle => write!(f, "Circle"),
            Self::Square => write!(f, "Square"),
            Self::Triangle => write!(f, "Triangle"),
            Self::Star => write!(f, "Star"),
            Self::Diamond => write!(f, "Diamond"),
        }
    }
}
