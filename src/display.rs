use termion::color;

use crate::state;
use crate::types;

const W: color::Fg<&'static dyn color::Color> = color::Fg(&color::White);
const R: color::Fg<&'static dyn color::Color> = color::Fg(&color::Red);
const B: color::Fg<&'static dyn color::Color> = color::Fg(&color::Blue);
const Y: color::Fg<&'static dyn color::Color> = color::Fg(&color::Yellow);
const G: color::Fg<&'static dyn color::Color> = color::Fg(&color::Green);
const O: color::Fg<&'static dyn color::Color> = color::Fg(&color::Magenta);

macro_rules! cube { () => {
"        {}{}
          {}{}
{}   {}{}      {}
{}{} {}{} {}{} {}{}
  {}      {}{}   {}
     {}{}
       {}{}{}"
} }

impl std::fmt::Display for types::Color {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let color = match *self {
        | types::Color::W => W,
        | types::Color::R => R,
        | types::Color::B => B,
        | types::Color::Y => Y,
        | types::Color::G => G,
        | types::Color::O => O,
        };
        write!(fmt, "{}██", color)
    }
}

impl std::fmt::Display for types::Face {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let notation = match self {
        | types::Face::U => "U",
        | types::Face::D => "D",
        | types::Face::L => "L",
        | types::Face::R => "R",
        | types::Face::F => "F",
        | types::Face::B => "B",
        };
        write!(fmt, "{}", notation)
    }
}

impl std::fmt::Display for types::Turn {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.spin {
        | types::Spin::CW  => write!(fmt, "{}", self.face),
        | types::Spin::CCW => write!(fmt, "{}'", self.face),
        }
    }
}

impl std::fmt::Display for state::Cube {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            cube!(),
            self.get(00), self.get(03), self.get(01), self.get(02),
            self.get(04), self.get(19), self.get(16), self.get(15),
            self.get(05), self.get(07), self.get(18), self.get(17),
            self.get(08), self.get(11), self.get(14), self.get(12),
            self.get(06), self.get(09), self.get(10), self.get(13),
            self.get(21), self.get(22), self.get(20), self.get(23),
            color::Fg(color::Reset),
        )
    }
}
