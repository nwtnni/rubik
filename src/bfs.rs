use std::collections::{HashMap, VecDeque};

use crate::state;

pub fn search(cube: &state::Cube) -> Vec<usize> {

    if cube.is_solved() { return vec![] }

    let mut queue: VecDeque<state::Cube> = VecDeque::new();
    let mut cache: HashMap<state::Cube, (state::Cube, usize)> = HashMap::new();

    queue.push_back(cube.clone());
    cache.insert(cube.clone(), (cube.clone(), 0));

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
        for d in 0..12 {
            let mut next = cube.clone();
            next.rotate(d);
            if cache.contains_key(&next) { continue }
            cache.insert(next, (cube, d));
            queue.push_back(next);
        }
    }

    unreachable!()
}
