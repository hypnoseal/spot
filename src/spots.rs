use std::error::Error;
use std::fmt::{Display, Formatter};
use palette::{Srgb, Pixel};
use rand::Rng;

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

#[derive(Clone, PartialEq)]
pub struct Spot {
    pub color: Srgb,
}

#[derive(Clone, PartialEq)]
pub struct Spots {
    pub spots: Vec<Spot>,
    pub margin: f32,
    pub pointer: usize,
}

impl Spots {
    pub fn create_random_spots(num_spots: usize, margin: f32) -> Result<Spots, CreateError> {
        if num_spots == 0 {
            return Err(CreateError { msg: "The number of spots cannot be 0." })
        }
        if !is_perfect_square(num_spots) {
            return Err(CreateError {
                msg: "The number of spots must be a perfect square. Ie. 4, 9, 16, 25, etc..."
            })
        }
        let mut spots_vec = Vec::new();
        for _ in 0..num_spots {
            spots_vec.push(Spot {
                color: Srgb::new(rand_num(), rand_num(), rand_num()),
            })
        }
        let spots = Spots {
            spots: spots_vec,
            margin,
            pointer: 0,
        };
        Ok(spots)
    }

    pub fn get_num_spots(&self) -> usize {
        self.spots.len()
    }

    // pub fn is_perfect_square(&self) -> bool {
    //     let num_spots_sqrt = (self.get_num_spots() as f64).sqrt();
    //     num_spots_sqrt % 1.0 == 0.0
    // }

    pub fn get_spot_color(&self, n: usize, rgb: RgbMatch) -> u8 {
        let raw_color: [u8; 3] = Srgb::into_raw(self.spots[n].color.into_format());
        match rgb {
            RgbMatch::Red => raw_color[0],
            RgbMatch::Green => raw_color[1],
            RgbMatch::Blue => raw_color[2],
        }
    }
}

fn is_perfect_square(num: usize) -> bool {
    let num_spots_sqrt = (num as f64).sqrt();
    num_spots_sqrt % 1.0 == 0.0
}

fn rand_num() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}