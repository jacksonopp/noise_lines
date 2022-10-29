use nannou::prelude::*;
mod model;
mod lines;

fn main() {
    nannou::sketch(view).size(800, 800).run();
}

fn view(app: &App, frame: Frame) {
    let frames = app.elapsed_frames();
    let rect = app.window_rect();

    let mut m: Option<model::Model> = None;

    let draw = app.draw();

    if frames == 0 {
        m = Some(model::Model::new(rect));
        draw.background().color(CORNFLOWERBLUE);
    }


    if let Some(m) = m {
        m.lines.iter().for_each(|line| {
            line.points.iter().for_each(|p| {
                draw.ellipse()
                    .xy(*p)
                    .radius(2.0)
                    .color(BLACK);
            })
        })
    }

    draw.to_frame(app, &frame).unwrap();
}
