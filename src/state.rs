use crate::types;
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
        fn $name(&mut self) {
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
        fn $name(&mut self) {
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
    pub fn get(&self, index: usize) -> types::Color {
        let div = index / 4;
        let rem = index % 4;
        types::Color::new_unchecked((self.0[div] >> ((3 - rem) << 2)) & 0b0111)
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

    fn rotate_f_cw(&mut self) {
        let swap = (self[1] & 0b0000_0000_1111_1111).rotate_left(4);
        self[1] &= !0b0000_0000_1111_1111; self[1] |= (self[5] & 0b1111_0000_0000_1111).rotate_left(4);
        self[5] &= !0b1111_0000_0000_1111; self[5] |= (self[3] & 0b1111_1111_0000_0000).rotate_left(4);
        self[3] &= !0b1111_1111_0000_0000; self[3] |= (self[0] & 0b0000_1111_1111_0000).rotate_left(4);
        self[0] &= !0b0000_1111_1111_0000; self[0] |= swap;
        self[2] = self[2].rotate_left(4);
    }

    fn rotate_f_ccw(&mut self) {
        let swap = (self[0] & 0b0000_1111_1111_0000).rotate_right(4);
        self[0] &= !0b0000_1111_1111_0000; self[0] |= (self[3] & 0b1111_1111_0000_0000).rotate_right(4);
        self[3] &= !0b1111_1111_0000_0000; self[3] |= (self[5] & 0b1111_0000_0000_1111).rotate_right(4);
        self[5] &= !0b1111_0000_0000_1111; self[5] |= (self[1] & 0b0000_0000_1111_1111).rotate_right(4);
        self[1] &= !0b0000_0000_1111_1111; self[1] |= swap;
        self[2] = self[2].rotate_right(4);
    }

    fn rotate_b_cw(&mut self) {
        let swap = (self[0] & 0b1111_0000_0000_1111).rotate_right(4);
        self[0] &= !0b1111_0000_0000_1111; self[0] |= (self[3] & 0b0000_0000_1111_1111).rotate_right(4);
        self[3] &= !0b0000_0000_1111_1111; self[3] |= (self[5] & 0b0000_1111_1111_0000).rotate_right(4);
        self[5] &= !0b0000_1111_1111_0000; self[5] |= (self[1] & 0b1111_1111_0000_0000).rotate_right(4);
        self[1] &= !0b1111_1111_0000_0000; self[1] |= swap;
        self[4] = self[4].rotate_left(4);
    }

    fn rotate_b_ccw(&mut self) {
        let swap = (self[1] & 0b1111_1111_0000_0000).rotate_left(4);
        self[1] &= !0b1111_1111_0000_0000; self[1] |= (self[5] & 0b0000_1111_1111_0000).rotate_left(4);
        self[5] &= !0b0000_1111_1111_0000; self[5] |= (self[3] & 0b0000_0000_1111_1111).rotate_left(4);
        self[3] &= !0b0000_0000_1111_1111; self[3] |= (self[0] & 0b1111_0000_0000_1111).rotate_left(4);
        self[0] &= !0b1111_0000_0000_1111; self[0] |= swap;
        self[4] = self[4].rotate_right(4);
    }

    pub fn rotate<I: Into<types::Turn>>(&mut self, turn: I) {
        use types::Face::*;
        use types::Spin::*;
        let turn = turn.into();
        match (turn.face, turn.spin) {
        | (U, CW)  => self.rotate_u_cw(),
        | (U, CCW) => self.rotate_u_ccw(),
        | (D, CW)  => self.rotate_d_cw(),
        | (D, CCW) => self.rotate_d_ccw(),
        | (L, CW)  => self.rotate_l_cw(),
        | (L, CCW) => self.rotate_l_ccw(),
        | (R, CW)  => self.rotate_r_cw(),
        | (R, CCW) => self.rotate_r_ccw(),
        | (F, CW)  => self.rotate_f_cw(),
        | (F, CCW) => self.rotate_f_ccw(),
        | (B, CW)  => self.rotate_b_cw(),
        | (B, CCW) => self.rotate_b_ccw(),
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

#[cfg(test)]
mod tests {

    use super::*;

    /// Check that rotating 180 degrees CW and 180 degrees CCW results
    /// in the same state, starting from a solved state.
    macro_rules! rotate_180 {
        ($test:ident, $cw:ident, $ccw:ident) => {
            #[quickcheck_macros::quickcheck]
            fn $test(count: usize) -> quickcheck::TestResult {
                if count % 4 != 0 { return quickcheck::TestResult::discard() }
                let mut cube_cw = Cube::default();
                let mut cube_ccw = Cube::default();
                for _ in 0..count + 2 { cube_cw.$cw(); cube_ccw.$ccw(); }
                quickcheck::TestResult::from_bool(cube_cw == cube_ccw)
            }
        }
    }

    rotate_180!(rotate_u_180, rotate_u_cw, rotate_u_ccw);
    rotate_180!(rotate_d_180, rotate_d_cw, rotate_d_ccw);
    rotate_180!(rotate_l_180, rotate_l_cw, rotate_l_ccw);
    rotate_180!(rotate_r_180, rotate_r_cw, rotate_r_ccw);
    rotate_180!(rotate_f_180, rotate_f_cw, rotate_f_ccw);
    rotate_180!(rotate_b_180, rotate_b_cw, rotate_b_ccw);

    /// Check that rotating 180 degrees CW and 180 degrees CCW results
    /// in the same state, starting from a random state.
    macro_rules! rotate_180_random {
        ($test:ident, $cw:ident, $ccw:ident) => {
            #[quickcheck_macros::quickcheck]
            fn $test((path, count): (Vec<usize>, usize)) -> quickcheck::TestResult {
                if count % 4 != 0 { return quickcheck::TestResult::discard() }
                let mut cube = Cube::default();
                for d in path { cube.rotate(d); }
                let mut cube_cw = cube.clone();
                let mut cube_ccw = cube.clone();
                for _ in 0..count + 2 { cube_cw.$cw(); cube_ccw.$ccw(); }
                quickcheck::TestResult::from_bool(cube_cw == cube_ccw)
            }
        }
    }

    rotate_180_random!(rotate_u_180_random, rotate_u_cw, rotate_u_ccw);
    rotate_180_random!(rotate_d_180_random, rotate_d_cw, rotate_d_ccw);
    rotate_180_random!(rotate_l_180_random, rotate_l_cw, rotate_l_ccw);
    rotate_180_random!(rotate_r_180_random, rotate_r_cw, rotate_r_ccw);
    rotate_180_random!(rotate_f_180_random, rotate_f_cw, rotate_f_ccw);
    rotate_180_random!(rotate_b_180_random, rotate_b_cw, rotate_b_ccw);

    /// Check that rotating 360 degrees CW or CCW results
    /// in the same state, starting from a solved state.
    macro_rules! rotate_360 {
        ($test:ident, $method:ident) => {
            #[quickcheck_macros::quickcheck]
            fn $test(count: usize) -> quickcheck::TestResult {
                if count % 4 != 0 { return quickcheck::TestResult::discard() }
                let mut cube = Cube::default();
                for _ in 0..count { cube.$method(); }
                quickcheck::TestResult::from_bool(cube.is_solved())
            }
        }
    }

    rotate_360!(rotate_u_cw_360, rotate_u_cw);
    rotate_360!(rotate_u_ccw_360, rotate_u_ccw);
    rotate_360!(rotate_d_cw_360, rotate_d_cw);
    rotate_360!(rotate_d_ccw_360, rotate_d_ccw);
    rotate_360!(rotate_l_cw_360, rotate_l_cw);
    rotate_360!(rotate_l_ccw_360, rotate_l_ccw);
    rotate_360!(rotate_r_cw_360, rotate_r_cw);
    rotate_360!(rotate_r_ccw_360, rotate_r_ccw);
    rotate_360!(rotate_f_cw_360, rotate_f_cw);
    rotate_360!(rotate_f_ccw_360, rotate_f_ccw);
    rotate_360!(rotate_b_cw_360, rotate_b_cw);
    rotate_360!(rotate_b_ccw_360, rotate_b_ccw);

    /// Check that rotating 360 degrees CW or CCW results
    /// in the same state, starting from a random state.
    macro_rules! rotate_360_random {
        ($test:ident, $method:ident) => {
            #[quickcheck_macros::quickcheck]
            fn $test((path, count): (Vec<usize>, usize)) -> quickcheck::TestResult {
                if count % 4 != 0 { return quickcheck::TestResult::discard() }
                let mut cube = Cube::default();
                for d in path { cube.rotate(d); }
                let init = cube.clone();
                for _ in 0..count { cube.$method(); }
                quickcheck::TestResult::from_bool(cube == init)
            }
        }
    }

    rotate_360_random!(rotate_u_cw_360_random, rotate_u_cw);
    rotate_360_random!(rotate_u_ccw_360_random, rotate_u_ccw);
    rotate_360_random!(rotate_d_cw_360_random, rotate_d_cw);
    rotate_360_random!(rotate_d_ccw_360_random, rotate_d_ccw);
    rotate_360_random!(rotate_l_cw_360_random, rotate_l_cw);
    rotate_360_random!(rotate_l_ccw_360_random, rotate_l_ccw);
    rotate_360_random!(rotate_r_cw_360_random, rotate_r_cw);
    rotate_360_random!(rotate_r_ccw_360_random, rotate_r_ccw);
    rotate_360_random!(rotate_f_cw_360_random, rotate_f_cw);
    rotate_360_random!(rotate_f_ccw_360_random, rotate_f_ccw);
    rotate_360_random!(rotate_b_cw_360_random, rotate_b_cw);
    rotate_360_random!(rotate_b_ccw_360_random, rotate_b_ccw);

    /// Checks that reversing a random walk restores the original state.
    #[quickcheck_macros::quickcheck]
    fn retrace(path: Vec<usize>) -> bool {
        let mut cube = Cube::default();
        for &d in &path { cube.rotate(d); }
        for &d in path.iter().rev() {
            if d % 2 == 0 {
                cube.rotate(d + 1);
            } else {
                cube.rotate(d - 1);
            }
        }
        cube.is_solved()
    }
}
