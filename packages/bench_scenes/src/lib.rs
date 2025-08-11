use glam::DVec3;
use itertools::Itertools;
use ranim::animation::transform::TransformAnim;
use ranim::color::palettes::manim::RED_C;
use ranim::glam;
use ranim::items::Group;
use ranim::items::vitem::VItem;
use ranim::items::vitem::geometry::{Circle, Square};
use ranim::items::vitem::svg::SvgItem;
use ranim::prelude::*;

#[scene(name = "Ranim")]
#[output(dir = "empty")]
pub fn empty(r: &mut RanimScene) {
    let _r_cam = r.insert_and_show(CameraFrame::default());
    r.timelines_mut().forward(1.0);
}

#[scene(name = "Ranim")]
#[output(dir = "static_tiger")]
pub fn static_tiger(r: &mut RanimScene) {
    let _r_cam = r.insert_and_show(CameraFrame::default());
    let tiger = SvgItem::new(include_str!("../../../assets/Ghostscript_Tiger.svg"));
    r.insert_and_show(tiger);
    r.timelines_mut().forward(1.0);
}

pub fn static_squares(r: &mut RanimScene, n: usize) {
    let _r_cam = r.insert_and_show(CameraFrame::default());
    let buff = 0.1;
    let size = 8.0 / n as f64 - buff;

    let unit = size + buff;
    let start = DVec3::splat(-4.0 + unit / 2.0);
    let group = (0..n)
        .cartesian_product(0..n)
        .map(|(i, j)| {
            Square::new(size).with(|s| {
                s.put_center_on(start + unit * DVec3::X * j as f64 + unit * DVec3::Y * i as f64);
            })
        })
        .collect::<Group<_>>();
    r.insert_and_show(group);
    r.timelines_mut().forward(1.0);
}

pub fn transform_squares(r: &mut RanimScene, n: usize) {
    let _r_cam = r.insert_and_show(CameraFrame::default());
    let buff = 0.1;
    let size = 8.0 / n as f64 - buff;

    let unit = size + buff;
    let start = DVec3::splat(-4.0 + unit / 2.0);
    let group = (0..n)
        .cartesian_product(0..n)
        .map(|(i, j)| {
            Square::new(size).with(|s| {
                s.put_center_on(start + unit * DVec3::X * j as f64 + unit * DVec3::Y * i as f64);
            })
        })
        .map(VItem::from)
        .collect::<Group<_>>();
    let group_target = (0..n)
        .cartesian_product(0..n)
        .map(|(i, j)| {
            Circle::new(size / 2.0).with(|c| {
                c.set_stroke_color(RED_C)
                    .put_center_on(start + unit * DVec3::X * j as f64 + unit * DVec3::Y * i as f64);
            })
        })
        .map(VItem::from)
        .collect::<Group<_>>();

    let r_group = r.insert(group);
    r.timeline_mut(&r_group)
        .play_with(|g| g.transform_to(group_target));
}
