use rand::prelude::*;
use rand_pcg::Pcg64;
use wassily::prelude::*;
use image::imageops::colorops::*;

pub struct Model {
    noise_scale: f32,
    niose_factor: f32,
    margin: u32,
    img_index: usize,
    desat: f32,
    seed: u64,
    photo: &'static str,
}

impl Model {
    pub fn new(
        noise_scale: f32,
        niose_factor: f32,
        margin: u32,
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
            noise_scale: 0.01,
            niose_factor: 0.0,
            margin: 750,
            img_index: 59,
            desat: 0.5,
            seed: 717,
            photo: "/Users/jeffreyrosenbluth/Rust/sketches/perspective/assets/rivendell.png",
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

fn sharpen(img: &DynamicImage, factor: f32) -> DynamicImage {
    let mut kernel = [0.0; 9];
    let n = -factor;
    let p = factor * 4.0 + 1.0;
    kernel[1] = n;
    kernel[3] = n;
    kernel[4] = p;
    kernel[5] = n;
    kernel[7] = n;
    img.filter3x3(&kernel)
}

fn view_fn(canvas: &mut Canvas, model: &Model) {
    let mut palette = Palette::with_img(model.photo, Some(100));
    palette.desaturate(model.desat);
    // canvas.fill((*LIGHTSKYBLUE).tint(0.80));
    let mut img = open(model.photo).unwrap();
    img = sharpen(&img, 10.0);
    img = DynamicImage::ImageRgba8(grayscale_with_type_alpha(&img));
    img = img.resize_to_fill(
        canvas.width() as u32,
        canvas.height() as u32,
        imageops::FilterType::Lanczos3,
    );
    // img = sharpen(&img, 50.0);
    let photo: Canvas = match img {
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
    let cv: Canvas = photo.into();
    *canvas = cv;
    let left_stops = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.9, 1.0];
    let right_stops = [0.0, 0.1, 0.5, 0.65, 0.75, 0.9, 1.0];
    let vert_stops = [0.0, 0.1, 0.2, 0.3, 0.7, 0.8, 0.9, 1.0];
    let left_top = pt(0, canvas.height() / 2 - model.margin);
    let left_bottom = pt(0, canvas.height() / 2 + model.margin);
    let right_top = pt(canvas.width(), canvas.height() / 2 - model.margin);
    let right_bottom = pt(canvas.width(), canvas.height() / 2 + model.margin);
    let middle_top = pt(canvas.width() / 2 - 1000, 100);
    let middle_bottom = pt(canvas.width() / 2 - 1000, canvas.height() - 400);

    let mut quads = Grid::perspective_grid(
        left_top,
        left_bottom,
        middle_top,
        middle_bottom,
        right_top,
        right_bottom,
        &vert_stops,
        &left_stops,
        &right_stops,
    );

    quads.0.data = warp_points(&quads.0.data, model.noise_scale, model.niose_factor);

    let mut rng = Pcg64::seed_from_u64(model.seed);
    for (k, quad) in quads.quads().iter().enumerate() {
        let c = palette.rand_color();
        let q = if k == model.img_index {
            100
        } else {
            rng.gen_range(0..100)
        };
        let bbox = bounding_box(quad, 100.0);
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
                    .points(&chaiken(quad, n, Trail::Closed))
                    .fill_color(c)
                    .stroke_color(c)
                    .build()
                    .draw(canvas);
                ShapeBuilder::new()
                    .points(&chaiken(quad, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            // Wood
            10..=15 => {
                let c1 = palette.rand_color();
                let c2 = palette.rand_color();
                let texture = wood(w, h, c1, c2, 0.5);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(quad, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            // Marble
            16..=29 => {
                let c1 = palette.rand_color();
                let c2 = palette.rand_color();
                let texture = marble(w, h, c1, c2, 0.5);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(quad, n, Trail::Closed))
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
                    .points(&chaiken(quad, n, Trail::Closed))
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
                    .points(&chaiken(quad, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            // Sand
            50..=55 => {
                let c1 = palette.rand_color();
                let c2 = palette.rand_color();
                let texture = sand(w, h, c1, c2, 10.0);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(quad, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            // Granite
            56..=69 => {
                let c1 = palette.rand_color();
                let c2 = palette.rand_color();
                let texture = granite(w, h, c1, c2, 1.0);
                let mut pattern_canvas = Canvas::new(canvas.width(), canvas.height());
                let paint = pattern(&texture, &mut pattern_canvas, bbox);
                ShapeBuilder::new()
                    .points(&chaiken(quad, n, Trail::Closed))
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
                    imageops::FilterType::Lanczos3,
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
                    .points(&chaiken(quad, n, Trail::Closed))
                    .fill_paint(&paint)
                    .no_stroke()
                    .build()
                    .draw(canvas);
            }
            _ => {
                ShapeBuilder::new()
                    .points(&chaiken(quad, n, Trail::Closed))
                    .fill_color(c)
                    .stroke_color(c)
                    .build()
                    .draw(canvas);
                let g = Grain::new(bbox.width() as u32, bbox.height() as u32, 0.15, 0.10);
                let p = g.grain();
                ShapeBuilder::new()
                    .points(&chaiken(quad, n, Trail::Closed))
                    .fill_paint(&p)
                    .stroke_color(c)
                    .build()
                    .draw(canvas);
            }
        }
    }
}
