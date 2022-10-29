use super::lines::solidline::SolidLine;
use nannou::prelude::*;

#[derive(Debug)]
pub struct Line {
    pub points: Vec<Vec2>,
}

impl Line {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
}

#[derive(Debug)]
pub struct Model {
    pub lines: Vec<Line>,
}

const NUM_LINES: u8 = 20;
const NUM_STEPS: u8 = 20;

impl Model {
    pub fn new(rect: Rect) -> Self {
        println!("Height: {}", rect.h());

        let height_div = rect.h() / NUM_LINES as f32;
        let half_height = rect.h() / 2.0;

        let width_div = rect.w() / NUM_STEPS as f32;
        let half_width = rect.w() / 2.0;

        let mut lines = vec![];

        for i in 0..NUM_LINES {
            for j in 0..NUM_STEPS {
                let x = (j as f32 * width_div) - half_width;
                let y = (i as f32 * height_div) - half_height;
                let point = Point2::new(x, y);
                let line = Line::new(vec![point]);
                lines.push(line);
            }

        }

        println!("point 0: {:?}, last: {:?}", &lines[0], &lines[&lines.len() - 1]);

        Self { lines }
    }
}
