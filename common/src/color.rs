use derive_more::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign};
use maths_rs::saturate;
// use num::complex::ComplexFloat;

use crate::math::vs_math::interpolate_float;


#[derive(PartialEq, Debug, Clone, Copy, Add, Sub, AddAssign, SubAssign, Mul, MulAssign)]
pub struct VScolor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Default for VScolor {
    fn default() -> Self {
        Self { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }
}

impl VScolor {
    pub const PURE_WHITE: VScolor = VScolor { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 };
    pub const WHITE: VScolor = VScolor { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 };
    pub const GREY: VScolor = VScolor { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 };
    pub const BLUE: VScolor = VScolor { red: 0.2, green: 0.2, blue: 1.0, alpha: 1.0 };
    pub const LIGHT_BLUE: VScolor = VScolor { red: 0.5, green: 0.5, blue: 1.0, alpha: 1.0 };
    pub const DARK_BLUE: VScolor = VScolor { red: 0.0, green: 0.0, blue: 0.25, alpha: 1.0 };
    pub const RED: VScolor = VScolor { red: 1.0, green: 0.2, blue: 0.2, alpha: 1.0 };
    pub const GREEN: VScolor = VScolor { red: 0.2, green: 1.0, blue: 0.2, alpha: 1.0 };
    pub const LIGHT_GREEN: VScolor = VScolor { red: 0.5, green: 1.0, blue: 0.5, alpha: 1.0 };
    pub const YELLOW: VScolor = VScolor { red: 1.0, green: 1.0, blue: 0.2, alpha: 1.0 };
    pub const ORANGE: VScolor = VScolor { red: 1.0, green: 0.5, blue: 0.2, alpha: 1.0 };
    pub const PURPLE: VScolor = VScolor { red: 1.0, green: 0.2, blue: 1.0, alpha: 1.0 };
    pub const BLACK: VScolor = VScolor { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 };
    pub const CLEAR: VScolor = VScolor { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 };

    pub fn new ( red: f32, green: f32, blue: f32, alpha: f32 ) -> Self {
        assert!(!red.is_nan());
        assert!(!green.is_nan());
        assert!(!blue.is_nan());
        assert!(!alpha.is_nan());
        VScolor { red, green, blue, alpha }
    }

    pub fn from_bytes ( red: u8, green: u8, blue: u8, alpha: u8 ) -> Self {
        Self {
            red: f32::from(red) / 255.0,
            green: f32::from(green) / 255.0,
            blue: f32::from(blue) / 255.0,
            alpha: f32::from(alpha) / 255.0,
        }
    }

    pub fn from_hsv ( hue: f32, saturation: f32, value: f32 ) -> Self {
        assert!(!hue.is_nan());
        assert!(!saturation.is_nan());
        assert!(!value.is_nan());
        VScolor::from(&VScolorHSV { hue, saturation, value, alpha: 1.0})
    }

    pub fn saturated ( &self ) -> Self {
        VScolor {
            red: saturate(self.red),
            green: saturate(self.green),
            blue: saturate(self.blue),
            alpha: saturate(self.alpha),
        }
    }

    pub fn get_hue ( &self ) -> f32 {
        VScolorHSV::from(self).hue
    }

    pub fn get_saturation ( &self ) -> f32 {
        VScolorHSV::from(self).saturation
    }

    pub fn get_value ( &self ) -> f32 {
        VScolorHSV::from(self).value
    }

    pub fn magnitude ( &self ) -> f32 {
        ((self.red * self.red) + (self.green * self.green) +
        (self.blue * self.blue) + (self.alpha * self.alpha)).sqrt()
    }

    pub fn saturate ( &mut self ) {
        self.red = saturate(self.red);
        self.green = saturate(self.green);
        self.blue = saturate(self.blue);
        self.alpha = saturate(self.alpha);
    }

    pub fn interpolate ( &self, other: VScolor, progress: f32 ) -> VScolor {
        (*self * (1.0 - progress)) + (other * progress)
    }

