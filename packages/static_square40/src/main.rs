use log::LevelFilter;
use ranim::prelude::*;

#[scene(name = "Ranim")]
#[output(dir = "static_square40")]
fn static_square_40(r: &mut RanimScene) {
    bench_scenes::static_squares(r, 40);
}

fn main() {
    #[cfg(debug_assertions)]
    pretty_env_logger::formatted_timed_builder()
        .filter(Some("ranim"), LevelFilter::Trace)
        .init();
    #[cfg(not(debug_assertions))]
    pretty_env_logger::formatted_timed_builder()
        .filter(Some("ranim"), LevelFilter::Info)
        .init();
    render_scene(static_square_40_scene);
}
