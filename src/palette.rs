use std::fmt::{self, Display};

pub const TRANSPARENT_BLACK: &[&str] = &["00000011"];
pub const TRANSPARENT_WHITE: &[&str] = &["FFFFFF11"];
pub const CM1: &[&str] = &["534627", "127797", "12CCDB", "D8DB96", "F4421E"];
pub const CM2: &[&str] = &["18454D", "3F7DA8", "F1F5F3", "DA9E32", "BB5F42"];
pub const CM3: &[&str] = &["2F5564", "40A997", "DDB689", "E58844", "D66358"];

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

pub const ARNE: &[&str] = &[
    "000000", "9D9D9D", "FFFFFF", "BE2633", "E06F8B", "493C2B", "A46422", "EB8931", "F7E26B",
    "2F484E", "44891A", "A3CE27", "1B2632", "005784", "31A2F2", "B2DCEF",
];

pub const JMP: &[&str] = &[
    "000000", "191028", "46af45", "a1d685", "453e78", "7664fe", "833129", "9ec2e8", "dc534b",
    "e18d79", "d6b97b", "e9d8a1", "216c4b", "d365c8", "afaab9", "f5f4eb",
];

pub const ZUGHY_32: &[&str] = &[
    "472d3c", "5e3643", "7a444a", "a05b53", "bf7958", "eea160", "f4cca1", "b6d53c", "71aa34",
    "397b44", "3c5956", "302c2e", "5a5353", "7d7071", "a0938e", "cfc6b8", "dff6f5", "8aebf1",
    "28ccdf", "3978a8", "394778", "39314b", "564064", "8e478c", "cd6093", "ffaeb6", "f4b41b",
    "f47e1b", "e6482e", "a93b3b", "827094", "4f546b",
];

pub const ENDESGA_64: &[&str] = &[
    "ff0040", "131313", "1b1b1b", "272727", "3d3d3d", "5d5d5d", "858585", "b4b4b4", "ffffff",
    "c7cfdd", "92a1b9", "657392", "424c6e", "2a2f4e", "1a1932", "0e071b", "1c121c", "391f21",
    "5d2c28", "8a4836", "bf6f4a", "e69c69", "f6ca9f", "f9e6cf", "edab50", "e07438", "c64524",
    "8e251d", "ff5000", "ed7614", "ffa214", "ffc825", "ffeb57", "d3fc7e", "99e65f", "5ac54f",
    "33984b", "1e6f50", "134c4c", "0c2e44", "00396d", "0069aa", "0098dc", "00cdf9", "0cf1ff",
    "94fdff", "fdd2ed", "f389f5", "db3ffd", "7a09fa", "3003d9", "0c0293", "03193f", "3b1443",
    "622461", "93388f", "ca52c9", "c85086", "f68187", "f5555d", "ea323c", "c42430", "891e2b",
    "571c27",
];

pub const FANTASY: &[&str] = &[
    "1f240a", "39571c", "a58c27", "efac28", "efd8a1", "ab5c1c", "183f39", "ef692f", "efb775",
    "a56243", "773421", "724113", "2a1d0d", "392a1c", "684c3c", "927e6a", "276468", "ef3a0c",
    "45230d", "3c9f9c", "9b1a0a", "36170c", "550f0a", "300f0a",
];

pub enum Palette {
    Arne,
    CM1,
    CM2,
    CM3,
    Endesga64,
    Fantasy,
    JMP,
    Magma,
    TransparentBlack,
    TransparentWhite,
    Turbo,
    Viridis,
    Zughy32,
}

impl Palette {
    pub fn next(&self) -> Self {
        match self {
            Self::Arne => Self::CM1,
            Self::CM1 => Self::CM2,
            Self::CM2 => Self::CM3,
            Self::CM3 => Self::Endesga64,
            Self::Endesga64 => Self::Fantasy,
            Self::Fantasy => Self::JMP,
            Self::JMP => Self::Magma,
            Self::Magma => Self::TransparentBlack,
            Self::TransparentBlack => Self::TransparentWhite,
            Self::TransparentWhite => Self::Turbo,
            Self::Turbo => Self::Viridis,
            Self::Viridis => Self::Zughy32,
            Self::Zughy32 => Self::Arne,
        }
    }

    pub fn as_colors(&self) -> &'static [&'static str] {
        match self {
            Self::Arne => ARNE,
            Self::CM1 => CM1,
            Self::CM2 => CM2,
            Self::CM3 => CM3,
            Self::Endesga64 => ENDESGA_64,
            Self::Fantasy => FANTASY,
            Self::JMP => JMP,
            Self::Magma => MAGMA,
            Self::TransparentBlack => TRANSPARENT_BLACK,
            Self::TransparentWhite => TRANSPARENT_WHITE,
            Self::Turbo => TURBO,
            Self::Viridis => VIRIDIS,
            Self::Zughy32 => ZUGHY_32,
        }
    }
}

impl Display for Palette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Arne => "Arne",
                Self::CM1 => "CM1",
                Self::CM2 => "CM2",
                Self::CM3 => "CM3",
                Self::Endesga64 => "Endesga 64",
                Self::Fantasy => "Fantasy",
                Self::JMP => "JMP",
                Self::Magma => "Magma",
                Self::TransparentBlack => "Black Fog",
                Self::TransparentWhite => "White Fog",
                Self::Turbo => "Turbo",
                Self::Viridis => "Viridis",
                Self::Zughy32 => "Zughy 32",
            }
        )
    }
}

impl Default for Palette {
    fn default() -> Self {
        Palette::Viridis
    }
}
