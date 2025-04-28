use std::process::{self, Stdio};

use criterion::{BenchmarkId, Criterion, SamplingMode, criterion_group, criterion_main};

fn bench_scene(c: &mut Criterion, scene_name: &str) {
    let mut group = c.benchmark_group(scene_name);
    group.sampling_mode(SamplingMode::Linear).sample_size(10);
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

fn bench_square_scene(c: &mut Criterion, scene_name: &str) {
    let mut group = c.benchmark_group(scene_name);
    group.sampling_mode(SamplingMode::Linear).sample_size(10);
    let mut bench_group = |engine: &str| {
        for n in [5, 10, 20, 40] {
            group.bench_with_input(BenchmarkId::new(engine, n), &n, |b, n| {
                b.iter(|| {
                    process::Command::new("just")
                        .args([engine, format!("{}{n}", scene_name).as_str()])
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                });
            });
        }
    };
    for engine in ["ranim", "janim", "manim", "manimce"] {
        bench_group(engine);
    }
}

macro_rules! bench_scene {
    ($scene_name:expr, $custom_bench:ident) => {
        paste::paste! {
            mod [<$scene_name>]{
                use super::*;
                fn [<bench_ $scene_name>](c: &mut Criterion) {
                    $custom_bench(c, $scene_name);
                }
                criterion_group!(benches, [<bench_ $scene_name>]);
            }
        }
    };
}

// bench_scene!("static_tiger", bench_scene);
bench_scene!("static_square", bench_square_scene);

criterion_main!(static_square::benches);
