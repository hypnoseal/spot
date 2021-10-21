use std::io;
use actix_web::{App, web, get, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde_json::json;
use crate::svg_builder::create_random_spot;

#[get("/")]
pub async fn index(
    hb: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    // Edit the number of dots by changing the integer num_spots to another perfect square. If not a
    // square it will complain.
    let num_spots = 81;
    // Change the margin here. Must be an f32 (include the decimal 1.0 for example). Margin must be
    // less than half of `dimension`.
    let margin = 5.0;
    // Change the dimension of the SVG, this modifies the `viewBox` of the SVG. I suggest leaving it
    // alone for now.
    let dimension = 100;
    let art_piece = create_random_spot(num_spots, dimension, margin);

    let data = json!({
        "art_piece": art_piece.unwrap()
    });
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn start() -> io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(index)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
