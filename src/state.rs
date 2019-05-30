#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Face {
    W,
    R,
    B,
    Y,
    G,
    O,
}

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
pub struct Cube([Face; 24]);

impl Default for Cube {
    fn default() -> Self {
        use Face::*;
        Cube([
             W, W, W, W,
             R, R, B, B, O, O, G, G,
             R, R, B, B, O, O, G, G,
             Y, Y, Y, Y,
        ])
    }
}

impl std::ops::Index<usize> for Cube {
    type Output = Face;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
