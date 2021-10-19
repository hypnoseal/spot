use palette::Srgb;
use rand::Rng;
mod svg_builder;



fn rand_num() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

fn main() {
    // Edit the number of dots by changing the integer num_spots to another perfect square. If not a
    // square it will complain.
    let num_spots = 16;
    // Change the margin here. Must be an f32 (include the decimal 1.0 for example). Margin must be
    // less than half of `dimension`.
    let margin = 10.0;
    // Change the dimension of the SVG, this modifies the `viewBox` of the SVG. I suggest leaving it
    // alone for now.
    let dimension = 100;

    // Leave the rest alone for now.
    let mut spots_vec = Vec::new();
    for _ in 0..num_spots {
        spots_vec.push(svg_builder::Spot{
            color:Srgb::new(rand_num(), rand_num(), rand_num()), pointer: false
        })
    }
    let spots = svg_builder::Spots { spots: spots_vec };

    let output = svg_builder::create(spots, dimension, margin);
    match output {
        Ok(()) => println!(
            "SVG is available in root folder. Thank you very much, have a nice day. <3"
        ),
        Err(e) => println!("Uh oh, something went wrong: {} Specifically: {}", e, e.msg),
    }
}