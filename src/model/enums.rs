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

#[derive(PartialEq, Debug)]
pub enum Background {
    Black,
    White,
    Vectors,
}

impl Background {
    pub fn next(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Vectors,
            Self::Vectors => Self::Black,
        }
    }
}
