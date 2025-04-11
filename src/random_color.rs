use macroquad::color::Color;
use rand::seq::SliceRandom;

pub fn random_color(palette: &[&'static str]) -> Color {
    let random_hex = palette.choose(&mut rand::thread_rng()).unwrap();
    match hex::decode(random_hex).unwrap().as_slice() {
        [r, g, b, a] => Color::new(*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0, *a as f32 / 255.0),
        [r, g, b] => Color::new(*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0, 1.0),
        _ => panic!("Random color generation is broken :("),
    }
}
