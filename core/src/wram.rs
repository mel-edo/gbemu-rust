pub const WRAM_START: u16 = 0xC000;
pub const WRAM_STOP: u16 = 0xDFFF;
pub const ECHO_START: u16 = 0xE000;
pub const ECHO_STOP: u16 = 0xFDFF;

const WRAM_BANK_SIZE: usize = 0x1000;
const CGB_WRAM_SIZE: usize = 8 * WRAM_BANK_SIZE;

pub struct WRAM {
    wram: [u8; CGB_WRAM_SIZE],
    svbk: u8,
    is_cgb: bool,
}

impl WRAM {
    pub fn new() -> Self {
        Self {
            wram: [0; CGB_WRAM_SIZE],
            svbk: 1,
            is_cgb: false,
        }
    }

    pub fn set_cgb(&mut self, is_cgb: bool) {
        self.is_cgb = is_cgb;
    }

    fn map_addr(&self, addr: u16) -> usize {
        let relative_addr = if addr >= ECHO_START {
            addr - ECHO_START
        } else {
            addr - WRAM_START
        };

        if relative_addr < 0x1000 {
            // Bank 0 (0xC000-0xCFFF) is always fixed
            relative_addr as usize
        } else {
            // Bank 1-7 (0xD000-0xDFFF)
            let bank = if self.is_cgb {
                let b = self.svbk & 0x07;
                if b == 0 { 1 } else { b }
            } else {
                1
            };
            (bank as usize * WRAM_BANK_SIZE) + (relative_addr as usize - 0x1000)
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.wram[self.map_addr(addr)]
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        let mapped = self.map_addr(addr);
        self.wram[mapped] = val;
    }

    pub fn read_svbk(&self) -> u8 {
        if self.is_cgb {
            self.svbk | 0xF8
        } else {
            0xFF
        }
    }

    pub fn write_svbk(&mut self, val: u8) {
        if self.is_cgb {
            self.svbk = val & 0x07;
        }
    }
}