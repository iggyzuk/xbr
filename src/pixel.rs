/// Pixel in format: RRGGBBAA
///
/// Has functions to get components as u8 and f32.
pub trait Pixel {
    fn red_f32(&self) -> f32;
    fn blue_f32(&self) -> f32;
    fn green_f32(&self) -> f32;
    fn alpha_f32(&self) -> f32;
    fn red_u8(&self) -> u8;
    fn green_u8(&self) -> u8;
    fn blue_u8(&self) -> u8;
    fn alpha_u8(&self) -> u8;
}

pub fn color_f32_to_u32(r: f32, g: f32, b: f32, a: f32) -> u32 {
    color_u8_to_u32(r as u8, g as u8, b as u8, a as u8)
}

#[rustfmt::skip]
pub fn color_u8_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32) << 24 | 
    (g as u32) << 16 | 
    (b as u32) << 8 | 
    (a as u32) << 0
}

// pub fn color_u32_to_u8(color: u32) -> [u8; 4] {
//     let r = ((color & REDMASK) >> 24 & 0xFF) as u8;
//     let g = ((color & GREENMASK) >> 16 & 0xFF) as u8;
//     let b = ((color & BLUEMASK) >> 8 & 0xFF) as u8;
//     let a = ((color & ALPHAMASK) >> 0 & 0xFF) as u8;
//     [r, g, b, a]
// }

impl Pixel for u32 {
    fn red_f32(&self) -> f32 {
        self.red_u8() as f32
    }
    fn green_f32(&self) -> f32 {
        self.green_u8() as f32
    }
    fn blue_f32(&self) -> f32 {
        self.blue_u8() as f32
    }
    fn alpha_f32(&self) -> f32 {
        self.alpha_u8() as f32
    }
    fn red_u8(&self) -> u8 {
        (self >> 24) as u8
    }
    fn green_u8(&self) -> u8 {
        (self >> 16) as u8
    }
    fn blue_u8(&self) -> u8 {
        (self >> 8) as u8
    }

    fn alpha_u8(&self) -> u8 {
        (self >> 0) as u8
    }
}

/// Calculates the weighted difference between two pixels.
///
/// These are the steps:
///
/// 1. Finds absolute color diference between two pixels.
/// 2. Converts color difference into Y'UV, seperating color from light.
/// 3. Applies Y'UV thresholds, giving importance to luminance.
pub fn diff<T: Pixel>(pixel_a: T, pixel_b: T) -> f32 {
    // Weights should emphasize luminance (Y), in order to work. Feel free to experiment.
    const Y_WEIGHT: f32 = 48.0;
    const U_WEIGHT: f32 = 7.0;
    const V_WEIGHT: f32 = 6.0;

    let r = (pixel_a.red_f32() - pixel_b.red_f32()).abs();
    let b = (pixel_a.blue_f32() - pixel_b.blue_f32()).abs();
    let g = (pixel_a.green_f32() - pixel_b.green_f32()).abs();
    let y = r * 0.299000 + g * 0.587000 + b * 0.114000;
    let u = r * -0.168736 + g * -0.331264 + b * 0.500000;
    let v = r * 0.500000 + g * -0.418688 + b * -0.081312;
    let weight = (y * Y_WEIGHT) + (u * U_WEIGHT) + (v * V_WEIGHT);
    weight
}

/// Blends two pixels together and retuns an new Pixel.
pub fn blend<T: Pixel>(pixel_a: T, pixel_b: T, alpha: f32) -> u32 {
    let reverse_alpha = 1.0 - alpha;

    color_f32_to_u32(
        (alpha * pixel_b.red_f32()) + (reverse_alpha * pixel_a.red_f32()),
        (alpha * pixel_b.green_f32()) + (reverse_alpha * pixel_a.green_f32()),
        (alpha * pixel_b.blue_f32()) + (reverse_alpha * pixel_a.blue_f32()),
        pixel_b.alpha_f32().min(pixel_a.alpha_f32()), // fix: alpha is wrong!
    )
}

pub fn blend_exp<T: Pixel>(pixel_a: T, pixel_b: T, alpha: f32, alpha2: f32) -> u32 {
    color_f32_to_u32(
        (alpha * pixel_b.red_f32()) + (alpha2 * pixel_a.red_f32()),
        (alpha * pixel_b.green_f32()) + (alpha2 * pixel_a.green_f32()),
        (alpha * pixel_b.blue_f32()) + (alpha2 * pixel_a.blue_f32()),
        pixel_b.alpha_f32().min(pixel_a.alpha_f32()), // fix: alpha is wrong!
    )
}