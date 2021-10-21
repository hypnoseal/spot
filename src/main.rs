mod svg_builder;
mod spot_web;

use std::io;

use spot_web::start;



#[actix_web::main]
async fn main() -> io::Result<()> {
    start().await
}