use wassily::prelude::*;
use rand::prelude::*;

pub struct Model {}

fn main() {
    let model = Model {};
    let mut sketch = Sketch::new(3200, 2400, view_fn)
        .dir("output")
        .name("perspective")
        .source(file!());
    sketch.run(&model);
    sketch.save();
}

fn view_fn(canvas: &mut Canvas, model: &Model) {
    let mut palette = Palette::new(vec![]);
    canvas.fill(*CORNSILK);
    let ts = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.9, 1.0];
    let us = [0.0, 0.1, 0.2, 0.3, 0.5, 0.6, 0.7, 1.0];
    let vs = [0.0, 0.1, 0.2, 0.3, 0.5, 0.6, 0.8, 0.9, 1.0];
    let left = pt(-500, canvas.height() / 2);
    let right = pt(500 + canvas.width(), canvas.height() / 2);
    let top = pt(canvas.width() / 2 - 250, 50);
    let bottom = pt(canvas.width() / 2 - 250, canvas.height() - 50);

    let quads = perspective_quads(left, top, bottom, right, &ts, &us, &vs);
    palette.set_seed(42);

    let mut rng = thread_rng();
    for q in quads {
        let c = palette.rand_lab();
        let ps: Vec<Point> = q.to_vec();
        let (n, d) = if rng.gen_range(0.0..1.0) < 0.15 {(5, 0.0)} else{(0, 0.7)};
        let s = chaiken(ps, n, Trail::Closed);
        ShapeBuilder::new()
            .points(&s)
            .fill_color(c.desaturate(d))
            .no_stroke()
            .build()
            .draw(canvas);
    }
}
