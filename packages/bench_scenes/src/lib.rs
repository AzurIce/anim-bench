use itertools::Itertools;
use ranim::{color::palettes::manim::RED_C, core::animation::StaticAnim};
use std::f64::consts::TAU;

use ranim::{
    anims::morph::MorphAnim,
    color::palettes::manim,
    glam::{usizevec3, DVec3},
    items::vitem::{
        geometry::{Circle, Square},
        svg::SvgItem,
        typst::typst_svg,
        VItem,
    },
    prelude::*,
};

#[scene(name = "Ranim")]
#[output(dir = "empty")]
pub fn empty(r: &mut RanimScene) {
    let _r_cam = r.insert(CameraFrame::default());
    r.timelines_mut().forward(1.0);
}

#[scene(name = "Ranim")]
#[output(dir = "static_tiger")]
pub fn static_tiger(r: &mut RanimScene) {
    let _r_cam = r.insert(CameraFrame::default());
    let tiger = SvgItem::new(include_str!("../../../assets/Ghostscript_Tiger.svg")).with(|s| {
        s.scale_to_with_stroke(ScaleHint::PorportionalY(8.0));
    });
    r.insert(tiger);
    r.timelines_mut().forward(1.0);
}

pub fn static_squares(r: &mut RanimScene, n: usize) {
    let _r_cam = r.insert(CameraFrame::default());
    let buff = 0.1;
    let size = 8.0 / n as f64 - buff;

    let unit = size + buff;
    let start = DVec3::splat(-4.0 + unit / 2.0);
    let group = (0..n)
        .cartesian_product(0..n)
        .map(|(i, j)| {
            Square::new(size).with(|s| {
                s.move_to(start + unit * DVec3::X * j as f64 + unit * DVec3::Y * i as f64);
            })
        })
        .collect::<Vec<_>>();
    r.insert(group);
    r.timelines_mut().forward(1.0);
}

pub fn transform_squares(r: &mut RanimScene, n: usize) {
    let _r_cam = r.insert(CameraFrame::default());
    let buff = 0.1;
    let size = 8.0 / n as f64 - buff;

    let unit = size + buff;
    let start = DVec3::splat(-4.0 + unit / 2.0);
    let mut group = (0..n)
        .cartesian_product(0..n)
        .map(|(i, j)| {
            Square::new(size).with(|s| {
                s.move_to(start + unit * DVec3::X * j as f64 + unit * DVec3::Y * i as f64);
            })
        })
        .map(VItem::from)
        .collect::<Vec<_>>();
    let group_target = (0..n)
        .cartesian_product(0..n)
        .map(|(i, j)| {
            Circle::new(size / 2.0).with(|c| {
                c.set_stroke_color(RED_C)
                    .move_to(start + unit * DVec3::X * j as f64 + unit * DVec3::Y * i as f64);
            })
        })
        .map(VItem::from)
        .collect::<Vec<_>>();

    let r_group = r.insert_empty();
    r.timeline_mut(r_group).play(group.morph_to(group_target));
}

#[scene(name = "Ranim")]
#[output(dir = "animating_pi")]
pub fn animating_pi(r: &mut RanimScene) {
    let cam = CameraFrame::default();
    let _r_cam = r.insert(cam.clone());

    let (w, h) = (10, 10);
    let mut pis = (0..h)
        .cartesian_product(0..w)
        .map(|(_, _)| {
            SvgItem::new(typst_svg("$pi$"))
                .with(|x| {
                    x.set_color(manim::WHITE);
                })
                .with(|x| {
                    x.scale_to_min(&[
                        ScaleHint::PorportionalY(cam.frame_height / h as f64),
                        ScaleHint::PorportionalX(cam.frame_height / w as f64),
                    ])
                    .discard()
                })
        })
        .collect::<Vec<_>>();
    pis.arrange_in_grid(
        usizevec3(10, 10, usize::MAX),
        DVec3::splat(8.0 / 10.0),
        DVec3::splat(0.2),
    )
    .move_to(DVec3::ZERO);

    let mut vitems = pis
        .into_iter()
        .flat_map(Vec::<VItem>::from)
        .collect::<Vec<_>>();
    vitems
        .scale_to(ScaleHint::PorportionalY(TAU - 0.25))
        .move_to(DVec3::ZERO);

    r.insert_with(|t| {
        t.play(vitems.clone().show())
            .forward(1.0)
            .play(
                vitems
                    .morph(|x| {
                        x.apply_complex_map(|c| c.exp());
                    })
                    .with_duration(5.0),
            )
            .forward(1.0)
            .play(
                vitems
                    .morph(|x| {
                        x.apply_point_func(|p| {
                            p.x += 0.5 * p.y.sin();
                            p.y += 0.5 * p.x.cos();
                        });
                    })
                    .with_duration(5.0),
            )
            .forward(1.0);
    });
    r.insert_time_mark(5.0, TimeMark::Capture("preview.png".to_string()));
}
