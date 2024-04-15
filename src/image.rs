use std::path::Path;

use image::io::Reader as ImageReader;
use image::{ImageBuffer, ImageError, RgbImage};

use crate::Block;

/// Loads an image from path into a [`Block`].
pub fn from_path(path: &Path) -> Result<Block, ImageError> {
  let img = ImageReader::open(path)?.decode()?;

  let width = img.width();
  let height = img.height();
  let bytes = img.into_rgb8().into_vec();

  Ok(Block::new(bytes, width, height))
}

/// Saves a [`Block`] into a specified [`Path`].
pub fn save(block: Block, path: &Path) -> Result<(), ImageError> {
  // Create an ImageBuffer from RGB pixel values.
  let img: RgbImage = ImageBuffer::from_vec(block.width, block.height, block.bytes)
    .expect("failed to create image buffer");

  // Save the image as a PNG file.
  img.save(path)?;

  Ok(())
}
