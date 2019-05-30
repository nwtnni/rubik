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
        Face((self.0[div] >> ((3 - rem) << 2)) as u8 & 0b0111)
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
