pub mod color;
pub mod image;

use crate::color::{ColorMapper, TrueColorMapper};
use crate::image::{load_from_bytes, Format};
use ::image::GenericImageView;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{}")]
pub enum Error {
  #[error("{0}")]
  Image(#[from] ::image::ImageError),
}

pub fn render(image: &[u8], format: Format) {
  let image = load_from_bytes(image, format).unwrap();
  let mapper = TrueColorMapper;

  let mut nl = 0;

  for (x, y, color) in image.pixels() {
    let color = color.0;
    let mapped = mapper.map(color);

    if y > nl {
      println!();
      nl += 1;
    }

    if color[3] == 0 {
      print!("  ");
    } else {
      print!("{}##", mapped);
    }
  }
}
