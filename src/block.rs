/// Block of bytes that we'll use as input and output of xbr algorithm.
pub struct Block {
  pub bytes: Vec<u8>,
  pub width: u32,
  pub height: u32,
}

impl Block {
  /// Construct a new block from a sequence bytes – must be in RGBA format.
  pub fn new(bytes: Vec<u8>, width: u32, height: u32) -> Self {
    Self {
      bytes,
      width,
      height,
    }
  }

  /// Construct a new block from a sequence of rgba colors.
  pub fn from_rgba(colors: Vec<u32>, width: u32, height: u32) -> Self {
    Self {
      bytes: Block::flatten_rgba(colors),
      width,
      height,
    }
  }

  /// Returns bytes as rgba colors.
  pub fn into_rgba(&self) -> Vec<u32> {
    self
      .bytes
      .chunks(4)
      .map(|c| {
        let r = (c[0] as u32) << 24;
        let g = (c[1] as u32) << 16;
        let b = (c[2] as u32) << 8;
        let a = c[3] as u32;
        (r | g | b | a) as u32
      })
      .collect()
  }

  /// Flattens 4 byte integers into continuous 4 bytes.
  ///
  /// `[int(bytebytebytebyte), ...] => [byte, byte, byte, byte, ...]`
  pub fn flatten_rgba(colors: Vec<u32>) -> Vec<u8> {
    let mut result = Vec::new();
    for color in colors {
      // todo: careful with formats
      for i in (0..4).rev() {
        result.push(((color >> (i * 8)) & 0xFF) as u8);
      }
    }
    result
  }
}
