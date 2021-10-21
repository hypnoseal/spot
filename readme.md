# SPOT NFT Generator

## Install

Git pull this repository.

## Run

`cargo run` in root folder. Then visit with web browser `127.0.0.1:8080`. Each get call to `127.0.0.1:8080` will 
generate a new art piece. Terminal will output messages.

## Modify

To change the number of dots, edit the `spot_web.rs` file's `num_spots` integer variable. Please use a perfect square.

Change the margin by editing the `margin` variable. Please use an f32 or cast as f32.

Change the viewBox dimensions with `dimension` variable.

Restart service with crtl-c and `cargo run`.