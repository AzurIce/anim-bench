use log::LevelFilter;
use ranim::prelude::*;

#[scene(name = "Ranim")]
#[output(dir = "transform_square5")]
fn transform_square_5(r: &mut RanimScene) {
    bench_scenes::transform_squares(r, 5);
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
    render_scene(transform_square_5_scene);
}
