use image::{DynamicImage, ImageFormat};
use std::io::{BufRead, Seek};

pub enum Format {
  Bmp,
  Png,
  Jpeg,
}

#[allow(clippy::from_over_into)]
impl Into<ImageFormat> for Format {
  fn into(self) -> ImageFormat {
    match self {
      Format::Bmp => ImageFormat::Bmp,
      Format::Png => ImageFormat::Png,
      Format::Jpeg => ImageFormat::Jpeg,
    }
  }
}

/// same as [image::load], but checks for supported features
pub fn load_from_reader<R: BufRead + Seek>(
  reader: R,
  format: Format,
) -> image::ImageResult<DynamicImage> {
  #[cfg(not(any(feature = "bmp", feature = "png", feature = "jpeg")))]
  compile_error!(r#"must have a least one format feature enabled: "bmp", "png", or "jpeg""#);

  #[cfg(any(feature = "bmp", feature = "png", feature = "jpeg"))]
  image::load(reader, format.into())
}

/// same as [image::load_from_memory_with_format], but checks for supported features
pub fn load_from_bytes(bytes: &[u8], format: Format) -> image::ImageResult<DynamicImage> {
  #[cfg(not(any(feature = "bmp", feature = "png", feature = "jpeg")))]
  compile_error!(r#"must have a least one format feature enabled: "bmp", "png", or "jpeg""#);

  #[cfg(any(feature = "bmp", feature = "png", feature = "jpeg"))]
  image::load_from_memory_with_format(bytes, format.into())
}
