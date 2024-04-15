/// Block of bytes that we use inside of xbr algorithm.
/// This block of bytes encapsules a bit of meta-data (format, width, height)
pub struct Block {
  pub bytes: Vec<u8>,
  pub format: Format,
  pub width: u32,
  pub height: u32,
}

pub enum Format {
  RGB,
  RGBA, // todo
}

impl Block {
  pub fn new(bytes: Vec<u8>, width: u32, height: u32) -> Self {
    Self {
      bytes,
      format: Format::RGB,
      width,
      height,
    }
  }

  pub fn from_colors(colors: Vec<u32>, width: u32, height: u32) -> Self {
    Self {
      bytes: Block::flatten(colors),
      format: Format::RGB,
      width,
      height,
    }
  }

  /// Returns bytes as colors taking into account the format (todo).
  pub fn colors(&self) -> Vec<u32> {
    self
      .bytes
      .chunks(3)
      .map(|c| {
        let r = (c[0] as u32) << 16;
        let g = (c[1] as u32) << 8;
        let b = c[2] as u32;
        (r | g | b) as u32
      })
      .collect()
  }

  // Flattens 4 byte integers into 3 or 4 bytes depending on the format (todo: it's 3 for now: RGB)
  // e.g. [int(bytebytebytebyte), ...] => [byte, byte, byte, byte, ...]
  pub fn flatten(colors: Vec<u32>) -> Vec<u8> {
    let mut result = Vec::new();
    for color in colors {
      // todo: careful with formats
      for i in (0..3).rev() {
        result.push(((color >> (i * 8)) & 0xFF) as u8);
      }
    }
    result
  }
}
