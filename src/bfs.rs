use std::collections::{HashMap, VecDeque};

use crate::state;
use crate::types;

pub fn search(cube: &state::Cube) -> Vec<types::Turn> {

    if cube.is_solved() { return vec![] }

    let mut queue: VecDeque<state::Cube> = VecDeque::new();
    let mut cache: HashMap<state::Cube, (state::Cube, types::Turn)> = HashMap::new();

    queue.push_back(*cube);
    cache.insert(*cube, (*cube, (types::Face::U, types::Spin::CW).into()));

    while let Some(cube) = queue.pop_front() {
        if cube.is_solved() {
            let mut back = &cache[&cube];
            let mut path = Vec::new();
            path.insert(0, back.1);
            while let Some(next) = cache.get(&back.0) {
                if next.0 == back.0 { break }
                path.insert(0, next.1);
                back = next;
            }
            return path
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
