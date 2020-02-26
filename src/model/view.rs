use super::enums::{Background, RedrawBackground};
use super::Model;
use nannou::prelude::{App, Frame, BLACK, WHITE};

pub fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    if model.redraw_background == RedrawBackground::Pending {
        // order of loop is window_event -> update -> draw
        // that means this should never happen but if it does
        // then I messed up my logic.
        unreachable!();
    }

    if model.redraw_background == RedrawBackground::InProgress {
        match model.background {
            Background::Black => {
                draw.background().color(BLACK);
            }
            Background::White => {
                draw.background().color(WHITE);
            }
            Background::Vectors => {
                draw.background().color(WHITE);

                for fv in &model.flow_vectors {
                    fv.draw(&draw);
                }
            }
        };
    }

    for fp in &model.flow_particles {
        fp.draw(&draw);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();

    // TODO figure out how to save the background, drawing, and ui layers separately
    if model.show_ui {
        // Draw the state of the `Ui` to the frame.
        model.ui.draw_to_frame(app, &frame).unwrap();
    }
}
