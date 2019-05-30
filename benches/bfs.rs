use criterion::{criterion_group, criterion_main};

fn bfs_benchmark(c: &mut criterion::Criterion) {
    c.bench_function_over_inputs(
        "BFS",
        |b, &&turns| {
            let mut cube = rubik::state::Cube::default();
            for _ in 0..turns { cube.rotate(rand::random::<usize>()); }
            b.iter(|| rubik::bfs::search(&cube));
        },
        &[1, 2, 3, 4, 5],
    );
}

criterion_group!(benches, bfs_benchmark);
criterion_main!(benches);