    pub fn interpolate_hsv ( &self, other: &VScolor, progress: f32 ) -> VScolor {
        // assert!(!progress.is_nan());  // covered in interpolate
        let s_hsv = VScolorHSV::from(self);
        let o_hsv = VScolorHSV::from(other);
        let hue = {
            let mut hue_self = s_hsv.hue;
            let mut hue_other = o_hsv.hue;

            if (hue_other - hue_self).abs() >= 0.5 {
                if hue_self > hue_other { hue_self -= 1.0; } else { hue_other -= 1.0 }
            }

            interpolate_float(hue_self, hue_other, progress)
        };
        let saturation = interpolate_float(s_hsv.saturation, o_hsv.saturation, progress);
        let value = interpolate_float(s_hsv.value, o_hsv.value, progress);
        VScolor::from_hsv(hue, saturation, value)
    }
}

impl From<u32> for VScolor {
    fn from(value: u32) -> Self {
        let bytes = value.to_be_bytes();
        VScolor::from_bytes(bytes[0], bytes[1], bytes[2], bytes[3])
    }
}

impl From<&VScolor> for u32 {
    fn from(value: &VScolor) -> Self {
        u32::from_be_bytes([
            (value.red * U8_MAX_FLOAT) as u8,
            (value.green * U8_MAX_FLOAT) as u8,
            (value.blue * U8_MAX_FLOAT) as u8,
            (value.alpha * U8_MAX_FLOAT) as u8,
        ])
    }
}

impl From<u64> for VScolor {
    fn from(value: u64) -> Self {
        let bytes = value.to_be_bytes();
        VScolor {
            red: f32::from(read_be_u16(&mut &bytes[0..2]))/U16_MAX_FLOAT,
            green: f32::from(read_be_u16(&mut &bytes[2..4]))/U16_MAX_FLOAT,
            blue: f32::from(read_be_u16(&mut &bytes[4..6]))/U16_MAX_FLOAT,
            alpha: f32::from(read_be_u16(&mut &bytes[6..8]))/U16_MAX_FLOAT,
        }
    }
}

impl From<&VScolor> for u64 {
    fn from(value: &VScolor) -> Self {
        let red = ((value.red * U16_MAX_FLOAT) as u16).to_be_bytes();
        let green = ((value.green * U16_MAX_FLOAT) as u16).to_be_bytes();
        let blue = ((value.blue * U16_MAX_FLOAT) as u16).to_be_bytes();
        let alpha = ((value.alpha * U16_MAX_FLOAT) as u16).to_be_bytes();
        u64::from_be_bytes([red[0], red[1], green[0], green[1], blue[0], blue[1], alpha[0], alpha[1]])
    }
}

impl From<&VScolorPacked> for VScolor {
    fn from(value: &VScolorPacked) -> Self {
        VScolor::from_bytes(value.red, value.green, value.blue, value.alpha)
    }
}

impl From<&VScolorHSV> for VScolor {
    fn from(value: &VScolorHSV) -> Self {
        assert!(value.value >= 0.0 && value.value <= 1.0);
        assert!(value.saturation >= 0.0 && value.saturation <= 1.0);
        if value.saturation <= 0.0 {
            VScolor { red: value.value, green: value.value, blue: value.value, alpha: value.alpha }
        } else {
            let hh = (wrap_hue(value.hue) * 360.0)/60.0; // 0.0 -> <6.0
            let hex_section = hh.floor();
            let hsv_to_rgb = HSVtoRGB { value: value.value, saturation: value.saturation, ff: hh - hex_section};
            match hex_section as u8 {
                0 => VScolor { red: value.value, green: hsv_to_rgb.t(), blue: hsv_to_rgb.p(), alpha: value.alpha },
                1 => VScolor { red: hsv_to_rgb.q(), green: value.value, blue: hsv_to_rgb.p(), alpha: value.alpha },
                2 => VScolor { red: hsv_to_rgb.p(), green: value.value, blue: hsv_to_rgb.t(), alpha: value.alpha },
                3 => VScolor { red: hsv_to_rgb.p(), green: hsv_to_rgb.q(), blue: value.value, alpha: value.alpha },
                4 => VScolor { red: hsv_to_rgb.t(), green: hsv_to_rgb.p(), blue: value.value, alpha: value.alpha },
                5 => VScolor { red: value.value, green: hsv_to_rgb.p(), blue: hsv_to_rgb.q(), alpha: value.alpha },
                _ => unreachable!("impossible hue in hsv color conversion")
            }
        }
    }
}

// Autogenerated? Should verify
// impl Mul<f32> for VScolor {
//     type Output = VScolor;
//     fn mul(self, rhs: f32) -> Self::Output {
//         VScolor { red: self.red*rhs, green: self.green*rhs, blue: self.blue*rhs, alpha:self.alpha*rhs }
//     }
// }


//-------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VScolorPacked {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Default for VScolorPacked {
    fn default ( ) -> Self {
        VScolorPacked { red: 0, green: 0, blue: 0, alpha: u8::MAX }
    }
}

