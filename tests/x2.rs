//! Behavior of the whole filter, through the public API.
use xbr::{x2, Block, Pixel};

const BLACK: u32 = 0x000000FF;
const WHITE: u32 = 0xFFFFFFFF;
const RED: u32 = 0xFF0000FF;
const CLEAR: u32 = 0x00000000;

/// Builds a `size` x `size` image from a function of each pixel's position.
fn image(size: u32, color_at: impl Fn(u32, u32) -> u32) -> Vec<u32> {
  (0..size)
    .flat_map(|y| (0..size).map(move |x| (x, y)))
    .map(|(x, y)| color_at(x, y))
    .collect()
}

fn upscale(colors: Vec<u32>, width: u32, height: u32) -> Vec<u32> {
  let block = x2(Block::from_rgba(colors, width, height));
  assert_eq!(block.width, width * 2);
  assert_eq!(block.height, height * 2);
  block.to_rgba()
}

/// Deterministic noise, every call with the same seed gives the same image.
fn noise(size: u32, seed: u32, to_color: impl Fn(u32) -> u32) -> Vec<u32> {
  let mut state = seed;
  (0..size * size)
    .map(|_| {
      state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
      to_color(state >> 8)
    })
    .collect()
}

/// Grey level of each pixel of a black and white image, as hexadecimal rows.
fn greys(colors: &[u32], width: usize) -> Vec<String> {
  colors
    .chunks(width)
    .map(|row| {
      let cells: Vec<_> = row.iter().map(|c| format!("{:02x}", c.red_u8())).collect();
      cells.join(" ")
    })
    .collect()
}

fn transpose(colors: &[u32], size: usize) -> Vec<u32> {
  (0..size * size)
    .map(|i| colors[(i % size) * size + i / size])
    .collect()
}

#[test]
fn empty_image_stays_empty() {
  assert!(upscale(vec![], 0, 0).is_empty());
}

#[test]
fn single_pixel_is_repeated() {
  assert_eq!(upscale(vec![RED], 1, 1), [RED; 4]);
}

#[test]
fn non_square_images_keep_pixels_in_place() {
  let colors = upscale(vec![RED, RED, RED, RED, RED, RED], 3, 2);
  assert_eq!(colors, [RED; 24]);

  let colors = upscale(vec![RED, BLACK, RED], 3, 1);
  assert_eq!(colors[0], RED);
  assert_eq!(colors[5], RED);
  assert_eq!(colors[3], BLACK);
  assert_eq!(colors[6], RED);
}

#[test]
#[should_panic(expected = "width * height")]
fn panics_when_bytes_do_not_match_dimensions() {
  x2(Block::new(vec![0; 4], 2, 2));
}

#[test]
fn solid_color_is_unchanged() {
  assert_eq!(upscale(vec![RED; 36], 6, 6), [RED; 144]);
}

#[test]
fn straight_edges_stay_sharp() {
  let vertical = upscale(image(6, |x, _| if x < 3 { BLACK } else { WHITE }), 6, 6);
  assert_eq!(
    vertical,
    image(12, |x, _| if x < 6 { BLACK } else { WHITE })
  );

  let horizontal = upscale(image(6, |_, y| if y < 3 { BLACK } else { WHITE }), 6, 6);
  assert_eq!(
    horizontal,
    image(12, |_, y| if y < 6 { BLACK } else { WHITE })
  );
}

#[test]
fn checkerboard_is_softened_evenly() {
  // Both diagonals are equally likely edges, so each corner ties and gets a light blend.
  let board = |x: u32, y: u32| if (x + y) & 1 == 0 { BLACK } else { WHITE };
  let colors = upscale(image(8, board), 8, 8);

  // Skips the border, where the repeated edge pixels change the neighborhood.
  for y in 4..12 {
    for x in 4..12 {
      let expected = if board(x / 2, y / 2) == BLACK {
        0x3F
      } else {
        0xBF
      };
      assert_eq!(colors[y as usize * 16 + x as usize].red_u8(), expected);
    }
  }
}

#[test]
fn diagonal_edge_is_blended_evenly() {
  let colors = upscale(image(6, |x, y| if x + y < 6 { BLACK } else { WHITE }), 6, 6);
  assert_eq!(
    greys(&colors, 12),
    [
      "00 00 00 00 00 00 00 00 00 00 00 00",
      "00 00 00 00 00 00 00 00 00 00 00 00",
      "00 00 00 00 00 00 00 00 00 00 3f bf",
      "00 00 00 00 00 00 00 00 00 7f ff ff",
      "00 00 00 00 00 00 00 00 7f ff ff ff",
      "00 00 00 00 00 00 00 7f ff ff ff ff",
      "00 00 00 00 00 00 7f ff ff ff ff ff",
      "00 00 00 00 00 7f ff ff ff ff ff ff",
      "00 00 00 00 7f ff ff ff ff ff ff ff",
      "00 00 00 7f ff ff ff ff ff ff ff ff",
      "00 00 3f ff ff ff ff ff ff ff ff ff",
      "00 00 bf ff ff ff ff ff ff ff ff ff",
    ]
  );
}

