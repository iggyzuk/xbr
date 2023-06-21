mod image;
mod pixel;
mod xbr;

use crate::pixel::*;
use std::path::Path;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: xbr <input_file> <output_file>");
        return;
    }
    let input_file = Path::new(&args[1]);
    let output_file = Path::new(&args[2]);

    // Load image
    let (img, info) = image::load_img(input_file).expect("Could not load input image");

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
        _ => unimplemented!(),
    };

    // Apply XBR
    let (mut out_buf, out_width, out_height) = get_buffer_for_size(info.width, info.height);
    xbr::apply(&mut out_buf[..], &input, info.width, info.height);

    // Save image
    image::save_img(output_file, out_width, out_height, &out_buf[..])
        .expect("Could not save output image");
}

fn get_buffer_for_size(width: u32, height: u32) -> (Vec<u32>, u32, u32) {
    (
        vec![0; (width as usize) * 2 * (height as usize) * 2],
        width * 2,
        height * 2,
    )
}
