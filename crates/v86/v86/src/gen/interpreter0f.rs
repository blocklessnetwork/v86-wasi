#![cfg_attr(rustfmt, rustfmt_skip)]
use cpu::cpu::{after_block_boundary, modrm_resolve};
use cpu::cpu::{read_imm8, read_imm16, read_imm32s};
use cpu::cpu::{task_switch_test, task_switch_test_mmx, trigger_ud};
use cpu::cpu::{DEBUG, PREFIX_66, PREFIX_F2, PREFIX_F3};
use cpu::instructions_0f;
use cpu::global_pointers::{instruction_pointer, prefixes};
pub unsafe fn run(opcode: u32) {
match opcode {
    0x00 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F00_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F00_0_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F00_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F00_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F00_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F00_2_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F00_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F00_3_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F00_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F00_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F00_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F00_5_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x100 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F00_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F00_0_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F00_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F00_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F00_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F00_2_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F00_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F00_3_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F00_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F00_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F00_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F00_5_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x01 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F01_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F01_0_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F01_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F01_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F01_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F01_2_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F01_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F01_3_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F01_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F01_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F01_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F01_6_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0F01_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0F01_7_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x101 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F01_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F01_0_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F01_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F01_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F01_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F01_2_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F01_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F01_3_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F01_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F01_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F01_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F01_6_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0F01_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0F01_7_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x02 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F02_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F02_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x102 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F02_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F02_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x03 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F03_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F03_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x103 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F03_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F03_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x04 | 0x104 => {
        instructions_0f::instr_0F04();
        after_block_boundary();
    },
    0x05 | 0x105 => {
        instructions_0f::instr_0F05();
        after_block_boundary();
    },
    0x06 | 0x106 => {
        instructions_0f::instr_0F06();
        after_block_boundary();
    },
    0x07 | 0x107 => {
        instructions_0f::instr_0F07();
        after_block_boundary();
    },
    0x08 | 0x108 => {
        instructions_0f::instr_0F08();
        after_block_boundary();
    },
    0x09 | 0x109 => {
        instructions_0f::instr_0F09();
        after_block_boundary();
    },
    0x0A | 0x10A => {
        instructions_0f::instr_0F0A();
        after_block_boundary();
    },
    0x0B | 0x10B => {
        instructions_0f::instr_0F0B();
        after_block_boundary();
    },
    0x0C | 0x10C => {
        instructions_0f::instr_0F0C();
        after_block_boundary();
    },
    0x0D | 0x10D => {
        instructions_0f::instr_0F0D();
        after_block_boundary();
    },
    0x0E | 0x10E => {
        instructions_0f::instr_0F0E();
        after_block_boundary();
    },
    0x0F | 0x10F => {
        instructions_0f::instr_0F0F();
        after_block_boundary();
    },
    0x10 | 0x110 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F10_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F10_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F10_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F10_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F10_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F10_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F10_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F10_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x11 | 0x111 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F11_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F11_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F11_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F11_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F11_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F11_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F11_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F11_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x12 | 0x112 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F12_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F12_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F12_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F12_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F12_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F12_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F12_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F12_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x13 | 0x113 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F13_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F13_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F13_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F13_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x14 | 0x114 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F14_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F14_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F14_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F14_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x15 | 0x115 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F15_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F15_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F15_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F15_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x16 | 0x116 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F16_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F16_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F16_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F16_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F16_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F16_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0x17 | 0x117 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F17_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F17_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F17_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F17_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0x18 | 0x118 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F18_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F18_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x19 | 0x119 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F19_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F19_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1A | 0x11A => {
        instructions_0f::instr_0F1A();
        after_block_boundary();
    },
    0x1B | 0x11B => {
        instructions_0f::instr_0F1B();
        after_block_boundary();
    },
    0x1C | 0x11C => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F1C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F1C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1D | 0x11D => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F1D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F1D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1E | 0x11E => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F1E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F1E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1F | 0x11F => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F1F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F1F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x20 | 0x120 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        instructions_0f::instr_0F20(modrm_byte & 7, modrm_byte >> 3 & 7);
        after_block_boundary();
    },
    0x21 | 0x121 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        instructions_0f::instr_0F21(modrm_byte & 7, modrm_byte >> 3 & 7);
        after_block_boundary();
    },
    0x22 | 0x122 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        instructions_0f::instr_0F22(modrm_byte & 7, modrm_byte >> 3 & 7);
        after_block_boundary();
    },
    0x23 | 0x123 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        instructions_0f::instr_0F23(modrm_byte & 7, modrm_byte >> 3 & 7);
        after_block_boundary();
    },
    0x24 | 0x124 => {
        instructions_0f::instr_0F24();
        after_block_boundary();
    },
    0x25 | 0x125 => {
        instructions_0f::instr_0F25();
        after_block_boundary();
    },
    0x26 | 0x126 => {
        instructions_0f::instr_0F26();
        after_block_boundary();
    },
    0x27 | 0x127 => {
        instructions_0f::instr_0F27();
        after_block_boundary();
    },
    0x28 | 0x128 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F28_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F28_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F28_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F28_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x29 | 0x129 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F29_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F29_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F29_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F29_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x2A | 0x12A => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F2A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F2A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F2A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F2A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F2A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F2A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F2A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F2A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x2B | 0x12B => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F2B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F2B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F2B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F2B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x2C | 0x12C => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F2C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F2C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F2C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F2C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F2C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F2C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F2C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F2C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0x2D | 0x12D => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F2D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F2D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F2D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F2D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F2D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F2D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F2D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F2D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0x2E | 0x12E => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F2E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F2E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F2E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F2E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x2F | 0x12F => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F2F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F2F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F2F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F2F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x30 | 0x130 => {
        instructions_0f::instr_0F30();
        after_block_boundary();
    },
    0x31 | 0x131 => {
        instructions_0f::instr_0F31();
    },
    0x32 | 0x132 => {
        instructions_0f::instr_0F32();
        after_block_boundary();
    },
    0x33 | 0x133 => {
        instructions_0f::instr_0F33();
        after_block_boundary();
    },
    0x34 | 0x134 => {
        instructions_0f::instr_0F34();
        after_block_boundary();
    },
    0x35 | 0x135 => {
        instructions_0f::instr_0F35();
        after_block_boundary();
    },
    0x36 | 0x136 => {
        instructions_0f::instr_0F36();
        after_block_boundary();
    },
    0x37 | 0x137 => {
        instructions_0f::instr_0F37();
        after_block_boundary();
    },
    0x38 | 0x138 => {
        instructions_0f::instr_0F38();
        after_block_boundary();
    },
    0x39 | 0x139 => {
        instructions_0f::instr_0F39();
        after_block_boundary();
    },
    0x3A | 0x13A => {
        instructions_0f::instr_0F3A();
        after_block_boundary();
    },
    0x3B | 0x13B => {
        instructions_0f::instr_0F3B();
        after_block_boundary();
    },
    0x3C | 0x13C => {
        instructions_0f::instr_0F3C();
        after_block_boundary();
    },
    0x3D | 0x13D => {
        instructions_0f::instr_0F3D();
        after_block_boundary();
    },
    0x3E | 0x13E => {
        instructions_0f::instr_0F3E();
        after_block_boundary();
    },
    0x3F | 0x13F => {
        instructions_0f::instr_0F3F();
        after_block_boundary();
    },
    0x40 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F40_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F40_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x140 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F40_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F40_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x41 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F41_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F41_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x141 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F41_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F41_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x42 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F42_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F42_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x142 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F42_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F42_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x43 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F43_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F43_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x143 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F43_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F43_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x44 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F44_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F44_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x144 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F44_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F44_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x45 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F45_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F45_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x145 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F45_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F45_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x46 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F46_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F46_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x146 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F46_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F46_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x47 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F47_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F47_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x147 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F47_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F47_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x48 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F48_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F48_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x148 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F48_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F48_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x49 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F49_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F49_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x149 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F49_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F49_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x4A => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F4A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F4A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x14A => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F4A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F4A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x4B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F4B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F4B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x14B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F4B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F4B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x4C => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F4C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F4C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x14C => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F4C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F4C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x4D => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F4D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F4D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x14D => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F4D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F4D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x4E => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F4E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F4E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x14E => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F4E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F4E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x4F => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0F4F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0F4F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x14F => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0F4F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0F4F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x50 | 0x150 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F50_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F50_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F50_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F50_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0x51 | 0x151 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F51_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F51_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F51_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F51_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F51_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F51_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F51_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F51_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x52 | 0x152 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F52_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F52_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F52_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F52_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x53 | 0x153 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F53_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F53_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F53_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F53_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x54 | 0x154 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F54_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F54_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F54_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F54_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x55 | 0x155 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F55_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F55_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F55_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F55_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x56 | 0x156 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F56_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F56_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F56_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F56_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x57 | 0x157 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F57_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F57_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F57_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F57_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x58 | 0x158 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F58_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F58_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F58_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F58_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F58_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F58_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F58_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F58_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x59 | 0x159 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F59_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F59_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F59_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F59_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F59_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F59_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F59_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F59_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x5A | 0x15A => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F5A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F5A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F5A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F5A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F5A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F5A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F5A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F5A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x5B | 0x15B => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F5B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F5B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F5B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F5B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F5B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F5B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x5C | 0x15C => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F5C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F5C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F5C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F5C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F5C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F5C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F5C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F5C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x5D | 0x15D => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F5D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F5D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F5D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F5D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F5D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F5D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F5D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F5D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x5E | 0x15E => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F5E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F5E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F5E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F5E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F5E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F5E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F5E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F5E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x5F | 0x15F => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F5F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F5F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F5F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20F5F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F5F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F5F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F5F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F5F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x60 | 0x160 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F60_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F60_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F60_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F60_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x61 | 0x161 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F61_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F61_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F61_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F61_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x62 | 0x162 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F62_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F62_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F62_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F62_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x63 | 0x163 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F63_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F63_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F63_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F63_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x64 | 0x164 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F64_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F64_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F64_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F64_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x65 | 0x165 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F65_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F65_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F65_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F65_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x66 | 0x166 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F66_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F66_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F66_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F66_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x67 | 0x167 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F67_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F67_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F67_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F67_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x68 | 0x168 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F68_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F68_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F68_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F68_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x69 | 0x169 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F69_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F69_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F69_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F69_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x6A | 0x16A => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F6A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F6A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F6A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F6A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x6B | 0x16B => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F6B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F6B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F6B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F6B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x6C | 0x16C => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F6C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F6C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F6C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F6C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0x6D | 0x16D => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F6D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F6D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F6D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F6D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0x6E | 0x16E => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F6E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F6E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F6E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F6E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x6F | 0x16F => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F6F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F6F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F6F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F6F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F6F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F6F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x70 | 0x170 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F70_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_660F70_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20F70_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_F20F70_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F70_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_F30F70_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F70_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_0F70_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
    },
    0x71 | 0x171 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            match modrm_byte >> 3 & 7 {
                2 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F71_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F71_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                4 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F71_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F71_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                6 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F71_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F71_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                _ => {
                    if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                    trigger_ud();
                }
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            match modrm_byte >> 3 & 7 {
                2 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_0F71_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_0F71_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                4 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_0F71_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_0F71_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                6 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_0F71_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_0F71_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                _ => {
                    if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                    trigger_ud();
                }
            }
        }
    },
    0x72 | 0x172 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            match modrm_byte >> 3 & 7 {
                2 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F72_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F72_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                4 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F72_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F72_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                6 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F72_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F72_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                _ => {
                    if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                    trigger_ud();
                }
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            match modrm_byte >> 3 & 7 {
                2 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_0F72_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_0F72_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                4 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_0F72_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_0F72_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                6 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_0F72_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_0F72_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                _ => {
                    if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                    trigger_ud();
                }
            }
        }
    },
    0x73 | 0x173 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            match modrm_byte >> 3 & 7 {
                2 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F73_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F73_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                3 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F73_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F73_3_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                6 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F73_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F73_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                7 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_660F73_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_660F73_7_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                _ => {
                    if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                    trigger_ud();
                }
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            match modrm_byte >> 3 & 7 {
                2 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_0F73_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_0F73_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                6 => {
                    if !task_switch_test_mmx() {
                        return;
                    }
                    if modrm_byte < 0xC0 {
                        instructions_0f::instr_0F73_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                    else {
                        instructions_0f::instr_0F73_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                    }
                },
                _ => {
                    if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                    trigger_ud();
                }
            }
        }
    },
    0x74 | 0x174 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F74_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F74_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F74_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F74_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x75 | 0x175 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F75_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F75_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F75_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F75_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x76 | 0x176 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F76_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F76_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F76_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F76_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x77 | 0x177 => {
        if !task_switch_test_mmx() {
            return;
        }
        instructions_0f::instr_0F77();
    },
    0x78 | 0x178 => {
        instructions_0f::instr_0F78();
        after_block_boundary();
    },
    0x79 | 0x179 => {
        instructions_0f::instr_0F79();
        after_block_boundary();
    },
    0x7A | 0x17A => {
        instructions_0f::instr_0F7A();
        after_block_boundary();
    },
    0x7B | 0x17B => {
        instructions_0f::instr_0F7B();
        after_block_boundary();
    },
    0x7C | 0x17C => {
        if !task_switch_test_mmx() {
            return;
        }
        instructions_0f::instr_0F7C();
        after_block_boundary();
    },
    0x7D | 0x17D => {
        if !task_switch_test_mmx() {
            return;
        }
        instructions_0f::instr_0F7D();
        after_block_boundary();
    },
    0x7E | 0x17E => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F7E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F7E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F7E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F7E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F7E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F7E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x7F | 0x17F => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660F7F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660F7F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30F7F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30F7F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0F7F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0F7F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0x80 => {
        instructions_0f::instr16_0F80(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x180 => {
        instructions_0f::instr32_0F80(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x81 => {
        instructions_0f::instr16_0F81(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x181 => {
        instructions_0f::instr32_0F81(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x82 => {
        instructions_0f::instr16_0F82(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x182 => {
        instructions_0f::instr32_0F82(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x83 => {
        instructions_0f::instr16_0F83(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x183 => {
        instructions_0f::instr32_0F83(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x84 => {
        instructions_0f::instr16_0F84(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x184 => {
        instructions_0f::instr32_0F84(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x85 => {
        instructions_0f::instr16_0F85(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x185 => {
        instructions_0f::instr32_0F85(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x86 => {
        instructions_0f::instr16_0F86(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x186 => {
        instructions_0f::instr32_0F86(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x87 => {
        instructions_0f::instr16_0F87(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x187 => {
        instructions_0f::instr32_0F87(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x88 => {
        instructions_0f::instr16_0F88(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x188 => {
        instructions_0f::instr32_0F88(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x89 => {
        instructions_0f::instr16_0F89(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x189 => {
        instructions_0f::instr32_0F89(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x8A => {
        instructions_0f::instr16_0F8A(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x18A => {
        instructions_0f::instr32_0F8A(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x8B => {
        instructions_0f::instr16_0F8B(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x18B => {
        instructions_0f::instr32_0F8B(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x8C => {
        instructions_0f::instr16_0F8C(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x18C => {
        instructions_0f::instr32_0F8C(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x8D => {
        instructions_0f::instr16_0F8D(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x18D => {
        instructions_0f::instr32_0F8D(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x8E => {
        instructions_0f::instr16_0F8E(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x18E => {
        instructions_0f::instr32_0F8E(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x8F => {
        instructions_0f::instr16_0F8F(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x18F => {
        instructions_0f::instr32_0F8F(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x90 | 0x190 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F90_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F90_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x91 | 0x191 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F91_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F91_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x92 | 0x192 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F92_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F92_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x93 | 0x193 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F93_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F93_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x94 | 0x194 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F94_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F94_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x95 | 0x195 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F95_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F95_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x96 | 0x196 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F96_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F96_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x97 | 0x197 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F97_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F97_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x98 | 0x198 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F98_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F98_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x99 | 0x199 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F99_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F99_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x9A | 0x19A => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F9A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F9A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x9B | 0x19B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F9B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F9B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x9C | 0x19C => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F9C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F9C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x9D | 0x19D => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F9D_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F9D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x9E | 0x19E => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F9E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F9E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x9F | 0x19F => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0F9F_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0F9F_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xA0 => {
        instructions_0f::instr16_0FA0();
    },
    0x1A0 => {
        instructions_0f::instr32_0FA0();
    },
    0xA1 => {
        instructions_0f::instr16_0FA1();
        after_block_boundary();
    },
    0x1A1 => {
        instructions_0f::instr32_0FA1();
        after_block_boundary();
    },
    0xA2 | 0x1A2 => {
        instructions_0f::instr_0FA2();
    },
    0xA3 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FA3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FA3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1A3 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FA3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FA3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xA4 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FA4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
        }
        else {
            instructions_0f::instr16_0FA4_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
        }
    },
    0x1A4 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FA4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
        }
        else {
            instructions_0f::instr32_0FA4_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
        }
    },
    0xA5 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FA5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FA5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1A5 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FA5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FA5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xA6 | 0x1A6 => {
        instructions_0f::instr_0FA6();
        after_block_boundary();
    },
    0xA7 | 0x1A7 => {
        instructions_0f::instr_0FA7();
        after_block_boundary();
    },
    0xA8 => {
        instructions_0f::instr16_0FA8();
    },
    0x1A8 => {
        instructions_0f::instr32_0FA8();
    },
    0xA9 => {
        instructions_0f::instr16_0FA9();
        after_block_boundary();
    },
    0x1A9 => {
        instructions_0f::instr32_0FA9();
        after_block_boundary();
    },
    0xAA | 0x1AA => {
        instructions_0f::instr_0FAA();
    },
    0xAB => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FAB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FAB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1AB => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FAB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FAB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xAC => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FAC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
        }
        else {
            instructions_0f::instr16_0FAC_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
        }
    },
    0x1AC => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FAC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
        }
        else {
            instructions_0f::instr32_0FAC_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
        }
    },
    0xAD => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FAD_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FAD_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1AD => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FAD_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FAD_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xAE | 0x1AE => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions_0f::instr_0FAE_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr_0FAE_0_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions_0f::instr_0FAE_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr_0FAE_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if !task_switch_test_mmx() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions_0f::instr_0FAE_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr_0FAE_2_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            3 => {
                if !task_switch_test_mmx() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions_0f::instr_0FAE_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr_0FAE_3_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr_0FAE_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr_0FAE_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr_0FAE_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr_0FAE_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr_0FAE_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr_0FAE_6_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr_0FAE_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr_0FAE_7_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xAF => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FAF_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FAF_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1AF => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FAF_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FAF_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xB0 | 0x1B0 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0FB0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0FB0_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0xB1 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FB1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FB1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1B1 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FB1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FB1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xB2 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FB2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FB2_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x1B2 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FB2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FB2_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0xB3 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FB3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FB3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1B3 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FB3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FB3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xB4 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FB4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FB4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x1B4 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FB4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FB4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0xB5 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FB5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FB5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x1B5 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FB5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FB5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0xB6 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FB6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FB6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1B6 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FB6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FB6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xB7 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FB7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FB7_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1B7 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FB7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FB7_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xB8 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                instructions_0f::instr16_F30FB8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr16_F30FB8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            if modrm_byte < 0xC0 {
                instructions_0f::instr16_0FB8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr16_0FB8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0x1B8 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                instructions_0f::instr32_F30FB8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr32_F30FB8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            if modrm_byte < 0xC0 {
                instructions_0f::instr32_0FB8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr32_0FB8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0xB9 | 0x1B9 => {
        instructions_0f::instr_0FB9();
        after_block_boundary();
    },
    0xBA => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            4 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0FBA_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0FBA_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0FBA_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0FBA_5_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0FBA_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0FBA_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0FBA_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0FBA_7_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x1BA => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            4 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0FBA_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0FBA_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0FBA_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0FBA_5_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0FBA_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0FBA_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0FBA_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0FBA_7_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xBB => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FBB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FBB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1BB => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FBB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FBB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xBC => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FBC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FBC_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1BC => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FBC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FBC_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xBD => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FBD_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FBD_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1BD => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FBD_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FBD_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xBE => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FBE_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FBE_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1BE => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FBE_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FBE_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xBF => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FBF_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FBF_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1BF => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FBF_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FBF_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xC0 | 0x1C0 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0FC0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0FC0_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0xC1 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr16_0FC1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr16_0FC1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1C1 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr32_0FC1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr32_0FC1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xC2 | 0x1C2 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FC2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_660FC2_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20FC2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_F20FC2_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30FC2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_F30FC2_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FC2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_0FC2_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
    },
    0xC3 | 0x1C3 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions_0f::instr_0FC3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions_0f::instr_0FC3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0xC4 | 0x1C4 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FC4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_660FC4_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FC4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_0FC4_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            after_block_boundary();
        }
    },
    0xC5 | 0x1C5 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FC5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_660FC5_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FC5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_0FC5_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            after_block_boundary();
        }
    },
    0xC6 | 0x1C6 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FC6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_660FC6_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FC6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
            else {
                instructions_0f::instr_0FC6_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
            }
        }
    },
    0xC7 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            1 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0FC7_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0FC7_1_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr16_0FC7_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr16_0FC7_6_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x1C7 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            1 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0FC7_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0FC7_1_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions_0f::instr32_0FC7_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions_0f::instr32_0FC7_6_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xC8 | 0x1C8 => {
        instructions_0f::instr_0FC8();
    },
    0xC9 | 0x1C9 => {
        instructions_0f::instr_0FC9();
    },
    0xCA | 0x1CA => {
        instructions_0f::instr_0FCA();
    },
    0xCB | 0x1CB => {
        instructions_0f::instr_0FCB();
    },
    0xCC | 0x1CC => {
        instructions_0f::instr_0FCC();
    },
    0xCD | 0x1CD => {
        instructions_0f::instr_0FCD();
    },
    0xCE | 0x1CE => {
        instructions_0f::instr_0FCE();
    },
    0xCF | 0x1CF => {
        instructions_0f::instr_0FCF();
    },
    0xD0 | 0x1D0 => {
        if !task_switch_test_mmx() {
            return;
        }
        instructions_0f::instr_0FD0();
        after_block_boundary();
    },
    0xD1 | 0x1D1 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FD1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FD1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FD1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FD1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xD2 | 0x1D2 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FD2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FD2_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FD2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FD2_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xD3 | 0x1D3 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FD3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FD3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FD3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FD3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xD4 | 0x1D4 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FD4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FD4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FD4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FD4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xD5 | 0x1D5 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FD5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FD5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FD5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FD5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xD6 | 0x1D6 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FD6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FD6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20FD6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20FD6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30FD6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30FD6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FD6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FD6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0xD7 | 0x1D7 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FD7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FD7_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FD7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FD7_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xD8 | 0x1D8 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FD8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FD8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FD8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FD8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xD9 | 0x1D9 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FD9_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FD9_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FD9_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FD9_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xDA | 0x1DA => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FDA_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FDA_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FDA_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FDA_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xDB | 0x1DB => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FDB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FDB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FDB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FDB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xDC | 0x1DC => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FDC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FDC_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FDC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FDC_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xDD | 0x1DD => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FDD_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FDD_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FDD_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FDD_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xDE | 0x1DE => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FDE_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FDE_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FDE_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FDE_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xDF | 0x1DF => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FDF_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FDF_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FDF_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FDF_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xE0 | 0x1E0 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE0_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE0_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xE1 | 0x1E1 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xE2 | 0x1E2 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE2_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE2_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xE3 | 0x1E3 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xE4 | 0x1E4 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xE5 | 0x1E5 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xE6 | 0x1E6 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F2 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F20FE6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F20FE6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_F30FE6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_F30FE6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0xE7 | 0x1E7 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE7_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE7_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
            after_block_boundary();
        }
    },
    0xE8 | 0x1E8 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xE9 | 0x1E9 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FE9_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FE9_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FE9_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FE9_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xEA | 0x1EA => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FEA_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FEA_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FEA_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FEA_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xEB | 0x1EB => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FEB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FEB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FEB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FEB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xEC | 0x1EC => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FEC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FEC_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FEC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FEC_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xED | 0x1ED => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FED_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FED_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FED_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FED_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xEE | 0x1EE => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FEE_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FEE_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FEE_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FEE_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xEF | 0x1EF => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FEF_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FEF_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FEF_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FEF_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xF0 | 0x1F0 => {
        if !task_switch_test_mmx() {
            return;
        }
        instructions_0f::instr_0FF0();
        after_block_boundary();
    },
    0xF1 | 0x1F1 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FF1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FF1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FF1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FF1_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xF2 | 0x1F2 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FF2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FF2_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FF2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FF2_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xF3 | 0x1F3 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FF3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FF3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FF3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FF3_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xF4 | 0x1F4 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FF4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FF4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FF4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FF4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xF5 | 0x1F5 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FF5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FF5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FF5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FF5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xF6 | 0x1F6 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FF6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FF6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FF6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FF6_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xF7 | 0x1F7 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FF7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FF7_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FF7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FF7_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xF8 | 0x1F8 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FF8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FF8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FF8_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FF8_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xF9 | 0x1F9 => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FF9_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FF9_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FF9_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FF9_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xFA | 0x1FA => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FFA_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FFA_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FFA_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FFA_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xFB | 0x1FB => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FFB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FFB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FFB_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FFB_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xFC | 0x1FC => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FFC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FFC_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FFC_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FFC_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xFD | 0x1FD => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FFD_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FFD_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FFD_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FFD_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xFE | 0x1FE => {
        let prefixes_ = *prefixes as i32;
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if prefixes_ & PREFIX_66 != 0 {
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_660FFE_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_660FFE_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_66 | PREFIX_F2 | PREFIX_F3)) == 0);
            if !task_switch_test_mmx() {
                return;
            }
            if modrm_byte < 0xC0 {
                instructions_0f::instr_0FFE_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
            }
            else {
                instructions_0f::instr_0FFE_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
            }
        }
    },
    0xFF | 0x1FF => {
        instructions_0f::instr_0FFF();
        after_block_boundary();
    },
    _ => {
        assert!(false);
    }
}
}
