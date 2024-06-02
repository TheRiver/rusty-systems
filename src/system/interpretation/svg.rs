//! Provides support for producing SVGs as well as interpreting [`ProductionString`] instances
//! as instructions for creating SVGs. 
//! 
//! TODO: more info on SVG support
//! See <https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths>
//! 
//! <div class="warning">
//! 
//! Note that this isn't meant to be a full featured SVG composition library. If you want 
//! more SVG features, crates such as [svg][svg] might be of use.
//! 
//! </div>
//! 
//! [svg]: https://crates.io/crates/svg
//!

use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::rc::Rc;
use crate::error::Error;

use crate::geometry::{Bounds, Path, Point, Vector};
use crate::prelude::{Interpretation, ProductionString, RunSettings, System};
use crate::tokens::TokenStore;

#[derive(Debug, Clone)]
pub struct SvgPathInterpretation<T>
    where T: Interpretation<Item=Vec<Path>>
{
    initial: T,
    width: usize,
    height: usize
}

impl<T> Default for SvgPathInterpretation<T>
    where T: Interpretation<Item=Vec<Path>>
{
    fn default() -> Self {
        SvgPathInterpretation {
            initial: T::default(),
            width: 500,
            height: 500,
        }
    }
}

impl<T> SvgPathInterpretation<T>
    where T: Interpretation<Item=Vec<Path>>
{
    pub fn new(width: usize, height: usize) -> Self {
        SvgPathInterpretation {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn new_with(width: usize, height: usize, interpretation: T) -> Self {
        SvgPathInterpretation {
            width,
            height,
            initial: interpretation
        }
    }


    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Interpretation for SvgPathInterpretation<T>
    where T: Interpretation<Item=Vec<Path>>
{
    type Item = Svg;

    fn system() -> crate::Result<System> {
        T::system()
    }

    fn interpret<S: TokenStore>(&self, tokens: &S, string: &ProductionString) -> crate::Result<Self::Item> {
        let paths = self.initial.interpret(tokens, string)?;
        let bounds = paths.bounds();

        let center = bounds.as_ref().unwrap().center();
        let canvas_centre = Point::new(self.width as f64 / 2.0, self.height as f64 / 2.0);

        let scale = bounds.as_ref()
            .map(|bounds| {
                let scale_y = self.height() as f64 / bounds.height();
                let scale_x = self.width() as f64 / bounds.width();
                let scale = scale_x.min(scale_y);

                Point::new(scale, -scale)
            })
            .unwrap_or(Point::new(1.0, -1.0));

        let elements: Vec<_> = paths.into_iter()
            .map(SvgPath::from)
            .map(Rc::new)
            .map(|rc| rc as Rc<dyn SvgElement>)
            .collect();

        #[allow(clippy::needless_update)]
        let group = SvgGroup {
            elements,
            decorations: SvgDecorations {
                stroke: Some(String::from("black")),
                stroke_width: Some(0.2),
                fill: Some(String::from("none")),
            },
            transforms: vec![
                Rc::new(SvgTranslate(Vector::from(canvas_centre))),
                Rc::new(SvgScale(scale)),
                Rc::new(SvgTranslate(-Vector::from(center))),
            ],
            ..SvgGroup::default()
        };

        Ok(Svg {
            elements: vec![Rc::new(group)],
            width: self.width,
            height: self.height
        })
    }

    fn run_settings(&self) -> RunSettings {
        self.initial.run_settings()
    }
}


pub trait SvgElement : Debug {
    fn to_svg(&self) -> String;

}

#[derive(Debug, Clone)]
pub struct Svg {
    elements: Vec<Rc<dyn SvgElement>>,
    width: usize,
    height: usize
}

impl Default for Svg {
    fn default() -> Self {
        Svg {
            elements: Vec::new(),
            width: 500,
            height: 500
        }
    }
}

impl Svg {
    /// Writes the SVG to file.
    pub fn save_file<P: AsRef<std::path::Path>>(&self, name: P) -> std::io::Result<()> {
        let mut file = File::create(name)?;
        file.write_all(self.to_svg().as_bytes())
    }
}

impl SvgElement for Svg {
    fn to_svg(&self) -> String {
        let mut string = String::new();
        string.push_str(format!("<svg version=\"1.1\" width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
                                self.width, self.height).as_str());

        for item in &self.elements {
            string.push_str(item.to_svg().as_str());
        }

        string.push_str("</svg>");
        string
    }
}



#[derive(Debug, Clone)]
pub struct SvgPath {
    path: Path,
    fill: Option<String>,
    stroke: Option<String>
}

impl SvgPath {
    pub fn fill(&self) -> Option<&String> {
        self.fill.as_ref()
    }

    pub fn stroke(&self) -> Option<&String> {
        self.stroke.as_ref()
    }
}

impl From<Path> for SvgPath {
    fn from(path: Path) -> Self {
        SvgPath { path, fill: None, stroke: None }
    }
}

impl SvgElement for SvgPath {
    fn to_svg(&self) -> String {
        let mut string = String::new();

        string.push_str("<path");

        if let Some(fill) = self.fill() {
            string.push_str(format!(" fill=\"{}\"", fill).as_str());
        }
        if let Some(stroke) = self.stroke() {
            string.push_str(format!(" stroke=\"{}\"", stroke).as_str());
        }

        if !self.is_empty() {
            string.push_str(" d=\"");
            let first = self.get(0).unwrap();
            string.push_str(format!("M {} {}", first.x(), first.y()).as_str());
            for point in self.iter().skip(1) {
                string.push_str(format!(" L {} {}", point.x(), point.y()).as_str());
            }
            string.push('"');
        }
        string.push_str("/>");

        string
    }
}

impl Deref for SvgPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

#[derive(Debug, Clone, Default)]
pub struct SvgDecorations {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
}

impl SvgDecorations {
    pub fn to_attr_string(&self) -> String {
        [
            self.fill.as_ref().map(|fill| format!("fill=\"{}\"", fill)).unwrap_or_default(),
            self.stroke.as_ref().map(|stroke| format!("stroke=\"{}\"", stroke)).unwrap_or_default(),
            self.stroke_width.as_ref().map(|width| format!("stroke-width=\"{}\"", width)).unwrap_or_default()
        ].join(" ")

    }
}

#[derive(Debug, Clone, Default)]
pub struct SvgGroup {
    elements: Vec<Rc<dyn SvgElement>>,
    decorations: SvgDecorations,

    transforms: Vec<Rc<dyn SvgTransformEl>>
}


impl SvgElement for SvgGroup {
    fn to_svg(&self) -> String {
        let mut string = String::new();

        string.push_str("<g ");
        string.push_str(self.decorations.to_attr_string().as_str());

        if !self.transforms.is_empty() {
            string.push_str(" transform=\"");
            for transform in &self.transforms {
                string.push_str(transform.to_transform().as_str());
                string.push(' ');
            }
            string.push('\"');
        }

        string.push('>');

        for element in &self.elements {
            string.push_str(element.to_svg().as_str());
        }

        string.push_str("</g>");

        string
    }
}

pub trait SvgTransformEl: Debug {
    fn to_transform(&self) -> String;
}

#[derive(Debug, Clone, Default)]
pub struct SvgScale(Point);

impl SvgTransformEl for SvgScale {
    fn to_transform(&self) -> String {
        format!("scale({} {})", self.0.x(), self.0.y())
    }
}

#[derive(Debug, Clone, Default)]
pub struct SvgTranslate(Vector);

impl SvgTransformEl for SvgTranslate {
    fn to_transform(&self) -> String {
        format!("translate({} {})", self.0.x(), self.0.y())
    }
}

#[derive(Debug, Clone)]
pub struct SvgCircle {
    pub centre: Point,
    pub radius: f64,
    pub decorations: SvgDecorations
}

impl SvgCircle {
    pub fn build<P: Into<Point>>(point: P, radius: f64) -> crate::Result<SvgCircle> {
        if radius < 0.0 {
            return Err(Error::general("radius should be non-negative"))
        }

        Ok(SvgCircle { centre: point.into(), radius, ..Default::default()})
    }
}

impl Default for SvgCircle {
    fn default() -> Self {
        SvgCircle {
            centre: Point::default(),
            radius: 5.0,
            decorations: SvgDecorations { stroke: Some(String::from("black")), ..Default::default() }
        }
    }
}

impl SvgElement for SvgCircle {
    fn to_svg(&self) -> String {
        format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" {}/>",
                self.centre.x(), self.centre.y(), self.radius, self.decorations.to_attr_string())
    }
}