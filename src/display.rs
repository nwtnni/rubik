use termion::color;

use crate::state;

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

impl std::fmt::Display for state::Face {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let color = match self {
        | state::Face::W => W,
        | state::Face::R => R,
        | state::Face::B => B,
        | state::Face::Y => Y,
        | state::Face::G => G,
        | state::Face::O => O,
        };
        write!(fmt, "{}██", color)
    }
}

impl std::fmt::Display for state::Cube {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            cube!(),
            self[00], self[03], self[01], self[02],
            self[04], self[11], self[10], self[09],
            self[12], self[05], self[19], self[18],
            self[06], self[07], self[17], self[08],
            self[13], self[14], self[15], self[16],
            self[20], self[23], self[21], self[22],
            color::Fg(color::Reset),
        )
    }
}
