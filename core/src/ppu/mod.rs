mod tile;
pub mod modes;
mod sprite;

use modes::{Lcd, LcdModeType, LcdResults};
use tile::Tile;
use sprite::Sprite;

use crate::utils::{BitOps, DISPLAY_BUFFER, GB_PALETTE, Point, SCREEN_WIDTH, unpack_u8};

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_STOP: u16 = 0x9FFF;
const TILE_SET_START: u16 = 0x8000;
const TILE_SET_STOP: u16 = 0x97FF;
const TILE_MAP_START: u16 = 0x9800;
const TILE_MAP_STOP: u16 = 0x9FFF;
const BYTES_PER_TILE: u16 = 16;
const NUM_TILES: usize = 384;
const TILE_MAP_SIZE: usize = (TILE_MAP_STOP - TILE_MAP_START + 1) as usize;
pub const LCD_REG_START: u16 = 0xFF40;
pub const LCD_REG_STOP: u16 = 0xFF4B;
const LCD_REG_SIZE: usize = (LCD_REG_STOP - LCD_REG_START + 1) as usize;

// 0xFF40 - LCD Control (LCDC)
const LCDC: u16 = 0xFF40;

// bit flags for LCDC
const LCDC_LCD_ENABLED_BIT: u8 = 7;
const LCDC_WNDW_MAP_BIT: u8 = 6;
const LCDC_WNDW_ENABLED_BIT: u8 = 5;
const LCDC_BG_WNDW_TILE_BIT: u8 = 4;
const LCDC_BG_MAP_BIT: u8 = 3;
const LCDC_SPR_SIZE_BIT: u8 = 2;
const LCDC_SPR_ENABLED_BIT: u8 = 1;
const LCDC_BG_WNDW_ENABLED_BIT: u8 = 0;

const LY: u16 = 0xFF44;
const STAT: u16 = 0xFF41;
const LYC: u16 = 0xFF45;
const SCY: u16 = 0xFF42;
const SCX: u16 = 0xFF43;
const WY: u16 = 0xFF4A;
const WX: u16 = 0xFF4B;
const BGP: u16 = 0xFF47;
const OBP0: u16 = 0xFF48;
const OBP1: u16 = 0xFF49;

const STAT_LY_LYC_IRQ_BIT: u8 = 6;
const STAT_OAM_IRQ_BIT: u8 = 5;
const STAT_VBLANK_IRQ_BIT: u8 = 4;
const STAT_HBLANK_IRQ_BIT: u8 = 3;
const STAT_LY_EQ_LYC_BIT: u8 = 2;

const TILE_MAP_TABLE_SIZE: usize = TILE_MAP_SIZE / 2;

const TILESIZE: usize = 8;
const LAYERSIZE: usize = 32;
const MAP_PIXELS: usize = TILESIZE * LAYERSIZE;

pub const OAM_START: u16 = 0xFE00;
pub const OAM_STOP: u16 = 0xFE9F;

const NUM_OAM_SPRITES: usize = 40;
const BYTES_PER_SPRITE: u16 = 4;

pub struct PpuUpdateResult {
    pub lcd_result: LcdResults,
    pub irq: bool,
}

