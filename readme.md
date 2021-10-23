# SPOT NFT Generator

## Install

Git pull this repository.

Install Trunk within root directory of project with:

`cargo install trunk wasm-bindgen-cli`

Add wasm Rust Compiler Target:

`rustup target add wasm32-unknown-unknown`

## Run

`trunk serve` in root folder. Default browser window will auto-open a tab to `127.0.0.1:8080`.

Each refresh will create a new random spot artwork.

## Modify

To change the number of dots, edit the `spot_web.rs` file's `num_spots` integer variable. Please use a perfect square.

Change the margin by editing the `margin` variable. Please use an f32 or cast as f32.

Change the viewBox dimensions with `dimension` variable.

Restart service with crtl-c and `cargo run`.