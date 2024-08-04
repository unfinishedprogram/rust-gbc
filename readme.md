# Game Boy Color 
> Written in rust, mainly targeting thew web

## Requirements

### Rust / Cargo

### Trunk

Trunk is used to easily bundle WASM into the webpage

```
cargo install trunk --locked
```

## Running

### Wasm platform (main target) 

```
trunk serve

# Then go to http://127.0.0.1:8080/
```

### CLI
The CLI version has been tested only on Kitty and Alacritty.
Your milage may vary with other terminal emulators

```
cd tui
cargo run --release
```