mod display;
mod state;

fn main() {
    println!("{}", std::mem::size_of::<state::Cube>());
    let cube = state::Cube::default();
    println!("{}", cube);
}
