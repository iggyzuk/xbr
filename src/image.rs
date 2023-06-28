use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

/// Loads an image from `Path` into a `Vec<u8>` (vector of bytes).
pub fn load_img(path: &Path) -> Result<(Vec<u8>, png::OutputInfo), std::io::Error> {
  // Open file and create PNG decoder for it.
  let file = File::open(Path::new(path))?;
  let ref mut reader = BufReader::new(file);
  let decoder = png::Decoder::new(reader);

  // Read it.
  let mut reader = decoder.read_info()?;
  // Allocate the output buffer.
  let mut buf = vec![0; reader.output_buffer_size()];
  // Read the next frame. An APNG might contain multiple frames.
  let info = reader.next_frame(&mut buf)?;

  Ok((buf, info))
}

/// Saves an image to `Path` given a vector of bytes, width, and height.
pub fn save_img(path: &Path, width: u32, height: u32, data: &[u32]) -> Result<(), std::io::Error> {
  let file = File::create(Path::new(path))?;
  let ref mut writer = BufWriter::new(file);
  let mut encoder = png::Encoder::new(writer, width, height);

  encoder.set_color(png::ColorType::Rgba);
  encoder.set_depth(png::BitDepth::Eight);
  encoder.set_compression(png::Compression::Default);
  encoder.set_filter(png::FilterType::NoFilter);

  let mut writer = encoder.write_header()?;

  writer.write_image_data(&explode_rgba(data)[..])?;

  Ok(())
}

/// Convert solid `u32`s into a bunch of `u8`s
fn explode_rgba(pixel_buffer: &[u32]) -> Vec<u8> {
  (0..pixel_buffer.len() * 4)
    .map(|i| ((pixel_buffer[(i / 4)] >> (8 * (3 - (i % 4)))) & 0xFF) as u8)
    .collect()
}
