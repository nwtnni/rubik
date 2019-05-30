#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Face(u8);

pub const W: Face = Face(0b0000_0001);
pub const R: Face = Face(0b0000_0010);
pub const B: Face = Face(0b0000_0011);
pub const Y: Face = Face(0b0000_0100);
pub const G: Face = Face(0b0000_0101);
pub const O: Face = Face(0b0000_0110);

/// Represents a 2x2x2 Rubik's Cube:
///
///         0003
///           0102
/// 04   1110      09
/// 1205 1918 0607 1708
///   13      1415   16
///      2023
///        2122
///
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cube(u16, u32, u32, u16);

impl Cube {
    pub fn get(&self, index: usize) -> Face {
        let div = index / 4;
        let rem = index % 4;
        let word = match div {
        | 0 => self.0,
        | 1 => (self.1 >> 16) as u16,
        | 2 => self.1 as u16,
        | 3 => (self.2 >> 16) as u16,
        | 4 => self.2 as u16,
        | 5 => self.3,
        | _ => panic!("Invalid index: {}", index),
        };
        Face((word >> ((3 - rem) << 2)) as u8 & 0b1111)
    }

    #[must_use]
    pub fn rotate_u_cw(&self) -> Self {
        Cube(self.0.rotate_left(8), self.1.rotate_left(8), self.2, self.3)
    }

    #[must_use]
    pub fn rotate_u_ccw(&self) -> Self {
        Cube(self.0.rotate_right(8), self.1.rotate_right(8), self.2, self.3)
    }

    #[must_use]
    pub fn rotate_d_cw(&self) -> Self {
        Cube(self.0, self.1, self.2.rotate_left(8), self.3.rotate_left(8))
    }

    #[must_use]
    pub fn rotate_d_ccw(&self) -> Self {
        Cube(self.0, self.1, self.2.rotate_right(8), self.3.rotate_right(8))
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube(
            // W    W    W    W
            0b_0001_0001_0001_0001,

            // R    R    B    B    O    O    G    G
            0b_0010_0010_0011_0011_0110_0110_0101_0101,

            // R    R    B    B    O    O    G    G
            0b_0010_0010_0011_0011_0110_0110_0101_0101,
            
            // Y    Y    Y    Y  
            0b_0100_0100_0100_0100,
        )
    }
}
