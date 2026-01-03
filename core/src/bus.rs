use crate::cart::{Cart, ROM_START, ROM_STOP};
use crate::ppu::{Ppu, VRAM_START, VRAM_STOP, PpuUpdateResult};

// Game boy has 16 bit address space (0x0000-0xFFFF)
pub struct Bus {
    rom: Cart,
    ppu: Ppu,
    ram: [u8; 0x6000],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom: Cart::new(),
            ppu: Ppu::new(),
            ram: [0; 0x6000],
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.rom.load_cart(data);
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.read_cart(addr)
            },
            VRAM_START..=VRAM_STOP => {
                self.ppu.read_vram(addr)
            },
            _ => {
                let offset = addr - VRAM_STOP - 1;
                self.ram[offset as usize]
            }
        }
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) {
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.write_cart(addr, val);
            },
            VRAM_START..=VRAM_STOP => {
                self.ppu.write_vram(addr, val);
            },
            _ => {
                let offset = addr - VRAM_STOP - 1;
                self.ram[offset as usize] = val;
            }
        }
    }

    pub fn update_ppu(&mut self, cycles: u8) -> PpuUpdateResult {
        return self.ppu.update(cycles)
    }
}