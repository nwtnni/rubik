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
/// 04   1916      15
/// 0507 1817 0811 1412
///   06      0910   13
///      2322
///        2021
///
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cube([u16; 6]);

impl Cube {
    pub fn get(&self, index: usize) -> Face {
        let div = index / 4;
        let rem = index % 4;
        Face::new_unchecked((self.0[div] >> ((3 - rem) << 2)) & 0b0111)
    }

    #[must_use]
    pub fn rotate_u_cw(&self) -> Self {
        let face = self[0].rotate_left(8);

        unimplemented!()
    }

}

impl Default for Cube {
    fn default() -> Self {
        Cube([
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
        ])
    }
}

impl std::ops::Index<usize> for Cube {
    type Output = u16;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
