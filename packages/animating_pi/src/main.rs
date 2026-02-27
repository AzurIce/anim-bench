use log::LevelFilter;

fn main() {
    #[cfg(debug_assertions)]
    pretty_env_logger::formatted_timed_builder()
        .filter(Some("ranim"), LevelFilter::Trace)
        .init();
    #[cfg(not(debug_assertions))]
    pretty_env_logger::formatted_timed_builder()
        .filter(Some("ranim"), LevelFilter::Info)
        .init();
    ranim::cmd::render_scene(&bench_scenes::animating_pi::scene(), 2);
}
