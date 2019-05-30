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
pub struct Cube([u32; 3]);

impl Cube {
    pub fn get(&self, index: usize) -> Face {
        let div = index / 8;
        let rem = index % 8;
        Face((self.0[div] >> ((7 - rem) << 2)) as u8 & 0b0111)
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube([
            //W    W    W    W    R    R    B    B 
            0b0001_0001_0001_0001_0010_0010_0011_0011,

            //O    O    G    G    R    R    B    B 
            0b0110_0110_0101_0101_0010_0010_0011_0011,

            //O    O    G    G    Y    Y    Y    Y 
            0b0110_0110_0101_0101_0100_0100_0100_0100,
        ])
    }
}
