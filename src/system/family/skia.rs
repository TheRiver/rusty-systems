use tiny_skia::{Color, LineCap, Paint, PathBuilder, Pixmap, Stroke, Transform};

use crate::geometry::{Path};
use crate::prelude::{ProductionString, System};
use crate::Result;
use crate::system::family::{abop_family, get_or_init_family, Interpretation};
use crate::tokens::TokenStore;

#[derive(Debug, Clone)]
pub struct SkiaInterpretation<T>
    where T: Interpretation<Item=Vec<Path>>
{
    /// Skia only interprets collections of paths. This is
    /// an interpretation to produce that initial collection.
    pub initial: T,
    pub canvas_width: u32,
    pub canvas_height: u32
}

impl<T> Default for SkiaInterpretation<T>
    where T: Interpretation<Item=Vec<Path>>
{
    fn default() -> Self {
        SkiaInterpretation {
            initial: T::default(),
            canvas_width: 500,
            canvas_height: 500
        }
    }
}




impl<T> Interpretation for SkiaInterpretation<T>
    where T: Interpretation<Item=Vec<Path>>
{
    type Item = Pixmap;

    /// Returns a system initialised to understand tokens from [`abop_family`].
    fn system() -> Result<System> {
        let family = get_or_init_family("ABOP", abop_family);
        System::of_family(family)
    }

    fn interpret<S: TokenStore>(&self, tokens: &S, string: &ProductionString) -> Result<Self::Item> {
        let paths = self.initial.interpret(tokens, string)?;
        // let bounds = paths.bounds();

        draw_paths(paths)
    }
}

/// Given a collection of [`Path`] objects, this returns a pixmap image
/// drawn using [`tiny_skia`].
pub fn draw_paths<I: IntoIterator<Item=Path>>(paths: I) -> Result<Pixmap> {
    const WIDTH : u32 = 500;

    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 0, 0, 255);
    paint.anti_alias = true;

    let mut skia_paths: Vec<tiny_skia::Path> = Vec::new();

    for path in paths {
        if path.len() > 1 {
            let mut pb = PathBuilder::new();
            let start = path.get_start().unwrap();
            pb.move_to(start.x() as f32, start.y() as f32);

            for point in path.iter().skip(1) {
                pb.line_to(point.x() as f32, point.y() as f32);
            }

            if let Some(path) = pb.finish() {
                skia_paths.push(path);
            }
        }
    }

    let stroke = Stroke { width: 1.0, line_cap: LineCap::Round, .. Stroke::default()};
    let mut pixmap = Pixmap::new(WIDTH, WIDTH).unwrap();
    pixmap.fill(Color::from_rgba8(255, 255, 255, 255));
    for path in &skia_paths {
        pixmap.stroke_path(path, &paint, &stroke, Transform::identity(), None);
    }

    Ok(pixmap)
}
