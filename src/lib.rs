// mods
mod block;
mod error;
mod pixel;
mod xbr;

// features
#[cfg(feature = "image")]
pub mod image;

// rexports
pub use crate::block::Block;
pub use crate::error::XBRError;
pub use crate::pixel::*;
pub use crate::xbr::x2;