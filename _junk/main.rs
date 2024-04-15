mod error;
mod pixel;
mod xbr;

// #[cfg(feature = "image")]
mod image;

use crate::pixel::*;

fn main() {
  // Parse command line arguments.
  let args: Vec<String> = std::env::args().collect();

  // Find the path of the exe.
  let path = std::env::current_exe().unwrap();

  let mut input_file = path.clone();
  let mut output_file = path.clone();

  if let Ok(contents) = std::fs::read_to_string(&path) {
    println!("{:?}", contents);
  }

  if args.len() == 1 {
    input_file.set_file_name("in.png");
    output_file.set_file_name("out.png");
    println!("Using defaults: in.png -> out.png")
  } else if args.len() == 3 {
    input_file = args[1].to_string().into();
    output_file = args[2].to_string().into();
  } else {
    panic!("Usage: xbr <input_file> <output_file>");
  }

  // Load image.
  let (img, info) = image::bytes_from_path(input_file.as_path()).expect("Could not load input image");

  let input: Vec<u32> = match info.color_type {
    png::ColorType::Rgb => (0..(info.width * info.height) as usize)
      .map(|i| color_u8_to_u32(img[i * 3 + 0], img[i * 3 + 1], img[i * 3 + 2], 255))
      .collect(),
    png::ColorType::Rgba => (0..(info.width * info.height) as usize)
      .map(|i| {
        color_u8_to_u32(
          img[i * 4 + 0],
          img[i * 4 + 1],
          img[i * 4 + 2],
          img[i * 4 + 3],
        )
      })
      .collect(),
    _ => panic!("Image format not implemented!"),
  };

  // Apply XBR.
  let (mut out_buf, out_width, out_height) = get_buffer_for_size(info.width, info.height);
  xbr::apply_x2(&mut out_buf[..], &input, info.width, info.height);

  // Save image.
  image::save_img(output_file.as_path(), out_width, out_height, &out_buf[..])
    .expect("Could not save output image");
}

fn get_buffer_for_size(width: u32, height: u32) -> (Vec<u32>, u32, u32) {
  (
    vec![0; (width as usize) * 2 * (height as usize) * 2],
    width * 2,
    height * 2,
  )
}
