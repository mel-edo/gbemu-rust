use crate::cpu::*;
use crate::utils::*;

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
    // 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
    nop_00, ld_01, ld_02, inc_03, inc_04, dec_05, ld_06, todo, ld_08, add_09, ld_0a, dec_0b, inc_0c, dec_0d, ld_0e, todo,  // 0x00
    todo, ld_11, ld_12, inc_13, inc_14, dec_15, ld_16, todo, todo, add_19, ld_1a, dec_1b, inc_1c, dec_1d, ld_1e, todo,  // 0x10
    todo, ld_21, ld_22, inc_23, inc_24, dec_25, ld_26, todo, todo, add_29, ld_2a, dec_2b, inc_2c, dec_2d, ld_2e, todo,  // 0x20
    todo, ld_31, ld_32, inc_33, inc_34, dec_35, ld_36, todo, todo, add_39, ld_3a, dec_3b, inc_3c, dec_3d, ld_3e, todo,  // 0x30
    ld_40, ld_41, ld_42, ld_43, ld_44, ld_45, ld_46, ld_47, ld_48, ld_49, ld_4a, ld_4b, ld_4c, ld_4d, ld_4e, ld_4f,  // 0x40
    ld_50, ld_51, ld_52, ld_53, ld_54, ld_55, ld_56, ld_57, ld_58, ld_59, ld_5a, ld_5b, ld_5c, ld_5d, ld_5e, ld_5f,  // 0x50
    ld_60, ld_61, ld_62, ld_63, ld_64, ld_65, ld_66, ld_67, ld_68, ld_69, ld_6a, ld_6b, ld_6c, ld_6d, ld_6e, ld_6f,  // 0x60
    ld_70, ld_71, ld_72, ld_73, ld_74, ld_75, todo, ld_77, ld_78, ld_79, ld_7a, ld_7b, ld_7c, ld_7d, ld_7e, ld_7f,  // 0x70
    add_80, add_81, add_82, add_83, add_84, add_85, add_86, add_87, add_88, add_89, add_8a, add_8b, add_8c, add_8d, add_8e, add_8f,  // 0x80
    sub_90, sub_91, sub_92, sub_93, sub_94, sub_95, sub_96, sub_97, sub_98, sub_99, sub_9a, sub_9b, sub_9c, sub_9d, sub_9e, sub_9f,  // 0x90
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xA0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xB0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xC0
    todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo, todo,  // 0xD0
    ld_e0, todo, ld_e2, todo, todo, todo, todo, todo, todo, todo, ld_ea, todo, todo, todo, todo, todo,  // 0xE0
    ld_f0, todo, ld_f2, todo, todo, todo, todo, todo, ld_f8, ld_f9, ld_fa, todo, todo, todo, todo, todo,  // 0xF0
];

pub fn execute(cpu: &mut Cpu) -> u8 {
    let op_index = cpu.fetch();
    OPCODES[op_index as usize](cpu)
}

fn todo(_cpu: &mut Cpu) -> u8 {
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

// LD BC, u16 ----
fn ld_01(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::BC, val);
    3
}

// LD (BC), A ----
fn ld_02(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    let addr = cpu.get_r16(Regs16::BC);
    cpu.write_ram(addr, val);
    2
}

// LD B, u8 ----
fn ld_06(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::B, val);
    2
}

// LD (u16), SP ----
fn ld_08(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_u16();
    let val = cpu.get_r16(Regs16::SP);
    cpu.write_ram(addr, val.low_byte());
    cpu.write_ram(addr + 1, val.high_byte());
    5
}

// LD A, (BC) ----
fn ld_0a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::BC);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    2
}

// LD C, u8 ----
fn ld_0e(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::C, val);
    2
}

// LD DE, u16 ----
fn ld_11(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::DE, val);
    3
}

// LD (DE), A ----
fn ld_12(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    let addr = cpu.get_r16(Regs16::DE);
    cpu.write_ram(addr, val);
    2
}

