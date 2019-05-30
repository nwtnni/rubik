use std::io::Write;

use termion::event::{Event, Key};
use termion::input::TermRead;

#[cfg(test)]
mod test;
mod bfs;
mod display;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut cube = state::Cube::default();
    writeln!(stdout, "{}\n", cube)?;

    for event in stdin.events() {
        match event? {
        | Event::Key(Key::Char('q')) => break,
        | Event::Key(Key::Char('u')) => cube.rotate_u_cw(),
        | Event::Key(Key::Char('U')) => cube.rotate_u_ccw(),
        | Event::Key(Key::Char('d')) => cube.rotate_d_cw(),
        | Event::Key(Key::Char('D')) => cube.rotate_d_ccw(),
        | Event::Key(Key::Char('l')) => cube.rotate_l_cw(),
        | Event::Key(Key::Char('L')) => cube.rotate_l_ccw(),
        | Event::Key(Key::Char('r')) => cube.rotate_r_cw(),
        | Event::Key(Key::Char('R')) => cube.rotate_r_ccw(),
        | Event::Key(Key::Char('f')) => cube.rotate_f_cw(),
        | Event::Key(Key::Char('F')) => cube.rotate_f_ccw(),
        | Event::Key(Key::Char('b')) => cube.rotate_b_cw(),
        | Event::Key(Key::Char('B')) => cube.rotate_b_ccw(),
        | Event::Key(Key::Char('s')) => {
            let path = bfs::search(&cube); 
            for d in path {
                cube.rotate(d); 
                writeln!(stdout, "{}\n", cube)?;
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            continue
        }
        | _ => continue,
        };
        writeln!(stdout, "{}\n", cube)?;
    }
    
    Ok(())
}
