mod display;
mod state;

fn main() {
    println!("{}", std::mem::size_of::<state::Cube>());
    println!("{}", state::Cube::default());
}
