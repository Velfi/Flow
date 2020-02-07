use nannou::color::Rgb;
use rand::seq::SliceRandom;

pub fn random_color(palette: &[&'static str]) -> Rgb<u8> {
    let random_hex = palette.choose(&mut rand::thread_rng()).unwrap();
    if let [r, g, b] = hex::decode(random_hex).unwrap().as_slice() {
        Rgb::new(*r, *g, *b)
    } else {
        panic!("Random color generation is broken :(")
    }
}
