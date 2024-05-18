use tiny_skia::*;

use rusty_grammar::geometry::{Point, Vector};
use rusty_grammar::system::{RunSettings, System};
use rusty_grammar::tokens::{TokenKind, TokenStore};

fn main() {
    // Here we set up some tokens so that we can use them
    // later.
    let plant = System::default();
    let forward = plant.add_token("Forward", TokenKind::Production).unwrap();
    let right = plant.add_token("+", TokenKind::Terminal).unwrap();
    let left = plant.add_token("-", TokenKind::Terminal).unwrap();
    let push = plant.add_token("[", TokenKind::Terminal).unwrap();
    let pop = plant.add_token("]", TokenKind::Terminal).unwrap();

    // For a "forward" production, every iteration we just extend it.
    plant.parse_production("Forward -> Forward Forward").expect("Unable to parse production");

    // X is our apex / growth token.
    // Every iteration of our derivation we replace the X tokens with some forward growth,
    // as well as branches (represented by our push and pop tokens, [ and ] respectively), with
    // forward growth sometimes being modified by fixed size angle changes (+ and -).
    plant.parse_production("X -> Forward + [ [ X ] - X ] - Forward [ - Forward X ] + X")
        .expect("Unable to parse production");

    // We start off with just a single apex token, and iterate for only 6 times.
    let start = plant.parse_prod_string("X").unwrap();
    let result = plant.derive(start, RunSettings::for_max_iterations(6)).unwrap().unwrap();

    const WIDTH : u32 = 500;

    // We will interpret the tokens as instructions to a LOGO turtle. The following
    // variables keep track of the position that we're at and the direction we're facing.
    // the stack is for the push / pop tokens.
    let mut pos_stack : Vec<(Point, Vector)> = Vec::new();
    let mut pos = Point::new(WIDTH as f64 / 2.0, WIDTH as f64);
    let mut dir = Vector::new(0.0, -3.0);
    let angle : f64 = -25.0; // degrees

    // Here we are using skia-tiny to draw, but we could use any appropriate library.
    // rusty-grammar has no dependencies on skia-tiny
    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 0, 0, 255);
    paint.anti_alias = true;

    // Every time we "branch" (using push and pop), we start a new path.
    let mut paths: Vec<Path> = Vec::new();

    let mut pb = PathBuilder::new();
    pb.move_to(pos.x() as f32, pos.y() as f32);

    for token in result {
        if token == forward {                   // interpret forward tokens.
            pos += dir;
            pb.line_to(pos.x() as f32, pos.y() as f32);
        } else if token == push {               // interpret push tokens. This starts "a branch" of the plant.
            pos_stack.push((pos, dir));
        } else if token == pop {                // interpret pop tokens. This ends "a branch", returning to where the branch started.
            (pos, dir) = pos_stack.pop().expect("Nothing to pop");
            if !pb.is_empty() {
                match pb.finish() {
                    None => {}
                    Some(p) => { paths.push(p) }
                }
            }
            pb = PathBuilder::new();
            pb.move_to(pos.x() as f32, pos.y() as f32);
        } else if token == left {               // Rotate a bit
            dir = dir.rotate(-angle);
        } else if token == right {              // Rotate in the other direction.
            dir = dir.rotate(angle);
        }
    }

    if !pb.is_empty() {
        match pb.finish() {
            None => {}
            Some(p) => { paths.push(p) }
        }
    }

    // Save all of this to an image.
    let stroke = Stroke { width: 1.0, line_cap: LineCap::Round, .. Stroke::default()};
    let mut pixmap = Pixmap::new(WIDTH, WIDTH).unwrap();
    pixmap.fill(Color::from_rgba8(255, 255, 255, 255));
    for path in &paths {
        pixmap.stroke_path(path, &paint, &stroke, Transform::identity(), None);
    }

    pixmap.save_png("skia-plant.png").unwrap();
}