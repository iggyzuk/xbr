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

/// Calculates the weighted difference between two `Pixel`s.
///
///
/// 1. Finds absolute color diference between two pixels.
/// 2. Converts color difference into Y'UV, seperating color from light.
/// 3. Applies Y'UV thresholds, giving importance to luminance.
pub fn diff<T: Pixel>(pixel_a: T, pixel_b: T) -> f32 {
  let alpha_a = pixel_a.alpha_u8();
  let alpha_b = pixel_b.alpha_u8();

  if alpha_a == 0 && alpha_b == 0 {
    return 0.0;
  }

  if alpha_a == 0 || alpha_b == 0 {
    return 1_000_000.0;
  }

  // Weights should emphasize luminance (Y), in order to work. Feel free to experiment.
  const Y_WEIGHT: f32 = 48.0;
  const U_WEIGHT: f32 = 7.0;
  const V_WEIGHT: f32 = 6.0;

  let r = (pixel_a.red_f32() - pixel_b.red_f32()).abs();
  let b = (pixel_a.blue_f32() - pixel_b.blue_f32()).abs();
  let g = (pixel_a.green_f32() - pixel_b.green_f32()).abs();

  let yuv = rgb_to_yuv(r, g, b);

  let weight = (yuv.y * Y_WEIGHT) + (yuv.u * U_WEIGHT) + (yuv.v * V_WEIGHT);
  weight
}

/// A structure for conveniently working with YUV colors.
/// https://en.wikipedia.org/wiki/YUV
pub struct Yuv {
  pub y: f32,
  pub u: f32,
  pub v: f32,
}

/// Converts `Pixel` from `RGB` to `YUV` color space.
pub fn yuv<T: Pixel>(pixel: T) -> Yuv {
  rgb_to_yuv(pixel.red_f32(), pixel.green_f32(), pixel.blue_f32())
}

/// Converts `RGB` to `YUV` color space.
pub fn rgb_to_yuv(r: f32, g: f32, b: f32) -> Yuv {
  let y = r * 0.299000 + g * 0.587000 + b * 0.114000;
  let u = r * -0.168736 + g * -0.331264 + b * 0.500000;
  let v = r * 0.500000 + g * -0.418688 + b * -0.081312;
  Yuv { y, u, v }
}

/// Compares `LUV` of two `Pixel`s and depending on thresholds tells us if pixels are considered equal.
pub fn is_equal<T: Pixel>(pixel_a: T, pixel_b: T) -> bool {
  const THRESHOLD_Y: f32 = 48.0;
  const THRESHOLD_U: f32 = 7.0;
  const THRESHOLD_V: f32 = 6.0;

  let alpha_a = pixel_a.alpha_u8();
  let alpha_b = pixel_b.alpha_u8();

  if alpha_a == 0 && alpha_b == 0 {
    return true;
  }

  if alpha_a == 0 || alpha_b == 0 {
    return false;
  }

  let yuv_a = yuv(pixel_a);
  let yuv_b = yuv(pixel_b);

  if (yuv_a.y - yuv_b.y).abs() > THRESHOLD_Y {
    return false;
  }
  if (yuv_a.u - yuv_b.u).abs() > THRESHOLD_U {
    return false;
  }
  if (yuv_a.v - yuv_b.v).abs() > THRESHOLD_V {
    return false;
  }

  return true;
}

/// Blends two `Pixel`s together and retuns the new pixel as `u32`.
pub fn blend<T: Pixel>(pixel_a: T, pixel_b: T, q1: f32, q2: f32) -> u32 {
  let dist = q1 + q2;
  let one_over_dist = 1.0 / dist;

  color_f32_to_u32(
    (q1 * pixel_a.red_f32() + q2 * pixel_b.red_f32()) * one_over_dist,
    (q1 * pixel_a.green_f32() + q2 * pixel_b.green_f32()) * one_over_dist,
    (q1 * pixel_a.blue_f32() + q2 * pixel_b.blue_f32()) * one_over_dist,
    (q1 * pixel_a.alpha_f32() + q2 * pixel_b.alpha_f32()) * one_over_dist,
  )
}

// Weights
// 32W 7:1
// 64W 3:1
// 128W 1:1
// 192W 1:3
// 224W 1:7
pub fn blend_64w<T: Pixel>(dst: T, src: T) -> u32 {
  blend(dst, src, 3.0, 1.0)
}

pub fn blend_128w<T: Pixel>(dst: T, src: T) -> u32 {
  blend(dst, src, 1.0, 1.0)
}

pub fn blend_192w<T: Pixel>(dst: T, src: T) -> u32 {
  blend(dst, src, 1.0, 3.0)
}

pub fn left2<T: Pixel + Copy>(n3: T, n2: T, pixel: T) -> [u32; 2] {
  [blend_192w(n3, pixel), blend_64w(n2, pixel)]
}

pub fn up2<T: Pixel + Copy>(n3: T, n1: T, pixel: T) -> [u32; 2] {
  [blend_192w(n3, pixel), blend_64w(n1, pixel)]
}

pub fn dia<T: Pixel + Copy>(n3: T, pixel: T) -> u32 {
  blend_128w(n3, pixel)
}
