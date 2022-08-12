#![cfg_attr(rustfmt, rustfmt_skip)]
use cpu::cpu::{after_block_boundary, modrm_resolve};
use cpu::cpu::{read_imm8, read_imm8s, read_imm16, read_imm32s, read_moffs};
use cpu::cpu::{task_switch_test, trigger_ud, DEBUG, PREFIX_F2, PREFIX_F3};
use cpu::instructions;
use cpu::global_pointers::{instruction_pointer, prefixes};
pub unsafe fn run(opcode: u32) {
match opcode {
    0x00 | 0x100 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_00_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_00_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x01 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_01_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_01_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x101 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_01_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_01_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x02 | 0x102 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_02_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_02_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x03 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_03_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_03_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x103 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_03_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_03_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x04 | 0x104 => {
        instructions::instr_04(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0x05 => {
        instructions::instr16_05(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x105 => {
        instructions::instr32_05(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x06 => {
        instructions::instr16_06();
    },
    0x106 => {
        instructions::instr32_06();
    },
    0x07 => {
        instructions::instr16_07();
        after_block_boundary();
    },
    0x107 => {
        instructions::instr32_07();
        after_block_boundary();
    },
    0x08 | 0x108 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_08_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_08_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x09 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_09_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_09_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x109 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_09_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_09_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x0A | 0x10A => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_0A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_0A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x0B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_0B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_0B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x10B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_0B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_0B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x0C | 0x10C => {
        instructions::instr_0C(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0x0D => {
        instructions::instr16_0D(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x10D => {
        instructions::instr32_0D(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x0E => {
        instructions::instr16_0E();
    },
    0x10E => {
        instructions::instr32_0E();
    },
    0x0F => {
        instructions::instr16_0F();
    },
    0x10F => {
        instructions::instr32_0F();
    },
    0x10 | 0x110 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_10_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_10_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x11 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_11_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_11_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x111 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_11_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_11_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x12 | 0x112 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_12_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_12_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x13 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_13_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_13_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x113 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_13_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_13_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x14 | 0x114 => {
        instructions::instr_14(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0x15 => {
        instructions::instr16_15(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x115 => {
        instructions::instr32_15(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x16 => {
        instructions::instr16_16();
    },
    0x116 => {
        instructions::instr32_16();
    },
    0x17 => {
        instructions::instr16_17();
        after_block_boundary();
    },
    0x117 => {
        instructions::instr32_17();
        after_block_boundary();
    },
    0x18 | 0x118 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_18_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_18_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x19 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_19_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_19_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x119 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_19_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_19_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1A | 0x11A => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_1A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_1A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_1B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_1B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x11B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_1B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_1B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x1C | 0x11C => {
        instructions::instr_1C(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0x1D => {
        instructions::instr16_1D(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x11D => {
        instructions::instr32_1D(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x1E => {
        instructions::instr16_1E();
    },
    0x11E => {
        instructions::instr32_1E();
    },
    0x1F => {
        instructions::instr16_1F();
        after_block_boundary();
    },
    0x11F => {
        instructions::instr32_1F();
        after_block_boundary();
    },
    0x20 | 0x120 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_20_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_20_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x21 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_21_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_21_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x121 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_21_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_21_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x22 | 0x122 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_22_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_22_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x23 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_23_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_23_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x123 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_23_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_23_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x24 | 0x124 => {
        instructions::instr_24(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0x25 => {
        instructions::instr16_25(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x125 => {
        instructions::instr32_25(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x26 | 0x126 => {
        instructions::instr_26();
    },
    0x27 | 0x127 => {
        instructions::instr_27();
    },
    0x28 | 0x128 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_28_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_28_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x29 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_29_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_29_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x129 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_29_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_29_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x2A | 0x12A => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_2A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_2A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x2B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_2B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_2B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x12B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_2B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_2B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x2C | 0x12C => {
        instructions::instr_2C(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0x2D => {
        instructions::instr16_2D(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x12D => {
        instructions::instr32_2D(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x2E | 0x12E => {
        instructions::instr_2E();
    },
    0x2F | 0x12F => {
        instructions::instr_2F();
    },
    0x30 | 0x130 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_30_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_30_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x31 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_31_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_31_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x131 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_31_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_31_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x32 | 0x132 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_32_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_32_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x33 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_33_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_33_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x133 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_33_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_33_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x34 | 0x134 => {
        instructions::instr_34(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0x35 => {
        instructions::instr16_35(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x135 => {
        instructions::instr32_35(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x36 | 0x136 => {
        instructions::instr_36();
    },
    0x37 | 0x137 => {
        instructions::instr_37();
    },
    0x38 | 0x138 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_38_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_38_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x39 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_39_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_39_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x139 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_39_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_39_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x3A | 0x13A => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_3A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_3A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x3B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_3B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_3B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x13B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_3B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_3B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x3C | 0x13C => {
        instructions::instr_3C(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0x3D => {
        instructions::instr16_3D(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x13D => {
        instructions::instr32_3D(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x3E | 0x13E => {
        instructions::instr_3E();
    },
    0x3F | 0x13F => {
        instructions::instr_3F();
    },
    0x40 => {
        instructions::instr16_40();
    },
    0x140 => {
        instructions::instr32_40();
    },
    0x41 => {
        instructions::instr16_41();
    },
    0x141 => {
        instructions::instr32_41();
    },
    0x42 => {
        instructions::instr16_42();
    },
    0x142 => {
        instructions::instr32_42();
    },
    0x43 => {
        instructions::instr16_43();
    },
    0x143 => {
        instructions::instr32_43();
    },
    0x44 => {
        instructions::instr16_44();
    },
    0x144 => {
        instructions::instr32_44();
    },
    0x45 => {
        instructions::instr16_45();
    },
    0x145 => {
        instructions::instr32_45();
    },
    0x46 => {
        instructions::instr16_46();
    },
    0x146 => {
        instructions::instr32_46();
    },
    0x47 => {
        instructions::instr16_47();
    },
    0x147 => {
        instructions::instr32_47();
    },
    0x48 => {
        instructions::instr16_48();
    },
    0x148 => {
        instructions::instr32_48();
    },
    0x49 => {
        instructions::instr16_49();
    },
    0x149 => {
        instructions::instr32_49();
    },
    0x4A => {
        instructions::instr16_4A();
    },
    0x14A => {
        instructions::instr32_4A();
    },
    0x4B => {
        instructions::instr16_4B();
    },
    0x14B => {
        instructions::instr32_4B();
    },
    0x4C => {
        instructions::instr16_4C();
    },
    0x14C => {
        instructions::instr32_4C();
    },
    0x4D => {
        instructions::instr16_4D();
    },
    0x14D => {
        instructions::instr32_4D();
    },
    0x4E => {
        instructions::instr16_4E();
    },
    0x14E => {
        instructions::instr32_4E();
    },
    0x4F => {
        instructions::instr16_4F();
    },
    0x14F => {
        instructions::instr32_4F();
    },
    0x50 => {
        instructions::instr16_50();
    },
    0x150 => {
        instructions::instr32_50();
    },
    0x51 => {
        instructions::instr16_51();
    },
    0x151 => {
        instructions::instr32_51();
    },
    0x52 => {
        instructions::instr16_52();
    },
    0x152 => {
        instructions::instr32_52();
    },
    0x53 => {
        instructions::instr16_53();
    },
    0x153 => {
        instructions::instr32_53();
    },
    0x54 => {
        instructions::instr16_54();
    },
    0x154 => {
        instructions::instr32_54();
    },
    0x55 => {
        instructions::instr16_55();
    },
    0x155 => {
        instructions::instr32_55();
    },
    0x56 => {
        instructions::instr16_56();
    },
    0x156 => {
        instructions::instr32_56();
    },
    0x57 => {
        instructions::instr16_57();
    },
    0x157 => {
        instructions::instr32_57();
    },
    0x58 => {
        instructions::instr16_58();
    },
    0x158 => {
        instructions::instr32_58();
    },
    0x59 => {
        instructions::instr16_59();
    },
    0x159 => {
        instructions::instr32_59();
    },
    0x5A => {
        instructions::instr16_5A();
    },
    0x15A => {
        instructions::instr32_5A();
    },
    0x5B => {
        instructions::instr16_5B();
    },
    0x15B => {
        instructions::instr32_5B();
    },
    0x5C => {
        instructions::instr16_5C();
    },
    0x15C => {
        instructions::instr32_5C();
    },
    0x5D => {
        instructions::instr16_5D();
    },
    0x15D => {
        instructions::instr32_5D();
    },
    0x5E => {
        instructions::instr16_5E();
    },
    0x15E => {
        instructions::instr32_5E();
    },
    0x5F => {
        instructions::instr16_5F();
    },
    0x15F => {
        instructions::instr32_5F();
    },
    0x60 => {
        instructions::instr16_60();
        after_block_boundary();
    },
    0x160 => {
        instructions::instr32_60();
        after_block_boundary();
    },
    0x61 => {
        instructions::instr16_61();
        after_block_boundary();
    },
    0x161 => {
        instructions::instr32_61();
        after_block_boundary();
    },
    0x62 | 0x162 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_62_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_62_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x63 | 0x163 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_63_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_63_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x64 | 0x164 => {
        instructions::instr_64();
    },
    0x65 | 0x165 => {
        instructions::instr_65();
    },
    0x66 | 0x166 => {
        instructions::instr_66();
    },
    0x67 | 0x167 => {
        instructions::instr_67();
    },
    0x68 => {
        instructions::instr16_68(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x168 => {
        instructions::instr32_68(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0x69 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_69_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
        }
        else {
            instructions::instr16_69_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
        }
    },
    0x169 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_69_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
        }
        else {
            instructions::instr32_69_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
        }
    },
    0x6A => {
        instructions::instr16_6A(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x16A => {
        instructions::instr32_6A(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x6B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_6B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
        }
        else {
            instructions::instr16_6B_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
        }
    },
    0x16B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_6B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
        }
        else {
            instructions::instr32_6B_reg(modrm_byte & 7, modrm_byte >> 3 & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
        }
    },
    0x6C | 0x16C => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr_F26C();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr_F36C();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr_6C();
            after_block_boundary();
        }
    },
    0x6D => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr16_F26D();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr16_F36D();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr16_6D();
            after_block_boundary();
        }
    },
    0x16D => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr32_F26D();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr32_F36D();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr32_6D();
            after_block_boundary();
        }
    },
    0x6E | 0x16E => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr_F26E();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr_F36E();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr_6E();
            after_block_boundary();
        }
    },
    0x6F => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr16_F26F();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr16_F36F();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr16_6F();
            after_block_boundary();
        }
    },
    0x16F => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr32_F26F();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr32_F36F();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr32_6F();
            after_block_boundary();
        }
    },
    0x70 => {
        instructions::instr16_70(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x170 => {
        instructions::instr32_70(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x71 => {
        instructions::instr16_71(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x171 => {
        instructions::instr32_71(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x72 => {
        instructions::instr16_72(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x172 => {
        instructions::instr32_72(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x73 => {
        instructions::instr16_73(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x173 => {
        instructions::instr32_73(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x74 => {
        instructions::instr16_74(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x174 => {
        instructions::instr32_74(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x75 => {
        instructions::instr16_75(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x175 => {
        instructions::instr32_75(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x76 => {
        instructions::instr16_76(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x176 => {
        instructions::instr32_76(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x77 => {
        instructions::instr16_77(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x177 => {
        instructions::instr32_77(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x78 => {
        instructions::instr16_78(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x178 => {
        instructions::instr32_78(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x79 => {
        instructions::instr16_79(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x179 => {
        instructions::instr32_79(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x7A => {
        instructions::instr16_7A(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x17A => {
        instructions::instr32_7A(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x7B => {
        instructions::instr16_7B(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x17B => {
        instructions::instr32_7B(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x7C => {
        instructions::instr16_7C(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x17C => {
        instructions::instr32_7C(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x7D => {
        instructions::instr16_7D(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x17D => {
        instructions::instr32_7D(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x7E => {
        instructions::instr16_7E(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x17E => {
        instructions::instr32_7E(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x7F => {
        instructions::instr16_7F(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x17F => {
        instructions::instr32_7F(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x80 | 0x180 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_80_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_80_0_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_80_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_80_1_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_80_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_80_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_80_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_80_3_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_80_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_80_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_80_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_80_5_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_80_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_80_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_80_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_80_7_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x81 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_81_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_81_0_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_81_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_81_1_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_81_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_81_2_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_81_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_81_3_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_81_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_81_4_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_81_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_81_5_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_81_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_81_6_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_81_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_81_7_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x181 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_81_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_81_0_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_81_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_81_1_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_81_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_81_2_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_81_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_81_3_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_81_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_81_4_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_81_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_81_5_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_81_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_81_6_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_81_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_81_7_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x82 | 0x182 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_82_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_82_0_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_82_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_82_1_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_82_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_82_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_82_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_82_3_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_82_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_82_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_82_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_82_5_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_82_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_82_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_82_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_82_7_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x83 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_83_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_83_0_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_83_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_83_1_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_83_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_83_2_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_83_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_83_3_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_83_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_83_4_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_83_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_83_5_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_83_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_83_6_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_83_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_83_7_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x183 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_83_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_83_0_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_83_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_83_1_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_83_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_83_2_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_83_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_83_3_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_83_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_83_4_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_83_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_83_5_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_83_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_83_6_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_83_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_83_7_reg(modrm_byte & 7, match read_imm8s() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x84 | 0x184 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_84_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_84_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x85 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_85_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_85_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x185 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_85_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_85_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x86 | 0x186 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_86_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_86_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x87 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_87_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_87_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x187 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_87_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_87_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x88 | 0x188 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_88_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_88_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x89 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_89_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_89_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x189 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_89_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_89_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x8A | 0x18A => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_8A_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_8A_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x8B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_8B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_8B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x18B => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_8B_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_8B_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x8C => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_8C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_8C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x18C => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_8C_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_8C_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x8D => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_8D_mem(modrm_byte, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_8D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x18D => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_8D_mem(modrm_byte, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_8D_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
    },
    0x8E | 0x18E => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr_8E_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr_8E_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x8F => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_8F_0_mem(modrm_byte);
                }
                else {
                    instructions::instr16_8F_0_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x18F => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_8F_0_mem(modrm_byte);
                }
                else {
                    instructions::instr32_8F_0_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x90 | 0x190 => {
        instructions::instr_90();
    },
    0x91 => {
        instructions::instr16_91();
    },
    0x191 => {
        instructions::instr32_91();
    },
    0x92 => {
        instructions::instr16_92();
    },
    0x192 => {
        instructions::instr32_92();
    },
    0x93 => {
        instructions::instr16_93();
    },
    0x193 => {
        instructions::instr32_93();
    },
    0x94 => {
        instructions::instr16_94();
    },
    0x194 => {
        instructions::instr32_94();
    },
    0x95 => {
        instructions::instr16_95();
    },
    0x195 => {
        instructions::instr32_95();
    },
    0x96 => {
        instructions::instr16_96();
    },
    0x196 => {
        instructions::instr32_96();
    },
    0x97 => {
        instructions::instr16_97();
    },
    0x197 => {
        instructions::instr32_97();
    },
    0x98 => {
        instructions::instr16_98();
    },
    0x198 => {
        instructions::instr32_98();
    },
    0x99 => {
        instructions::instr16_99();
    },
    0x199 => {
        instructions::instr32_99();
    },
    0x9A => {
        instructions::instr16_9A(match read_imm16() { Ok(o) => o, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0x19A => {
        instructions::instr32_9A(match read_imm32s() { Ok(o) => o, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0x9B | 0x19B => {
        instructions::instr_9B();
        after_block_boundary();
    },
    0x9C => {
        instructions::instr16_9C();
    },
    0x19C => {
        instructions::instr32_9C();
    },
    0x9D => {
        instructions::instr16_9D();
    },
    0x19D => {
        instructions::instr32_9D();
    },
    0x9E | 0x19E => {
        instructions::instr_9E();
    },
    0x9F | 0x19F => {
        instructions::instr_9F();
    },
    0xA0 | 0x1A0 => {
        instructions::instr_A0(match read_moffs() { Ok(o) => o, Err(()) => return });
    },
    0xA1 => {
        instructions::instr16_A1(match read_moffs() { Ok(o) => o, Err(()) => return });
    },
    0x1A1 => {
        instructions::instr32_A1(match read_moffs() { Ok(o) => o, Err(()) => return });
    },
    0xA2 | 0x1A2 => {
        instructions::instr_A2(match read_moffs() { Ok(o) => o, Err(()) => return });
    },
    0xA3 => {
        instructions::instr16_A3(match read_moffs() { Ok(o) => o, Err(()) => return });
    },
    0x1A3 => {
        instructions::instr32_A3(match read_moffs() { Ok(o) => o, Err(()) => return });
    },
    0xA4 | 0x1A4 => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr_F2A4();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr_F3A4();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr_A4();
        }
    },
    0xA5 => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr16_F2A5();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr16_F3A5();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr16_A5();
        }
    },
    0x1A5 => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr32_F2A5();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr32_F3A5();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr32_A5();
        }
    },
    0xA6 | 0x1A6 => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr_F2A6();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr_F3A6();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr_A6();
            after_block_boundary();
        }
    },
    0xA7 => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr16_F2A7();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr16_F3A7();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr16_A7();
            after_block_boundary();
        }
    },
    0x1A7 => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr32_F2A7();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr32_F3A7();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr32_A7();
            after_block_boundary();
        }
    },
    0xA8 | 0x1A8 => {
        instructions::instr_A8(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xA9 => {
        instructions::instr16_A9(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1A9 => {
        instructions::instr32_A9(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xAA | 0x1AA => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr_F2AA();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr_F3AA();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr_AA();
        }
    },
    0xAB => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr16_F2AB();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr16_F3AB();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr16_AB();
        }
    },
    0x1AB => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr32_F2AB();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr32_F3AB();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr32_AB();
        }
    },
    0xAC | 0x1AC => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr_F2AC();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr_F3AC();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr_AC();
        }
    },
    0xAD => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr16_F2AD();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr16_F3AD();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr16_AD();
        }
    },
    0x1AD => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr32_F2AD();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr32_F3AD();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr32_AD();
        }
    },
    0xAE | 0x1AE => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr_F2AE();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr_F3AE();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr_AE();
        }
    },
    0xAF => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr16_F2AF();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr16_F3AF();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr16_AF();
        }
    },
    0x1AF => {
        let prefixes_ = *prefixes as i32;
        if prefixes_ & PREFIX_F2 != 0 {
            instructions::instr32_F2AF();
            after_block_boundary();
        }
        else if prefixes_ & PREFIX_F3 != 0 {
            instructions::instr32_F3AF();
            after_block_boundary();
        }
        else {
            dbg_assert!((prefixes_ & (PREFIX_F2 | PREFIX_F3)) == 0);
            instructions::instr32_AF();
        }
    },
    0xB0 | 0x1B0 => {
        instructions::instr_B0(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xB1 | 0x1B1 => {
        instructions::instr_B1(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xB2 | 0x1B2 => {
        instructions::instr_B2(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xB3 | 0x1B3 => {
        instructions::instr_B3(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xB4 | 0x1B4 => {
        instructions::instr_B4(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xB5 | 0x1B5 => {
        instructions::instr_B5(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xB6 | 0x1B6 => {
        instructions::instr_B6(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xB7 | 0x1B7 => {
        instructions::instr_B7(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xB8 => {
        instructions::instr16_B8(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1B8 => {
        instructions::instr32_B8(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xB9 => {
        instructions::instr16_B9(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1B9 => {
        instructions::instr32_B9(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xBA => {
        instructions::instr16_BA(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1BA => {
        instructions::instr32_BA(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xBB => {
        instructions::instr16_BB(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1BB => {
        instructions::instr32_BB(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xBC => {
        instructions::instr16_BC(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1BC => {
        instructions::instr32_BC(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xBD => {
        instructions::instr16_BD(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1BD => {
        instructions::instr32_BD(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xBE => {
        instructions::instr16_BE(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1BE => {
        instructions::instr32_BE(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xBF => {
        instructions::instr16_BF(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1BF => {
        instructions::instr32_BF(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xC0 | 0x1C0 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_C0_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_C0_0_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_C0_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_C0_1_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_C0_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_C0_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_C0_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_C0_3_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_C0_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_C0_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_C0_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_C0_5_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_C0_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_C0_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_C0_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_C0_7_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xC1 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_C1_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_C1_0_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_C1_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_C1_1_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_C1_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_C1_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_C1_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_C1_3_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_C1_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_C1_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_C1_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_C1_5_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_C1_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_C1_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_C1_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_C1_7_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x1C1 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_C1_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_C1_0_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_C1_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_C1_1_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_C1_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_C1_2_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_C1_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_C1_3_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_C1_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_C1_4_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_C1_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_C1_5_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_C1_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_C1_6_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_C1_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_C1_7_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xC2 => {
        instructions::instr16_C2(match read_imm16() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0x1C2 => {
        instructions::instr32_C2(match read_imm16() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xC3 => {
        instructions::instr16_C3();
        after_block_boundary();
    },
    0x1C3 => {
        instructions::instr32_C3();
        after_block_boundary();
    },
    0xC4 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_C4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_C4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x1C4 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_C4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_C4_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0xC5 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr16_C5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr16_C5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0x1C5 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        if modrm_byte < 0xC0 {
            instructions::instr32_C5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, modrm_byte >> 3 & 7);
        }
        else {
            instructions::instr32_C5_reg(modrm_byte & 7, modrm_byte >> 3 & 7);
        }
        after_block_boundary();
    },
    0xC6 | 0x1C6 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_C6_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_C6_0_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xC7 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_C7_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_C7_0_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
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
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_C7_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_C7_0_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xC8 => {
        instructions::instr16_C8(match read_imm16() { Ok(o) => o, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0x1C8 => {
        instructions::instr32_C8(match read_imm16() { Ok(o) => o, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xC9 => {
        instructions::instr16_C9();
    },
    0x1C9 => {
        instructions::instr32_C9();
    },
    0xCA => {
        instructions::instr16_CA(match read_imm16() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0x1CA => {
        instructions::instr32_CA(match read_imm16() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xCB => {
        instructions::instr16_CB();
        after_block_boundary();
    },
    0x1CB => {
        instructions::instr32_CB();
        after_block_boundary();
    },
    0xCC | 0x1CC => {
        instructions::instr_CC();
        after_block_boundary();
    },
    0xCD | 0x1CD => {
        instructions::instr_CD(match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xCE | 0x1CE => {
        instructions::instr_CE();
        after_block_boundary();
    },
    0xCF => {
        instructions::instr16_CF();
        after_block_boundary();
    },
    0x1CF => {
        instructions::instr32_CF();
        after_block_boundary();
    },
    0xD0 | 0x1D0 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D0_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D0_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D0_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D0_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D0_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D0_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D0_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D0_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D0_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D0_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D0_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D0_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D0_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D0_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D0_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D0_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xD1 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D1_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D1_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D1_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D1_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D1_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D1_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D1_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D1_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D1_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D1_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D1_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D1_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D1_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D1_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D1_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D1_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x1D1 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D1_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D1_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D1_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D1_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D1_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D1_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D1_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D1_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D1_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D1_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D1_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D1_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D1_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D1_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D1_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D1_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xD2 | 0x1D2 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D2_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D2_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D2_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D2_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D2_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D2_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D2_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D2_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D2_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D2_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D2_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D2_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D2_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D2_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_D2_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D2_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xD3 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D3_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D3_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D3_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D3_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D3_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D3_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D3_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D3_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D3_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D3_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D3_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D3_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D3_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D3_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_D3_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D3_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x1D3 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D3_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D3_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D3_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D3_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D3_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D3_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D3_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D3_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D3_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D3_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D3_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D3_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D3_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D3_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_D3_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D3_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xD4 | 0x1D4 => {
        instructions::instr_D4(match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xD5 | 0x1D5 => {
        instructions::instr_D5(match read_imm8() { Ok(o) => o, Err(()) => return });
    },
    0xD6 | 0x1D6 => {
        instructions::instr_D6();
    },
    0xD7 | 0x1D7 => {
        instructions::instr_D7();
    },
    0xD8 | 0x1D8 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_D8_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D8_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_D8_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D8_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_D8_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D8_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_D8_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D8_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_D8_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D8_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_D8_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D8_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_D8_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D8_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_D8_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_D8_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xD9 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_D9_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D9_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_D9_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D9_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_D9_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D9_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_D9_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D9_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_D9_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D9_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_D9_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D9_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_D9_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D9_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_D9_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_D9_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x1D9 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_D9_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D9_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_D9_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D9_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_D9_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D9_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_D9_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D9_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_D9_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D9_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_D9_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D9_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_D9_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D9_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_D9_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_D9_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xDA | 0x1DA => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DA_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DA_0_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DA_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DA_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DA_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DA_2_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DA_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DA_3_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DA_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DA_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DA_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DA_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DA_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DA_6_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DA_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DA_7_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xDB | 0x1DB => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DB_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DB_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DB_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DB_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DB_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DB_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DB_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DB_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DB_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DB_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DB_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DB_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DB_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DB_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DB_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DB_7_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xDC | 0x1DC => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DC_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DC_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DC_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DC_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DC_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DC_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DC_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DC_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DC_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DC_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DC_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DC_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DC_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DC_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DC_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DC_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xDD => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_DD_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_DD_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_DD_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_DD_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_DD_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_DD_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_DD_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_DD_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_DD_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_DD_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_DD_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_DD_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_DD_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_DD_6_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr16_DD_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_DD_7_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x1DD => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_DD_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_DD_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_DD_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_DD_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_DD_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_DD_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_DD_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_DD_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_DD_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_DD_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_DD_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_DD_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_DD_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_DD_6_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr32_DD_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_DD_7_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xDE | 0x1DE => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DE_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DE_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DE_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DE_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DE_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DE_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DE_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DE_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DE_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DE_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DE_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DE_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DE_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DE_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DE_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DE_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xDF | 0x1DF => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DF_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DF_0_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            1 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DF_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DF_1_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            2 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DF_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DF_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DF_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DF_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DF_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DF_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DF_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DF_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DF_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DF_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if !task_switch_test() {
                    return;
                }
                if modrm_byte < 0xC0 {
                    instructions::instr_DF_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_DF_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xE0 => {
        instructions::instr16_E0(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x1E0 => {
        instructions::instr32_E0(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0xE1 => {
        instructions::instr16_E1(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x1E1 => {
        instructions::instr32_E1(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0xE2 => {
        instructions::instr16_E2(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x1E2 => {
        instructions::instr32_E2(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0xE3 => {
        instructions::instr16_E3(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x1E3 => {
        instructions::instr32_E3(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0xE4 | 0x1E4 => {
        instructions::instr_E4(match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xE5 => {
        instructions::instr16_E5(match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0x1E5 => {
        instructions::instr32_E5(match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xE6 | 0x1E6 => {
        instructions::instr_E6(match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xE7 => {
        instructions::instr16_E7(match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0x1E7 => {
        instructions::instr32_E7(match read_imm8() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xE8 => {
        instructions::instr16_E8(match read_imm16() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0x1E8 => {
        instructions::instr32_E8(match read_imm32s() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xE9 => {
        instructions::instr16_E9(match read_imm16() { Ok(o) => o, Err(()) => return });
    },
    0x1E9 => {
        instructions::instr32_E9(match read_imm32s() { Ok(o) => o, Err(()) => return });
    },
    0xEA => {
        instructions::instr16_EA(match read_imm16() { Ok(o) => o, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0x1EA => {
        instructions::instr32_EA(match read_imm32s() { Ok(o) => o, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
        after_block_boundary();
    },
    0xEB => {
        instructions::instr16_EB(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0x1EB => {
        instructions::instr32_EB(match read_imm8s() { Ok(o) => o, Err(()) => return });
    },
    0xEC | 0x1EC => {
        instructions::instr_EC();
        after_block_boundary();
    },
    0xED => {
        instructions::instr16_ED();
        after_block_boundary();
    },
    0x1ED => {
        instructions::instr32_ED();
        after_block_boundary();
    },
    0xEE | 0x1EE => {
        instructions::instr_EE();
        after_block_boundary();
    },
    0xEF => {
        instructions::instr16_EF();
        after_block_boundary();
    },
    0x1EF => {
        instructions::instr32_EF();
        after_block_boundary();
    },
    0xF0 | 0x1F0 => {
        instructions::instr_F0();
    },
    0xF1 | 0x1F1 => {
        instructions::instr_F1();
    },
    0xF2 | 0x1F2 => {
        instructions::instr_F2();
    },
    0xF3 | 0x1F3 => {
        instructions::instr_F3();
    },
    0xF4 | 0x1F4 => {
        instructions::instr_F4();
        after_block_boundary();
    },
    0xF5 | 0x1F5 => {
        instructions::instr_F5();
    },
    0xF6 | 0x1F6 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_F6_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_F6_0_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_F6_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr_F6_1_reg(modrm_byte & 7, match read_imm8() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_F6_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_F6_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_F6_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_F6_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_F6_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_F6_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_F6_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_F6_5_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_F6_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_F6_6_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_F6_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_F6_7_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xF7 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_F7_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_F7_0_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_F7_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr16_F7_1_reg(modrm_byte & 7, match read_imm16() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_F7_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_F7_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_F7_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_F7_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_F7_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_F7_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_F7_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_F7_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_F7_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_F7_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_F7_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_F7_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x1F7 => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_F7_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_F7_0_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_F7_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return }, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
                else {
                    instructions::instr32_F7_1_reg(modrm_byte & 7, match read_imm32s() { Ok(o) => o, Err(()) => return });
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_F7_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_F7_2_reg(modrm_byte & 7);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_F7_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_F7_3_reg(modrm_byte & 7);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_F7_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_F7_4_reg(modrm_byte & 7);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_F7_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_F7_5_reg(modrm_byte & 7);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_F7_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_F7_6_reg(modrm_byte & 7);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_F7_7_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_F7_7_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xF8 | 0x1F8 => {
        instructions::instr_F8();
    },
    0xF9 | 0x1F9 => {
        instructions::instr_F9();
    },
    0xFA | 0x1FA => {
        instructions::instr_FA();
    },
    0xFB | 0x1FB => {
        instructions::instr_FB();
    },
    0xFC | 0x1FC => {
        instructions::instr_FC();
    },
    0xFD | 0x1FD => {
        instructions::instr_FD();
    },
    0xFE | 0x1FE => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_FE_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_FE_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr_FE_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr_FE_1_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0xFF => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_FF_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_FF_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_FF_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_FF_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_FF_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_FF_2_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_FF_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_FF_3_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_FF_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_FF_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_FF_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_FF_5_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr16_FF_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr16_FF_6_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    0x1FF => {
        let modrm_byte = match read_imm8() { Ok(o) => o, Err(()) => return };
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_FF_0_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_FF_0_reg(modrm_byte & 7);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_FF_1_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_FF_1_reg(modrm_byte & 7);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_FF_2_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_FF_2_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_FF_3_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_FF_3_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_FF_4_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_FF_4_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_FF_5_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_FF_5_reg(modrm_byte & 7);
                }
                after_block_boundary();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    instructions::instr32_FF_6_mem(match modrm_resolve(modrm_byte) { Ok(a) => a, Err(()) => return });
                }
                else {
                    instructions::instr32_FF_6_reg(modrm_byte & 7);
                }
            },
            _ => {
                if DEBUG { panic!("Bad instruction at {:x}", *instruction_pointer); }
                trigger_ud();
            }
        }
    },
    _ => {
        assert!(false);
    }
}
}
