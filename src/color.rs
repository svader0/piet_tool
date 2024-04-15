#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PietColor {
    pub name: ColorName,
    pub lightness: Option<i8>,
    pub hue: Option<i8>,
}

impl PietColor {
    pub fn new(name: ColorName, lightness: Option<i8>, hue: Option<i8>) -> Self {
        PietColor {
            name,
            lightness,
            hue,
        }
    }
    // Hue Cycle: red -> yellow -> green -> cyan -> blue -> magenta -> red
    // The hue difference between two colors is the number of steps in the hue cycle
    // between the two colors. For example, the hue difference between red and yellow is 1.
    pub fn hue_difference(&self, other: &PietColor) -> i8 {
        let hue_diff = (self.hue.unwrap() as f64 - other.hue.unwrap() as f64).abs();
        if hue_diff <= 3.5 {
            hue_diff.round() as i8
        } else {
            (7.0 - hue_diff).round() as i8
        }
    }

    // Lightness Cycle: light -> normal -> dark -> light
    // The lightness difference between two colors is the number of steps in the lightness cycle
    pub fn lightness_difference(&self, other: &PietColor) -> i8 {
        (self.lightness.unwrap() - other.lightness.unwrap()).abs()
    }

    pub fn from_rgb(rgb: &[u8; 3]) -> Self {
        match rgb {
            [0, 0, 0] => PietColor {
                name: ColorName::Black,
                lightness: Some(2),
                hue: Some(0),
            },
            [255, 255, 255] => PietColor {
                name: ColorName::White,
                lightness: Some(0),
                hue: Some(0),
            },
            [255, 0, 0] => PietColor {
                name: ColorName::Red,
                lightness: Some(1),
                hue: Some(0),
            },
            [255, 255, 0] => PietColor {
                name: ColorName::Yellow,
                lightness: Some(1),
                hue: Some(1),
            },
            [0, 255, 0] => PietColor {
                name: ColorName::Green,
                lightness: Some(1),
                hue: Some(2),
            },
            [0, 255, 255] => PietColor {
                name: ColorName::Cyan,
                lightness: Some(1),
                hue: Some(3),
            },
            [0, 0, 255] => PietColor {
                name: ColorName::Blue,
                lightness: Some(1),
                hue: Some(4),
            },
            [255, 0, 255] => PietColor {
                name: ColorName::Magenta,
                lightness: Some(1),
                hue: Some(5),
            },
            [192, 0, 0] => PietColor {
                name: ColorName::DarkRed,
                lightness: Some(2),
                hue: Some(0),
            },
            [192, 192, 0] => PietColor {
                name: ColorName::DarkYellow,
                lightness: Some(2),
                hue: Some(1),
            },
            [0, 192, 0] => PietColor {
                name: ColorName::DarkGreen,
                lightness: Some(2),
                hue: Some(2),
            },
            [0, 192, 192] => PietColor {
                name: ColorName::DarkCyan,
                lightness: Some(2),
                hue: Some(3),
            },
            [0, 0, 192] => PietColor {
                name: ColorName::DarkBlue,
                lightness: Some(2),
                hue: Some(4),
            },
            [192, 0, 192] => PietColor {
                name: ColorName::DarkMagenta,
                lightness: Some(2),
                hue: Some(5),
            },
            [255, 192, 192] => PietColor {
                name: ColorName::LightRed,
                lightness: Some(0),
                hue: Some(0),
            },
            [255, 255, 192] => PietColor {
                name: ColorName::LightYellow,
                lightness: Some(0),
                hue: Some(1),
            },
            [192, 255, 192] => PietColor {
                name: ColorName::LightGreen,
                lightness: Some(0),
                hue: Some(2),
            },
            [192, 255, 255] => PietColor {
                name: ColorName::LightCyan,
                lightness: Some(0),
                hue: Some(3),
            },
            [192, 192, 255] => PietColor {
                name: ColorName::LightBlue,
                lightness: Some(0),
                hue: Some(4),
            },
            [255, 192, 255] => PietColor {
                name: ColorName::LightMagenta,
                lightness: Some(0),
                hue: Some(5),
            },
            _ => panic!("Invalid color: {:?}", rgb),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorName {
    Black,
    White,
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
    DarkRed,
    DarkYellow,
    DarkGreen,
    DarkCyan,
    DarkBlue,
    DarkMagenta,
    LightRed,
    LightYellow,
    LightGreen,
    LightCyan,
    LightBlue,
    LightMagenta,
}
