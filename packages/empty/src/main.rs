use std::time::Instant;

use log::LevelFilter;
use ranim::prelude::*;
use bench_scenes::empty_scene;

fn main() {
    let t = Instant::now();
    #[cfg(debug_assertions)]
    pretty_env_logger::formatted_timed_builder()
        .filter(Some("ranim"), LevelFilter::Trace)
        .init();
    #[cfg(not(debug_assertions))]
    pretty_env_logger::formatted_timed_builder()
        .filter(Some("ranim"), LevelFilter::Info)
        .init();
    render_scene(empty_scene);
    println!("total_cost: {:?}", t.elapsed());
}
