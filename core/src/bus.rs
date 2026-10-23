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
        self.io.set_cgb_timer(is_cgb);
        
        if is_cgb {
            // Initialize CGB palettes to DMG-equivalent grayscale palette
            // DMG default BGP=0xE4 maps: 0->Light, 1->Dark, 2->Black, 3->White
            // CGB palette format: 15-bit RGB555 per color (2 bytes little-endian)
            // 4 colors per palette * 8 palettes * 2 bytes = 64 bytes
            let gray_palette = [
                0xFF, 0x7F,  // Color 0: White (0x7FFF)
                0x94, 0x52,  // Color 1: Light gray (0x5294)
                0x08, 0x21,  // Color 2: Dark gray (0x2108)
                0x00, 0x00,  // Color 3: Black (0x0000)
            ];
            for pal in 0..8 {
                for color in 0..4 {
                    let idx = pal * 8 + color * 2;
                    self.bg_palette[idx] = gray_palette[color * 2];
                    self.bg_palette[idx + 1] = gray_palette[color * 2 + 1];
                    self.obj_palette[idx] = gray_palette[color * 2];
                    self.obj_palette[idx + 1] = gray_palette[color * 2 + 1];
                }
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
                        HDMA5 => {
                            if self.hdma_active {
                                self.hdma_len & 0x7F
                            } else {
                                0xFF // Bit 7=1 indicates HDMA is inactive
                            }
                        },
                        _ => 0xFF, // HDMA1-4 are write-only registers
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
                0xFF
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
                if self.is_cgb {
                    // Arm or disarm speed switch
                    self.speed_switch_pending = (val & 0x01) != 0;
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
                    self.hdma_dst = (self.hdma_dst & 0x00FF) | (((val & 0x1F) as u16) << 8);
                }
            },
            HDMA4 => {
                if self.is_cgb {
                    self.hdma_dst = (self.hdma_dst & 0xFF00) | (val as u16 & 0xF0);
                }
            },
            HDMA5 => {
                if self.is_cgb {
                    if (val & 0x80) == 0 {
                        // Bit 7 = 0
                        if self.hdma_active {
                            // Writing 0 to bit 7 while an HDMA is active cancels it
                            self.hdma_active = false;
                            self.hdma_len |= 0x80;
                        } else {
                            // Start General Purpose DMA (GDMA)
                            self.hdma_len = val & 0x7F;
                            self.execute_gdma();
                        }
                    } else {
                        // Bit 7 = 1: Start HBlank DMA (HDMA)
                        self.hdma_len = val & 0x7F;
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

        // HDMA transfers 1 block (16 bytes) during HBlank
        if self.is_cgb && self.hdma_active {
            self.execute_hdma_block();
        }
    }

    pub fn handle_stop(&mut self) {
        if self.is_cgb && self.speed_switch_pending {
            self.cgb_speed = !self.cgb_speed;
            self.speed_switch_pending = false;
        }
    }

    fn start_dma(&mut self, val: u8) {
        self.dma_active = true;
        self.dma_source = (val as u16) << 8;
        self.dma_index = 0;
    }

    pub fn update_dma(&mut self, cycles: u8) {
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
    }

    fn execute_gdma(&mut self) {
        let blocks = (self.hdma_len as usize) + 1;
        let total_bytes = blocks * HDMA_BLOCK_SIZE;
        let base_src = self.hdma_src & 0xFFF0;
        let base_dst = 0x8000 | (self.hdma_dst & 0x1FF0);

        for i in 0..total_bytes {
            let src = base_src.wrapping_add(i as u16);
            let dst = base_dst.wrapping_add(i as u16);
            let value = self.read_dma_source(src);
            self.ppu.write_vram(dst, value);
        }

        self.hdma_src = base_src.wrapping_add(total_bytes as u16);
        self.hdma_dst = (base_dst.wrapping_add(total_bytes as u16)) & 0x1FF0;
        self.hdma_len = 0xFF; // HDMA5 reads as 0xFF after GDMA
    }

    fn execute_hdma_block(&mut self) {
        let base_src = self.hdma_src & 0xFFF0;
        let base_dst = 0x8000 | (self.hdma_dst & 0x1FF0);

        for i in 0..HDMA_BLOCK_SIZE {
            let src = base_src.wrapping_add(i as u16);
            let dst = base_dst.wrapping_add(i as u16);
            let value = self.read_dma_source(src);
            self.ppu.write_vram(dst, value);
        }

        self.hdma_src = base_src.wrapping_add(HDMA_BLOCK_SIZE as u16);
        self.hdma_dst = (base_dst.wrapping_add(HDMA_BLOCK_SIZE as u16)) & 0x1FF0;

        if self.hdma_len == 0 {
            self.hdma_active = false;
            self.hdma_len = 0xFF;
        } else {
            self.hdma_len -= 1;
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