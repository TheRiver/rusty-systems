use tiny_skia::*;

use rusty_systems::geometry::{Point, Vector};
use rusty_systems::strings::ProductionString;
use rusty_systems::system::{RunSettings, System};
use rusty_systems::tokens::TokenStore;

fn main() {
    let plant = System::default();

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
    
    let pixmap = interpret(&plant, &result);
    pixmap.save_png("target/skia-plant.png").unwrap();
}

fn interpret<T: TokenStore>(system: &T, string: &ProductionString) -> Pixmap {
    const WIDTH : u32 = 500;

    // We need token values to interpret the strings.
    let forward = system.get_token("Forward").unwrap();
    let right = system.get_token("+").unwrap();
    let left = system.get_token("-").unwrap();
    let push = system.get_token("[").unwrap();
    let pop = system.get_token("]").unwrap();

    // We will interpret the tokens as instructions to a LOGO turtle. The following
    // variables keep track of the position that we're at and the direction we're facing.
    // the stack is for the push / pop tokens.
    let mut pos_stack : Vec<(Point, Vector)> = Vec::new();
    let mut pos = Point::new(WIDTH as f64 / 2.0, WIDTH as f64);
    let mut dir = Vector::new(0.0, -3.0);
    let angle : f64 = 22.5; // degrees

    // Here we are using skia-tiny to draw, but we could use any appropriate library.
    // rusty-systems has no dependencies on skia-tiny
    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 0, 0, 255);
    paint.anti_alias = true;

    // Every time we "branch" (using push and pop), we start a new path.
    let mut paths: Vec<Path> = Vec::new();

    let mut pb = PathBuilder::new();
    pb.move_to(pos.x() as f32, pos.y() as f32);

    for token in string {
        if token == forward {                   // interpret forward tokens.
            pos = pos + dir;
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

    pixmap
}
