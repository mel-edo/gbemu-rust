## Gameboy emulator using Rust

### Work in progress

## Helpful resources

- Great book for [gb emulator development](https://github.com/aquova/gb-book). It is also in rust.
- Really great article explaining the [DAA instruction](https://ehaskins.com/2018-01-30%20Z80%20DAA/)
- Complete technical reference for gameboy [pandocs](https://gbdev.io/pandocs/)
- [Ultimate gameboy talk](https://www.youtube.com/watch?v=HyzD8pNlpwI)
- [Homebrew titles for testing](https://opusgames.com/games/GBDev/GBDev.html)

### Prerequisites

- Rust

- SDL2 development libraries

On Arch:
```
sudo pacman -S sdl2
```

### Installation

```
git clone https://github.com/mel-edo/gbemu-rust.git
cd gbemu-rust
cargo build --release
```

## Contributing

Suggestions, fixes and improvments are welcome. Feel free to open an issue or a PR.

## License

This project is licensed under [GNU GPLv3](LICENSE)
