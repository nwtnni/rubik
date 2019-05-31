use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let stdin = BufReader::new(std::io::stdin());
    let mut stdout = std::io::stdout();
    let mut cube = rubik::state::Cube::default();

    writeln!(stdout, "{}\n", cube)?;

    for line in stdin.lines() {
        use rubik::types::Face::*;
        use rubik::types::Spin::*;
        match line?.as_ref() {
        | "q" => break,
        | "u" => cube.rotate((U, CW)),
        | "U" => cube.rotate((U, CCW)),
        | "d" => cube.rotate((D, CW)),
        | "D" => cube.rotate((D, CCW)),
        | "l" => cube.rotate((L, CW)),
        | "L" => cube.rotate((L, CCW)),
        | "r" => cube.rotate((R, CW)),
        | "R" => cube.rotate((R, CCW)),
        | "f" => cube.rotate((F, CW)),
        | "F" => cube.rotate((F, CCW)),
        | "b" => cube.rotate((B, CW)),
        | "B" => cube.rotate((B, CCW)),
        | "s" => {
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
        | randomize => {
            if let Ok(randomize) = randomize.parse::<usize>() {
                for _ in 0..randomize {
                    cube.rotate(rand::random::<usize>());
                }
            } else {
                continue
            }
        }
        };
        writeln!(stdout, "{}\n", cube)?;
    }
    
    Ok(())
}
