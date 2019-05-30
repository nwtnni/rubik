use std::collections::{HashMap, VecDeque};

use crate::state;
use crate::types;

pub struct Path {
    pub turns: Vec<types::Turn>,
    pub nodes: usize,
    pub bench: std::time::Duration,
}

pub fn search(cube: &state::Cube) -> Path {

    if cube.is_solved() {
        return Path {
            turns: vec![],
            nodes: 0,
            bench: std::time::Duration::default(),
        }
    }

    let now = std::time::Instant::now();
    let mut queue: VecDeque<state::Cube> = VecDeque::new();
    let mut cache: HashMap<state::Cube, (state::Cube, types::Turn)> = HashMap::new();
    let mut nodes = 0;

    queue.push_back(*cube);
    cache.insert(*cube, (*cube, (types::Face::U, types::Spin::CW).into()));

    while let Some(cube) = queue.pop_front() {

        nodes += 1;

        if cube.is_solved() {
            let mut back = &cache[&cube];
            let mut turns = Vec::new();
            turns.insert(0, back.1);
            while let Some(next) = cache.get(&back.0) {
                if next.0 == back.0 { break }
                turns.insert(0, next.1);
                back = next;
            }
            return Path {
                turns,
                nodes,
                bench: std::time::Instant::now() - now,
            }
        }

        for turn in types::Turn::all() {
            let mut next = cube.clone();
            next.rotate(turn);
            if cache.contains_key(&next) { continue }
            cache.insert(next, (cube, turn));
            queue.push_back(next);
        }
    }

    unreachable!()
}
