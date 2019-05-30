#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    W = 0b0001,
    R = 0b0010,
    B = 0b0011,
    Y = 0b0100,
    G = 0b0101,
    O = 0b0110,
}

impl Color {
    pub fn new_unchecked(color: u16) -> Self {
        match color {
        | 0b0001 => Color::W,
        | 0b0010 => Color::R,
        | 0b0011 => Color::B,
        | 0b0100 => Color::Y,
        | 0b0101 => Color::G,
        | 0b0110 => Color::O,
        | _ => panic!("Invalid color: {}", color),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Spin {
    CW,
    CCW,
}

impl Spin {
    pub fn all() -> impl Iterator<Item = &'static Self> {
        [Spin::CW, Spin::CCW].into_iter()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Face {
    U,
    D,
    L,
    R,
    F,
    B,
}

impl Face {
    pub fn all() -> impl Iterator<Item = &'static Self> {
        [Face::U, Face::D, Face::L, Face::R, Face::F, Face::B].into_iter()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Turn {
    pub face: Face,
    pub spin: Spin,
}

impl Turn {
    pub fn all() -> impl Iterator<Item = Self> {
        Face::all().flat_map(|&face| Spin::all().map(move |&spin| Turn { face, spin }))
    }
}

impl From<(Face, Spin)> for Turn {
    fn from((face, spin): (Face, Spin)) -> Turn {
        Turn { face, spin }
    }
}

impl From<usize> for Turn {
    fn from(n: usize) -> Turn {
        Turn::all().nth(n % 12).unwrap()
    }
}
