use gb_core::cpu::Cpu;
use std::{env, fs, path::Path, process};

const MAX_CYCLES: u64 = 100_000_000;

fn find_roms(dir: &Path, roms: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let path = entry.unwrap().path();

        if path.is_dir() {
            find_roms(&path, roms);
        } else if path.extension().is_some_and(|ext| ext == "gb") {
            roms.push(path);
        }
    }
}

fn run_test(path: &Path) -> bool {
    let rom = match fs::read(path) {
        Ok(rom) => rom,
        Err(e) => {
            eprintln!("Failed to read {}: {e}", path.display());
            return false;
        }
    };

    let mut cpu = Cpu::new();
    cpu.load_rom(&rom);

    let mut cycles = 0u64;
    let mut output = Vec::new();

    while cycles < MAX_CYCLES {
        cpu.tick();
        cycles += 1;

        let (pc, b, c, d, e, h, l, opcode) = cpu.test_state();

        if opcode == 0x40 {
            if (b, c, d, e, h, l) == (3, 5, 8, 13, 21, 34) {
                return true;
            }

            if (b, c, d, e, h, l) == (0x42, 0x42, 0x42, 0x42, 0x42, 0x42) {
                return false;
            }
        }

        output.extend(cpu.take_serial_output());

        if output.windows(6).any(|w| w == b"Passed") {
            return true;
        }

        if output.windows(6).any(|w| w == b"Failed") {
            println!("\n--- {} ---", path.display());
            println!("{}", String::from_utf8_lossy(&output));
            return false;
        }
    }
    let (pc, b, c, d, e, h, l, opcode) = cpu.test_state();

    println!(
        "PC={:04X} B={:02X} C={:02X} D={:02X} E={:02X} H={:02X} L={:02X} opcode={:02X}",
        pc, b, c, d, e, h, l, opcode
    );
    println!("\n--- {} TIMEOUT ---", path.display());
    println!("{}", String::from_utf8_lossy(&output));

    false
}

fn main() {
    let path = match env::args().nth(1) {
        Some(path) => std::path::PathBuf::from(path),
        None => {
            eprintln!("Usage: gb-tests <rom.gb | directory>");
            process::exit(1);
        }
    };

    // Run a single ROM.
    if path.is_file() {
        let passed = run_test(&path);

        if passed {
            println!("PASS");
        } else {
            println!("FAIL");
            process::exit(1);
        }

        return;
    }

    if !path.is_dir() {
        eprintln!("Invalid path: {}", path.display());
        process::exit(1);
    }

    // Recursively find every .gb ROM.
    let mut tests = Vec::new();
    find_roms(&path, &mut tests);
    tests.sort();

    if tests.is_empty() {
        eprintln!("No .gb ROMs found in {}", path.display());
        process::exit(1);
    }

    let mut passed = 0;
    let total = tests.len();

    for rom in &tests {
        let name = rom.strip_prefix(&path).unwrap_or(rom);

        print!("{:<60}", name.display());

        if run_test(rom) {
            println!("PASS");
            passed += 1;
        } else {
            println!("FAIL");
        }
    }

    println!();
    println!("{}/{} tests passed", passed, total);

    if passed != total {
        process::exit(1);
    }
}