impl VScolorPacked {
    pub fn new ( red: u8, green: u8, blue: u8, alpha: u8 ) -> Self {
        VScolorPacked { red, green, blue, alpha }
    }

    pub fn set ( &mut self, red: f32, green: f32, blue: f32, alpha: f32 ) {
        self.red = (red * U8_MAX_FLOAT) as u8;
        self.green = (green * U8_MAX_FLOAT) as u8;
        self.blue = (blue * U8_MAX_FLOAT) as u8;
        self.alpha = (alpha * U8_MAX_FLOAT) as u8;
    }
}


//-------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VScolorHSV {
    hue: f32,
    saturation: f32,  // technically chroma, not saturation
    value: f32,
    alpha: f32,
}

impl Default for VScolorHSV {
    fn default() -> Self {
        VScolorHSV { hue: 0.0, saturation: 0.0, value: 0.0, alpha: 0.0 }
    }
}

impl VScolorHSV {
    pub fn new ( hue: f32, saturation: f32, value: f32, alpha: f32 ) -> Self {
        assert!(!hue.is_nan());
        assert!(!saturation.is_nan());
        assert!(!value.is_nan());
        assert!(!alpha.is_nan());
        VScolorHSV { hue, saturation, value, alpha }
    }
}

impl From<&VScolor> for VScolorHSV {
    fn from(value: &VScolor) -> Self {
        let big_m = value.red.max(value.green).max(value.blue);
        let small_m = value.red.min(value.green).min(value.blue);
        let delta_m = big_m - small_m;

        if delta_m < 0.00001 {
            VScolorHSV { hue: 0.0, saturation: 0.0, value: big_m, alpha: value.alpha }
        } else if big_m > 0.0 {
            // always true unless some of the color components are -ve
            let h = match big_m {
                m if value.red == m => (value.green - value.blue) / delta_m,
                m if value.green == m => 2.0 + (value.blue - value.red) / delta_m,
                m if value.red == m => 4.0 + (value.red - value.green) / delta_m,
                _ => unreachable!(),
            } * 60.0;
            let hue = (if h < 0.0 { h + 360.0 } else { h }) / 360.0;
            VScolorHSV { hue, saturation: delta_m / big_m, value: big_m, alpha: value.alpha }
        } else {
            VScolorHSV { hue: 0.0, saturation: 0.0, value: big_m, alpha: value.alpha }
        }
    }
}


//-------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VScolorHSL {
    hue: f32,
    saturation: f32,  // technically chroma, not saturation
    lightness: f32,
    alpha: f32,
}

impl Default for VScolorHSL {
    fn default() -> Self {
        VScolorHSL { hue: 0.0, saturation: 0.0, lightness: 0.0, alpha: 0.0 }
    }
}

impl VScolorHSL {
    pub fn new ( hue: f32, saturation: f32, lightness: f32, alpha: f32 ) -> Self {
        assert!(!hue.is_nan());
        assert!(!saturation.is_nan());
        assert!(!lightness.is_nan());
        assert!(!alpha.is_nan());
        VScolorHSL { hue, saturation, lightness, alpha }
    }
}

