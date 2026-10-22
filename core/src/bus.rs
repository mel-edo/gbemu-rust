use crate::cart::{Cart, EXT_RAM_START, EXT_RAM_STOP, ROM_START, ROM_STOP};
use crate::ppu::{LCD_REG_START, LCD_REG_STOP, OAM_START, OAM_STOP, Ppu, PpuUpdateResult, VRAM_START, VRAM_STOP};
use crate::utils::*;
use crate::io::{Buttons, IO, IO_START, IO_STOP};
use crate::wram::{WRAM, ECHO_STOP, WRAM_START};
use crate::apu::{Apu, APU_START, APU_STOP};
use crate::serial::{Serial, SB, SC};

const OAM_DMA: u16 = 0xFF46;
const HRAM_START: u16 = 0xFF80;
const HRAM_STOP: u16 = 0xFFFF;
const HRAM_SIZE: usize = (HRAM_STOP - HRAM_START + 1) as usize;

// Game boy has 16 bit address space (0x0000-0xFFFF)
pub struct Bus {
    rom: Cart,
    ppu: Ppu,
    io: IO,
    wram: WRAM,
    hram: [u8; HRAM_SIZE],
    pub apu: Apu,
    pub serial: Serial,
    dma_active: bool,
    dma_source: u16,
    dma_index: u16,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom: Cart::new(),
            ppu: Ppu::new(),
            io: IO::new(),
            wram: WRAM::new(),
            hram: [0; HRAM_SIZE],
            apu: Apu::new(),
            serial: Serial::new(),
            dma_active: false,
            dma_source: 0,
            dma_index: 0,
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.rom.load_cart(data);
        let is_cgb = self.rom.is_cgb();
        self.ppu.set_cgb(is_cgb);
        self.wram.set_cgb(is_cgb);
    }

    pub fn is_cgb(&self) -> bool {
        self.rom.is_cgb()
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        if self.dma_active && !(HRAM_START..=0xFFFE).contains(&addr) {
            return 0xFF;
        }
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.read_cart(addr)
            },
            VRAM_START..=VRAM_STOP => {
                self.ppu.read_vram(addr)
            },
            EXT_RAM_START..=EXT_RAM_STOP => {
                self.rom.read_ram(addr)
            },
            WRAM_START..=ECHO_STOP => {
                self.wram.read_u8(addr)
            },
            OAM_START..=OAM_STOP => {
                self.ppu.read_oam(addr)
            },
            LCD_REG_START..=LCD_REG_STOP => {
                self.ppu.read_lcd_reg(addr)
            },
            0xFF4F => {
                self.ppu.read_vbk()
            },
            APU_START..=APU_STOP => {
                self.apu.read_u8(addr)
            },
            SB | SC => {
                self.serial.read_u8(addr)
            },
            IO_START..=IO_STOP => {
                self.io.read_u8(addr)
            },
            HRAM_START..=HRAM_STOP => {
                let relative_addr = addr - HRAM_START;
                self.hram[relative_addr as usize]
            },
            0xFF70 => {
                self.wram.read_svbk()
            },
            _ => {
                0
            }
        }
    }

    pub fn press_button(&mut self, button: Buttons, pressed: bool) {
        self.io.set_button(button, pressed);
    }

    pub fn take_serial_output(&mut self) -> Vec<u8> {
        self.serial.take_output()
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) -> bool {
        if self.dma_active && !(HRAM_START..=0xFFFE).contains(&addr) {
            return false;
        }
        let mut battery_write = false;
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.write_cart(addr, val);
            },
            VRAM_START..=VRAM_STOP => {
                self.ppu.write_vram(addr, val);
            },
            EXT_RAM_START..=EXT_RAM_STOP => {
                self.rom.write_ram(addr, val);
                battery_write = true;
            }
            WRAM_START..=ECHO_STOP => {
                self.wram.write_u8(addr, val);
            },
            OAM_START..=OAM_STOP => {
                self.ppu.write_oam(addr, val);
            },
            LCD_REG_START..=LCD_REG_STOP => {
                if addr == OAM_DMA {
                    self.start_dma(val);
                }
                self.ppu.write_lcd_reg(addr, val);
            },
            0xFF4F => {
                self.ppu.write_vbk(val);
            },
            APU_START..=APU_STOP => {
                self.apu.write_u8(addr, val);
            },
            SB | SC => {
                self.serial.write_u8(addr, val);
            },
            IO_START..=IO_STOP => {
                self.io.write_u8(addr, val);
            },
            HRAM_START..=HRAM_STOP => {
                let relative_addr = addr - HRAM_START;
                self.hram[relative_addr as usize] = val;
            },
            0xFF70 => {
                self.wram.write_svbk(val);
            },
            _ => {}
        }
        battery_write
    }

    pub fn update_ppu(&mut self, cycles: u8) -> PpuUpdateResult {
        return self.ppu.update(cycles)
    }

    pub fn update_timer(&mut self, cycles: u8) -> bool {
        self.io.update_timer(cycles)
    }

    pub fn update_serial(&mut self, cycles: u8) -> bool {
        self.serial.tick(cycles)
    }

    pub fn render(&self) -> [u8; DISPLAY_BUFFER] {
        self.ppu.render()
    }

    pub fn render_scanline(&mut self) {
        self.ppu.render_scanline();
    }

    fn start_dma(&mut self, val: u8) {
        self.dma_active = true;
        self.dma_source = (val as u16) << 8;
        self.dma_index = 0;
    }

    pub fn update_dma(&mut self, cycles: u8) {
        for _ in 0..cycles {
            if !self.dma_active {
                return;
            }

            let src = self.dma_source + self.dma_index;
            let value = self.read_dma_source(src);

            self.ppu.write_oam(OAM_START + self.dma_index, value);

            self.dma_index += 1;

            if self.dma_index >= 0xA0 {
                self.dma_active = false;
            }
        }
    }

    fn read_dma_source(&self, addr: u16) -> u8 {
        match addr {
            ROM_START..=ROM_STOP => self.rom.read_cart(addr),
            VRAM_START..=VRAM_STOP => self.ppu.read_vram(addr),
            EXT_RAM_START..=EXT_RAM_STOP => self.rom.read_ram(addr),
            WRAM_START..=ECHO_STOP => self.wram.read_u8(addr),
            OAM_START..=OAM_STOP => self.ppu.read_oam(addr),
            _ => self.read_ram(addr),
        }
    }

    pub fn get_title(&self) -> &str {
        self.rom.get_title()
    }

    pub fn get_battery_data(&self) -> &[u8] {
        self.rom.get_battery_data()
    }

    pub fn has_battery(&self) -> bool {
        self.rom.has_battery()
    }

    pub fn set_battery_data(&mut self, data: &[u8]) {
        self.rom.set_battery_data(data);
    }
}