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
    (a as u32)
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
    *self as u8
  }
}

/// Calculates the weighted difference between two `Pixel`s.
///
/// 1. Finds the color difference between two pixels.
/// 2. Converts color difference into Y'UV, separating color from light.
/// 3. Sums the absolute Y'UV differences, giving importance to luminance.
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

  // The sign of each channel must survive until after the conversion, otherwise
  // opposite changes (e.g. red -> green) add up instead of cancelling out in luma.
  let r = pixel_a.red_f32() - pixel_b.red_f32();
  let g = pixel_a.green_f32() - pixel_b.green_f32();
  let b = pixel_a.blue_f32() - pixel_b.blue_f32();

  let yuv = rgb_to_yuv(r, g, b);

  yuv.y.abs() * Y_WEIGHT + yuv.u.abs() * U_WEIGHT + yuv.v.abs() * V_WEIGHT
}

/// A structure for conveniently working with YUV colors.
/// <https://en.wikipedia.org/wiki/YUV>
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

/// Compares `YUV` of two `Pixel`s and depending on thresholds tells us if pixels are considered equal.
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

  true
}

/// Blends two `Pixel`s together and returns the new pixel as `u32`.
///
/// Colors are weighted by their alpha as well as by `q1` and `q2`, so that a
/// transparent pixel's (invisible) color can't bleed into the result.
pub fn blend<T: Pixel>(pixel_a: T, pixel_b: T, q1: f32, q2: f32) -> u32 {
  let weight_a = q1 * pixel_a.alpha_f32();
  let weight_b = q2 * pixel_b.alpha_f32();
  let total = weight_a + weight_b;

  if total == 0.0 {
    return 0;
  }

  color_f32_to_u32(
    (weight_a * pixel_a.red_f32() + weight_b * pixel_b.red_f32()) / total,
    (weight_a * pixel_a.green_f32() + weight_b * pixel_b.green_f32()) / total,
    (weight_a * pixel_a.blue_f32() + weight_b * pixel_b.blue_f32()) / total,
    total / (q1 + q2),
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

#[cfg(test)]
mod tests {
  use super::*;

  const BLACK: u32 = 0x000000FF;
  const WHITE: u32 = 0xFFFFFFFF;
  const RED: u32 = 0xFF0000FF;
  const GREEN: u32 = 0x00FF00FF;
  const BLUE: u32 = 0x0000FFFF;
  const CLEAR: u32 = 0x00000000;

  fn assert_close(actual: f32, expected: f32) {
    assert!(
      (actual - expected).abs() < 0.01,
      "{actual} is not close to {expected}"
    );
  }

  #[test]
  fn reads_components_of_rrggbbaa() {
    let pixel = 0x11223344u32;
    assert_eq!(pixel.red_u8(), 0x11);
    assert_eq!(pixel.green_u8(), 0x22);
    assert_eq!(pixel.blue_u8(), 0x33);
    assert_eq!(pixel.alpha_u8(), 0x44);
    assert_eq!(pixel.red_f32(), 17.0);
    assert_eq!(pixel.green_f32(), 34.0);
    assert_eq!(pixel.blue_f32(), 51.0);
    assert_eq!(pixel.alpha_f32(), 68.0);
  }

  #[test]
  fn builds_rrggbbaa_from_components() {
    assert_eq!(color_u8_to_u32(0x11, 0x22, 0x33, 0x44), 0x11223344);
    assert_eq!(color_f32_to_u32(17.9, 34.0, 51.0, 68.0), 0x11223344);
  }

  #[test]
  fn yuv_of_grey_has_no_chroma() {
    let grey = yuv(0x808080FFu32);
    assert_close(grey.y, 128.0);
    assert_close(grey.u, 0.0);
    assert_close(grey.v, 0.0);
  }

  #[test]
  fn diff_is_zero_for_identical_pixels() {
    assert_eq!(diff(RED, RED), 0.0);
  }

  #[test]
  fn diff_is_symmetric() {
    assert_eq!(diff(RED, 0x123456FF), diff(0x123456FF, RED));
  }

  #[test]
  fn diff_sums_absolute_yuv_differences() {
    // |Y| * 48 + |U| * 7 + |V| * 6, the negative V of blue must add to the total.
    assert_close(
      diff(BLUE, BLACK),
      29.07 * 48.0 + 127.5 * 7.0 + 20.73456 * 6.0,
    );
  }

  #[test]
  fn diff_tells_apart_channels_moving_in_opposite_directions() {
    // Both pairs change red and green by 255, but only yellow vs black changes
    // both in the same direction, which is a far larger change of luminance.
    let red_green = diff(RED, GREEN);
    let yellow_black = diff(0xFFFF00FF, BLACK);
    assert!(
      red_green < yellow_black / 2.0,
      "{red_green} vs {yellow_black}"
    );
  }

  #[test]
  fn diff_ignores_color_of_transparent_pixels() {
    assert_eq!(diff(0xFF000000u32, 0x00FF0000), 0.0);
  }

  #[test]
  fn diff_is_huge_between_transparent_and_visible() {
    assert_eq!(diff(CLEAR, BLACK), 1_000_000.0);
    assert_eq!(diff(BLACK, CLEAR), 1_000_000.0);
  }

  #[test]
  fn is_equal_applies_thresholds_per_channel() {
    assert!(is_equal(RED, RED));
    assert!(!is_equal(BLACK, WHITE)); // Y
    assert!(!is_equal(0x808000FF, 0x8080FFFF)); // U
    assert!(is_equal(0x808080FF, 0x828282FF)); // Almost the same grey.
  }

  #[test]
  fn is_equal_treats_transparent_pixels_as_one_color() {
    assert!(is_equal(0xFF000000u32, 0x00FF0000));
    assert!(!is_equal(CLEAR, BLACK));
    assert!(!is_equal(BLACK, CLEAR));
  }

  #[test]
  fn blend_applies_weights() {
    assert_eq!(blend_64w(WHITE, BLACK), 0xBFBFBFFF);
    assert_eq!(blend_128w(WHITE, BLACK), 0x7F7F7FFF);
    assert_eq!(blend_192w(WHITE, BLACK), 0x3F3F3FFF);
  }

  #[test]
  fn blend_of_a_color_with_itself_is_that_color() {
    for v in 0..=255 {
      let pixel = color_u8_to_u32(v, v, v, 255);
      assert_eq!(blend_64w(pixel, pixel), pixel);
      assert_eq!(blend_128w(pixel, pixel), pixel);
      assert_eq!(blend_192w(pixel, pixel), pixel);
    }
  }

  #[test]
  fn blend_with_transparent_only_fades_alpha() {
    assert_eq!(blend_128w(RED, CLEAR), 0xFF00007F);
    assert_eq!(blend_128w(CLEAR, RED), 0xFF00007F);
    assert_eq!(blend_192w(CLEAR, RED), 0xFF0000BF);
    // Whatever color the transparent pixel holds stays invisible.
    assert_eq!(blend_128w(RED, 0x00FF0000), 0xFF00007F);
  }

  #[test]
  fn blend_weights_colors_by_alpha() {
    // Red is 3 times as opaque as blue, so it has 3 times as much say.
    assert_eq!(blend_128w(0xFF0000C0u32, 0x0000FF40), 0xBF003F80);
  }

  #[test]
  fn blend_of_two_transparent_pixels_is_transparent() {
    assert_eq!(blend_128w(CLEAR, CLEAR), CLEAR);
    assert_eq!(blend_128w(0x12345600u32, 0x65432100), CLEAR);
  }

  #[test]
  fn left2_and_up2_blend_both_outputs_towards_pixel() {
    assert_eq!(left2(WHITE, WHITE, BLACK), [0x3F3F3FFF, 0xBFBFBFFF]);
    assert_eq!(up2(WHITE, WHITE, BLACK), [0x3F3F3FFF, 0xBFBFBFFF]);
  }

  #[test]
  fn dia_blends_evenly() {
    assert_eq!(dia(WHITE, BLACK), 0x7F7F7FFF);
  }
}
