use tiny_skia::*;

use rusty_systems::geometry::{Point, Vector};
use rusty_systems::strings::ProductionString;
use rusty_systems::system::{RunSettings, System};
use rusty_systems::symbols::SymbolStore;
use rusty_systems::parser::parse_prod_string;
use rusty_systems::productions::ProductionStore;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let plant = System::default();

    // For a "forward" production, every iteration we just extend it.
    plant.add_production("Forward -> Forward Forward")?;

    // X is our apex / growth symbol.
    // Every iteration of our derivation we replace the X symbols with some forward growth,
    // as well as branches (represented by our push and pop symbols, [ and ] respectively), with
    // forward growth sometimes being modified by fixed size angle changes (+ and -).
    plant.add_production("X -> Forward + [ [ X ] - X ] - Forward [ - Forward X ] + X")?;

    // We start off with just a single apex symbol, and iterate for only 6 times.
    let start = parse_prod_string("X")?;
    let result = plant.derive(start, RunSettings::for_max_iterations(6))?;
    
    let pixmap = interpret(&plant, &result);
    pixmap.save_png("target/skia-plant.png")?;
    
    Ok(())
}

fn interpret<T: SymbolStore>(system: &T, string: &ProductionString) -> Pixmap {
    const WIDTH : u32 = 500;

    // We need symbol values to interpret the strings.
    let forward = system.get_symbol("Forward").unwrap();
    let right = system.get_symbol("+").unwrap();
    let left = system.get_symbol("-").unwrap();
    let push = system.get_symbol("[").unwrap();
    let pop = system.get_symbol("]").unwrap();

    // We will interpret the symbols as instructions to a LOGO turtle. The following
    // variables keep track of the position that we're at and the direction we're facing.
    // the stack is for the push / pop symbols.
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

    for symbol in string {
        if symbol == forward {                   // interpret forward symbols.
            pos = pos + dir;
            pb.line_to(pos.x() as f32, pos.y() as f32);
        } else if symbol == push {               // interpret push symbols. This starts "a branch" of the plant.
            pos_stack.push((pos, dir));
        } else if symbol == pop {                // interpret pop symbols. This ends "a branch", returning to where the branch started.
            (pos, dir) = pos_stack.pop().expect("Nothing to pop");
            if !pb.is_empty() {
                match pb.finish() {
                    None => {}
                    Some(p) => { paths.push(p) }
                }
            }
            pb = PathBuilder::new();
            pb.move_to(pos.x() as f32, pos.y() as f32);
        } else if symbol == left {               // Rotate a bit
            dir = dir.rotate(-angle);
        } else if symbol == right {              // Rotate in the other direction.
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
