use super::enums::{Background, RedrawBackground};
use super::Model;
use macroquad::prelude::*;

pub fn view(model: &Model) {
    if model.redraw_background == RedrawBackground::Pending {
        // order of loop is window_event -> update -> draw
        // that means this should never happen but if it does
        // then I messed up my logic.
        unreachable!();
    }

    if model.redraw_background == RedrawBackground::InProgress {
        match model.background {
            Background::Black => {
                clear_background(BLACK);
            }
            Background::White => {
                clear_background(WHITE);
            }
            Background::Vectors => {
                clear_background(WHITE);

                for fv in &model.flow_vectors {
                    fv.draw();
                }
            }
        };
    }

    for fp in &model.flow_particles {
        fp.draw();
    }
}
