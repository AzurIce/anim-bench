use std::{
    process::{self, Stdio},
    time::Duration,
};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_scene(c: &mut Criterion, scene_name: &str) {
    let mut group = c.benchmark_group(scene_name);
    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(1200));
    let mut bench_group = |engine: &str| {
        group.bench_with_input(BenchmarkId::new(engine, 0), &0, |b, _| {
            b.iter(|| {
                process::Command::new("just")
                    .args([engine, scene_name])
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            });
        });
    };
    for engine in ["ranim", "janim", "manim", "manimce"] {
        bench_group(engine);
    }
}

macro_rules! bench_scene {
    ($scene_name:expr) => {
        paste::paste! {
            mod [<$scene_name>]{
                use super::*;
                fn [<bench_ $scene_name>](c: &mut Criterion) {
                    bench_scene(c, $scene_name);
                }
                criterion_group!(benches, [<bench_ $scene_name>]);
            }
        }
    };
}

bench_scene!("static_tiger");
bench_scene!("static_square_matrix");

criterion_main!(static_tiger::benches, static_square_matrix::benches);
