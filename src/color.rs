struct Color {
    id: i32,
    name: &'static str,
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn new(id: i32, name: &'static str, red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            id: id,
            name: name,
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    fn index(id: i32) -> Color {
        match id {
            -1 => BaseColors::Transparent.value(),
            1 => BaseColors::Gainsboro.value(),
            2 => BaseColors::Grey.value(),
            3 => BaseColors::Nero.value(),
            4 => BaseColors::CarnationPink.value(),
            5 => BaseColors::Red.value(),
            6 => BaseColors::Orange.value(),
            7 => BaseColors::Brown.value(),
            8 => BaseColors::Yellow.value(),
            9 => BaseColors::Conifer.value(),
            10 => BaseColors::Green.value(),
            11 => BaseColors::DarkTurquoise.value(),
            12 => BaseColors::PacificBlue.value(),
            13 => BaseColors::Blue.value(),
            14 => BaseColors::Violet.value(),
            15 => BaseColors::Purple.value(),
            _ => BaseColors::White.value(),
        }
    }

    fn rgb(rgb: RGB, silent: bool, sensitive: u8, brightness: u8) {}

    // fn rgb(rgb: (u8, u8, u8), silent: bool, sensitive: u8, brightness: u8) -> Color {
    //     let diff_min = [(255, 255, 255), 1038366];

    //     for color in BaseColors {
    //         if (color[0] == rgb[0] && color[1] == rgb[1] && color[2] == rgb[2]) {
    //             color
    //         }
    //     }

    //     if (rgb[3] != 255) {
    //         TRANSPARENT
    //     }

    //     for color in BaseColors {
    //         if (color.id == -1) {
    //             continue;
    //         }

    //         let diff_r =((rgb[0]) + brightness) - color.rgba[0] * ((rgb[0] + brightness) - color.rgba[0]);
    //         let diff_g =((rgb[1]) + brightness) - color.rgba[1] * ((rgb[1] + brightness) - color.rgba[1]);
    //         let diff_b =((rgb[2]) + brightness) - color.rgba[2] * ((rgb[2] + brightness) - color.rgba[2]);

    //         let x = cmp::max(diff_r, diff_g, diff_b);
    //         println!("{}", x);
    //     }
    // }
}

enum BaseColors {
    Transparent,
    White,
    Gainsboro,
    Grey,
    Nero,
    CarnationPink,
    Red,
    Orange,
    Brown,
    Yellow,
    Conifer,
    Green,
    DarkTurquoise,
    PacificBlue,
    Blue,
    Violet,
    Purple,
}

impl BaseColors {
    fn value(&self) -> Color {
        match *self {
            BaseColors::Transparent => Color {
                id: -1,
                name: "transparent",
                red: 255,
                green: 255,
                blue: 255,
                alpha: 0,
            },
            BaseColors::White => Color {
                id: 0,
                name: "white",
                red: 255,
                green: 255,
                blue: 255,
                alpha: 255,
            },
            BaseColors::Gainsboro => Color {
                id: 1,
                name: "gainsboro",
                red: 228,
                green: 228,
                blue: 228,
                alpha: 255,
            },
            BaseColors::Grey => Color {
                id: 2,
                name: "grey",
                red: 136,
                green: 136,
                blue: 136,
                alpha: 255,
            },
            BaseColors::Nero => Color {
                id: 3,
                name: "nero",
                red: 34,
                green: 34,
                blue: 34,
                alpha: 255,
            },
            BaseColors::CarnationPink => Color {
                id: 4,
                name: "carnation pink",
                red: 255,
                green: 167,
                blue: 209,
                alpha: 255,
            },
            BaseColors::Red => Color {
                id: 5,
                name: "red",
                red: 229,
                green: 0,
                blue: 0,
                alpha: 255,
            },
            BaseColors::Orange => Color {
                id: 6,
                name: "orange",
                red: 229,
                green: 149,
                blue: 0,
                alpha: 255,
            },
            BaseColors::Brown => Color {
                id: 7,
                name: "brown",
                red: 160,
                green: 106,
                blue: 66,
                alpha: 255,
            },
            BaseColors::Yellow => Color {
                id: 8,
                name: "yellow",
                red: 229,
                green: 217,
                blue: 0,
                alpha: 255,
            },
            BaseColors::Conifer => Color {
                id: 9,
                name: "conifer",
                red: 148,
                green: 224,
                blue: 68,
                alpha: 255,
            },
            BaseColors::Green => Color {
                id: 10,
                name: "green",
                red: 2,
                blue: 190,
                green: 1,
                alpha: 255,
            },
            BaseColors::DarkTurquoise => Color {
                id: 11,
                name: "dark turquoise",
                red: 0,
                blue: 211,
                green: 221,
                alpha: 255,
            },
            BaseColors::PacificBlue => Color {
                id: 12,
                name: "pacific blue",
                red: 0,
                green: 131,
                blue: 199,
                alpha: 255,
            },
            BaseColors::Blue => Color {
                id: 13,
                name: "blue",
                red: 0,
                green: 0,
                blue: 234,
                alpha: 255,
            },
            BaseColors::Violet => Color {
                id: 14,
                name: "violet",
                red: 207,
                green: 110,
                blue: 228,
                alpha: 255,
            },
            BaseColors::Purple => Color {
                id: 15,
                name: "purple",
                red: 130,
                green: 0,
                blue: 128,
                alpha: 255,
            },
        }
    }
}
