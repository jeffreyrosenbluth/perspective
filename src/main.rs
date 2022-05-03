use rand::prelude::*;
use rand_pcg::Pcg64;
use wassily::prelude::*;

pub struct Model {
    noise_scale: f32,
    niose_factor: f32,
    margin: f32,
    img_index: usize,
    desat: f32,
    seed: u64,
    photo: &'static str,
}

impl Model {
    pub fn new(
        noise_scale: f32,
        niose_factor: f32,
        margin: f32,
        img_index: usize,
        desat: f32,
        seed: u64,
        photo: &'static str,
    ) -> Self {
        Self {
            noise_scale,
            niose_factor,
            margin,
            img_index,
            desat,
            seed,
            photo,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            noise_scale: 0.033,
            niose_factor: 0.0,
            margin: 250.0,
            img_index: 58,
            desat: 0.75,
            seed: 1,
            photo: "/Users/jeffreyrosenbluth/Rust/sketches/perspective/assets/balcony.png",
        }
    }
}

fn main() {
    let model = Model::default();
    let mut sketch = Sketch::new(10_800, 7_200, view_fn)
        .dir("output")
        .name("perspective")
        .source(file!());
    sketch.run(&model);
    sketch.save();
}

fn view_fn(canvas: &mut Canvas, model: &Model) {
    let mut palette = Palette::with_img(model.photo, Some(50));
    palette.desaturate(model.desat);
    canvas.fill((*LIGHTSKYBLUE).tint(0.80));
    let ts = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.9, 1.0];
    let us = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.9, 1.0];
    let vs = [0.0, 0.1, 0.2, 0.3, 0.7, 0.8, 0.9, 1.0];
    let left = pt(-model.margin, canvas.height() / 2);
    let right = pt(canvas.width() + model.margin as u32, canvas.height() / 2);
    let top = pt(canvas.width() / 2, 0);
    let bottom = pt(canvas.width() / 2, canvas.height());

    let mut quads = perspective_quads(
        left,
        top,
        bottom,
        right,
        &ts,
        &us,
        &vs,
        model.noise_scale,
        model.niose_factor,
    );

    let mut rng = Pcg64::seed_from_u64(model.seed);
    for (k, quad) in quads.iter().enumerate() {
        let c = palette.rand_color();
        let ps: Vec<Point> = quad.to_vec();
        let q = if k == model.img_index {
            100
        } else {
            rng.gen_range(0..100)
        };
        let bbox = bounding_box(&ps, 100.0);
        let w = bbox.width() as u32;
        let h = bbox.height() as u32;
        let n = 0;
        match q {
            // Stipple
            0..=9 => {
                let texture = stipple_texture(w, h, *WHITE, 10.0);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(ps.clone(), n, Trail::Closed))
                    .fill_color(c)
                    .stroke_color(c)
                    .build()
                    .draw(canvas);
                ShapeBuilder::new()
                    .points(&chaiken(ps, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            // Wood
            10..=19 => {
                let c1 = palette.rand_color();
                let c2 = palette.rand_color();
                let texture = wood(w, h, c1, c2, 0.5);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(ps, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            // Marble
            20..=29 => {
                let c1 = palette.rand_color();
                let c2 = palette.rand_color();
                let texture = marble(w, h, c1, c2, 0.5);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(ps, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            // Ridge
            30..=39 => {
                let c1 = palette.rand_color();
                let c2 = palette.rand_color();
                let texture = ridge(w, h, c1, c2, 0.5);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(ps, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            // Stripe
            40..=49 => {
                let c1 = palette.rand_color();
                let c2 = palette.rand_color();
                let texture = horizontal_stripe(w, h, c1, c2, 8.0);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(ps, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            // Sand
            50..=59 => {
                let c1 = palette.rand_color();
                let c2 = palette.rand_color();
                let texture = sand(w, h, c1, c2, 10.0);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(ps, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            100 => {
                let mut img = open(model.photo).unwrap();
                img = img.resize_to_fill(
                    bbox.width() as u32,
                    bbox.height() as u32,
                    imageops::FilterType::CatmullRom,
                );
                let texture: Canvas = match img {
                    DynamicImage::ImageLuma8(_) => panic!("image format not supported"),
                    DynamicImage::ImageLumaA8(_) => panic!("image format not supported"),
                    DynamicImage::ImageRgb8(_) => img.as_rgb8().unwrap().into(),
                    DynamicImage::ImageRgba8(_) => img.as_rgba8().unwrap().into(),
                    DynamicImage::ImageLuma16(_) => panic!("image format not supported"),
                    DynamicImage::ImageLumaA16(_) => panic!("image format not supported"),
                    DynamicImage::ImageRgb16(_) => panic!("image format not supported"),
                    DynamicImage::ImageRgba16(_) => panic!("image format not supported"),
                    DynamicImage::ImageRgb32F(_) => panic!("image format not supported"),
                    DynamicImage::ImageRgba32F(_) => panic!("image format not supported"),
                    _ => todo!(),
                };
                let mut canvas_pat = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut canvas_pat, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(ps, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            _ => {
                ShapeBuilder::new()
                    .points(&chaiken(ps.clone(), n, Trail::Closed))
                    .fill_color(c)
                    .stroke_color(c)
                    .build()
                    .draw(canvas);
                let g = Grain::new(bbox.width() as u32, bbox.height() as u32, 0.15, 0.10);
                let p = g.grain();
                ShapeBuilder::new()
                    .points(&chaiken(ps, n, Trail::Closed))
                    .fill_paint(&p)
                    .stroke_color(c)
                    .build()
                    .draw(canvas);
            }
        }
    }
}
