# gbemu

A complete, cycle-accurate Game Boy (DMG) and Game Boy Color (CGB) emulator written from scratch in Rust.

> **Play it in your browser:** The WebAssembly build of this emulator is hosted live on **[moonweft](https://mel-edo.github.io/moonweft/emulator.html)**!

<p align="center">
  <img src="assets/gbemu_demo.webp" alt="gbemu demo" width="650">
</p>

---

## Features

- **CPU (Sharp LR35902)**: Complete instruction set with cycle-accurate timing, interrupts, and CGB double-speed mode.
- **PPU**: Cycle-accurate scanline timing, DMG & CGB palettes, 2-bank VRAM, sprites, window, and CGB DMA (GDMA/HDMA).
- **APU**: 4-channel stereo sound synthesis (two pulse channels, custom wave, noise generator) with frequency sweep and volume envelopes.
- **Cartridges & MBCs**: ROM-only, MBC1, MBC2, MBC3 (with RTC), and MBC5 support with battery-backed save persistence.
- **Timers**: Accurate divider and timer counters (`DIV`, `TIMA`, `TMA`, `TAC`) with hardware overflow edge-case modeling.
- **Accuracy & Testing**: Passes Blargg test suites (`cpu_instrs`, `instr_timing`, `mem_timing`, `dmg_sound`), Mooneye acceptance tests, and Acid2 visual tests (`dmg-acid2`, `cgb-acid2`).

---

## Controls

### Desktop (SDL2)
| Button / Action | Key |
|---|---|
| **D-Pad** | Arrow Keys (`Up`, `Down`, `Left`, `Right`) |
| **A** | `X` |
| **B** | `Z` |
| **Start** | `Enter` |
| **Select** | `Backspace` |
| **Fast Forward (4×)** | `Shift` (Hold Left or Right Shift) |
| **Interactive Debugger** | `Space` |
| **Quit** | `Escape` |

### Web (Browser)
| Button / Action | Primary Key | Alternate Key |
|---|---|---|
| **D-Pad** | Arrow Keys | `W`, `A`, `S`, `D` |
| **A** | `X` | `K` |
| **B** | `Z` | `J` |
| **Start** | `Enter` | — |
| **Select** | `Backspace` | `Tab`, `Space` |
| **Fast Forward** | `Shift` (Hold) | — |

---

## Architecture

The project is structured as a modular workspace:

```text
gbemu/
├── core/       # Pure emulation engine: CPU, Bus, PPU, APU, Timers, Cartridges & MBCs
├── desktop/    # Native desktop frontend (SDL2 video rendering, audio queues, and input)
├── wasm/       # WebAssembly bridge bindings (wasm-bindgen)
├── html/       # Standalone HTML/JS frontend using the compiled WebAssembly package
└── tests/      # Automated test runner with headless visual verification and Blargg/Mooneye support
```

---

## Getting Started

### Prerequisites

#### Desktop (SDL2)
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- SDL2 development libraries:
  - **Ubuntu / Debian**: `sudo apt install libsdl2-dev`
  - **Arch Linux**: `sudo pacman -S sdl2`
  - **macOS**: `brew install sdl2`
  - **Windows**: Install via vcpkg or download SDL2 development packages.

#### WebAssembly
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/):
  ```bash
  cargo install wasm-pack
  ```
- A local web server (e.g., Python `http.server`, `caddy`, or `serve`).

---

### Building and Running

#### Native Desktop
```bash
git clone https://github.com/mel-edo/gbemu-rust.git
cd gbemu-rust/desktop
cargo run --release -- path/to/game.gb
```

#### WebAssembly (Browser)
```bash
git clone https://github.com/mel-edo/gbemu-rust.git
cd gbemu-rust/wasm
wasm-pack build --target web
```
Then, serve the `html` directory:
```bash
cd ../html
python3 -m http.server 8000
```
Open `http://localhost:8000` in your browser.

#### Running Tests
To run a test ROM or an entire suite directory:
```bash
cd gbemu-rust/tests
cargo run --release -- roms/blargg/gb-test-roms/cpu_instrs/cpu_instrs.gb
```

---

## Helpful Resources

- [Pan Docs](https://gbdev.io/pandocs/) — Comprehensive Game Boy technical reference.
- [The Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI) — In-depth architectural breakdown by Michael Steil.
- [Game Boy Emulator Development Guide](https://github.com/aquova/gb-book) — Practical guide to writing a Game Boy emulator in Rust.
- [Understanding the Z80 DAA Instruction](https://ehaskins.com/2018-01-30%20Z80%20DAA/) — Detailed breakdown of DAA logic and edge cases.
- [Homebrew titles for testing](https://opusgames.com/games/GBDev/GBDev.html) — Curated collection of test homebrew titles.

---

## Contributing

Suggestions, bug reports, and improvements are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
