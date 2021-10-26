use svg::{Document, Node};
use svg::node::element::{Circle, SVG};
use svg::node::element::Group;
// use palette::{Srgb, Pixel};
use web_sys::Node as WebNode;
use yew::Html;
use yew::virtual_dom::VNode;

use crate::spots::{Spots, RgbMatch, CreateError};

pub struct SpotsArt {
    art: SVG
}

impl SpotsArt {
    pub fn new(art: SVG) -> SpotsArt {
        SpotsArt {
            art
        }
    }

    pub fn get_html(&self) -> Html {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&self.art.to_string());
        let node = WebNode::from(div);
        VNode::VRef(node)
    }
}

pub fn create_svg(spots: Spots, dimension: usize, margin: f32) -> Result<SpotsArt, CreateError> {
    if margin > (dimension / 2) as f32 {
        return Err(CreateError {
            msg: "Margin cannot be greater than half of dimension. Mathematically: margin > (dimension / 2)"
        })
    }
    let num_spots: usize = spots.get_num_spots();
    let sqrt_spots: f32 = (num_spots as f64).sqrt() as f32;
    let radius: f32 = (dimension as f32 - 2.0 * margin) / (sqrt_spots * 4.0);
    let spacing: f32 = 4.0 * radius;
    let mut pos_x: f32 = margin + 2.0 * radius;
    let mut pos_y: f32 = margin + 2.0 * radius;
    let mut col: usize = 1;
    let mut row: usize = 1;
    let mut circles = Group::new();
    println!("Spot Artwork Generated!");
    for n in 0..num_spots {
        let color = format!(
            "rgb({}, {}, {})",
            spots.get_spot_color(n, RgbMatch::Red),
            spots.get_spot_color(n, RgbMatch::Green),
            spots.get_spot_color(n, RgbMatch::Blue)
        );
        println!("Spot {}: pos_x {}, pos_y {}, col {}, row {}, color: {}", n + 1, pos_x, pos_y, col, row, color);
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
    Ok(SpotsArt::new(art_piece))
}