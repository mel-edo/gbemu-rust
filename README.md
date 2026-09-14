## Gameboy emulator using Rust

> 🎮 **Play it in your browser:** The WebAssembly build of this emulator is hosted on my personal site! **[Try the emulator here](https://mel-edo.github.io/emulator.html)**.

### Work in progress

## Helpful resources

- Great book for [gb emulator development](https://github.com/aquova/gb-book). It is also in rust.
- Really great article explaining the [DAA instruction](https://ehaskins.com/2018-01-30%20Z80%20DAA/)
- Complete technical reference for gameboy [pandocs](https://gbdev.io/pandocs/)
- [Ultimate gameboy talk](https://www.youtube.com/watch?v=HyzD8pNlpwI)
- [Homebrew titles for testing](https://opusgames.com/games/GBDev/GBDev.html)

### Structure

The emulator is split into three main components:
- `core`: The main emulator logic (CPU, Memory Bus, PPU, APU).
- `desktop`: Native frontend using SDL2 for rendering and input.
- `wasm`: WebAssembly bindings for running the emulator in a web browser, complete with an HTML frontend.

### Prerequisites

#### Desktop (SDL2)
- Rust
- SDL2 development libraries

On Arch:
```
sudo pacman -S sdl2
```

#### WebAssembly (Browser)
- `wasm-pack` (install via `cargo install wasm-pack`)
- A local web server (e.g., `python3 -m http.server`)

### Building and Running

#### Desktop
```
git clone https://github.com/mel-edo/gbemu-rust.git
cd gbemu-rust/desktop
cargo run --release -- path/to/rom.gb
```

#### WebAssembly
```
git clone https://github.com/mel-edo/gbemu-rust.git
cd gbemu-rust/wasm
wasm-pack build --target web
```
Then, serve the `html` directory (which uses the compiled Wasm):
```
cd ../html
python3 -m http.server 8000
```
Open `http://localhost:8000` in your browser.

## Contributing

Suggestions, fixes and improvments are welcome. Feel free to open an issue or a PR.

## License

This project is licensed under [GNU GPLv3](LICENSE)
