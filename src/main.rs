mod display;
mod state;

fn main() {
    println!("{}", std::mem::size_of::<state::Cube>());
    let mut cube = state::Cube::default();
    println!("{}\n", cube);
    cube.rotate_l_ccw();
    println!("{}\n", cube);
}
