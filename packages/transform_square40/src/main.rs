use itertools::Itertools;
use log::LevelFilter;
use ranim::{
    animation::transform::GroupTransformAnimSchedule,
    color::palettes::manim::RED_C,
    glam::DVec3,
    items::{
        group::Group,
        vitem::{Circle, Square},
    },
    prelude::*,
};

#[scene]
pub struct TransformSquare40Scene;

const N: usize = 40;

impl TimelineConstructor for TransformSquare40Scene {
    fn construct(self, timeline: &RanimTimeline, _camera: &mut Rabject<CameraFrame>) {
        let buff = 0.1;
        let size = 8.0 / N as f64 - buff;

        let unit = size + buff;
        let start = DVec3::splat(-4.0 + unit / 2.0);
        let group = (0..N)
            .cartesian_product(0..N)
            .map(|(i, j)| {
                let mut item = Square(size).build();
                item.put_center_on(start + unit * DVec3::X * j as f64 + unit * DVec3::Y * i as f64);
                item
            })
            .collect::<Group<_>>();
        let mut group = timeline.insert(group);
        let group_target = (0..N)
            .cartesian_product(0..N)
            .map(|(i, j)| {
                let mut item = Circle(size / 2.0).build();
                item.set_stroke_color(RED_C)
                    .put_center_on(start + unit * DVec3::X * j as f64 + unit * DVec3::Y * i as f64);
                item
            })
            .collect::<Group<_>>();
        timeline.play(group.transform_to(group_target));
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
        TransformSquare40Scene,
        &AppOptions {
            pixel_size: (1920, 1080),
            frame_rate: 60,
            output_dir: "./output",
            output_filename: "Ranim.mp4",
            ..Default::default()
        },
    );
}
