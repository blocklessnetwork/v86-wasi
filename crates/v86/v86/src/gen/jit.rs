#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn jit(opcode: u32, ctx: &mut ::jit::JitContext, instr_flags: &mut u32) {
match opcode {
    0x00 | 0x100 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_00_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_00_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x01 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_01_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_01_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x101 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_01_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_01_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x02 | 0x102 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_02_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_02_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x03 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_03_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_03_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x103 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_03_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_03_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x04 | 0x104 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_04_jit(ctx, imm);
    },
    0x05 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_05_jit(ctx, imm);
    },
    0x105 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_05_jit(ctx, imm);
    },
    0x06 => {
        ::jit_instructions::instr16_06_jit(ctx);
    },
    0x106 => {
        ::jit_instructions::instr32_06_jit(ctx);
    },
    0x07 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_07");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x107 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_07");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x08 | 0x108 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_08_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_08_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x09 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_09_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_09_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x109 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_09_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_09_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x0A | 0x10A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x0B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x10B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x0C | 0x10C => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_0C_jit(ctx, imm);
    },
    0x0D => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0D_jit(ctx, imm);
    },
    0x10D => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0D_jit(ctx, imm);
    },
    0x0E => {
        ::jit_instructions::instr16_0E_jit(ctx);
    },
    0x10E => {
        ::jit_instructions::instr32_0E_jit(ctx);
    },
    0x0F => {
        ::jit_instructions::instr16_0F_jit(ctx, instr_flags);
    },
    0x10F => {
        ::jit_instructions::instr32_0F_jit(ctx, instr_flags);
    },
    0x10 | 0x110 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_10_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_10_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x11 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_11_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_11_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x111 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_11_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_11_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x12 | 0x112 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_12_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_12_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x13 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_13_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_13_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x113 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_13_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_13_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x14 | 0x114 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_14_jit(ctx, imm);
    },
    0x15 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_15_jit(ctx, imm);
    },
    0x115 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_15_jit(ctx, imm);
    },
    0x16 => {
        ::jit_instructions::instr16_16_jit(ctx);
    },
    0x116 => {
        ::jit_instructions::instr32_16_jit(ctx);
    },
    0x17 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_17");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x117 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_17");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x18 | 0x118 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_18_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_18_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x19 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_19_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_19_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x119 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_19_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_19_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1A | 0x11A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_1A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_1A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_1B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_1B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x11B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_1B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_1B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1C | 0x11C => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_1C_jit(ctx, imm);
    },
    0x1D => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_1D_jit(ctx, imm);
    },
    0x11D => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_1D_jit(ctx, imm);
    },
    0x1E => {
        ::jit_instructions::instr16_1E_jit(ctx);
    },
    0x11E => {
        ::jit_instructions::instr32_1E_jit(ctx);
    },
    0x1F => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_1F");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x11F => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_1F");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x20 | 0x120 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_20_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_20_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x21 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_21_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_21_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x121 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_21_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_21_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x22 | 0x122 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_22_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_22_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x23 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_23_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_23_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x123 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_23_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_23_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x24 | 0x124 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_24_jit(ctx, imm);
    },
    0x25 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_25_jit(ctx, imm);
    },
    0x125 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_25_jit(ctx, imm);
    },
    0x26 | 0x126 => {
        ::jit_instructions::instr_26_jit(ctx, instr_flags);
    },
    0x27 | 0x127 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_27");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x28 | 0x128 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_28_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_28_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x29 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_29_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_29_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x129 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_29_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_29_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x2A | 0x12A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_2A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_2A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x2B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_2B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_2B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x12B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_2B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_2B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x2C | 0x12C => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_2C_jit(ctx, imm);
    },
    0x2D => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_2D_jit(ctx, imm);
    },
    0x12D => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_2D_jit(ctx, imm);
    },
    0x2E | 0x12E => {
        ::jit_instructions::instr_2E_jit(ctx, instr_flags);
    },
    0x2F | 0x12F => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_2F");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x30 | 0x130 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_30_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_30_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x31 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_31_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_31_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x131 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_31_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_31_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x32 | 0x132 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_32_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_32_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x33 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_33_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_33_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x133 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_33_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_33_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x34 | 0x134 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_34_jit(ctx, imm);
    },
    0x35 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_35_jit(ctx, imm);
    },
    0x135 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_35_jit(ctx, imm);
    },
    0x36 | 0x136 => {
        ::jit_instructions::instr_36_jit(ctx, instr_flags);
    },
    0x37 | 0x137 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_37");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x38 | 0x138 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_38_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_38_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x39 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_39_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_39_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x139 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_39_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_39_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x3A | 0x13A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_3A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_3A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x3B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_3B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_3B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x13B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_3B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_3B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x3C | 0x13C => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_3C_jit(ctx, imm);
    },
    0x3D => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_3D_jit(ctx, imm);
    },
    0x13D => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_3D_jit(ctx, imm);
    },
    0x3E | 0x13E => {
        ::jit_instructions::instr_3E_jit(ctx, instr_flags);
    },
    0x3F | 0x13F => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_3F");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x40 => {
        ::jit_instructions::instr16_40_jit(ctx);
    },
    0x140 => {
        ::jit_instructions::instr32_40_jit(ctx);
    },
    0x41 => {
        ::jit_instructions::instr16_41_jit(ctx);
    },
    0x141 => {
        ::jit_instructions::instr32_41_jit(ctx);
    },
    0x42 => {
        ::jit_instructions::instr16_42_jit(ctx);
    },
    0x142 => {
        ::jit_instructions::instr32_42_jit(ctx);
    },
    0x43 => {
        ::jit_instructions::instr16_43_jit(ctx);
    },
    0x143 => {
        ::jit_instructions::instr32_43_jit(ctx);
    },
    0x44 => {
        ::jit_instructions::instr16_44_jit(ctx);
    },
    0x144 => {
        ::jit_instructions::instr32_44_jit(ctx);
    },
    0x45 => {
        ::jit_instructions::instr16_45_jit(ctx);
    },
    0x145 => {
        ::jit_instructions::instr32_45_jit(ctx);
    },
    0x46 => {
        ::jit_instructions::instr16_46_jit(ctx);
    },
    0x146 => {
        ::jit_instructions::instr32_46_jit(ctx);
    },
    0x47 => {
        ::jit_instructions::instr16_47_jit(ctx);
    },
    0x147 => {
        ::jit_instructions::instr32_47_jit(ctx);
    },
    0x48 => {
        ::jit_instructions::instr16_48_jit(ctx);
    },
    0x148 => {
        ::jit_instructions::instr32_48_jit(ctx);
    },
    0x49 => {
        ::jit_instructions::instr16_49_jit(ctx);
    },
    0x149 => {
        ::jit_instructions::instr32_49_jit(ctx);
    },
    0x4A => {
        ::jit_instructions::instr16_4A_jit(ctx);
    },
    0x14A => {
        ::jit_instructions::instr32_4A_jit(ctx);
    },
    0x4B => {
        ::jit_instructions::instr16_4B_jit(ctx);
    },
    0x14B => {
        ::jit_instructions::instr32_4B_jit(ctx);
    },
    0x4C => {
        ::jit_instructions::instr16_4C_jit(ctx);
    },
    0x14C => {
        ::jit_instructions::instr32_4C_jit(ctx);
    },
    0x4D => {
        ::jit_instructions::instr16_4D_jit(ctx);
    },
    0x14D => {
        ::jit_instructions::instr32_4D_jit(ctx);
    },
    0x4E => {
        ::jit_instructions::instr16_4E_jit(ctx);
    },
    0x14E => {
        ::jit_instructions::instr32_4E_jit(ctx);
    },
    0x4F => {
        ::jit_instructions::instr16_4F_jit(ctx);
    },
    0x14F => {
        ::jit_instructions::instr32_4F_jit(ctx);
    },
    0x50 => {
        ::jit_instructions::instr16_50_jit(ctx);
    },
    0x150 => {
        ::jit_instructions::instr32_50_jit(ctx);
    },
    0x51 => {
        ::jit_instructions::instr16_51_jit(ctx);
    },
    0x151 => {
        ::jit_instructions::instr32_51_jit(ctx);
    },
    0x52 => {
        ::jit_instructions::instr16_52_jit(ctx);
    },
    0x152 => {
        ::jit_instructions::instr32_52_jit(ctx);
    },
    0x53 => {
        ::jit_instructions::instr16_53_jit(ctx);
    },
    0x153 => {
        ::jit_instructions::instr32_53_jit(ctx);
    },
    0x54 => {
        ::jit_instructions::instr16_54_jit(ctx);
    },
    0x154 => {
        ::jit_instructions::instr32_54_jit(ctx);
    },
    0x55 => {
        ::jit_instructions::instr16_55_jit(ctx);
    },
    0x155 => {
        ::jit_instructions::instr32_55_jit(ctx);
    },
    0x56 => {
        ::jit_instructions::instr16_56_jit(ctx);
    },
    0x156 => {
        ::jit_instructions::instr32_56_jit(ctx);
    },
    0x57 => {
        ::jit_instructions::instr16_57_jit(ctx);
    },
    0x157 => {
        ::jit_instructions::instr32_57_jit(ctx);
    },
    0x58 => {
        ::jit_instructions::instr16_58_jit(ctx);
    },
    0x158 => {
        ::jit_instructions::instr32_58_jit(ctx);
    },
    0x59 => {
        ::jit_instructions::instr16_59_jit(ctx);
    },
    0x159 => {
        ::jit_instructions::instr32_59_jit(ctx);
    },
    0x5A => {
        ::jit_instructions::instr16_5A_jit(ctx);
    },
    0x15A => {
        ::jit_instructions::instr32_5A_jit(ctx);
    },
    0x5B => {
        ::jit_instructions::instr16_5B_jit(ctx);
    },
    0x15B => {
        ::jit_instructions::instr32_5B_jit(ctx);
    },
    0x5C => {
        ::jit_instructions::instr16_5C_jit(ctx);
    },
    0x15C => {
        ::jit_instructions::instr32_5C_jit(ctx);
    },
    0x5D => {
        ::jit_instructions::instr16_5D_jit(ctx);
    },
    0x15D => {
        ::jit_instructions::instr32_5D_jit(ctx);
    },
    0x5E => {
        ::jit_instructions::instr16_5E_jit(ctx);
    },
    0x15E => {
        ::jit_instructions::instr32_5E_jit(ctx);
    },
    0x5F => {
        ::jit_instructions::instr16_5F_jit(ctx);
    },
    0x15F => {
        ::jit_instructions::instr32_5F_jit(ctx);
    },
    0x60 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_60");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x160 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_60");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x61 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_61");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x161 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_61");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x62 | 0x162 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr_62_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr_62_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x63 | 0x163 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr_63_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr_63_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x64 | 0x164 => {
        ::jit_instructions::instr_64_jit(ctx, instr_flags);
    },
    0x65 | 0x165 => {
        ::jit_instructions::instr_65_jit(ctx, instr_flags);
    },
    0x66 | 0x166 => {
        ::jit_instructions::instr_66_jit(ctx, instr_flags);
    },
    0x67 | 0x167 => {
        ::jit_instructions::instr_67_jit(ctx, instr_flags);
    },
    0x68 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_68_jit(ctx, imm);
    },
    0x168 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_68_jit(ctx, imm);
    },
    0x69 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            let imm = ctx.cpu.read_imm16() as u32;
            ::jit_instructions::instr16_69_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
        }
        else {
            let imm = ctx.cpu.read_imm16() as u32;
            ::jit_instructions::instr16_69_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
        }
    },
    0x169 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            let imm = ctx.cpu.read_imm32() as u32;
            ::jit_instructions::instr32_69_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
        }
        else {
            let imm = ctx.cpu.read_imm32() as u32;
            ::jit_instructions::instr32_69_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
        }
    },
    0x6A => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_6A_jit(ctx, imm);
    },
    0x16A => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_6A_jit(ctx, imm);
    },
    0x6B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            let imm = ctx.cpu.read_imm8s() as u32;
            ::jit_instructions::instr16_6B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
        }
        else {
            let imm = ctx.cpu.read_imm8s() as u32;
            ::jit_instructions::instr16_6B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
        }
    },
    0x16B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            let imm = ctx.cpu.read_imm8s() as u32;
            ::jit_instructions::instr32_6B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
        }
        else {
            let imm = ctx.cpu.read_imm8s() as u32;
            ::jit_instructions::instr32_6B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
        }
    },
    0x6C | 0x16C => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr_F26C_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr_F36C_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr_6C_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0x6D => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr16_F26D_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr16_F36D_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr16_6D_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0x16D => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr32_F26D_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr32_F36D_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr32_6D_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0x6E | 0x16E => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr_F26E_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr_F36E_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr_6E_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0x6F => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr16_F26F_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr16_F36F_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr16_6F_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0x16F => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr32_F26F_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr32_F36F_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr32_6F_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0x70 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_70_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x170 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_70_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x71 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_71_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x171 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_71_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x72 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_72_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x172 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_72_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x73 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_73_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x173 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_73_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x74 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_74_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x174 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_74_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x75 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_75_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x175 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_75_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x76 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_76_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x176 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_76_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x77 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_77_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x177 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_77_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x78 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_78_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x178 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_78_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x79 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_79_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x179 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_79_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x7A => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_7A_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x17A => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_7A_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x7B => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_7B_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x17B => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_7B_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x7C => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_7C_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x17C => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_7C_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x7D => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_7D_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x17D => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_7D_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x7E => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_7E_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x17E => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_7E_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x7F => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_7F_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x17F => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_7F_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x80 | 0x180 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_2_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_3_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_80_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x81 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_2_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_3_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_81_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x181 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_2_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_3_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_81_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x82 | 0x182 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_2_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_3_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_82_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x83 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_2_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_3_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr16_83_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x183 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_2_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_3_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8s() as u32;
                    ::jit_instructions::instr32_83_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x84 | 0x184 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_84_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_84_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x85 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_85_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_85_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x185 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_85_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_85_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x86 | 0x186 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_86_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_86_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x87 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_87_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_87_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x187 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_87_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_87_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x88 | 0x188 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_88_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_88_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x89 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_89_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_89_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x189 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_89_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_89_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x8A | 0x18A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_8A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_8A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x8B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_8B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_8B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x18B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_8B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_8B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x8C => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_8C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_8C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x18C => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_8C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_8C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x8D => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_8D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_8D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0x18D => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_8D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_8D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0x8E | 0x18E => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr_8E_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr_8E_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x8F => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_8F_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_8F_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x18F => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_8F_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_8F_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x90 | 0x190 => {
        ::jit_instructions::instr_90_jit(ctx);
    },
    0x91 => {
        ::jit_instructions::instr16_91_jit(ctx);
    },
    0x191 => {
        ::jit_instructions::instr32_91_jit(ctx);
    },
    0x92 => {
        ::jit_instructions::instr16_92_jit(ctx);
    },
    0x192 => {
        ::jit_instructions::instr32_92_jit(ctx);
    },
    0x93 => {
        ::jit_instructions::instr16_93_jit(ctx);
    },
    0x193 => {
        ::jit_instructions::instr32_93_jit(ctx);
    },
    0x94 => {
        ::jit_instructions::instr16_94_jit(ctx);
    },
    0x194 => {
        ::jit_instructions::instr32_94_jit(ctx);
    },
    0x95 => {
        ::jit_instructions::instr16_95_jit(ctx);
    },
    0x195 => {
        ::jit_instructions::instr32_95_jit(ctx);
    },
    0x96 => {
        ::jit_instructions::instr16_96_jit(ctx);
    },
    0x196 => {
        ::jit_instructions::instr32_96_jit(ctx);
    },
    0x97 => {
        ::jit_instructions::instr16_97_jit(ctx);
    },
    0x197 => {
        ::jit_instructions::instr32_97_jit(ctx);
    },
    0x98 => {
        ::jit_instructions::instr16_98_jit(ctx);
    },
    0x198 => {
        ::jit_instructions::instr32_98_jit(ctx);
    },
    0x99 => {
        ::jit_instructions::instr16_99_jit(ctx);
    },
    0x199 => {
        ::jit_instructions::instr32_99_jit(ctx);
    },
    0x9A => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm16() as u32;
        let imm2 = ctx.cpu.read_imm16() as u32;
        ::codegen::gen_fn2_const(ctx.builder, "instr16_9A", imm, imm2);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x19A => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm32() as u32;
        let imm2 = ctx.cpu.read_imm16() as u32;
        ::codegen::gen_fn2_const(ctx.builder, "instr32_9A", imm, imm2);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x9B | 0x19B => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_9B");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x9C => {
        ::jit_instructions::instr16_9C_jit(ctx);
    },
    0x19C => {
        ::jit_instructions::instr32_9C_jit(ctx);
    },
    0x9D => {
        ::jit_instructions::instr16_9D_jit(ctx);
    },
    0x19D => {
        ::jit_instructions::instr32_9D_jit(ctx);
    },
    0x9E | 0x19E => {
        ::jit_instructions::instr_9E_jit(ctx);
    },
    0x9F | 0x19F => {
        ::jit_instructions::instr_9F_jit(ctx);
    },
    0xA0 | 0x1A0 => {
        let imm = ctx.cpu.read_moffs() as u32;
        ::jit_instructions::instr_A0_jit(ctx, imm);
    },
    0xA1 => {
        let imm = ctx.cpu.read_moffs() as u32;
        ::jit_instructions::instr16_A1_jit(ctx, imm);
    },
    0x1A1 => {
        let imm = ctx.cpu.read_moffs() as u32;
        ::jit_instructions::instr32_A1_jit(ctx, imm);
    },
    0xA2 | 0x1A2 => {
        let imm = ctx.cpu.read_moffs() as u32;
        ::jit_instructions::instr_A2_jit(ctx, imm);
    },
    0xA3 => {
        let imm = ctx.cpu.read_moffs() as u32;
        ::jit_instructions::instr16_A3_jit(ctx, imm);
    },
    0x1A3 => {
        let imm = ctx.cpu.read_moffs() as u32;
        ::jit_instructions::instr32_A3_jit(ctx, imm);
    },
    0xA4 | 0x1A4 => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr_F2A4_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr_F3A4_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr_A4_jit(ctx);
        }
    },
    0xA5 => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr16_F2A5_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr16_F3A5_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr16_A5_jit(ctx);
        }
    },
    0x1A5 => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr32_F2A5_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr32_F3A5_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr32_A5_jit(ctx);
        }
    },
    0xA6 | 0x1A6 => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr_F2A6_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr_F3A6_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr_A6_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0xA7 => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr16_F2A7_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr16_F3A7_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr16_A7_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0x1A7 => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr32_F2A7_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr32_F3A7_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr32_A7_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0xA8 | 0x1A8 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_A8_jit(ctx, imm);
    },
    0xA9 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_A9_jit(ctx, imm);
    },
    0x1A9 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_A9_jit(ctx, imm);
    },
    0xAA | 0x1AA => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr_F2AA_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr_F3AA_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr_AA_jit(ctx);
        }
    },
    0xAB => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr16_F2AB_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr16_F3AB_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr16_AB_jit(ctx);
        }
    },
    0x1AB => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr32_F2AB_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr32_F3AB_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr32_AB_jit(ctx);
        }
    },
    0xAC | 0x1AC => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr_F2AC_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr_F3AC_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr_AC_jit(ctx);
        }
    },
    0xAD => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr16_F2AD_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr16_F3AD_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr16_AD_jit(ctx);
        }
    },
    0x1AD => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr32_F2AD_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr32_F3AD_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr32_AD_jit(ctx);
        }
    },
    0xAE | 0x1AE => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr_F2AE_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr_F3AE_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr_AE_jit(ctx);
        }
    },
    0xAF => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr16_F2AF_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr16_F3AF_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr16_AF_jit(ctx);
        }
    },
    0x1AF => {
        if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::jit_instructions::instr32_F2AF_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::jit_instructions::instr32_F3AF_jit(ctx);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
        else {
            ::jit_instructions::instr32_AF_jit(ctx);
        }
    },
    0xB0 | 0x1B0 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_B0_jit(ctx, imm);
    },
    0xB1 | 0x1B1 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_B1_jit(ctx, imm);
    },
    0xB2 | 0x1B2 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_B2_jit(ctx, imm);
    },
    0xB3 | 0x1B3 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_B3_jit(ctx, imm);
    },
    0xB4 | 0x1B4 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_B4_jit(ctx, imm);
    },
    0xB5 | 0x1B5 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_B5_jit(ctx, imm);
    },
    0xB6 | 0x1B6 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_B6_jit(ctx, imm);
    },
    0xB7 | 0x1B7 => {
        let imm = ctx.cpu.read_imm8() as u32;
        ::jit_instructions::instr_B7_jit(ctx, imm);
    },
    0xB8 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_B8_jit(ctx, imm);
    },
    0x1B8 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_B8_jit(ctx, imm);
    },
    0xB9 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_B9_jit(ctx, imm);
    },
    0x1B9 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_B9_jit(ctx, imm);
    },
    0xBA => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_BA_jit(ctx, imm);
    },
    0x1BA => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_BA_jit(ctx, imm);
    },
    0xBB => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_BB_jit(ctx, imm);
    },
    0x1BB => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_BB_jit(ctx, imm);
    },
    0xBC => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_BC_jit(ctx, imm);
    },
    0x1BC => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_BC_jit(ctx, imm);
    },
    0xBD => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_BD_jit(ctx, imm);
    },
    0x1BD => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_BD_jit(ctx, imm);
    },
    0xBE => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_BE_jit(ctx, imm);
    },
    0x1BE => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_BE_jit(ctx, imm);
    },
    0xBF => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_BF_jit(ctx, imm);
    },
    0x1BF => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_BF_jit(ctx, imm);
    },
    0xC0 | 0x1C0 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_2_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_3_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C0_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xC1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_2_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_3_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_C1_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x1C1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_2_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_3_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_C1_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xC2 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_C2_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x1C2 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr32_C2_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0xC3 => {
        ::jit_instructions::instr16_C3_jit(ctx);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x1C3 => {
        ::jit_instructions::instr32_C3_jit(ctx);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0xC4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr16_C4_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr16_C4_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1C4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr32_C4_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr32_C4_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xC5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr16_C5_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr16_C5_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1C5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr32_C5_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr32_C5_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xC6 | 0x1C6 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C6_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_C6_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xC7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_C7_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_C7_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x1C7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_C7_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_C7_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xC8 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm16() as u32;
        let imm2 = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn2_const(ctx.builder, "instr16_C8", imm, imm2);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1C8 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm16() as u32;
        let imm2 = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn2_const(ctx.builder, "instr32_C8", imm, imm2);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xC9 => {
        ::jit_instructions::instr16_C9_jit(ctx);
    },
    0x1C9 => {
        ::jit_instructions::instr32_C9_jit(ctx);
    },
    0xCA => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm16() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr16_CA", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1CA => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm16() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr32_CA", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xCB => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_CB");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1CB => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_CB");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xCC | 0x1CC => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_CC");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xCD | 0x1CD => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr_CD", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xCE | 0x1CE => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_CE");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xCF => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_CF");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1CF => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_CF");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xD0 | 0x1D0 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D0_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D0_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D0_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D0_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D0_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D0_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D0_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D0_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D0_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D0_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D0_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D0_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D0_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D0_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D0_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D0_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xD1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D1_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D1_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D1_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D1_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D1_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D1_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D1_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D1_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D1_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D1_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D1_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D1_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D1_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D1_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D1_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D1_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x1D1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D1_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D1_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D1_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D1_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D1_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D1_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D1_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D1_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D1_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D1_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D1_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D1_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D1_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D1_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D1_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D1_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xD2 | 0x1D2 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D2_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D2_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D2_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D2_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D2_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D2_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D2_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D2_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D2_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D2_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D2_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D2_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D2_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D2_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D2_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D2_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xD3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D3_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D3_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D3_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D3_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D3_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D3_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D3_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D3_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D3_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D3_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D3_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D3_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D3_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D3_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D3_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D3_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x1D3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D3_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D3_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D3_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D3_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D3_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D3_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D3_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D3_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D3_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D3_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D3_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D3_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D3_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D3_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D3_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D3_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xD4 | 0x1D4 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr_D4", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xD5 | 0x1D5 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr_D5", imm);
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xD6 | 0x1D6 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_D6");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xD7 | 0x1D7 => {
        ::jit_instructions::instr_D7_jit(ctx);
    },
    0xD8 | 0x1D8 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D8_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D8_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D8_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D8_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D8_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D8_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D8_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D8_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D8_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D8_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D8_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D8_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D8_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D8_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_D8_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_D8_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xD9 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D9_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D9_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D9_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D9_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D9_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D9_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D9_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D9_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D9_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D9_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D9_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D9_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D9_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D9_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_D9_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_D9_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x1D9 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D9_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D9_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D9_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D9_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D9_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D9_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D9_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D9_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D9_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D9_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D9_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D9_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D9_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D9_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_D9_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_D9_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xDA | 0x1DA => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DA_0_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DA_0_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DA_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DA_1_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DA_2_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DA_2_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DA_3_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DA_3_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DA_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DA_4_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DA_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DA_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DA_6_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DA_6_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DA_7_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DA_7_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xDB | 0x1DB => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DB_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DB_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DB_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DB_1_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DB_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DB_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DB_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DB_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DB_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DB_4_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DB_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DB_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DB_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DB_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DB_7_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DB_7_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xDC | 0x1DC => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DC_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DC_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DC_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DC_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DC_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DC_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DC_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DC_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DC_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DC_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DC_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DC_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DC_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DC_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DC_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DC_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xDD => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_DD_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_DD_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_DD_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_DD_1_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_DD_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_DD_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_DD_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_DD_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_DD_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_DD_4_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_DD_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_DD_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_DD_6_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_DD_6_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_DD_7_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_DD_7_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x1DD => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_DD_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_DD_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_DD_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_DD_1_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_DD_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_DD_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_DD_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_DD_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_DD_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_DD_4_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_DD_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_DD_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_DD_6_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_DD_6_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_DD_7_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_DD_7_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xDE | 0x1DE => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DE_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DE_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DE_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DE_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DE_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DE_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DE_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DE_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DE_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DE_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DE_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DE_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DE_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DE_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DE_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DE_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xDF | 0x1DF => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DF_0_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DF_0_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            1 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_DF_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_DF_1_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DF_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DF_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DF_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DF_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DF_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DF_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DF_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DF_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DF_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DF_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                ::codegen::gen_task_switch_test(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_DF_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_DF_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xE0 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_E0_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x1E0 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_E0_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0xE1 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_E1_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x1E1 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_E1_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0xE2 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_E2_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x1E2 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_E2_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0xE3 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_E3_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x1E3 => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_E3_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0xE4 | 0x1E4 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr_E4", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xE5 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr16_E5", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1E5 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr32_E5", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xE6 | 0x1E6 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr_E6", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xE7 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr16_E7", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1E7 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm8() as u32;
        ::codegen::gen_fn1_const(ctx.builder, "instr32_E7", imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xE8 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_E8_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x1E8 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_E8_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0xE9 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_E9_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x1E9 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_E9_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0xEA => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm16() as u32;
        let imm2 = ctx.cpu.read_imm16() as u32;
        ::codegen::gen_fn2_const(ctx.builder, "instr16_EA", imm, imm2);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1EA => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        let imm = ctx.cpu.read_imm32() as u32;
        let imm2 = ctx.cpu.read_imm16() as u32;
        ::codegen::gen_fn2_const(ctx.builder, "instr32_EA", imm, imm2);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xEB => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr16_EB_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x1EB => {
        let imm = ctx.cpu.read_imm8s() as u32;
        ::jit_instructions::instr32_EB_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0xEC | 0x1EC => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_EC");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xED => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_ED");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1ED => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_ED");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xEE | 0x1EE => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_EE");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xEF => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_EF");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1EF => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_EF");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xF0 | 0x1F0 => {
        ::jit_instructions::instr_F0_jit(ctx, instr_flags);
    },
    0xF1 | 0x1F1 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_F1");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xF2 | 0x1F2 => {
        ::jit_instructions::instr_F2_jit(ctx, instr_flags);
    },
    0xF3 | 0x1F3 => {
        ::jit_instructions::instr_F3_jit(ctx, instr_flags);
    },
    0xF4 | 0x1F4 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_F4");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xF5 | 0x1F5 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_F5");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xF6 | 0x1F6 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_F6_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_F6_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_F6_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr_F6_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_F6_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_F6_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_F6_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_F6_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_F6_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_F6_4_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            5 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_F6_5_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_F6_5_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            6 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_F6_6_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_F6_6_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            7 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_F6_7_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_F6_7_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xF7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_F7_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_F7_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_F7_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm16() as u32;
                    ::jit_instructions::instr16_F7_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_F7_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_F7_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_F7_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_F7_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_F7_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_F7_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_F7_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_F7_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_F7_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_F7_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_F7_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_F7_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x1F7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_F7_0_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_F7_0_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_F7_1_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm32() as u32;
                    ::jit_instructions::instr32_F7_1_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_F7_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_F7_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_F7_3_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_F7_3_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_F7_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_F7_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_F7_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_F7_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_F7_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_F7_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_F7_7_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_F7_7_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xF8 | 0x1F8 => {
        ::jit_instructions::instr_F8_jit(ctx);
    },
    0xF9 | 0x1F9 => {
        ::jit_instructions::instr_F9_jit(ctx);
    },
    0xFA | 0x1FA => {
        ::jit_instructions::instr_FA_jit(ctx);
    },
    0xFB | 0x1FB => {
        ::jit_instructions::instr_FB_jit(ctx);
    },
    0xFC | 0x1FC => {
        ::jit_instructions::instr_FC_jit(ctx);
    },
    0xFD | 0x1FD => {
        ::jit_instructions::instr_FD_jit(ctx);
    },
    0xFE | 0x1FE => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_FE_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_FE_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_FE_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_FE_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xFF => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_FF_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_FF_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_FF_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_FF_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_FF_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_FF_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            },
            3 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_FF_3_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_FF_3_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_FF_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_FF_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            },
            5 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_FF_5_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_FF_5_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_FF_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_FF_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x1FF => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_FF_0_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_FF_0_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_FF_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_FF_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_FF_2_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_FF_2_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            },
            3 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_FF_3_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_FF_3_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_FF_4_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_FF_4_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            },
            5 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_FF_5_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_FF_5_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_FF_6_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_FF_6_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    _ => {
        assert!(false);
    }
}
}
