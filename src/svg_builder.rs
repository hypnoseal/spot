use std::error::Error;
use std::fmt::{Display, Formatter};
use svg::{Document, Node};
use svg::node::element::Circle;
use svg::node::element::Group;
use palette::{Srgb, Pixel};

#[derive(Debug, PartialEq)]
pub struct CreateError {
    pub msg: &'static str
}

impl Display for CreateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was an error during the call to create!")
    }
}

impl Error for CreateError {}

pub enum RgbMatch {
    Red,
    Green,
    Blue
}

pub struct Spot {
    pub color: Srgb,
    pub pointer: bool,
}

pub struct Spots {
    pub spots: Vec<Spot>
}

impl Spots {
    fn get_num_spots(&self) -> usize {
        self.spots.len()
    }

    fn is_perfect_square(&self) -> bool {
        let num_spots_sqrt = (self.get_num_spots() as f64).sqrt();
        num_spots_sqrt % 1.0 == 0.0
    }

    fn get_spot_color(&self, n: usize, rgb: RgbMatch) -> u8 {
        let raw_color: [u8; 3] = Srgb::into_raw(self.spots[n].color.into_format());
        match rgb {
            RgbMatch::Red => raw_color[0],
            RgbMatch::Green => raw_color[1],
            RgbMatch::Blue => raw_color[2],
        }
    }
}

pub fn create(spots: Spots, dimension: usize, margin: f32) -> Result<(), CreateError> {
    if spots.get_num_spots() == 0 {
        return Err(CreateError { msg: "The number of spots cannot be 0." })
    }
    if !spots.is_perfect_square() {
        return Err(CreateError {
            msg: "The number of spots must be a perfect square. Ie. 4, 9, 16, 25, etc..."
        })
    }
    if margin > (dimension / 2) as f32 {
        return Err(CreateError {
            msg: "Margin cannot be greater than half of dimension. Mathematically: margin > (dimension / 2)"
        })
    }
    let num_spots: usize = spots.get_num_spots();
    let sqrt_spots: f32 = (num_spots as f64).sqrt() as f32;
    let radius: f32 = (dimension as f32 - 2.0 * margin) / (sqrt_spots * 4.0);
    let spacing: f32 = 4.0 * radius;
    println!("Spacing is {}", spacing);
    let mut pos_x: f32 = margin + 2.0 * radius;
    let mut pos_y: f32 = margin + 2.0 * radius;
    let mut col: usize = 1;
    let mut row: usize = 1;
    let mut circles = Group::new();
    for n in 0..num_spots {
        let color = format!(
            "rgb({}, {}, {})",
            spots.get_spot_color(n, RgbMatch::Red),
            spots.get_spot_color(n, RgbMatch::Green),
            spots.get_spot_color(n, RgbMatch::Blue)
        );
        println!("Spot {}: pos_x {}, pos_y {}, col {}, row {}", n + 1, pos_x, pos_y, col, row);
        let circle = Circle::new()
            .set("cx", pos_x)
            .set("cy", pos_y)
            .set("r", radius)
            .set("fill", color);
        if col < sqrt_spots as usize {
            pos_x += spacing as f32;
            col += 1;
        } else {
            pos_x = margin + 2.0 * radius;
            pos_y = margin + 2.0 * radius + spacing * row as f32;
            row += 1;
            col = 1;
        }
        circles.append(circle);
    }

    let art_piece = Document::new()
        .set("viewBox", (0, 0, dimension, dimension))
        .set("width", "100%")
        .set("height", "100%")
        .add(circles);

    svg::save("art.svg", &art_piece).unwrap();
    Ok(())
}