// LD D, u8 ----
fn ld_16(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::D, val);
    2
}

// LD A, (DE) ----
fn ld_1a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::DE);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    2
}

// LD E, u8 ----
fn ld_1e(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::E, val);
    2
}

// LD HL, u16 ----
fn ld_21(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::HL, val);
    3
}

// LD (HL+), A ----
fn ld_22(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    cpu.set_r16(Regs16::HL, addr.wrapping_add(1));
    2
}

// LD H, u8 ----
fn ld_26(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::H, val);
    2
}

// LD A, (HL+) ----
fn ld_2a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    cpu.set_r16(Regs16::HL, addr.wrapping_add(1));
    2
}

// LD L, u8 ----
fn ld_2e(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::L, val);
    2
}

// LD SP, u16 ----
fn ld_31(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::SP, val);
    3
}

// LD (HL-), A ----
fn ld_32(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    cpu.set_r16(Regs16::HL, addr.wrapping_sub(1));
    2
}

// LD (HL), u8 ----
fn ld_36(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    3
}

// LD A, (HL-)
fn ld_3a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    cpu.set_r16(Regs16::HL, addr.wrapping_sub(1));
    2
}

// LD A, u8 ----
fn ld_3e(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::A, val);
    2
}

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

// LD B, D ----
fn ld_42(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.set_r8(Regs::B, val);
    1
}

// LD B, E ----
fn ld_43(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.set_r8(Regs::B, val);
    1
}

// LD B, H ----
fn ld_44(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.set_r8(Regs::B, val);
    1
}

// LD B, L ----
fn ld_45(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.set_r8(Regs::B, val);
    1
}

// LD B, (HL) ----
fn ld_46(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::B, val);
    2
}

// LD B, A ----
fn ld_47(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.set_r8(Regs::B, val);
    1
}

// LD C, B ----
fn ld_48(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.set_r8(Regs::C, val);
    1
}

// LD C, C ----
fn ld_49(_cpu: &mut Cpu) -> u8 {
    1
}

// LD C, D ----
fn ld_4a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.set_r8(Regs::C, val);
    1
}

// LD C, E ----
fn ld_4b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.set_r8(Regs::C, val);
    1
}

// LD C, H ----
fn ld_4c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.set_r8(Regs::C, val);
    1
}

// LD C, L ----
fn ld_4d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.set_r8(Regs::C, val);
    1
}

// LD C, (HL) ----
fn ld_4e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::C, val);
    2
}

// LD C, A ----
fn ld_4f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.set_r8(Regs::C, val);
    1
}

// LD D, B ----
fn ld_50(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.set_r8(Regs::D, val);
    1
}

// LD D, C ----
fn ld_51(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.set_r8(Regs::D, val);
    1
}

// LD D, D ----
fn ld_52(_cpu: &mut Cpu) -> u8 {
    1
}

// LD D, E ----
fn ld_53(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.set_r8(Regs::D, val);
    1
}

// LD D, H ----
fn ld_54(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.set_r8(Regs::D, val);
    1
}

// LD D, L ----
fn ld_55(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.set_r8(Regs::D, val);
    1
}

// LD D, (HL) ----
fn ld_56(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::D, val);
    2
}

// LD D, A ----
fn ld_57(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.set_r8(Regs::D, val);
    1
}

// LD E, B ----
fn ld_58(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.set_r8(Regs::E, val);
    1
}

// LD E, C ----
fn ld_59(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.set_r8(Regs::E, val);
    1
}

// LD E, D ----
fn ld_5a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.set_r8(Regs::E, val);
    1
}

// LD E, E ----
fn ld_5b(_cpu: &mut Cpu) -> u8 {
    1
}

// LD E, H ----
fn ld_5c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.set_r8(Regs::E, val);
    1
}

// LD E, L ----
fn ld_5d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.set_r8(Regs::E, val);
    1
}

// LD E, (HL) ----
fn ld_5e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::E, val);
    2
}

// LD E, A ----
fn ld_5f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.set_r8(Regs::E, val);
    1
}

