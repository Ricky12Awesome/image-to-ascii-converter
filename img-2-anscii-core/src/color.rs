use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
  Black,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan,
  White,
  BrightBlack,
  BrightRed,
  BrightGreen,
  BrightYellow,
  BrightBlue,
  BrightMagenta,
  BrightCyan,
  BrightWhite,
  Reset,
  Color256(u8),
  TrueColor(u8, u8, u8),
}

impl Color {
  pub fn fg_fmt<W: fmt::Write>(&self, mut f: W) -> fmt::Result {
    write!(f, "\x1B[")?;

    match self {
      Color::Black => write!(f, "30"),
      Color::Red => write!(f, "31"),
      Color::Green => write!(f, "32"),
      Color::Yellow => write!(f, "33"),
      Color::Blue => write!(f, "34"),
      Color::Magenta => write!(f, "35"),
      Color::Cyan => write!(f, "36"),
      Color::White => write!(f, "37"),
      Color::BrightBlack => write!(f, "90"),
      Color::BrightRed => write!(f, "91"),
      Color::BrightGreen => write!(f, "92"),
      Color::BrightYellow => write!(f, "93"),
      Color::BrightBlue => write!(f, "94"),
      Color::BrightMagenta => write!(f, "95"),
      Color::BrightCyan => write!(f, "96"),
      Color::BrightWhite => write!(f, "97"),
      Color::Reset => write!(f, "39;49"),
      Color::Color256(n) => write!(f, "38;5;{n}"),
      Color::TrueColor(r, g, b) => write!(f, "38;2;{r};{g};{b}"),
    }?;

    write!(f, "m")
  }

  pub fn bg_fmt<W: fmt::Write>(&self, mut f: W) -> fmt::Result {
    write!(f, "\x1B[")?;

    match *self {
      Color::Black => write!(f, "40"),
      Color::Red => write!(f, "41"),
      Color::Green => write!(f, "42"),
      Color::Yellow => write!(f, "43"),
      Color::Blue => write!(f, "44"),
      Color::Magenta => write!(f, "45"),
      Color::Cyan => write!(f, "46"),
      Color::White => write!(f, "47"),
      Color::BrightBlack => write!(f, "100"),
      Color::BrightRed => write!(f, "101"),
      Color::BrightGreen => write!(f, "102"),
      Color::BrightYellow => write!(f, "103"),
      Color::BrightBlue => write!(f, "104"),
      Color::BrightMagenta => write!(f, "105"),
      Color::BrightCyan => write!(f, "106"),
      Color::BrightWhite => write!(f, "107"),
      Color::Reset => write!(f, "39;49"),
      Color::Color256(n) => write!(f, "48;5;{n}"),
      Color::TrueColor(r, g, b) => write!(f, "48;2;{r};{g};{b}"),
    }?;

    write!(f, "m")
  }
}

impl FromStr for Color {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let src = s.to_lowercase();
    let mut result = Err(format!("'{}' is not a valid color", s));

    result = match src.as_ref() {
      "black" => Ok(Color::Black),
      "red" => Ok(Color::Red),
      "green" => Ok(Color::Green),
      "yellow" => Ok(Color::Yellow),
      "blue" => Ok(Color::Blue),
      "magenta" | "purple" => Ok(Color::Magenta),
      "cyan" => Ok(Color::Cyan),
      "gray" | "grey" | "white" => Ok(Color::White),
      "bright_black" | "bright black" => Ok(Color::BrightBlack),
      "bright_red" | "bright red" => Ok(Color::BrightRed),
      "bright_green" | "bright green" => Ok(Color::BrightGreen),
      "bright_yellow" | "bright yellow" => Ok(Color::BrightYellow),
      "bright_blue" | "bright blue" => Ok(Color::BrightBlue),
      "bright_magenta" | "bright magenta" => Ok(Color::BrightMagenta),
      "bright_cyan" | "bright cyan" => Ok(Color::BrightCyan),
      "bright_white" | "bright white" => Ok(Color::BrightWhite),
      "reset" => Ok(Color::Reset),
      _ => result,
    };

