//! Skia renderer.

use skulpin::skia_safe::Canvas;
use skulpin::skia_safe::Picture;
use skulpin::skia_safe::{Matrix, Paint, PaintStyle, Path};

use std::fs::{self, File};
use std::io::Read;

static SVG_SPLASH: &str = include_str!("../resources/about_flat.svgp");
static SVG_FRAMES: &[&str] = &[include_str!("../resources/0001.svgp"), include_str!("../resources/0002.svgp"), include_str!("../resources/0003.svgp"), include_str!("../resources/0004.svgp"), include_str!("../resources/0005.svgp"), include_str!("../resources/0006.svgp"), include_str!("../resources/0007.svgp"), include_str!("../resources/0008.svgp"), include_str!("../resources/0009.svgp"), include_str!("../resources/0010.svgp"), include_str!("../resources/0011.svgp"), include_str!("../resources/0012.svgp"), include_str!("../resources/0013.svgp"), include_str!("../resources/0014.svgp"), include_str!("../resources/0015.svgp"), include_str!("../resources/0016.svgp"), include_str!("../resources/0017.svgp"), include_str!("../resources/0018.svgp"), include_str!("../resources/0019.svgp"), include_str!("../resources/0020.svgp"), include_str!("../resources/0021.svgp"), include_str!("../resources/0022.svgp"), include_str!("../resources/0023.svgp"), include_str!("../resources/0024.svgp"), include_str!("../resources/0025.svgp"), include_str!("../resources/0026.svgp"), include_str!("../resources/0027.svgp"), include_str!("../resources/0028.svgp"), include_str!("../resources/0029.svgp"), include_str!("../resources/0030.svgp"), include_str!("../resources/0031.svgp"), include_str!("../resources/0032.svgp"), include_str!("../resources/0033.svgp"), include_str!("../resources/0034.svgp"), include_str!("../resources/0035.svgp"), include_str!("../resources/0036.svgp"), include_str!("../resources/0037.svgp"), include_str!("../resources/0038.svgp"), include_str!("../resources/0039.svgp"), include_str!("../resources/0040.svgp"), include_str!("../resources/0041.svgp"), include_str!("../resources/0042.svgp"), include_str!("../resources/0043.svgp"), include_str!("../resources/0044.svgp"), include_str!("../resources/0045.svgp"), include_str!("../resources/0046.svgp"), include_str!("../resources/0047.svgp"), include_str!("../resources/0048.svgp"), include_str!("../resources/0049.svgp"), include_str!("../resources/0050.svgp"), include_str!("../resources/0051.svgp"), include_str!("../resources/0052.svgp"), include_str!("../resources/0053.svgp"), include_str!("../resources/0054.svgp"), include_str!("../resources/0055.svgp"), include_str!("../resources/0056.svgp"), include_str!("../resources/0057.svgp"), include_str!("../resources/0058.svgp"), include_str!("../resources/0059.svgp"), include_str!("../resources/0060.svgp"), include_str!("../resources/0061.svgp"), include_str!("../resources/0062.svgp"), include_str!("../resources/0063.svgp"), include_str!("../resources/0064.svgp"), include_str!("../resources/0065.svgp"), include_str!("../resources/0066.svgp"), include_str!("../resources/0067.svgp"), include_str!("../resources/0068.svgp"), include_str!("../resources/0069.svgp"), include_str!("../resources/0070.svgp"), include_str!("../resources/0071.svgp"), include_str!("../resources/0072.svgp"), include_str!("../resources/0073.svgp"), include_str!("../resources/0074.svgp"), include_str!("../resources/0075.svgp"), include_str!("../resources/0076.svgp"), include_str!("../resources/0077.svgp"), include_str!("../resources/0078.svgp"), include_str!("../resources/0079.svgp"), include_str!("../resources/0080.svgp"), include_str!("../resources/0081.svgp"), include_str!("../resources/0082.svgp"), include_str!("../resources/0083.svgp"), include_str!("../resources/0084.svgp"), include_str!("../resources/0085.svgp"), include_str!("../resources/0086.svgp"), include_str!("../resources/0087.svgp"), include_str!("../resources/0088.svgp"), include_str!("../resources/0089.svgp"), include_str!("../resources/0090.svgp"), include_str!("../resources/0091.svgp"), include_str!("../resources/0092.svgp"), include_str!("../resources/0093.svgp"), include_str!("../resources/0094.svgp"), include_str!("../resources/0095.svgp"), include_str!("../resources/0096.svgp"), include_str!("../resources/0097.svgp"), include_str!("../resources/0098.svgp"), include_str!("../resources/0099.svgp"), include_str!("../resources/0100.svgp"), include_str!("../resources/0101.svgp"), include_str!("../resources/0102.svgp"), include_str!("../resources/0103.svgp"), include_str!("../resources/0104.svgp"), include_str!("../resources/0105.svgp"), include_str!("../resources/0106.svgp"), include_str!("../resources/0107.svgp"), include_str!("../resources/0108.svgp"), include_str!("../resources/0109.svgp"), include_str!("../resources/0110.svgp"), include_str!("../resources/0111.svgp"), include_str!("../resources/0112.svgp"), include_str!("../resources/0113.svgp"), include_str!("../resources/0114.svgp"), include_str!("../resources/0115.svgp"), include_str!("../resources/0116.svgp"), include_str!("../resources/0117.svgp"), include_str!("../resources/0118.svgp"), include_str!("../resources/0119.svgp"), include_str!("../resources/0120.svgp"), include_str!("../resources/0121.svgp"), include_str!("../resources/0122.svgp"), include_str!("../resources/0123.svgp"), include_str!("../resources/0124.svgp"), include_str!("../resources/0125.svgp"), include_str!("../resources/0126.svgp"), include_str!("../resources/0127.svgp"), include_str!("../resources/0128.svgp"), include_str!("../resources/0129.svgp"), include_str!("../resources/0130.svgp"), include_str!("../resources/0131.svgp"), include_str!("../resources/0132.svgp"), include_str!("../resources/0133.svgp"), include_str!("../resources/0134.svgp"), include_str!("../resources/0135.svgp"), include_str!("../resources/0136.svgp"), include_str!("../resources/0137.svgp"), include_str!("../resources/0138.svgp"), include_str!("../resources/0139.svgp"), include_str!("../resources/0140.svgp"), include_str!("../resources/0141.svgp"), include_str!("../resources/0142.svgp"), include_str!("../resources/0143.svgp"), include_str!("../resources/0144.svgp"), include_str!("../resources/0145.svgp"), include_str!("../resources/0146.svgp"), include_str!("../resources/0147.svgp"), include_str!("../resources/0148.svgp"), include_str!("../resources/0149.svgp"), include_str!("../resources/0150.svgp"), include_str!("../resources/0151.svgp"), include_str!("../resources/0152.svgp"), include_str!("../resources/0153.svgp"), include_str!("../resources/0154.svgp"), include_str!("../resources/0155.svgp"), include_str!("../resources/0156.svgp"), include_str!("../resources/0157.svgp"), include_str!("../resources/0158.svgp"), include_str!("../resources/0159.svgp"), include_str!("../resources/0160.svgp"), include_str!("../resources/0161.svgp"), include_str!("../resources/0162.svgp"), include_str!("../resources/0163.svgp"), include_str!("../resources/0164.svgp"), include_str!("../resources/0165.svgp"), include_str!("../resources/0166.svgp"), include_str!("../resources/0167.svgp"), include_str!("../resources/0168.svgp"), include_str!("../resources/0169.svgp"), include_str!("../resources/0170.svgp"), include_str!("../resources/0171.svgp"), include_str!("../resources/0172.svgp"), include_str!("../resources/0173.svgp"), include_str!("../resources/0174.svgp"), include_str!("../resources/0175.svgp"), include_str!("../resources/0176.svgp"), include_str!("../resources/0177.svgp"), include_str!("../resources/0178.svgp"), include_str!("../resources/0179.svgp"), include_str!("../resources/0180.svgp"), include_str!("../resources/0181.svgp"),];