// LD H, B ----
fn ld_60(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.set_r8(Regs::H, val);
    1
}

// LD H, C ----
fn ld_61(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.set_r8(Regs::H, val);
    1
}

// LD H, D ----
fn ld_62(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.set_r8(Regs::H, val);
    1
}

// LD H, E ----
fn ld_63(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.set_r8(Regs::H, val);
    1
}

// LD H, H ----
fn ld_64(_cpu: &mut Cpu) -> u8 {
    1
}

// LD H, L ----
fn ld_65(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.set_r8(Regs::H, val);
    1
}

// LD H, (HL) ----
fn ld_66(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::H, val);
    2
}

// LD H, A ----
fn ld_67(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.set_r8(Regs::H, val);
    1
}

// LD L, B ----
fn ld_68(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.set_r8(Regs::L, val);
    1
}

// LD L, C ----
fn ld_69(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.set_r8(Regs::L, val);
    1
}

// LD L, D ----
fn ld_6a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.set_r8(Regs::L, val);
    1
}

// LD L, E ----
fn ld_6b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.set_r8(Regs::L, val);
    1
}

// LD L, H ----
fn ld_6c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.set_r8(Regs::L, val);
    1
}

// LD L, L ----
fn ld_6d(_cpu: &mut Cpu) -> u8 {
    1
}

// LD L, (HL) ----
fn ld_6e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::L, val);
    2
}

// LD L, A ----
fn ld_6f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.set_r8(Regs::L, val);
    1
}

// LD (HL), B ----
fn ld_70(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    2
}

// LD (HL), C ----
fn ld_71(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    2
}

// LD (HL), D ----
fn ld_72(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    2
}

// LD (HL), E ----
fn ld_73(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    2
}

// LD (HL), H ----
fn ld_74(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    2
}

// LD (HL), L ----
fn ld_75(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    2
}

// LD (HL), A ----
fn ld_77(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    let addr = cpu.get_r16(Regs16::HL);
    cpu.write_ram(addr, val);
    2
}

// LD A, B ----
fn ld_78(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.set_r8(Regs::A, val);
    1
}

// LD A, C ----
fn ld_79(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.set_r8(Regs::A, val);
    1
}

// LD A, D ----
fn ld_7a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.set_r8(Regs::A, val);
    1
}

// LD A, E ----
fn ld_7b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.set_r8(Regs::A, val);
    1
}

// LD A, H ----
fn ld_7c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.set_r8(Regs::A, val);
    1
}

// LD A, L ----
fn ld_7d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.set_r8(Regs::A, val);
    1
}

// LD A, (HL) ----
fn ld_7e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    2
}

// LD A, A ----
fn ld_7f(_cpu: &mut Cpu) -> u8 {
    1
}

// LD (FF00+u8), A ----
fn ld_e0(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    let offset = cpu.fetch() as u16;
    let addr = 0xFF00 + offset;
    cpu.write_ram(addr, val);
    3
}

// LD (FF00+C), A ----
fn ld_e2(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    let offset = cpu.get_r8(Regs::C) as u16;
    let addr = 0xFF00 + offset;
    cpu.write_ram(addr, val);
    2
}

// LD (u16), A ----
fn ld_ea(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    let addr = cpu.fetch_u16();
    cpu.write_ram(addr, val);
    4
}

// LD A, (FF00+u8) ----
fn ld_f0(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as u16;
    let addr = 0xFF00 + offset;
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    3
}

