mod display;
mod state;

fn main() {
    println!("{}", std::mem::size_of::<state::Cube>());
    let mut cube = state::Cube::default();
    cube.rotate_l_cw();
    println!("{}", cube);
}
