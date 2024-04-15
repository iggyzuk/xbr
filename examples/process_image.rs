use std::path::Path;

fn main() {
  // Load an image from a path and construct a block of bytes
  // for whatever the format might be (e.g. png) that the image
  // crate thinks it to be.
  let base_path = std::env::current_dir().expect("failed to determine the current directory");
  let path = base_path.join(Path::new("examples/assets/input1.png"));
  let original_block = xbr::image::from_path(&path).expect("should have img.png in /assets");

  // Apply x2 xbr to the block and return a new upscaled block.
  let processed_block = xbr::x2(original_block);

  // Save processed block as image in /examples/assets
  xbr::image::save(
    processed_block,
    &base_path.join("examples/assets/output1.png"),
  )
  .expect("should save");
}
