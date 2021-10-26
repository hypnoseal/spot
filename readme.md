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

Interact with the form to modify the Spots Art.