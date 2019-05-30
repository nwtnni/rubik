use std::io::Write;

use termion::event::{Event, Key};
use termion::input::TermRead;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut cube = rubik::state::Cube::default();
    writeln!(stdout, "{}\n", cube)?;

    for event in stdin.events() {
        use rubik::types::Face::*;
        use rubik::types::Spin::*;
        match event? {
        | Event::Key(Key::Char('q')) => break,
        | Event::Key(Key::Char('u')) => cube.rotate((U, CW)),
        | Event::Key(Key::Char('U')) => cube.rotate((U, CCW)),
        | Event::Key(Key::Char('d')) => cube.rotate((D, CW)),
        | Event::Key(Key::Char('D')) => cube.rotate((D, CCW)),
        | Event::Key(Key::Char('l')) => cube.rotate((L, CW)),
        | Event::Key(Key::Char('L')) => cube.rotate((L, CCW)),
        | Event::Key(Key::Char('r')) => cube.rotate((R, CW)),
        | Event::Key(Key::Char('R')) => cube.rotate((R, CCW)),
        | Event::Key(Key::Char('f')) => cube.rotate((F, CW)),
        | Event::Key(Key::Char('F')) => cube.rotate((F, CCW)),
        | Event::Key(Key::Char('b')) => cube.rotate((B, CW)),
        | Event::Key(Key::Char('B')) => cube.rotate((B, CCW)),
        | Event::Key(Key::Char('s')) => {
            let path = rubik::bfs::search(&cube); 
            for turn in &path.turns {
                cube.rotate(*turn); 
                writeln!(stdout, "{}\n", cube)?;
                std::thread::sleep(std::time::Duration::from_secs(1));
            }

            for turn in &path.turns {
                write!(stdout, "{} ", turn)?;
            }

            writeln!(
                stdout,
                "\nSolution length: {}\nNodes explored: {}\nTime taken (ms): {}\n\n",
                path.turns.len(),
                path.nodes,
                path.bench.as_millis(),
            )?;
        }
        | _ => continue,
        };
        writeln!(stdout, "{}\n", cube)?;
    }
    
    Ok(())
}
