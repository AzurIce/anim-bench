use std::fs;

use ranim::{components::ScaleHint, items::{group::Group, vitem::VItem}, prelude::*};

#[scene]
pub struct StaticTigerScene;

impl TimelineConstructor for StaticTigerScene {
    fn construct(self, timeline: &RanimTimeline, _camera: &mut Rabject<CameraFrame>) {
        let mut tiger =
            Group::<VItem>::from_svg(fs::read_to_string("assets/Ghostscript_Tiger.svg").unwrap());
        tiger.scale_to_with_stroke(ScaleHint::PorportionalHeight(8.0));
        let _tiger = timeline.insert(tiger);
        timeline.forward(1.0);
    }
}

fn main() {
    render_scene(
        StaticTigerScene,
        &AppOptions {
            pixel_size: (1920, 1080),
            frame_rate: 60,
            output_dir: "./output",
            output_filename: "Ranim.mp4",
            ..Default::default()
        },
    );
}
