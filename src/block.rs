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
  pub fn to_rgba(&self) -> Vec<u32> {
    self
      .bytes
      .chunks_exact(4)
      .map(|c| u32::from_be_bytes([c[0], c[1], c[2], c[3]]))
      .collect()
  }

  /// Flattens 4 byte integers into continuous 4 bytes.
  ///
  /// `[int(bytebytebytebyte), ...] => [byte, byte, byte, byte, ...]`
  pub fn flatten_rgba(colors: Vec<u32>) -> Vec<u8> {
    colors.into_iter().flat_map(u32::to_be_bytes).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn to_rgba_packs_bytes_with_red_first() {
    let block = Block::new(vec![0x11, 0x22, 0x33, 0x44, 0xAA, 0xBB, 0xCC, 0xDD], 2, 1);
    assert_eq!(block.to_rgba(), [0x11223344, 0xAABBCCDD]);
  }

  #[test]
  fn flatten_rgba_unpacks_colors_with_red_first() {
    assert_eq!(
      Block::flatten_rgba(vec![0x11223344, 0xAABBCCDD]),
      [0x11, 0x22, 0x33, 0x44, 0xAA, 0xBB, 0xCC, 0xDD]
    );
  }

  #[test]
  fn from_rgba_round_trips() {
    let colors = vec![0x01020304, 0xFFFEFDFC, 0x00000000, 0xFFFFFFFF];
    let block = Block::from_rgba(colors.clone(), 2, 2);
    assert_eq!(block.width, 2);
    assert_eq!(block.height, 2);
    assert_eq!(block.bytes.len(), 16);
    assert_eq!(block.to_rgba(), colors);
  }
}