use std::convert::TryInto;
fn frames_to_paths() -> [Path; 180] {
    let mut paths = vec![Path::new(); 180];
    for i in 0..180 {
        let d = &SVG_FRAMES[i];
        let matrix = Matrix::scale((0.333, 0.333));
        paths[i] = Path::from_svg(d).unwrap();
        paths[i].transform(&matrix);
    }
    paths.try_into().unwrap_or_else(|_v: Vec<Path>| unreachable!())
}

use lazy_static::lazy_static;
lazy_static! {
    pub static ref PATHS: [Path; 180] = frames_to_paths();
    pub static ref SPLASH: Path = { 
        let mut path = Path::from_svg(&SVG_SPLASH).unwrap();
        let matrix = Matrix::translate((300., 0.)) * Matrix::scale((1.33, 1.33));
        path.transform(&matrix);
        path
    };
}

use std::num::Wrapping;
pub fn render_frame(canvas: &mut Canvas, i: u128) {
    canvas.clear(0xff_ffffff);
    let mut paint = Paint::default();
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(5.);
    paint.set_anti_alias(true);
    // i is a number out of 180, corrected to force 45fps
    canvas.draw_path(&PATHS[i as usize], &paint);
    paint.set_style(PaintStyle::Fill);
    canvas.draw_path(&SPLASH, &paint);
}
