use img_2_anscii_core::image::Format;
use img_2_anscii_core::render;

fn main() {
  /// Just for testing
  const IMAGE_SHAPE: &[u8] = include_bytes!("../../test/rainbow_shape.png");

  render(IMAGE_SHAPE, Format::Png);
}
