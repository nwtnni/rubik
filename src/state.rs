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
    ($name:ident, $from:expr, $face:expr, $rotate_dir:ident, $ring:expr) => {
        pub fn $name(&mut self) {
            let swap = self[$ring.3] & $from;
            self[$ring.3] &= !$from; self[$ring.3] |= self[$ring.2] & $from;
            self[$ring.2] &= !$from; self[$ring.2] |= self[$ring.1] & $from;
            self[$ring.1] &= !$from; self[$ring.1] |= self[$ring.0] & $from;
            self[$ring.0] &= !$from; self[$ring.0] |= swap & $from;
            self[$face] = self[$face].$rotate_dir(4);
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

    // rotate!(rotate_l_cw,  0b1111_1111_0000_0000, 1, rotate_left,  (2, 5, 4, 0));
    // rotate!(rotate_l_ccw, 0b1111_1111_0000_0000, 1, rotate_right, (0, 4, 5, 2));

    // rotate!(rotate_r_cw,  0b0000_0000_1111_1111, 3, rotate_left,  (0, 4, 5, 2));
    // rotate!(rotate_r_ccw, 0b0000_0000_1111_1111, 3, rotate_right, (2, 5, 4, 0));

    rotate!(rotate_u_cw,  0b1111_0000_0000_1111, 0, rotate_left,  (4, 3, 2, 1));
    rotate!(rotate_u_ccw, 0b1111_0000_0000_1111, 0, rotate_right, (1, 2, 3, 4));

    rotate!(rotate_d_cw,  0b0000_1111_1111_0000, 5, rotate_left,  (1, 2, 3, 4));
    rotate!(rotate_d_ccw, 0b0000_1111_1111_0000, 5, rotate_right, (4, 3, 2, 1));

    pub fn rotate_l_cw(&mut self) {
        let swap = (self[4] & 0b0000_0000_1111_1111).rotate_left(8);
        self[4] &= !0b0000_0000_1111_1111; self[4] |= (self[5] & 0b1111_1111_0000_0000).rotate_left(8);
        self[5] &= !0b1111_1111_0000_0000; self[5] |= self[2] & 0b1111_1111_0000_0000;
        self[2] &= !0b1111_1111_0000_0000; self[2] |= self[0] & 0b1111_1111_0000_0000;
        self[0] &= !0b1111_1111_0000_0000; self[0] |= swap;
        self[1] = self[1].rotate_left(4);
    }

    pub fn rotate_l_ccw(&mut self) {
        let swap = (self[0] & 0b1111_1111_0000_0000).rotate_left(8);
        self[0] &= !0b1111_1111_0000_0000; self[0] |= self[2] & 0b1111_1111_0000_0000;
        self[2] &= !0b1111_1111_0000_0000; self[2] |= self[5] & 0b1111_1111_0000_0000;
        self[5] &= !0b0111_1111_0000_0000; self[5] |= (self[4] & 0b0000_0000_1111_1111).rotate_left(8);
        self[4] &= !0b0000_0000_1111_1111; self[4] |= swap;
        self[1] = self[1].rotate_right(4);
    }

    pub fn rotate_r_cw(&mut self) {
        let swap = (self[0] & 0b0000_0000_1111_1111).rotate_left(8);
        self[0] &= !0b0000_0000_1111_1111; self[0] |= self[2] & 0b0000_0000_1111_1111;
        self[2] &= !0b0000_0000_1111_1111; self[2] |= self[5] & 0b0000_0000_1111_1111;
        self[5] &= !0b0000_0000_1111_1111; self[5] |= (self[4] & 0b1111_1111_0000_0000).rotate_left(8);
        self[4] &= !0b1111_1111_0000_0000; self[4] |= swap;
        self[3] = self[3].rotate_left(4);
    }

    pub fn rotate_r_ccw(&mut self) {
        let swap = (self[4] & 0b1111_1111_0000_0000).rotate_left(8);
        self[4] &= !0b1111_1111_0000_0000; self[4] |= (self[5] & 0b0000_0000_1111_1111).rotate_left(8);
        self[5] &= !0b0000_0000_1111_1111; self[5] |= self[2] & 0b0000_0000_1111_1111;
        self[2] &= !0b0000_0000_1111_1111; self[2] |= self[0] & 0b0000_0000_1111_1111;
        self[0] &= !0b0000_0000_1111_1111; self[0] |= swap;
        self[3] = self[3].rotate_right(4);
    }

    pub fn rotate_f_cw(&mut self) {
        let swap = (self[1] & 0b0000_0000_1111_1111).rotate_left(4);
        self[1] &= !0b0000_0000_1111_1111; self[1] |= (self[5] & 0b1111_0000_0000_1111).rotate_left(4);
        self[5] &= !0b1111_0000_0000_1111; self[5] |= (self[3] & 0b1111_1111_0000_0000).rotate_left(4);
        self[3] &= !0b1111_1111_0000_0000; self[3] |= (self[0] & 0b0000_1111_1111_0000).rotate_left(4);
        self[0] &= !0b0000_1111_1111_0000; self[0] |= swap;
        self[2] = self[2].rotate_left(4);
    }

    pub fn rotate_f_ccw(&mut self) {
        let swap = (self[0] & 0b0000_1111_1111_0000).rotate_right(4);
        self[0] &= !0b0000_1111_1111_0000; self[0] |= (self[3] & 0b1111_1111_0000_0000).rotate_right(4);
        self[3] &= !0b1111_1111_0000_0000; self[3] |= (self[5] & 0b1111_0000_0000_1111).rotate_right(4);
        self[5] &= !0b1111_0000_0000_1111; self[5] |= (self[1] & 0b0000_0000_1111_1111).rotate_right(4);
        self[1] &= !0b0000_0000_1111_1111; self[1] |= swap;
        self[2] = self[2].rotate_right(4);
    }

    pub fn rotate_b_cw(&mut self) {
        let swap = (self[0] & 0b1111_0000_0000_1111).rotate_right(4);
        self[0] &= !0b1111_0000_0000_1111; self[0] |= (self[3] & 0b0000_0000_1111_1111).rotate_right(4);
        self[3] &= !0b0000_0000_1111_1111; self[3] |= (self[5] & 0b0000_1111_1111_0000).rotate_right(4);
        self[5] &= !0b0000_1111_1111_0000; self[5] |= (self[1] & 0b1111_1111_0000_0000).rotate_right(4);
        self[1] &= !0b1111_1111_0000_0000; self[1] |= swap;
        self[4] = self[4].rotate_left(4);
    }

    pub fn rotate_b_ccw(&mut self) {
        let swap = (self[1] & 0b1111_1111_0000_0000).rotate_left(4);
        self[1] &= !0b1111_1111_0000_0000; self[1] |= (self[5] & 0b0000_1111_1111_0000).rotate_left(4);
        self[5] &= !0b0000_1111_1111_0000; self[5] |= (self[3] & 0b0000_0000_1111_1111).rotate_left(4);
        self[3] &= !0b0000_0000_1111_1111; self[3] |= (self[0] & 0b1111_0000_0000_1111).rotate_left(4);
        self[0] &= !0b1111_0000_0000_1111; self[0] |= swap;
        self[4] = self[4].rotate_right(4);
    }
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
