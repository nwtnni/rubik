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
    pub fn flip(&self) -> Self {
        match self {
        | Spin::CW => Spin::CCW,
        | Spin::CCW => Spin::CW,
        }
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub face: Face,
    pub spin: Spin,
}