pub struct Ppu {
    screen_buffer: [u8; DISPLAY_BUFFER],
    mode: Lcd,
    tiles: [Tile; NUM_TILES * 2],
    maps: [u8; TILE_MAP_SIZE * 2],
    lcd_regs: [u8; LCD_REG_SIZE],
    oam: [Sprite; NUM_OAM_SPRITES],
    vbk: u8,
    is_cgb: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            screen_buffer: [0; DISPLAY_BUFFER],
            mode: Lcd::new(),
            tiles: [Tile::new(); NUM_TILES * 2],
            maps: [0; TILE_MAP_SIZE * 2],
            lcd_regs: [0; LCD_REG_SIZE],
            oam: [Sprite::new(); NUM_OAM_SPRITES],
            vbk: 0,
            is_cgb: false,
        }
    }

    pub fn set_cgb(&mut self, is_cgb: bool) {
        self.is_cgb = is_cgb;
        for sprite in &mut self.oam {
            sprite.set_cgb(is_cgb);
        }
    }

    pub fn update(&mut self, m_cycles: u8) -> PpuUpdateResult {
        let t_cycles = m_cycles * 4;
        let old_mode = self.mode.get_mode();
        let old_line = self.mode.get_line();
        let lcd_result = self.mode.step(t_cycles);
        let mut stat = self.read_lcd_reg(STAT);
        let mut irq = false;

        let scanline = self.mode.get_line();
        if old_line != scanline {
            let lyc = self.read_lcd_reg(LYC);
            stat.set_bit(STAT_LY_EQ_LYC_BIT, scanline == lyc);
            irq = (scanline == lyc) && stat.get_bit(STAT_LY_LYC_IRQ_BIT);
            self.write_lcd_reg(LY, scanline);
        }

        let mode = self.mode.get_mode();
        if old_mode != mode {
            match mode {
                LcdModeType::HBLANK => {
                    irq |= stat.get_bit(STAT_HBLANK_IRQ_BIT);
                },
                LcdModeType::VBLANK => {
                    irq |= stat.get_bit(STAT_VBLANK_IRQ_BIT);
                },
                LcdModeType::OAMReadMode => {
                    irq |= stat.get_bit(STAT_OAM_IRQ_BIT);
                },
                _ => {},
            }
        }

        stat &= 0b1111_1100;  // first 2 bits of stat register tell use the ppu mode
        stat |= mode.get_idx();
        self.write_lcd_reg(STAT, stat);

        PpuUpdateResult { lcd_result, irq }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        let bank_offset_tiles = if self.vbk & 1 == 1 { NUM_TILES } else { 0 };
        let bank_offset_maps = if self.vbk & 1 == 1 { TILE_MAP_SIZE } else { 0 };
        match addr {
            TILE_SET_START..=TILE_SET_STOP => {
                let relative_addr = addr - TILE_SET_START;
                let tile_idx = relative_addr / BYTES_PER_TILE;
                let offset = relative_addr % BYTES_PER_TILE;
                self.tiles[tile_idx as usize + bank_offset_tiles].read_u8(offset)
            },
            TILE_MAP_START..=TILE_MAP_STOP => {
                let relative_addr = addr - TILE_MAP_START;
                self.maps[relative_addr as usize + bank_offset_maps]
            },
            _ => 0xFF
        }
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        let bank_offset_tiles = if self.vbk & 1 == 1 { NUM_TILES } else { 0 };
        let bank_offset_maps = if self.vbk & 1 == 1 { TILE_MAP_SIZE } else { 0 };
        match addr {
            TILE_SET_START..=TILE_SET_STOP => {
                let relative_addr = addr - TILE_SET_START;
                let tile_idx = relative_addr / BYTES_PER_TILE;
                let offset = relative_addr % BYTES_PER_TILE;
                self.tiles[tile_idx as usize + bank_offset_tiles].write_u8(offset, val);
            },
            TILE_MAP_START..=TILE_MAP_STOP => {
                let relative_addr = addr - TILE_MAP_START;
                self.maps[relative_addr as usize + bank_offset_maps] = val;
            },
            _ => {}
        }
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        let relative_addr = addr - OAM_START;
        let oam_idx = relative_addr / BYTES_PER_SPRITE;
        self.oam[oam_idx as usize].read_u8(addr)
    }

    pub fn write_oam(&mut self, addr: u16, val: u8) {
        let relative_addr = addr - OAM_START;
        let oam_idx = relative_addr / BYTES_PER_SPRITE;
        self.oam[oam_idx as usize].write_u8(addr, val);
    }

    pub fn read_lcd_reg(&self, addr: u16) -> u8 {
        let relative_addr = addr - LCD_REG_START;
        self.lcd_regs[relative_addr as usize]
    }

    pub fn write_lcd_reg(&mut self, addr: u16, val: u8) {
        let relative_addr = addr - LCD_REG_START;
        self.lcd_regs[relative_addr as usize] = val;
    }

    pub fn read_vbk(&self) -> u8 {
        if self.is_cgb { self.vbk | 0xFE } else { 0xFF }
    }

    pub fn write_vbk(&mut self, val: u8) {
        if self.is_cgb { self.vbk = val & 1; }
    }

    fn is_lcd_enabled(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_LCD_ENABLED_BIT)
    }

    fn get_wndw_tile_map_index(&self) -> u8 {
        let lcdc = self.read_lcd_reg(LCDC);
        if lcdc.get_bit(LCDC_WNDW_MAP_BIT) { 1 } else { 0 }
    }

    // bit 5 and bit 0 toggle bg and wndw layers. bit 0 takes priority
    fn is_window_layer_displayed(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_WNDW_ENABLED_BIT) && lcdc.get_bit(LCDC_BG_WNDW_ENABLED_BIT)
    }

    fn get_bg_wndw_tile_set_index(&self) -> u8 {
        let lcdc = self.read_lcd_reg(LCDC);
        if lcdc.get_bit(LCDC_BG_WNDW_TILE_BIT) { 1 } else { 0 }
    }

    fn get_bg_tile_map_index(&self) -> u8 {
        let lcdc = self.read_lcd_reg(LCDC);
        if lcdc.get_bit(LCDC_BG_MAP_BIT) { 1 } else { 0 }
    }

    fn are_sprites_8x16(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_SPR_SIZE_BIT)
    }

    fn is_sprite_layer_displayed(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_SPR_ENABLED_BIT)
    }

    fn is_bg_layer_displayed(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_BG_WNDW_ENABLED_BIT)
    }

    fn get_viewport_coords(&self) -> Point {
        let x = self.read_lcd_reg(SCX);
        let y = self.read_lcd_reg(SCY);
        Point::new(x, y)
    }

    fn get_window_coords(&self) -> Point {
        let x = self.read_lcd_reg(WX);
        let y = self.read_lcd_reg(WY);
        Point::new(x.saturating_sub(7), y)
    }

    fn get_bg_palette(&self) -> [u8; 4] {
        unpack_u8(self.read_lcd_reg(BGP))
    }

    fn get_sprite_palette(&self, palette1: bool) -> [u8; 4] {
        if palette1 {
            unpack_u8(self.read_lcd_reg(OBP1))
        } else {
            unpack_u8(self.read_lcd_reg(OBP0))
        }
    }

    pub fn render(&self) -> [u8; DISPLAY_BUFFER] {
        if self.is_lcd_enabled() {
            self.screen_buffer
        } else {
            [0; DISPLAY_BUFFER]
        }
    }

    pub fn render_scanline(&mut self, bg_palette: [u8; 64], obj_palette: [u8; 64]) {
        let line = self.read_lcd_reg(LY);
        let mut row = [0xFF; SCREEN_WIDTH * 4];
        let mut bg_color_indices = [0u8; SCREEN_WIDTH];
        let mut bg_priorities = [false; SCREEN_WIDTH];

        let master_bg_enabled = self.is_bg_layer_displayed();

        if self.is_cgb || master_bg_enabled {
            self.render_bg(&mut row, line, bg_palette, &mut bg_color_indices, &mut bg_priorities);
        }

        if self.is_window_layer_displayed() && (self.is_cgb || master_bg_enabled) {
            self.render_window(&mut row, line, bg_palette, &mut bg_color_indices, &mut bg_priorities);
        }

        if self.is_sprite_layer_displayed() {
            self.render_sprites(&mut row, line, obj_palette, &bg_color_indices, &bg_priorities);
        }

        let start_idx = line as usize * SCREEN_WIDTH * 4;
        let end_idx = (line + 1) as usize * SCREEN_WIDTH * 4;
        self.screen_buffer[start_idx..end_idx].copy_from_slice(&row);
    }

    fn render_sprites(
        &self,
        buffer: &mut [u8],
        line: u8,
        obj_palette_data: [u8; 64],
        bg_color_indices: &[u8; SCREEN_WIDTH],
        bg_priorities: &[bool; SCREEN_WIDTH],
    ) {
        let sprites = self.get_sorted_sprites_for_line(line);
        let is_8x16 = self.are_sprites_8x16();
        let master_prio = self.is_bg_layer_displayed();

        for spr in sprites {
            let coords = spr.get_coords();
            let y = (line as isize - coords.1) as usize;
            let palette_idx = spr.get_cgb_palette();
            let palette = if self.is_cgb {
                None
            } else {
                Some(self.get_sprite_palette(spr.use_palette1()))
            };
            let y_flipped = spr.is_y_flipped();

            let spr_idx = if is_8x16 {
                if (y < 8 && !y_flipped) || (y >= 8 && y_flipped) {
                    spr.get_tile_num() & 0xFE
                } else {
                    spr.get_tile_num() | 0x01
                }
            } else {
                spr.get_tile_num()
            };

            let tile = if self.is_cgb && spr.get_vram_bank() {
                self.tiles[(spr_idx as usize) + NUM_TILES]
            } else {
                self.tiles[spr_idx as usize]
            };

            let height = if is_8x16 { 16 } else { 8 };
            let mut data_y = if y_flipped { height - y - 1 } else { y };
            data_y %= 8;
            let row = tile.get_row(data_y as u8);

            for x in 0..8 {
                let data_x = if spr.is_x_flipped() { 7 - x } else { x };
                let cell = row[data_x as usize];
                if cell == 0 {
                    continue;
                }
                let screen_x = coords.0 + x as isize;
                if screen_x < 0 || screen_x >= SCREEN_WIDTH as isize {
                    continue;
                }
                let px = screen_x as usize;
                let bg_color_idx = bg_color_indices[px];
                let bg_prio = bg_priorities[px];

                if self.is_cgb {
                    // CGB Priority rules:
                    // If LCDC bit 0 is cleared, sprites are always on top of background
                    if master_prio && bg_color_idx != 0 {
                        // If BG Map attribute bit 7 is set, BG has priority
                        if bg_prio {
                            continue;
                        }
                        // If Sprite attribute bit 7 is set, BG has priority
                        if spr.get_bg_priority() {
                            continue;
                        }
                    }
                } else {
                    // DMG Priority rules:
                    if spr.get_bg_priority() && bg_color_idx != 0 {
                        continue;
                    }
                }

                let color = if self.is_cgb {
                    Self::rgb555_to_rgba(obj_palette_data, palette_idx, cell)
                } else {
                    let color_idx = palette.unwrap()[cell as usize];
                    GB_PALETTE[color_idx as usize]
                };

                let buffer_idx = 4 * px;
                for i in 0..4 {
                    buffer[buffer_idx + i] = color[i];
                }
            }
        }
    }

    fn get_sorted_sprites_for_line(&self, line: u8) -> Vec<&Sprite> {
        let is_8x16 = self.are_sprites_8x16();
        let height = if is_8x16 { 16 } else { 8 };
        let signed_line = line as isize;

        let mut line_sprites = Vec::with_capacity(40);
        for (i, spr) in self.oam.iter().enumerate() {
            let coords = spr.get_coords();
            if signed_line >= coords.1 && signed_line < coords.1 + height {
                line_sprites.push((i, spr));
            }
        }

        if self.is_cgb {
            // In CGB mode: priority is determined solely by OAM index (lower index = higher priority).
            // Since later drawn sprites overwrite earlier drawn ones, reverse so sprite 0 is drawn last.
            line_sprites.reverse();
        } else {
            // In DMG mode: priority is determined by X position (lower X = higher priority).
            // Ties broken by lower OAM index.
            // Since later drawn sprites overwrite earlier drawn ones, sort descending by X, then descending by OAM index.
            line_sprites.sort_by(|(idx_a, a), (idx_b, b)| {
                b.get_coords().0.cmp(&a.get_coords().0).then_with(|| idx_b.cmp(idx_a))
            });
        }

        line_sprites.into_iter().map(|(_, spr)| spr).collect()
    }

    fn render_bg(
        &self,
        buffer: &mut [u8],
        line: u8,
        bg_palette_data: [u8; 64],
        bg_color_indices: &mut [u8; SCREEN_WIDTH],
        bg_priorities: &mut [bool; SCREEN_WIDTH],
    ) {
        let map_offset = self.get_bg_tile_map_index() as usize * TILE_MAP_TABLE_SIZE;
        let palette = self.get_bg_palette();
        let viewport = self.get_viewport_coords();
        let current_y = viewport.y as usize + line as usize;
        let y = current_y % MAP_PIXELS;
        let row = current_y % TILESIZE;
        for px in 0..SCREEN_WIDTH {
            let current_x = viewport.x as usize + px as usize;
            let x = current_x % MAP_PIXELS;
            let col = current_x % TILESIZE;
            let map_num = (y / TILESIZE) * LAYERSIZE + (x / TILESIZE);
            let tile_index = self.maps[map_offset + map_num] as usize;
            
            let (tile_vram_bank, tile_palette, tile_x_flip, tile_y_flip, tile_priority) = if self.is_cgb {
                let attr = self.maps[map_offset + map_num + TILE_MAP_SIZE];
                (
                    (attr & 0x08) != 0,
                    attr & 0x07,
                    (attr & 0x20) != 0,
                    (attr & 0x40) != 0,
                    (attr & 0x80) != 0,
                )
            } else {
                (false, 0, false, false, false)
            };
            
            let adjusted_tile_index = if self.get_bg_wndw_tile_set_index() == 1 {
                tile_index as usize
            } else {
                (256 + tile_index as i8 as isize) as usize
            };
            
            let tile = if self.is_cgb && tile_vram_bank {
                self.tiles[adjusted_tile_index + NUM_TILES]
            } else {
                self.tiles[adjusted_tile_index]
            };
            
            let data = tile.get_row(if tile_y_flip { 7 - row as u8 } else { row as u8 });
            let cell = if tile_x_flip { data[7 - col] } else { data[col] };

            bg_color_indices[px] = cell;
            bg_priorities[px] = tile_priority;

            let color = if self.is_cgb {
                Self::rgb555_to_rgba(bg_palette_data, tile_palette, cell)
            } else {
                let color_idx = palette[cell as usize];
                GB_PALETTE[color_idx as usize]
            };
            
            for i in 0..4 {
                buffer[4 * px + i] = color[i];
            }
        }
    }

    fn render_window(
        &self,
        buffer: &mut [u8],
        line: u8,
        bg_palette_data: [u8; 64],
        bg_color_indices: &mut [u8; SCREEN_WIDTH],
        bg_priorities: &mut [bool; SCREEN_WIDTH],
    ) {
        let map_offset = self.get_wndw_tile_map_index() as usize * TILE_MAP_TABLE_SIZE;
        let palette = self.get_bg_palette();
        let coords = self.get_window_coords();
        if (coords.x as usize > SCREEN_WIDTH) || (coords.y > line) {
            return;
        }
        let y = (line - coords.y) as usize; 
        let row = y % TILESIZE;
        for x in (coords.x as usize)..SCREEN_WIDTH {
            let win_x = x - coords.x as usize;
            let col = win_x % TILESIZE;
            let map_num = (y / TILESIZE) * LAYERSIZE + (win_x / TILESIZE);
            let tile_index = self.maps[map_offset + map_num] as usize;
            
            let (tile_vram_bank, tile_palette, tile_x_flip, tile_y_flip, tile_priority) = if self.is_cgb {
                let attr = self.maps[map_offset + map_num + TILE_MAP_SIZE];
                (
                    (attr & 0x08) != 0,
                    attr & 0x07,
                    (attr & 0x20) != 0,
                    (attr & 0x40) != 0,
                    (attr & 0x80) != 0,
                )
            } else {
                (false, 0, false, false, false)
            };
            
            let adjusted_tile_index = if self.get_bg_wndw_tile_set_index() == 1 {
                tile_index as usize
            } else {
                (256 + tile_index as i8 as isize) as usize
            };
            
            let tile = if self.is_cgb && tile_vram_bank {
                self.tiles[adjusted_tile_index + NUM_TILES]
            } else {
                self.tiles[adjusted_tile_index]
            };
            
            let data = tile.get_row(if tile_y_flip { 7 - row as u8 } else { row as u8 });
            let cell = if tile_x_flip { data[7 - col] } else { data[col] };

            bg_color_indices[x] = cell;
            bg_priorities[x] = tile_priority;

            let color = if self.is_cgb {
                Self::rgb555_to_rgba(bg_palette_data, tile_palette, cell)
            } else {
                let color_idx = palette[cell as usize];
                GB_PALETTE[color_idx as usize]
            };
            
            let buffer_idx = 4 * x;
            for i in 0..4 {
                buffer[buffer_idx + i] = color[i];
            }
        }
    }

    // Helper function to convert CGB 15-bit RGB555 to RGBA
    fn rgb555_to_rgba(palette_data: [u8; 64], palette_idx: u8, color_idx: u8) -> [u8; 4] {
        let idx = ((palette_idx as usize) * 8 + (color_idx as usize) * 2) & 0x3F;
        let lo = palette_data[idx] as u16;
        let hi = palette_data[idx + 1] as u16;
        let color = lo | (hi << 8);
        let r = ((color & 0x1F) as u32 * 527 + 23) >> 6;
        let g = (((color >> 5) & 0x1F) as u32 * 527 + 23) >> 6;
        let b = (((color >> 10) & 0x1F) as u32 * 527 + 23) >> 6;
        [r as u8, g as u8, b as u8, 255]
    }
}