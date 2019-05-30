use crate::types::Color;
use crate::types::Color::*;

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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cube([u16; 6]);

pub const SOLVED: Cube = Cube([
    (W as u16) << 12 | (W as u16) << 08 | (W as u16) << 04 | (W as u16) << 00,
    (R as u16) << 12 | (R as u16) << 08 | (R as u16) << 04 | (R as u16) << 00,
    (B as u16) << 12 | (B as u16) << 08 | (B as u16) << 04 | (B as u16) << 00,
    (O as u16) << 12 | (O as u16) << 08 | (O as u16) << 04 | (O as u16) << 00,
    (G as u16) << 12 | (G as u16) << 08 | (G as u16) << 04 | (G as u16) << 00,
    (Y as u16) << 12 | (Y as u16) << 08 | (Y as u16) << 04 | (Y as u16) << 00,
]);

macro_rules! rotate_ud {
    ($name:ident, $mask:expr, $face:expr, $rotate_dir:ident, $ring:expr) => {
        pub fn $name(&mut self) {
            let swap = self[$ring.3] & $mask;
            self[$ring.3] &= !$mask; self[$ring.3] |= self[$ring.2] & $mask;
            self[$ring.2] &= !$mask; self[$ring.2] |= self[$ring.1] & $mask;
            self[$ring.1] &= !$mask; self[$ring.1] |= self[$ring.0] & $mask;
            self[$ring.0] &= !$mask; self[$ring.0] |= swap & $mask;
            self[$face] = self[$face].$rotate_dir(4);
        }
    }
}

macro_rules! rotate_lr {
    ($name:ident, $mask:expr, $face:expr, $rotate_dir:ident, $ring:expr) => {
        pub fn $name(&mut self) {
            let swap = (self[$ring.0] & !$mask).rotate_left(8);
            self[$ring.0] &= $mask; self[$ring.0] |= (self[$ring.1] & $mask).rotate_left(8);
            self[$ring.1] &= !$mask; self[$ring.1] |= self[$ring.2] & $mask;
            self[$ring.2] &= !$mask; self[$ring.2] |= self[$ring.3] & $mask;
            self[$ring.3] &= !$mask; self[$ring.3] |= swap;
            self[$face] = self[$face].$rotate_dir(4);
        }
    }
}

impl Cube {
    pub fn get(&self, index: usize) -> Color {
        let div = index / 4;
        let rem = index % 4;
        Color::new_unchecked((self.0[div] >> ((3 - rem) << 2)) & 0b0111)
    }

    pub fn is_solved(&self) -> bool {
        *self == SOLVED
    }

    rotate_ud!(rotate_u_cw,  0b1111_0000_0000_1111, 0, rotate_left,  (4, 3, 2, 1));
    rotate_ud!(rotate_u_ccw, 0b1111_0000_0000_1111, 0, rotate_right, (1, 2, 3, 4));

    rotate_ud!(rotate_d_cw,  0b0000_1111_1111_0000, 5, rotate_left,  (1, 2, 3, 4));
    rotate_ud!(rotate_d_ccw, 0b0000_1111_1111_0000, 5, rotate_right, (4, 3, 2, 1));

    rotate_lr!(rotate_l_cw,  0b1111_1111_0000_0000, 1, rotate_left,  (4, 5, 2, 0));
    rotate_lr!(rotate_l_ccw, 0b1111_1111_0000_0000, 1, rotate_right, (4, 0, 2, 5));

    rotate_lr!(rotate_r_cw,  0b0000_0000_1111_1111, 3, rotate_left,  (4, 0, 2, 5));
    rotate_lr!(rotate_r_ccw, 0b0000_0000_1111_1111, 3, rotate_right, (4, 5, 2, 0));

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

    pub fn rotate(&mut self, direction: usize) {
        match direction % 12 {
        | 00 => self.rotate_u_cw(),
        | 01 => self.rotate_u_ccw(),
        | 02 => self.rotate_d_cw(),
        | 03 => self.rotate_d_ccw(),
        | 04 => self.rotate_l_cw(),
        | 05 => self.rotate_l_ccw(),
        | 06 => self.rotate_r_cw(),
        | 07 => self.rotate_r_ccw(),
        | 08 => self.rotate_f_cw(),
        | 09 => self.rotate_f_ccw(),
        | 10 => self.rotate_b_cw(),
        | 11 => self.rotate_b_ccw(),
        | _ => unreachable!(),
        }
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
