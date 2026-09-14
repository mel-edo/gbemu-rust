use crate::{timer::*, utils::BitOps};

pub enum Buttons {
    A = 0,
    B = 1,
    Select = 2,
    Start = 3,
    Right = 4,
    Left = 5,
    Up = 6,
    Down = 7,
}

pub const IO_START: u16 = 0xFF00;
pub const IO_STOP: u16 = 0xFF3F;

const JOYPAD_ADDR: u16 = 0xFF00;
const IO_SIZE: usize = (IO_STOP - IO_START + 1) as usize;

const FACE_SELECT_BIT: u8 = 5;
const DPAD_SELECT_BIT: u8 = 4;

pub struct IO {
    buttons: [bool; 8],
    dpad_selected: bool,
    face_selected: bool,
    ram: [u8; IO_SIZE],
    timer: Timer,
}
impl IO {
    pub fn new() -> Self {
        Self {
            buttons: [false; 8],
            dpad_selected: false,
            face_selected: false,
            ram: [0; IO_SIZE],
            timer: Timer::new(),
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            DIV..=TAC => self.timer.read_timer(addr),
            JOYPAD_ADDR => self.read_joypad(),
            _ => {
                let relative_addr = addr - IO_START;
                self.ram[relative_addr as usize]
            }
        }
    }

    pub fn update_timer(&mut self, cycles: u8) -> bool {
        self.timer.tick(cycles)
    }

    fn read_joypad(&self) -> u8 {
        let mut ret = 0x0F;

        if self.dpad_selected {
            for btn in DPAD_BUTTONS {
                let idx = btn as usize;
                if self.buttons[idx] {
                    ret &= !(1 << (idx - 4));
                }
            }
        }

        if self.face_selected {
            for btn in FACE_BUTTONS {
                let idx = btn as usize;
                if self.buttons[idx] {
                    ret &= !(1 << idx);
                }
            }
        }

        ret |= (if self.dpad_selected { 0 } else { 1 }) << DPAD_SELECT_BIT;
        ret |= (if self.face_selected { 0 } else { 1 }) << FACE_SELECT_BIT;
        ret |= 0xC0; // Bits 6 and 7 are always 1

        ret
    }

    pub fn set_button(&mut self, button: Buttons, pressed: bool) {
        self.buttons[button as usize] = pressed;
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) {
        match addr {
            DIV..=TAC => {
                self.timer.write_timer(addr, val);
            }
            JOYPAD_ADDR => {
                self.face_selected = !val.get_bit(FACE_SELECT_BIT);
                self.dpad_selected = !val.get_bit(DPAD_SELECT_BIT);
            }
            _ => {
                let relative_addr = addr - IO_START;
                self.ram[relative_addr as usize] = val;
            }
        }
    }
}

const DPAD_BUTTONS: [Buttons; 4] = [Buttons::Right, Buttons::Left, Buttons::Up, Buttons::Down];

const FACE_BUTTONS: [Buttons; 4] = [Buttons::A, Buttons::B, Buttons::Select, Buttons::Start];
