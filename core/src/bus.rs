use crate::cart::{Cart, EXT_RAM_START, EXT_RAM_STOP, ROM_START, ROM_STOP};
use crate::ppu::{LCD_REG_START, LCD_REG_STOP, OAM_START, OAM_STOP, Ppu, PpuUpdateResult, VRAM_START, VRAM_STOP};
use crate::utils::*;
use crate::io::{Buttons, IO, IO_START, IO_STOP};
use crate::wram::{WRAM, ECHO_STOP, WRAM_START};
use crate::apu::{Apu, APU_START, APU_STOP};
use crate::serial::{Serial, SB, SC};

const OAM_DMA: u16 = 0xFF46;
const KEY1: u16 = 0xFF4D;
const HDMA1: u16 = 0xFF51;
const HDMA2: u16 = 0xFF52;
const HDMA3: u16 = 0xFF53;
const HDMA4: u16 = 0xFF54;
const HDMA5: u16 = 0xFF55;
const BCPS: u16 = 0xFF68;
const BCPD: u16 = 0xFF69;
const OCPS: u16 = 0xFF6A;
const OCPD: u16 = 0xFF6B;
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
    // CGB-specific
    is_cgb: bool,
    cgb_speed: bool,      // Double speed mode
    speed_switch_pending: bool,
    hdma_active: bool,
    hdma_src: u16,
    hdma_dst: u16,
    hdma_len: u8,
    hdma_mode_gdma: bool,
    // CGB Palettes (64 bytes each for BG and OBJ = 8 palettes * 4 colors * 2 bytes)
    bg_palette: [u8; 64],
    obj_palette: [u8; 64],
    bcps: u8,
    ocps: u8,
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
            is_cgb: false,
            cgb_speed: false,
            speed_switch_pending: false,
            hdma_active: false,
            hdma_src: 0,
            hdma_dst: 0,
            hdma_len: 0,
            hdma_mode_gdma: false,
            bg_palette: [0; 64],
            obj_palette: [0; 64],
            bcps: 0,
            ocps: 0,
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.rom.load_cart(data);
        let is_cgb = self.rom.is_cgb();
        self.is_cgb = is_cgb;
        self.ppu.set_cgb(is_cgb);
        self.wram.set_cgb(is_cgb);
        
        if is_cgb {
            // Initialize CGB palettes to default values (white)
            for i in 0..64 {
                self.bg_palette[i] = 0xFF;
                self.obj_palette[i] = 0xFF;
            }
        }
    }

    pub fn is_cgb(&self) -> bool {
        self.is_cgb
    }

    pub fn cgb_speed(&self) -> bool {
        self.cgb_speed
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
            KEY1 => {
                if self.is_cgb {
                    let mut val = 0x7E; // Bits 6-1 are always 1
                    if self.cgb_speed { val |= 0x80; } // Bit 7: current speed
                    if self.speed_switch_pending { val |= 0x01; } // Bit 0: switch pending
                    val
                } else {
                    0xFF
                }
            },
            HDMA1..=HDMA5 => {
                if self.is_cgb {
                    match addr {
                        HDMA1 => (self.hdma_src >> 8) as u8,
                        HDMA2 => (self.hdma_src & 0xFF) as u8,
                        HDMA3 => (self.hdma_dst >> 8) as u8,
                        HDMA4 => (self.hdma_dst & 0xFF) as u8,
                        HDMA5 => {
                            let mut val = self.hdma_len;
                            if self.hdma_mode_gdma { val |= 0x80; }
                            val
                        },
                        _ => 0xFF,
                    }
                } else {
                    0xFF
                }
            },
            BCPS => {
                if self.is_cgb { self.bcps } else { 0xFF }
            },
            BCPD => {
                if self.is_cgb { self.bg_palette[self.bcps as usize & 0x3F] } else { 0xFF }
            },
            OCPS => {
                if self.is_cgb { self.ocps } else { 0xFF }
            },
            OCPD => {
                if self.is_cgb { self.obj_palette[self.ocps as usize & 0x3F] } else { 0xFF }
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
            KEY1 => {
                if self.is_cgb && !self.cgb_speed && (val & 0x01) != 0 {
                    // Request speed switch (only from normal to double)
                    self.speed_switch_pending = true;
                }
            },
            HDMA1 => {
                if self.is_cgb {
                    self.hdma_src = (self.hdma_src & 0x00FF) | ((val as u16) << 8);
                }
            },
            HDMA2 => {
                if self.is_cgb {
                    self.hdma_src = (self.hdma_src & 0xFF00) | (val as u16 & 0xF0);
                }
            },
            HDMA3 => {
                if self.is_cgb {
                    self.hdma_dst = (self.hdma_dst & 0x00FF) | ((val as u16) << 8);
                    self.hdma_dst &= 0x1FF0; // Destination must be in VRAM (0x8000-0x9FF0)
                }
            },
            HDMA4 => {
                if self.is_cgb {
                    self.hdma_dst = (self.hdma_dst & 0xFF00) | (val as u16 & 0xF0);
                    self.hdma_dst &= 0x1FF0;
                }
            },
            HDMA5 => {
                if self.is_cgb {
                    self.hdma_len = val & 0x7F;
                    self.hdma_mode_gdma = (val & 0x80) == 0;
                    if self.hdma_mode_gdma {
                        self.execute_gdma();
                    } else {
                        self.hdma_active = true;
                    }
                }
            },
            BCPS => {
                if self.is_cgb {
                    self.bcps = val;
                }
            },
            BCPD => {
                if self.is_cgb {
                    let idx = self.bcps as usize & 0x3F;
                    self.bg_palette[idx] = val;
                    if (self.bcps & 0x80) != 0 {
                        self.bcps = (self.bcps & 0xC0) | ((self.bcps + 1) & 0x3F);
                    }
                }
            },
            OCPS => {
                if self.is_cgb {
                    self.ocps = val;
                }
            },
            OCPD => {
                if self.is_cgb {
                    let idx = self.ocps as usize & 0x3F;
                    self.obj_palette[idx] = val;
                    if (self.ocps & 0x80) != 0 {
                        self.ocps = (self.ocps & 0xC0) | ((self.ocps + 1) & 0x3F);
                    }
                }
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
        // Extract palette data needed for CGB rendering
        let bg_palette = self.bg_palette;
        let obj_palette = self.obj_palette;
        self.ppu.render_scanline(bg_palette, obj_palette);
    }

    fn start_dma(&mut self, val: u8) {
        self.dma_active = true;
        self.dma_source = (val as u16) << 8;
        self.dma_index = 0;
    }

    pub fn update_dma(&mut self, cycles: u8) {
        // Handle speed switch
        if self.speed_switch_pending {
            self.cgb_speed = true;
            self.speed_switch_pending = false;
        }

        // Handle OAM DMA
        for _ in 0..cycles {
            if !self.dma_active {
                break;
            }

            let src = self.dma_source + self.dma_index;
            let value = self.read_dma_source(src);

            self.ppu.write_oam(OAM_START + self.dma_index, value);

            self.dma_index += 1;

            if self.dma_index >= 0xA0 {
                self.dma_active = false;
            }
        }

        // Handle HDMA (during HBlank)
        if self.hdma_active {
            self.execute_hdma();
        }
    }

    fn execute_gdma(&mut self) {
        let blocks = (self.hdma_len as usize) + 1;
        let total_bytes = blocks * HDMA_BLOCK_SIZE;
        
        for i in 0..total_bytes {
            let src = self.hdma_src + i as u16;
            let dst = self.hdma_dst + i as u16;
            let value = self.read_dma_source(src);
            self.ppu.write_vram(dst, value);
        }
        
        self.hdma_src = self.hdma_src.wrapping_add(total_bytes as u16);
        self.hdma_dst = self.hdma_dst.wrapping_add(total_bytes as u16);
        self.hdma_len = 0xFF; // HDMA5 reads as 0xFF after GDMA
    }

    fn execute_hdma(&mut self) {
        if self.hdma_len == 0xFF {
            self.hdma_active = false;
            return;
        }

        let blocks = 1; // HDMA transfers 1 block per HBlank
        let total_bytes = blocks * HDMA_BLOCK_SIZE;
        
        for i in 0..total_bytes {
            let src = self.hdma_src + i as u16;
            let dst = self.hdma_dst + i as u16;
            let value = self.read_dma_source(src);
            self.ppu.write_vram(dst, value);
        }
        
        self.hdma_src = self.hdma_src.wrapping_add(total_bytes as u16);
        self.hdma_dst = self.hdma_dst.wrapping_add(total_bytes as u16);
        self.hdma_len = self.hdma_len.wrapping_sub(blocks as u8);
        
        if self.hdma_len == 0xFF {
            self.hdma_active = false;
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

    // Get CGB palette data for PPU rendering
    pub fn get_bg_palette_color(&self, palette_idx: u8, color_idx: u8) -> [u8; 4] {
        let idx = ((palette_idx as usize) * 8 + (color_idx as usize) * 2) & 0x3F;
        let lo = self.bg_palette[idx] as u16;
        let hi = self.bg_palette[idx + 1] as u16;
        let color = lo | (hi << 8);
        self.rgb555_to_rgba(color)
    }

    pub fn get_obj_palette_color(&self, palette_idx: u8, color_idx: u8) -> [u8; 4] {
        let idx = ((palette_idx as usize) * 8 + (color_idx as usize) * 2) & 0x3F;
        let lo = self.obj_palette[idx] as u16;
        let hi = self.obj_palette[idx + 1] as u16;
        let color = lo | (hi << 8);
        self.rgb555_to_rgba(color)
    }

    fn rgb555_to_rgba(&self, color: u16) -> [u8; 4] {
        let r = ((color & 0x1F) as u32 * 527 + 23) >> 6; // Scale 5-bit to 8-bit
        let g = (((color >> 5) & 0x1F) as u32 * 527 + 23) >> 6;
        let b = (((color >> 10) & 0x1F) as u32 * 527 + 23) >> 6;
        [r as u8, g as u8, b as u8, 255]
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