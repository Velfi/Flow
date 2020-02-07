use nannou::color::Rgba;
use rand::seq::SliceRandom;

pub fn random_color(palette: &[&'static str]) -> Rgba<u8> {
    let random_hex = palette.choose(&mut rand::thread_rng()).unwrap();
    match hex::decode(random_hex).unwrap().as_slice() {
        [r, g, b, a] => Rgba::new(*r, *g, *b, *a),
        [r, g, b] => Rgba::new(*r, *g, *b, 255),
        _ => panic!("Random color generation is broken :("),
    }
}
