use palette::Srgb;
use rand::Rng;
mod svg_builder;



fn rand_num() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

fn main() {
    let num_spots = 16;
    let mut spots_vec = Vec::new();
    for _ in 0..num_spots {
        spots_vec.push(svg_builder::Spot{color:Srgb::new(rand_num(), rand_num(), rand_num()), pointer: false })
    }
    let spots = svg_builder::Spots { spots: spots_vec };

    let output = svg_builder::create(spots, 100, 10.0);
    match output {
        Ok(()) => println!("SVG is available in root folder. Thank you very much, have a nice day. <3"),
        Err(e) => println!("Uh oh, something went wrong: {}", e),
    }
}