impl From<&VScolor> for VScolorHSL {
    fn from(value: &VScolor) -> Self {
        let big_m = value.red.max(value.green).max(value.blue);
        let small_m = value.red.min(value.green).min(value.blue);
        let lightness = (big_m + small_m) * 0.5;
        let delta_m = big_m - small_m;

        if delta_m < 0.00001 {
            VScolorHSL { hue: 0.0, saturation: 0.0, lightness, alpha: value.alpha }
        } else if big_m > 0.0 {
            // always true unless some of the color components are -ve
            let h = match big_m {
                m if value.red == m => (value.green - value.blue) / delta_m,
                m if value.green == m => 2.0 + (value.blue - value.red) / delta_m,
                m if value.red == m => 4.0 + (value.red - value.green) / delta_m,
                _ => unreachable!(),
            } * 60.0;
            let hue = (if h < 0.0 { h + 360.0 } else { h }) / 360.0;
            let saturation = delta_m / (1.0 - (2.0 * lightness - 1.0).abs());
            VScolorHSL { hue, saturation, lightness, alpha: value.alpha }
        } else {
            VScolorHSL { hue: 0.0, saturation: 0.0, lightness, alpha: value.alpha }
        }
    }
}


//-------------------------------
const U8_MAX_FLOAT: f32 = u8::MAX as f32;
const U16_MAX_FLOAT: f32 = u16::MAX as f32;

fn wrap_hue ( h: f32 ) -> f32 {
    let h = h - h.floor();
    if h >= 1.0 { 0.0 } else { h }
}

fn read_be_u16(input: &mut &[u8]) -> u16 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
    *input = rest;
    u16::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn saturation_to_chroma ( saturation: f32, lightness: f32 ) -> f32 {
    assert!(!saturation.is_nan());
    assert!(!lightness.is_nan());
    // TODO: should we verify lightness is 0..1?
    if lightness == 1.0 {
        0.0
    } else {
        let max_saturation = 1.0 - (2.0 * lightness - 1.0).abs();
        let chroma = saturation / max_saturation;
        chroma.clamp(0.0, 1.0)
    }
}

pub fn chroma_to_saturation ( chroma: f32, lightness: f32 ) -> f32 {
    assert!(!chroma.is_nan());
    assert!(!lightness.is_nan());
    // TODO: should we verify lightness is 0..1?
    if lightness == 1.0 {
        0.0
    } else {
        let max_saturation = 1.0 - (2.0 * lightness - 1.0).abs();

        chroma / max_saturation   // TODO: does this need .clamp(0.0, 1.0)
    }
}

struct HSVtoRGB {
    value: f32,
    saturation: f32,
    ff: f32,
}
impl HSVtoRGB {
    fn p(& self ) -> f32 {
        self.value * (1.0 - self.saturation)
    }
    fn q ( &self ) -> f32 {
        self.value * (1.0 - (self.saturation * self.ff))
    }
    fn t ( &self ) -> f32 {
        self.value * (1.0 - (self.saturation * (1.0 - self.ff)))
    }
}


#[cfg(test)]
mod tests {
    use crate::VScolor;


    #[test]
    fn u32_to_color () {
        let u32: u32 = 0x0345f300;
        let color = VScolor::from(u32);
        assert!(color.red < color.green);
        assert!(color.green < color.blue);
        assert!(color.alpha < color.red);
        assert!(color.alpha == 0.0);
        let back_u32 = u32::from(&color);
        assert_eq!(u32, back_u32);
    }

    #[test]
    fn u64_to_color () {
        let u64: u64 = 0x00334050f0300000;
        let color = VScolor::from(u64);
        assert!(color.red < color.green);
        assert!(color.green < color.blue);
        assert!(color.alpha < color.red);
        assert!(color.alpha == 0.0);
        let back_u64 = u64::from(&color);
        assert_eq!(u64, back_u64);
    }
}