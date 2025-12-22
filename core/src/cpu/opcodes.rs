use crate::cpu::*;
use crate::utils::*;

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
    // 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
    nop_00, todo, todo, inc_03, inc_04, dec_05, todo, todo, todo, todo, todo, dec_0b, inc_0c, dec_0d, todo, todo,  // 0x00
    todo, todo, todo, inc_13, inc_14, dec_15, todo, todo, todo, todo, todo, dec_1b, inc_1c, dec_1d, todo, todo,  // 0x10
    todo, todo, todo, inc_23, inc_24, dec_25, todo, todo, todo, todo, todo, dec_2b, inc_2c, dec_2d, todo, todo,  // 0x20
    todo, todo, todo, inc_33, inc_34, dec_35, todo, todo, todo, todo, todo, dec_3b, inc_3c, dec_3d, todo, todo,  // 0x30
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0x40
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0x50
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0x60
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0x70
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0x80
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0x90
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xA0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xB0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xC0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xD0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xE0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xF0
];

pub fn execute(cpu: &mut Cpu) -> u8 {
    let op_index = cpu.fetch();
    OPCODES[op_index as usize](cpu)
}

fn todo(cpu: &mut Cpu) -> u8 {
    todo!();
}

// NOP ----
fn nop_00(_cpu: &mut Cpu) -> u8 {
    1
}

// ALL INC AND DEC OPCODES

// INC BC ----
fn inc_03(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::BC);
    2
}

// INC B Z0H-
fn inc_04(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::B);
    1
}

// DEC B Z1H-
fn dec_05(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::B);
    1
}

// DEC BC ----
fn dec_0b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::BC);
    2
}

// INC C Z0H-
fn inc_0c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::C);
    1
}

// DEC C Z1H-
fn dec_0d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::C);
    1
}

// INC DE ----
fn inc_13(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::DE);
    2
}

// INC D Z0H-
fn inc_14(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::D);
    1
}

// DEC D Z1H-
fn dec_15(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::D);
    1
}

// DEC DE ----
fn dec_1b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::DE);
    2
}

// INC E Z0H-
fn inc_1c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::E);
    1
}

// DEC E Z1H-
fn dec_1d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::E);
    1
}

// INC HL ----
fn inc_23(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::HL);
    2
}

// INC H Z0H-
fn inc_24(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::H);
    1
}

// DEC H Z1H-
fn dec_25(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::H);
    1
}

// DEC HL ----
fn dec_2b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::HL);
    2
}

// INC L Z0H-
fn inc_2c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::L);
    1
}

// DEC L Z1H-
fn dec_2d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::L);
    1
}

// INC SP ----
fn inc_33(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::SP);
    1
}

// INC (HL) Z0H-
fn inc_34(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::HL);
    3
}

// DEC (HL) Z1H-
fn dec_35(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::HL);
    3
}

// DEC SP ----
fn dec_3b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::SP);
    1
}

// INC A Z0H-
fn inc_3c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::A);
    1
}

// DEC A Z1H-
fn dec_3d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::A);
    1
}

// ALL LOAD OPCODES

// LD B, B ----
fn ld_40(_cpu: &mut Cpu) -> u8 {
    1  // No need to copy value from register to itself
}

// LD B, C ----
fn ld_41(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.set_r8(Regs::B, val);
    1
}

// LD E, u8 ----
fn ld_1e(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::E, val);
    2
}

// LD BC, u16 ----
fn ld_01(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::BC, val);
    3
}

// LD (u16), SP ----
fn ld_08(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_u16();
    let val = cpu.get_r16(Regs16::SP);
    cpu.write_ram(addr, val.low_byte());
    cpu.write_ram(addr + 1, val.high_byte());
    5
}

// LD A, (HL+) ----
fn ld_2a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    cpu.set_r16(Regs16::HL, addr.wrapping_add(1));
    2
}

// LD (FF00+u8), A ----
fn ld_e0(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    let offset = cpu.fetch() as u16;
    let addr = 0xFF00 + offset;
    cpu.write_ram(addr, val);
    2
}

// LD HL, SP+i8 00HC
fn ld_f8(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as i8 as i16 as u16;
    let sp = cpu.get_r16(Regs16::SP);
    let set_c = check_c_carry_u8(sp.low_byte(), offset.low_byte());
    let set_h = check_h_carry_u8(sp.low_byte(), offset.low_byte());

    cpu.set_r16(Regs16::HL, offset.wrapping_add(sp));
    cpu.set_flag(Flags::Z, false);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::C, set_c);
    cpu.set_flag(Flags::H, set_h);
    3
}