    if result.is_ok() {
      return result;
    }

    result = match u8::from_str(&src) {
      Ok(n) => Ok(Color::Color256(n)),
      Err(err) => Err(format!("{err:?}")),
    };

    if result.is_ok() {
      return result;
    }

    result = match u32::from_str_radix(src.trim_start_matches('#'), 16) {
      Ok(n) => {
        let [_, r, g, b] = n.to_be_bytes();

        Ok(Color::TrueColor(r, g, b))
      }
      Err(err) => Err(format!("{err:?}")),
    };

    result
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Color::Black => write!(f, "black"),
      Color::Red => write!(f, "red"),
      Color::Green => write!(f, "green"),
      Color::Yellow => write!(f, "yellow"),
      Color::Blue => write!(f, "blue"),
      Color::Magenta => write!(f, "magenta"),
      Color::Cyan => write!(f, "cyan"),
      Color::White => write!(f, "white"),
      Color::BrightBlack => write!(f, "bright_black"),
      Color::BrightRed => write!(f, "bright_red"),
      Color::BrightGreen => write!(f, "bright_green"),
      Color::BrightYellow => write!(f, "bright_yellow"),
      Color::BrightBlue => write!(f, "bright_blue"),
      Color::BrightMagenta => write!(f, "bright_magenta"),
      Color::BrightCyan => write!(f, "bright_cyan"),
      Color::BrightWhite => write!(f, "bright_white"),
      Color::Reset => write!(f, "reset"),
      Color::Color256(n) => write!(f, "{n}"),
      Color::TrueColor(r, g, b) => write!(f, "#{:x}", u32::from_be_bytes([0, r, g, b])),
    }
  }
}

pub trait ColorMapper: Sized {
  fn map_fmt<W: fmt::Write>(&self, color: [u8; 4], f: W) -> fmt::Result;
  fn map(&self, color: [u8; 4]) -> String {
    let mut buf = String::with_capacity(10);
    self.map_fmt(color, &mut buf).unwrap();
    buf
  }
}

pub struct NoColorMapper;

impl ColorMapper for NoColorMapper {
  fn map_fmt<W: fmt::Write>(&self, _: [u8; 4], _: W) -> fmt::Result {
    Ok(())
  }
}

pub struct TrueColorMapper;

impl ColorMapper for TrueColorMapper {
  fn map_fmt<W: fmt::Write>(&self, [r, g, b, _]: [u8; 4], f: W) -> fmt::Result {
    Color::TrueColor(r, g, b).fg_fmt(f)
  }
}

pub struct Color256Mapper;

impl ColorMapper for Color256Mapper {
  fn map_fmt<W: fmt::Write>(&self, [r, g, b, _]: [u8; 4], f: W) -> fmt::Result {
    Color::Color256((r / 3) + (g / 3) + (b / 3)).fg_fmt(f)
  }
}

pub struct Color16Mapper;

impl ColorMapper for Color16Mapper {
  fn map_fmt<W: fmt::Write>(&self, [r, g, b, _]: [u8; 4], f: W) -> fmt::Result {
    let color = match ((r / 3) + (g / 3) + (b / 3)) / 16 {
      0 => Color::Black,
      1 => Color::Red,
      2 => Color::Green,
      3 => Color::Yellow,
      4 => Color::Blue,
      5 => Color::Magenta,
      6 => Color::Cyan,
      7 => Color::White,
      8 => Color::BrightBlack,
      9 => Color::BrightRed,
      10 => Color::BrightGreen,
      11 => Color::BrightYellow,
      12 => Color::BrightBlue,
      13 => Color::BrightMagenta,
      14 => Color::BrightCyan,
      15 => Color::BrightWhite,
      _ => return Err(fmt::Error::default()),
    };

    color.fg_fmt(f)
  }
}