#[test]
fn shallow_edge_is_blended_as_a_ramp() {
  let colors = upscale(
    image(6, |x, y| if 2 * y + x < 8 { BLACK } else { WHITE }),
    6,
    6,
  );
  assert_eq!(
    greys(&colors, 12),
    [
      "00 00 00 00 00 00 00 00 00 00 00 00",
      "00 00 00 00 00 00 00 00 00 00 00 00",
      "00 00 00 00 00 00 00 00 00 00 00 00",
      "00 00 00 00 00 00 00 00 00 00 00 00",
      "00 00 00 00 00 00 00 00 3f bf ff ff",
      "00 00 00 00 00 00 3f bf ff ff ff ff",
      "00 00 00 00 3f bf ff ff ff ff ff ff",
      "00 00 3f bf ff ff ff ff ff ff ff ff",
      "ff ff ff ff ff ff ff ff ff ff ff ff",
      "ff ff ff ff ff ff ff ff ff ff ff ff",
      "ff ff ff ff ff ff ff ff ff ff ff ff",
      "ff ff ff ff ff ff ff ff ff ff ff ff",
    ]
  );
}

#[test]
fn steep_edge_mirrors_shallow_edge() {
  let shallow = upscale(
    image(6, |x, y| if 2 * y + x < 8 { BLACK } else { WHITE }),
    6,
    6,
  );
  let steep = upscale(
    image(6, |x, y| if 2 * x + y < 8 { BLACK } else { WHITE }),
    6,
    6,
  );
  assert_eq!(steep, transpose(&shallow, 12));
}

#[test]
fn edges_are_handled_alike_in_every_direction() {
  // A diamond has a diagonal edge facing each of the four corners.
  let diamond = |x: i32, y: i32| {
    if (x - 8).abs() + (y - 8).abs() <= 5 {
      BLACK
    } else {
      WHITE
    }
  };
  let colors = upscale(image(17, |x, y| diamond(x as i32, y as i32)), 17, 17);
  let flipped_x: Vec<u32> = colors
    .chunks(34)
    .flat_map(|r| r.iter().rev().copied())
    .collect();
  let flipped_y: Vec<u32> = colors.chunks(34).rev().flatten().copied().collect();

  assert_eq!(colors, flipped_x);
  assert_eq!(colors, flipped_y);
  assert_eq!(colors, transpose(&colors, 34));
}

#[test]
fn opaque_images_stay_opaque() {
  // Includes the corners, where most of the neighbors are outside of the image.
  for seed in 0..50 {
    let colors = upscale(noise(12, seed, |n| n | 0xFF), 12, 12);
    assert!(colors.iter().all(|c| c.alpha_u8() == 255), "seed {seed}");
  }
  for seed in 0..50 {
    let colors = upscale(
      noise(12, seed, |n| if n & 1 == 0 { BLACK } else { RED }),
      12,
      12,
    );
    assert!(colors.iter().all(|c| c.alpha_u8() == 255), "seed {seed}");
  }
}

#[test]
fn image_corners_are_not_rounded_off() {
  let colors = upscale(vec![RED, BLACK, BLACK, RED], 2, 2);
  assert_eq!(colors[0], RED);
  assert_eq!(colors[3], BLACK);
  assert_eq!(colors[12], BLACK);
  assert_eq!(colors[15], RED);
}

#[test]
fn transparent_image_stays_transparent() {
  let colors = upscale(vec![CLEAR; 16], 4, 4);
  assert!(colors.iter().all(|c| c.alpha_u8() == 0));
}

#[test]
fn sprite_edges_fade_without_a_dark_fringe() {
  let sprite = image(9, |x, y| {
    if (x as i32 - 4).abs() + (y as i32 - 4).abs() <= 3 {
      RED
    } else {
      CLEAR
    }
  });
  let colors = upscale(sprite, 9, 9);
  let visible = || colors.iter().filter(|c| c.alpha_u8() > 0);

  assert!(visible().any(|c| c.alpha_u8() < 255), "edges should fade");
  assert!(visible().all(|c| c & 0xFFFFFF00 == RED & 0xFFFFFF00));
}

#[test]
fn colors_hidden_in_transparent_pixels_change_nothing() {
  let sprite = |clear: u32| {
    image(9, move |x, y| {
      if (x as i32 - 4).abs() + (y as i32 - 4).abs() <= 3 {
        RED
      } else if (x + y) % 2 == 0 {
        CLEAR
      } else {
        clear
      }
    })
  };
  assert_eq!(
    upscale(sprite(CLEAR), 9, 9),
    upscale(sprite(0x00FF0000), 9, 9)
  );
}

#[test]
fn blends_stay_between_the_input_colors() {
  let colors = upscale(
    noise(16, 7, |n| if n & 1 == 0 { BLACK } else { WHITE }),
    16,
    16,
  );
  assert!(colors.iter().all(|c| {
    let grey = c.red_u8();
    c.green_u8() == grey && c.blue_u8() == grey && c.alpha_u8() == 255
  }));
}
