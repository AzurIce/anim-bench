use itertools::Itertools;
use log::LevelFilter;
use ranim::{
    glam::DVec3,
    items::{group::Group, vitem::Square},
    prelude::*,
};

#[scene]
pub struct StaticSquare40Scene;

const N: usize = 40;

impl TimelineConstructor for StaticSquare40Scene {
    fn construct(self, timeline: &RanimTimeline, _camera: &mut Rabject<CameraFrame>) {
        let buff = 0.1;
        let size = 8.0 / N as f64 - buff;

        let unit = size + buff;
        let start = DVec3::splat(-4.0 + unit / 2.0);
        let group = (0..N)
            .cartesian_product(0..N)
            .map(|(i, j)| {
                let mut square = Square(size).build();
                square
                    .put_center_on(start + unit * DVec3::X * j as f64 + unit * DVec3::Y * i as f64);
                square
            })
            .collect::<Group<_>>();
        timeline.insert(group);
        timeline.forward(1.0);
    }
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
    render_scene(
        StaticSquare40Scene,
        &AppOptions {
            pixel_size: (1920, 1080),
            frame_rate: 60,
            output_dir: "./output",
            output_filename: "Ranim.mp4",
            ..Default::default()
        },
    );
}
