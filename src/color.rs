#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct HSLColor {
    hue: f64,
    saturation: f64,
    lightness: f64,
}

impl HSLColor {
    // Convert RGB HSLColor to HSL HSLColor
    pub fn from_rgb(rgb: [u8; 3]) -> Self {
        let r = rgb[0] as f64 / 255.0;
        let g = rgb[1] as f64 / 255.0;
        let b = rgb[2] as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let mut hue = if max == min {
            0.0
        } else if max == r {
            60.0 * ((g - b) / (max - min)).rem_euclid(6.0)
        } else if max == g {
            60.0 * ((b - r) / (max - min)) + 2.0
        } else {
            60.0 * ((r - g) / (max - min)) + 4.0
        };

        if hue < 0.0 {
            hue += 360.0;
        }

        let lightness = (max + min) / 2.0;

        let saturation = if max == min {
            0.0
        } else if lightness <= 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };

        HSLColor {
            hue,
            saturation,
            lightness,
        }
    }

    // Function to calculate the difference in hue between two colors
    pub fn hue_difference(&self, other: &HSLColor) -> f64 {
        let mut diff = (self.hue - other.hue).abs();
        if diff > 180.0 {
            diff = 360.0 - diff;
        }
        diff
    }

    // Function to calculate the difference in lightness between two colors
    pub fn lightness_difference(&self, other: &HSLColor) -> f64 {
        (self.lightness - other.lightness).abs()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PietColor {
    LightRed,
    LightYellow,
    LightGreen,
    LightCyan,
    LightBlue,
    LightMagenta,
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
    White,
    Black,
}

impl PietColor {
    // Constants for HSL values of Piet colors
    const LIGHT_RED: HSLColor = HSLColor {
        hue: 0.0,
        saturation: 1.0,
        lightness: 0.875,
    };
    const LIGHT_YELLOW: HSLColor = HSLColor {
        hue: 60.0,
        saturation: 1.0,
        lightness: 0.875,
    };
    const LIGHT_GREEN: HSLColor = HSLColor {
        hue: 120.0,
        saturation: 1.0,
        lightness: 0.875,
    };
    const LIGHT_CYAN: HSLColor = HSLColor {
        hue: 180.0,
        saturation: 1.0,
        lightness: 0.875,
    };
    const LIGHT_BLUE: HSLColor = HSLColor {
        hue: 240.0,
        saturation: 1.0,
        lightness: 0.875,
    };
    const LIGHT_MAGENTA: HSLColor = HSLColor {
        hue: 300.0,
        saturation: 1.0,
        lightness: 0.875,
    };

    const RED: HSLColor = HSLColor {
        hue: 0.0,
        saturation: 1.0,
        lightness: 0.5,
    };
    const YELLOW: HSLColor = HSLColor {
        hue: 60.0,
        saturation: 1.0,
        lightness: 0.5,
    };
    const GREEN: HSLColor = HSLColor {
        hue: 120.0,
        saturation: 1.0,
        lightness: 0.5,
    };
    const CYAN: HSLColor = HSLColor {
        hue: 180.0,
        saturation: 1.0,
        lightness: 0.5,
    };
    const BLUE: HSLColor = HSLColor {
        hue: 240.0,
        saturation: 1.0,
        lightness: 0.5,
    };
    const MAGENTA: HSLColor = HSLColor {
        hue: 300.0,
        saturation: 1.0,
        lightness: 0.5,
    };

    const DARK_RED: HSLColor = HSLColor {
        hue: 0.0,
        saturation: 1.0,
        lightness: 0.25,
    };
    const DARK_YELLOW: HSLColor = HSLColor {
        hue: 60.0,
        saturation: 1.0,
        lightness: 0.25,
    };
    const DARK_GREEN: HSLColor = HSLColor {
        hue: 120.0,
        saturation: 1.0,
        lightness: 0.25,
    };
    const DARK_CYAN: HSLColor = HSLColor {
        hue: 180.0,
        saturation: 1.0,
        lightness: 0.25,
    };
    const DARK_BLUE: HSLColor = HSLColor {
        hue: 240.0,
        saturation: 1.0,
        lightness: 0.25,
    };
    const DARK_MAGENTA: HSLColor = HSLColor {
        hue: 300.0,
        saturation: 1.0,
        lightness: 0.25,
    };
    const WHITE: HSLColor = HSLColor {
        hue: 0.0,
        saturation: 0.0,
        lightness: 1.0,
    };
    const BLACK: HSLColor = HSLColor {
        hue: 0.0,
        saturation: 0.0,
        lightness: 0.0,
    };

    // Function to get HSL value for a PietColor
    pub fn hsl(&self) -> HSLColor {
        match self {
            PietColor::LightRed => Self::LIGHT_RED,
            PietColor::LightYellow => Self::LIGHT_YELLOW,
            PietColor::LightGreen => Self::LIGHT_GREEN,
            PietColor::LightCyan => Self::LIGHT_CYAN,
            PietColor::LightBlue => Self::LIGHT_BLUE,
            PietColor::LightMagenta => Self::LIGHT_MAGENTA,
            PietColor::Red => Self::RED,
            PietColor::Yellow => Self::YELLOW,
            PietColor::Green => Self::GREEN,
            PietColor::Cyan => Self::CYAN,
            PietColor::Blue => Self::BLUE,
            PietColor::Magenta => Self::MAGENTA,
            PietColor::DarkRed => Self::DARK_RED,
            PietColor::DarkYellow => Self::DARK_YELLOW,
            PietColor::DarkGreen => Self::DARK_GREEN,
            PietColor::DarkCyan => Self::DARK_CYAN,
            PietColor::DarkBlue => Self::DARK_BLUE,
            PietColor::DarkMagenta => Self::DARK_MAGENTA,
            PietColor::White => Self::WHITE,
            PietColor::Black => Self::BLACK,
        }
    }
    pub fn from_rgb(rgb: &[u8; 3]) -> Self {
        match rgb {
            [255, 192, 192] => PietColor::LightRed,
            [255, 255, 192] => PietColor::LightYellow,
            [192, 255, 192] => PietColor::LightGreen,
            [192, 255, 255] => PietColor::LightCyan,
            [192, 192, 255] => PietColor::LightBlue,
            [255, 192, 255] => PietColor::LightMagenta,
            [255, 0, 0] => PietColor::Red,
            [255, 255, 0] => PietColor::Yellow,
            [0, 255, 0] => PietColor::Green,
            [0, 255, 255] => PietColor::Cyan,
            [0, 0, 255] => PietColor::Blue,
            [255, 0, 255] => PietColor::Magenta,
            [192, 0, 0] => PietColor::DarkRed,
            [192, 192, 0] => PietColor::DarkYellow,
            [0, 192, 0] => PietColor::DarkGreen,
            [0, 192, 192] => PietColor::DarkCyan,
            [0, 0, 192] => PietColor::DarkBlue,
            [192, 0, 192] => PietColor::DarkMagenta,
            [255, 255, 255] => PietColor::White,
            [0, 0, 0] => PietColor::Black,
            _ => panic!("Invalid color"),
        }
    }
}
