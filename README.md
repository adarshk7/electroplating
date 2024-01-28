# Nokia 3310 Game Jam 3 Project

Project for the Nokia 3310 Game Jam 3.

## Run in the browser

First, add WASM support to your Rust installation. Using Rustup:

```sh
rustup target install wasm32-unknown-unknown
```

Install `trunk` tool for building and running wasm build locally:

```sh
cargo install --locked trunk
```

Run the game:

```sh
trunk serve
```

### Creating a release build

Install `just` to run justfile commands:

```sh
cargo install just
```

Install `brotli` for compressing the wasm file:

```sh
python3 -m pip install --user brotli
```

Create a release build:

```sh
just build
```

Copy the created zip file in dist folder to itch.io.
