use tiny_skia::*;

use rusty_grammar::geometry::{Point, Vector};
use rusty_grammar::system::{RunSettings, System};
use rusty_grammar::tokens::{TokenKind, TokenStore};

fn main() {
    let plant = System::default();
    let forward = plant.add_token("Forward", TokenKind::Production).unwrap();
    let right = plant.add_token("+", TokenKind::Terminal).unwrap();
    let left = plant.add_token("-", TokenKind::Terminal).unwrap();
    let push = plant.add_token("[", TokenKind::Terminal).unwrap();
    let pop = plant.add_token("]", TokenKind::Terminal).unwrap();

    plant.parse_production("Forward -> Forward Forward")
        .expect("Unable to parse production");
    // X → F+[[X]-X]-F[-FX]+X)

    plant.parse_production("X -> Forward + [ [ X ] - X ] - Forward [ - Forward X ] + X")
        .expect("Unable to parse production");

    let start = plant.parse_prod_string("X").unwrap();
    let result = plant.derive(start, RunSettings::for_max_iterations(6)).unwrap().unwrap();
    
    const WIDTH : u32 = 500;

    let mut pos_stack : Vec<(Point, Vector)> = Vec::new();
    let mut pos = Point::new(WIDTH as f64 / 2.0, WIDTH as f64);
    let mut dir = Vector::new(0.0, -3.0);
    let angle : f64 = -25.0; // degrees


    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 0, 0, 255);
    paint.anti_alias = true;


    let mut paths : Vec<Path> = Vec::new();

        let mut pb = PathBuilder::new();
        pb.move_to(pos.x() as f32, pos.y() as f32);

        for token in result {
            if token == forward {
                pos += dir;
                pb.line_to(pos.x() as f32, pos.y() as f32);
            } else if token == push {
                pos_stack.push((pos,dir));
            } else if token == pop {
                (pos,dir) = pos_stack.pop().expect("Nothing to pop");
                if !pb.is_empty() {
                    match pb.finish() {
                        None => {}
                        Some(p) => {paths.push(p)}
                    }
                }
                pb = PathBuilder::new();
                pb.move_to(pos.x() as f32, pos.y() as f32);
            } else if token == left {
                dir = dir.rotate(-angle);
            } else if token == right {
                dir = dir.rotate(angle);
            }
        }

    if !pb.is_empty() {
        match pb.finish() {
            None => {}
            Some(p) => {paths.push(p)}
        }
    }

    let stroke = Stroke { width: 1.0, line_cap: LineCap::Round, .. Stroke::default()};

    let mut pixmap = Pixmap::new(WIDTH, WIDTH).unwrap();
    for path in &paths {
        pixmap.stroke_path(path, &paint, &stroke, Transform::identity(), None);
    }

    pixmap.save_png("image.png").unwrap();
}