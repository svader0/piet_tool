#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PietColor {
    pub name: ColorName,
    pub lightness: i8,
    pub hue: i8,
}

impl PietColor {
    pub fn new(name: ColorName, lightness: i8, hue: i8) -> Self {
        PietColor {
            name,
            lightness,
            hue,
        }
    }
    // Hue Cycle: red -> yellow -> green -> cyan -> blue -> magenta
    // The hue difference between two colors is the number of steps in the hue cycle
    // between the two colors.
    pub fn hue_difference(&self, other: &PietColor) -> i8 {
        (other.hue - self.hue + 6) % 6
    }

    // Lightness Cycle: light -> normal -> dark
    // The lightness difference between two colors is the number of steps in the lightness cycle.
    // The cycle can only be traversed in one direction, so the lightness difference between
    // light and dark is 2, while the lightness difference between dark and normal is 2.
    pub fn lightness_difference(&self, other: &PietColor) -> i8 {
        match (self.lightness, other.lightness) {
            // Light is stored as 1, normal as 0, and dark as -1
            // Instead of calculating the difference, we can just hard code them in.
            (-1, 0) => 2,
            (0, -1) => 1,
            (0, 1) => 2,
            (1, 0) => 1,
            (-1, 1) => 1,
            (1, -1) => 2,
            _ => 0,
        }
    }

    pub fn from_rgb(rgb: &[u8; 3]) -> Result<Self, &[u8; 3]> {
        match rgb {
            [0, 0, 0] => Ok(PietColor {
                name: ColorName::Black,
                lightness: -1,
                hue: 0,
            }),
            [255, 255, 255] => Ok(PietColor {
                name: ColorName::White,
                lightness: 1,
                hue: 0,
            }),
            [255, 0, 0] => Ok(PietColor {
                name: ColorName::Red,
                lightness: 0,
                hue: 0,
            }),
            [255, 255, 0] => Ok(PietColor {
                name: ColorName::Yellow,
                lightness: 0,
                hue: 1,
            }),
            [0, 255, 0] => Ok(PietColor {
                name: ColorName::Green,
                lightness: 0,
                hue: 2,
            }),
            [0, 255, 255] => Ok(PietColor {
                name: ColorName::Cyan,
                lightness: 0,
                hue: 3,
            }),
            [0, 0, 255] => Ok(PietColor {
                name: ColorName::Blue,
                lightness: 0,
                hue: 4,
            }),
            [255, 0, 255] => Ok(PietColor {
                name: ColorName::Magenta,
                lightness: 0,
                hue: 5,
            }),
            [192, 0, 0] => Ok(PietColor {
                name: ColorName::DarkRed,
                lightness: -1,
                hue: 0,
            }),
            [192, 192, 0] => Ok(PietColor {
                name: ColorName::DarkYellow,
                lightness: -1,
                hue: 1,
            }),
            [0, 192, 0] => Ok(PietColor {
                name: ColorName::DarkGreen,
                lightness: -1,
                hue: 2,
            }),
            [0, 192, 192] => Ok(PietColor {
                name: ColorName::DarkCyan,
                lightness: -1,
                hue: 3,
            }),
            [0, 0, 192] => Ok(PietColor {
                name: ColorName::DarkBlue,
                lightness: -1,
                hue: 4,
            }),
            [192, 0, 192] => Ok(PietColor {
                name: ColorName::DarkMagenta,
                lightness: -1,
                hue: 5,
            }),
            [255, 192, 192] => Ok(PietColor {
                name: ColorName::LightRed,
                lightness: 1,
                hue: 0,
            }),
            [255, 255, 192] => Ok(PietColor {
                name: ColorName::LightYellow,
                lightness: 1,
                hue: 1,
            }),
            [192, 255, 192] => Ok(PietColor {
                name: ColorName::LightGreen,
                lightness: 1,
                hue: 2,
            }),
            [192, 255, 255] => Ok(PietColor {
                name: ColorName::LightCyan,
                lightness: 1,
                hue: 3,
            }),
            [192, 192, 255] => Ok(PietColor {
                name: ColorName::LightBlue,
                lightness: 1,
                hue: 4,
            }),
            [255, 192, 255] => Ok(PietColor {
                name: ColorName::LightMagenta,
                lightness: 1,
                hue: 5,
            }),
            _ => Err(rgb),
        }
    }
}

impl Default for PietColor {
    fn default() -> Self {
        PietColor {
            name: ColorName::White,
            lightness: 0,
            hue: 0,
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
