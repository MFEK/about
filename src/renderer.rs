//! Skia renderer.
use skulpin::skia_safe::Canvas;
use skulpin::skia_safe::{Matrix, Paint, PaintStyle, Path};

use crate::consts::*;

use std::convert::TryInto;
fn frames_to_paths() -> [Path; LEN_FRAMES] {
    let mut paths = vec![Path::new(); LEN_FRAMES];
    for i in 0..LEN_FRAMES {
        let d = &SVG_FRAMES[i];
        let matrix = Matrix::scale((0.333, 0.333));
        paths[i] = Path::from_svg(d).unwrap();
        paths[i].transform(&matrix);
    }
    paths.try_into().unwrap()
}

use lazy_static::lazy_static;
lazy_static! {
    pub(crate) static ref PATHS: [Path; LEN_FRAMES] = frames_to_paths();
    pub(crate) static ref SPLASH: Path = {
        let mut path = Path::from_svg(&SVG_SPLASH).unwrap();
        let matrix = Matrix::translate((300., 0.)) * Matrix::scale((1.33, 1.33));
        path.transform(&matrix);
        path
    };
}

pub(crate) fn render_frame(canvas: &mut Canvas, frame: u128) {
    canvas.clear(0xff_ffffff);
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(5.);
    paint.set_anti_alias(true);
    // i is a number out of 180, corrected to force some fps ≈60fps
    canvas.draw_path(&PATHS[frame as usize], &paint);
    paint.set_style(PaintStyle::Fill);
    canvas.draw_path(&SPLASH, &paint);
}
