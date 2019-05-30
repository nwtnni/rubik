#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    W,
    R,
    B,
    Y,
    G,
    O,
}

/// Represents a 2x2x2 Rubik's Cube:
///
///               11    10
///
///            19    18
///
///           04                09
///
///               00    03
///        12                17
///                 01    02
///
///            05                08
///
///                 06    07
///
///         13                16
///
///              14    15
///
///             20    23
///
///                21    22
///
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cube([Color; 24]);

impl Default for Cube {
    fn default() -> Self {
        use Color::*;
        Cube([
             W, W, W, W,
             R, R, B, B, O, O, G, G,
             R, R, B, B, O, O, G, G,
             Y, Y, Y, Y,
        ])
    }
}
