pub const CASTLEVANIA: &[&str] = &["C7855C", "9A5F4B", "2F2227", "183AD0", "6B94D7"];
pub const DEFAULT: &[&str] = &["2F5564", "40A997", "DDB689", "E58844", "D66358"];
pub const FLOWERS: &[&str] = &["3A442B", "6F743A", "AFA393", "B3998F", "D13335"];
pub const LEAVES: &[&str] = &["325F59", "56BD79", "F1DE5B", "F8BB0A", "F54B24"];

pub const VIRIDIS: &[&str] = &[
    "440154", "481567", "482677", "453781", "404788", "39568C", "33638D", "2D708E", "287D8E",
    "238A8D", "1F968B", "20A387", "29AF7F", "3CBB75", "55C667", "73D055", "95D840", "B8DE29",
    "DCE319", "FDE725",
];

pub const MAGMA: &[&str] = &[
    "FCFFB2", "FCDF96", "FBC17D", "FBA368", "FA8657", "F66B4D", "ED504A", "E03B50", "C92D59",
    "B02363", "981D69", "81176D", "6B116F", "57096E", "43006A", "300060", "1E0848", "110B2D",
    "080616", "000005",
];

pub const TURBO: &[&str] = &[
    "30123b", "311542", "33184a", "341b51", "351e58", "36215f", "372466", "38266c", "392973",
    "3a2c79", "3b2f80", "3c3286", "3d358b", "3e3891", "3e3a97", "3f3d9c", "4040a2", "4043a7",
    "4146ac", "4248b1", "424bb6", "434eba", "4351bf", "4453c3", "4456c7", "4559cb", "455bcf",
    "455ed3", "4561d7", "4663da", "4666dd", "4669e1", "466be4", "466ee7", "4671e9", "4673ec",
    "4676ee", "4678f1", "467bf3", "467df5", "4680f7", "4682f9", "4685fa", "4587fc", "458afd",
    "448cfe", "448ffe", "4391ff", "4294ff", "4196ff", "3f99ff", "3e9bff", "3d9efe", "3ba1fd",
    "3aa3fd", "38a6fb", "36a8fa", "35abf9", "33adf7", "31b0f6", "2fb2f4", "2db5f2", "2cb7f0",
    "2ab9ee", "28bcec", "26beea", "25c0e7", "23c3e5", "21c5e2", "20c7e0", "1fc9dd", "1dccdb",
    "1cced8", "1bd0d5", "1ad2d3", "19d4d0", "18d6cd", "18d8cb", "18dac8", "17dbc5", "17ddc3",
    "17dfc0", "18e0be", "18e2bb", "19e3b9", "1ae5b7", "1be6b4", "1de8b2", "1ee9af", "20eaad",
    "22ecaa", "24eda7", "27eea4", "29efa1", "2cf09e", "2ff19b", "32f298", "35f394", "38f491",
    "3cf58e", "3ff68b", "43f787", "46f884", "4af980", "4efa7d", "51fa79", "55fb76", "59fc73",
    "5dfc6f", "61fd6c", "65fd69", "69fe65", "6dfe62", "71fe5f", "75ff5c", "79ff59", "7dff56",
    "80ff53", "84ff50", "88ff4e", "8bff4b", "8fff49", "92ff46", "96ff44", "99ff42", "9cfe40",
    "9ffe3e", "a2fd3d", "a4fd3b", "a7fc3a", "aafc39", "acfb38", "affa37", "b1f936", "b4f835",
    "b7f835", "b9f634", "bcf534", "bff434", "c1f334", "c4f233", "c6f033", "c9ef34", "cbee34",
    "ceec34", "d0eb34", "d2e934", "d5e835", "d7e635", "d9e435", "dbe236", "dde136", "e0df37",
    "e2dd37", "e4db38", "e6d938", "e7d738", "e9d539", "ebd339", "edd139", "eecf3a", "f0cd3a",
    "f1cb3a", "f3c93a", "f4c73a", "f5c53a", "f7c33a", "f8c13a", "f9bf39", "fabd39", "faba38",
    "fbb838", "fcb637", "fcb436", "fdb135", "fdaf35", "feac34", "fea933", "fea732", "fea431",
    "ffa12f", "ff9e2e", "ff9c2d", "ff992c", "fe962b", "fe932a", "fe9028", "fe8d27", "fd8a26",
    "fd8724", "fc8423", "fc8122", "fb7e20", "fb7b1f", "fa781e", "f9751c", "f8721b", "f86f1a",
    "f76c19", "f66917", "f56616", "f46315", "f36014", "f25d13", "f05b11", "ef5810", "ee550f",
    "ed530e", "eb500e", "ea4e0d", "e94b0c", "e7490b", "e6470a", "e4450a", "e34209", "e14009",
    "df3e08", "de3c07", "dc3a07", "da3806", "d83606", "d63405", "d43205", "d23105", "d02f04",
    "ce2d04", "cc2b03", "ca2903", "c82803", "c62602", "c32402", "c12302", "bf2102", "bc1f01",
    "ba1e01", "b71c01", "b41b01", "b21901", "af1801", "ac1601", "aa1501", "a71401", "a41201",
    "a11101", "9e1001", "9b0f01", "980d01", "950c01", "920b01", "8e0a01", "8b0901", "880801",
    "850701", "810602", "7e0502", "7a0402",
];

#[allow(dead_code)]
pub const TRANSPARENT_BLACK: &[&str] = &["00000011"];

#[allow(dead_code)]
pub const TRANSPARENT_WHITE: &[&str] = &["FFFFFF11"];

// pub fn new_random_palette() -> std::vec::Vec<&'static str> {
//     [CASTLEVANIA, DEFAULT, FLOWERS, LEAVES, VIRIDIS, MAGMA, TURBO]
//         .choose(&mut rand::thread_rng())
//         .unwrap()
//         .to_vec()
// }

pub enum Palette {
    Castlevania,
    Default,
    Flowers,
    Leaves,
    Magma,
    Viridis,
    Turbo,
    TransparentBlack,
    TransparentWhite,
}

impl Palette {
    pub fn next(&self) -> Self {
        match self {
            Self::Castlevania => Self::Default,
            Self::Default => Self::Flowers,
            Self::Flowers => Self::Leaves,
            Self::Leaves => Self::Magma,
            Self::Magma => Self::Viridis,
            Self::Viridis => Self::Turbo,
            Self::Turbo => Self::TransparentBlack,
            Self::TransparentBlack => Self::TransparentWhite,
            Self::TransparentWhite => Self::Castlevania,
        }
    }

    pub fn as_colors(&self) -> &'static [&'static str] {
        match self {
            Self::Castlevania => CASTLEVANIA,
            Self::Default => DEFAULT,
            Self::Flowers => FLOWERS,
            Self::Leaves => LEAVES,
            Self::Magma => MAGMA,
            Self::Viridis => VIRIDIS,
            Self::Turbo => TURBO,
            Self::TransparentBlack => TRANSPARENT_BLACK,
            Self::TransparentWhite => TRANSPARENT_WHITE,
        }
    }
}