// LD A, (FF00+C) ----
fn ld_f2(cpu: &mut Cpu) -> u8 {
    let offset = cpu.get_r8(Regs::C) as u16;
    let addr = 0xFF00 + offset;
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
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

// LD SP, HL ----
fn ld_f9(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r16(Regs16::HL);
    cpu.set_r16(Regs16::SP, val);
    2
}

// LD A, (u16) ----
fn ld_fa(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_u16();
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    4
}

// ALL BITWISE OPERATIONS OPCODES

// ADD HL, BC -0HC
fn add_09(cpu: &mut Cpu) -> u8 {
    cpu.add_r16(Regs16::HL, Regs16::BC);
    2
}

// ADD HL, DE -0HC
fn add_19(cpu: &mut Cpu) -> u8 {
    cpu.add_r16(Regs16::HL, Regs16::DE);
    2
}

// ADD HL, HL -0HC
fn add_29(cpu: &mut Cpu) -> u8 {
    cpu.add_r16(Regs16::HL, Regs16::HL);
    2
}

// ADD HL, SP -0HC
fn add_39(cpu: &mut Cpu) -> u8 {
    cpu.add_r16(Regs16::HL, Regs16::SP);
    2
}

// ADD A, B Z0HC
fn add_80(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.add_a_u8(val, false);
    1
}

// ADD A, C Z0HC
fn add_81(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.add_a_u8(val, false);
    1
}

// ADD A, D Z0HC
fn add_82(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.add_a_u8(val, false);
    1
}

// ADD A, E Z0HC
fn add_83(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.add_a_u8(val, false);
    1
}

// ADD A, H Z0HC
fn add_84(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.add_a_u8(val, false);
    1
}

// ADD A, L Z0HC
fn add_85(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.add_a_u8(val, false);
    1
}

// ADD A, (HL) Z0HC
fn add_86(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.add_a_u8(val, false);
    2
}

// ADD A, A Z0HC
fn add_87(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.add_a_u8(val, false);
    1
}

// ADC A, B Z0HC
fn add_88(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.add_a_u8(val, true);
    1
}

// ADC A, C Z0HC
fn add_89(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.add_a_u8(val, true);
    1
}

// ADC A, D Z0HC
fn add_8a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.add_a_u8(val, true);
    1
}

// ADC A, E Z0HC
fn add_8b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.add_a_u8(val, true);
    1
}

// ADC A, H Z0HC
fn add_8c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.add_a_u8(val, true);
    1
}

// ADC A, L Z0HC
fn add_8d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.add_a_u8(val, true);
    1
}

// ADC A, (HL) Z0HC
fn add_8e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.add_a_u8(val, true);
    2
}

// ADC A, A Z0HC
fn add_8f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.add_a_u8(val, true);
    1
}

// SUB A, B Z1HC
fn sub_90(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.sub_a_u8(val, false);
    1
}

// SUB A, C Z1HC
fn sub_91(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.sub_a_u8(val, false);
    1
}

// SUB A, D Z1HC
fn sub_92(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.sub_a_u8(val, false);
    1
}

// SUB A, E Z1HC
fn sub_93(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.sub_a_u8(val, false);
    1
}

// SUB A, H Z1HC
fn sub_94(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.sub_a_u8(val, false);
    1
}

// SUB A, L Z1HC
fn sub_95(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.sub_a_u8(val, false);
    1
}

// SUB A, (HL) Z1HC
fn sub_96(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.sub_a_u8(val, false);
    2
}

// SUB A, A Z1HC
fn sub_97(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.sub_a_u8(val, false);
    1
}

// SBC A, B Z1HC
fn sub_98(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::B);
    cpu.sub_a_u8(val, true);
    1
}

// SBC A, C Z1HC
fn sub_99(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.sub_a_u8(val, true);
    1
}

// SBC A, D Z1HC
fn sub_9a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::D);
    cpu.sub_a_u8(val, true);
    1
}

// SBC A, E Z1HC
fn sub_9b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::E);
    cpu.sub_a_u8(val, true);
    1
}

// SBC A, H Z1HC
fn sub_9c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::H);
    cpu.sub_a_u8(val, true);
    1
}

// SBC A, L Z1HC
fn sub_9d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::L);
    cpu.sub_a_u8(val, true);
    1
}

// SBC A, (HL) Z1HC
fn sub_9e(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.sub_a_u8(val, true);
    2
}

// SBC A, A Z1HC
fn sub_9f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::A);
    cpu.sub_a_u8(val, true);
    1
}