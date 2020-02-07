use rand::seq::SliceRandom;

pub const CASTLEVANIA: &[&'static str] = &["C7855C", "9A5F4B", "2F2227", "183AD0", "6B94D7"];
pub const DEFAULT: &[&'static str] = &["2F5564", "40A997", "DDB689", "E58844", "D66358"];
pub const FLOWERS: &[&'static str] = &["3A442B", "6F743A", "AFA393", "B3998F", "D13335"];
pub const LEAVES: &[&'static str] = &["325F59", "56BD79", "F1DE5B", "F8BB0A", "F54B24"];

pub const VIRIDIS: &[&'static str] = &[
    "440154", "481567", "482677", "453781", "404788", "39568C", "33638D", "2D708E", "287D8E",
    "238A8D", "1F968B", "20A387", "29AF7F", "3CBB75", "55C667", "73D055", "95D840", "B8DE29",
    "DCE319", "FDE725",
];

pub const MAGMA: &[&'static str] = &[
    "FCFFB2", "FCDF96", "FBC17D", "FBA368", "FA8657", "F66B4D", "ED504A", "E03B50", "C92D59",
    "B02363", "981D69", "81176D", "6B116F", "57096E", "43006A", "300060", "1E0848", "110B2D",
    "080616", "000005",
];

pub fn new_random_palette() -> std::vec::Vec<&'static str> {
    [CASTLEVANIA, DEFAULT, FLOWERS, LEAVES, VIRIDIS, MAGMA]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_vec()
}
