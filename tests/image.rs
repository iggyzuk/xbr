//! Loading and saving through the optional `image` feature.
#![cfg(feature = "image")]

use std::path::{Path, PathBuf};

use xbr::{image, x2, Block};

struct TempFile(PathBuf);

impl TempFile {
  fn new(name: &str) -> Self {
    let name = format!("xbr-test-{}-{name}", std::process::id());
    Self(std::env::temp_dir().join(name))
  }
}

impl Drop for TempFile {
  fn drop(&mut self) {
    let _ = std::fs::remove_file(&self.0);
  }
}

#[test]
fn save_then_load_round_trips() {
  let file = TempFile::new("round-trip.png");
  let colors = vec![
    0xFF0000FF, 0x00FF0080, 0x0000FF00, 0x12345678, 0, 0xFFFFFFFF,
  ];
  let bytes = Block::from_rgba(colors, 3, 2).bytes;

  image::save(Block::new(bytes.clone(), 3, 2), &file.0).unwrap();
  let loaded = image::from_path(&file.0).unwrap();

  assert_eq!(loaded.width, 3);
  assert_eq!(loaded.height, 2);
  assert_eq!(loaded.bytes, bytes);
}

#[test]
fn loading_a_missing_file_is_an_error() {
  assert!(image::from_path(Path::new("does/not/exist.png")).is_err());
}

#[test]
fn saving_to_an_unknown_format_is_an_error() {
  let file = TempFile::new("unknown.format");
  assert!(image::save(Block::new(vec![0; 4], 1, 1), &file.0).is_err());
}

#[test]
fn example_assets_upscale_without_losing_opacity() {
  for name in ["input1.png", "input2.png", "input3.png"] {
    let path = Path::new("examples/assets").join(name);
    let block = image::from_path(&path).unwrap();
    assert!(block.bytes.chunks(4).all(|c| c[3] == 255));

    let (width, height) = (block.width, block.height);
    let upscaled = x2(block);

    assert_eq!((upscaled.width, upscaled.height), (width * 2, height * 2));
    assert!(upscaled.bytes.chunks(4).all(|c| c[3] == 255), "{name}");
  }
}
