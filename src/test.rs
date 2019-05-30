#[allow(unused)]
use crate::state;

/// Check that rotating 180 degrees CW and 180 degrees CCW results
/// in the same state, starting from a solved state.
macro_rules! rotate_180 {
    ($test:ident, $cw:ident, $ccw:ident) => {
        #[quickcheck_macros::quickcheck]
        fn $test(count: usize) -> quickcheck::TestResult {
            if count % 4 != 0 { return quickcheck::TestResult::discard() }
            let mut cube_cw = state::Cube::default();
            let mut cube_ccw = state::Cube::default();
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
            let mut cube = state::Cube::default();
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
            let mut cube = state::Cube::default();
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
            let mut cube = state::Cube::default();
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
    let mut cube = state::Cube::default();
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
