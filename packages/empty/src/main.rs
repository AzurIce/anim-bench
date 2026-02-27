use std::time::Instant;

use log::LevelFilter;

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
    ranim::cmd::render_scene(&bench_scenes::empty::scene(), 2);
    println!("total_cost: {:?}", t.elapsed());
}
