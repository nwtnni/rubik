#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Face {
    W = 0b0001,
    R = 0b0010,
    B = 0b0011,
    Y = 0b0100,
    G = 0b0101,
    O = 0b0110,
}

impl Face {
    fn new_unchecked(face: u16) -> Self {
        match face {
        | 0b0001 => Face::W,
        | 0b0010 => Face::R,
        | 0b0011 => Face::B,
        | 0b0100 => Face::Y,
        | 0b0101 => Face::G,
        | 0b0110 => Face::O,
        | _ => panic!("Invalid face: {}", face),
        }
    }
}

/// Represents a 2x2x2 Rubik's Cube:
///
///         0003
///           0102
/// 04   1718      15
/// 0507 1619 0811 1412
///   06      0910   13
///      2122
///        2023
///
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cube([u16; 6]);

pub const SOLVED: Cube = Cube([
    // W    W    W    W
    0b_0001_0001_0001_0001,
    // R    R    R    R
    0b_0010_0010_0010_0010,
    // B    B    B    B
    0b_0011_0011_0011_0011,
    // O    O    O    O
    0b_0110_0110_0110_0110,
    // G    G    G    G
    0b_0101_0101_0101_0101,
    // Y    Y    Y    Y  
    0b_0100_0100_0100_0100,
]);

macro_rules! rotate {
    ($name:ident, $from:expr, $face:expr, $ring:expr) => {
        pub fn $name(&mut self) {
            let into = !$from;
            let from = $from;
            let swap = self[$ring.0] & from;
            self[$ring.0] &= into; self[$ring.0] |= self[$ring.1] & from;
            self[$ring.1] &= into; self[$ring.1] |= self[$ring.2] & from;
            self[$ring.2] &= into; self[$ring.2] |= self[$ring.3] & from;
            self[$ring.3] &= into; self[$ring.3] |= swap & from;
            self[$face] = self[$face].rotate_right(4);
        }
    }
}

impl Cube {
    pub fn get(&self, index: usize) -> Face {
        let div = index / 4;
        let rem = index % 4;
        Face::new_unchecked((self.0[div] >> ((3 - rem) << 2)) & 0b0111)
    }

    pub fn is_solved(&self) -> bool {
        *self == SOLVED
    }

    rotate!(rotate_l_ccw, 0b1111_1111_0000_0000, 1, (0, 2, 5, 4));
    rotate!(rotate_l_cw, 0b1111_1111_0000_0000, 1, (4, 5, 2, 0));

    // pub fn rotate_l_ccw(&mut self) {
    //     let into = 0b0000_0000_1111_1111;
    //     let from = 0b1111_1111_0000_0000;
    //     let swap = self[0] & from;
    //     self[0] &= into; self[0] |= self[2] & from;
    //     self[2] &= into; self[2] |= self[5] & from;
    //     self[5] &= into; self[5] |= self[4] & from;
    //     self[4] &= into; self[4] |= swap & from;
    //     self[1] = self[1].rotate_right(4);
    // }

}

impl Default for Cube {
    fn default() -> Self { SOLVED }
}

impl std::ops::Index<usize> for Cube {
    type Output = u16;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Cube {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
