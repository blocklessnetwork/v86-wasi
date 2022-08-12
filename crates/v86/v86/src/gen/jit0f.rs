#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn jit(opcode: u32, ctx: &mut ::jit::JitContext, instr_flags: &mut u32) {
match opcode {
    0x00 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F00_0_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F00_0_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            1 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F00_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F00_1_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F00_2_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F00_2_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            3 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F00_3_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F00_3_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            4 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F00_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F00_4_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            5 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F00_5_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F00_5_reg", (modrm_byte & 7) as u32);
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
    0x100 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F00_0_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F00_0_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            1 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F00_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F00_1_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F00_2_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F00_2_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            3 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F00_3_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F00_3_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            4 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F00_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F00_4_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            5 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F00_5_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F00_5_reg", (modrm_byte & 7) as u32);
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
    0x01 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F01_0_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F01_0_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            1 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F01_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F01_1_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F01_2_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F01_2_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            3 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F01_3_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F01_3_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            4 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F01_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F01_4_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            6 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F01_6_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F01_6_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            7 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0F01_7_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0F01_7_reg", (modrm_byte & 7) as u32);
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
    0x101 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F01_0_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F01_0_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            1 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F01_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F01_1_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F01_2_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F01_2_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            3 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F01_3_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F01_3_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            4 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F01_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F01_4_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            6 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F01_6_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F01_6_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            7 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0F01_7_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0F01_7_reg", (modrm_byte & 7) as u32);
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
    0x02 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr16_0F02_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr16_0F02_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x102 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr32_0F02_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr32_0F02_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x03 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr16_0F03_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr16_0F03_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x103 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr32_0F03_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr32_0F03_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x04 | 0x104 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F04");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x05 | 0x105 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F05");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x06 | 0x106 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F06");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x07 | 0x107 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F07");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x08 | 0x108 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F08");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x09 | 0x109 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F09");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x0A | 0x10A => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F0A");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x0B | 0x10B => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F0B");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x0C | 0x10C => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F0C");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x0D | 0x10D => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F0D");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x0E | 0x10E => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F0E");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x0F | 0x10F => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F0F");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x10 | 0x110 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F10_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F10_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F10_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F10_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F10_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F10_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F10_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F10_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x11 | 0x111 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F11_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F11_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F11_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F11_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F11_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F11_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F11_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F11_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x12 | 0x112 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F12_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F12_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_F20F12_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_F20F12_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_F30F12_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_F30F12_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F12_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F12_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x13 | 0x113 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F13_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F13_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F13_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F13_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x14 | 0x114 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F14_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F14_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F14_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F14_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x15 | 0x115 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F15_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F15_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F15_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F15_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x16 | 0x116 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_660F16_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_660F16_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_F30F16_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_F30F16_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0F16_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0F16_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0x17 | 0x117 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_660F17_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_660F17_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0F17_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0F17_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0x18 | 0x118 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F18_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F18_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x19 | 0x119 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F19_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F19_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1A | 0x11A => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F1A");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1B | 0x11B => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F1B");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1C | 0x11C => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F1C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F1C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1D | 0x11D => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F1D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F1D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1E | 0x11E => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F1E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F1E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1F | 0x11F => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F1F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F1F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x20 | 0x120 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn2_const(ctx.builder, "instr_0F20", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x21 | 0x121 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn2_const(ctx.builder, "instr_0F21", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x22 | 0x122 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn2_const(ctx.builder, "instr_0F22", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x23 | 0x123 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn2_const(ctx.builder, "instr_0F23", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x24 | 0x124 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F24");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x25 | 0x125 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F25");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x26 | 0x126 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F26");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x27 | 0x127 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F27");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x28 | 0x128 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F28_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F28_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F28_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F28_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x29 | 0x129 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F29_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F29_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F29_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F29_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x2A | 0x12A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F2A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F2A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F2A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F2A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F2A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F2A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F2A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F2A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x2B | 0x12B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F2B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F2B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F2B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F2B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x2C | 0x12C => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_660F2C_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_660F2C_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F2C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F2C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F2C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F2C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0F2C_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0F2C_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0x2D | 0x12D => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_660F2D_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_660F2D_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F2D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F2D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F2D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F2D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0F2D_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0F2D_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0x2E | 0x12E => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F2E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F2E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F2E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F2E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x2F | 0x12F => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F2F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F2F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F2F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F2F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x30 | 0x130 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F30");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x31 | 0x131 => {
        ::jit_instructions::instr_0F31_jit(ctx);
    },
    0x32 | 0x132 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F32");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x33 | 0x133 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F33");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x34 | 0x134 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F34");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x35 | 0x135 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F35");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x36 | 0x136 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F36");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x37 | 0x137 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F37");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x38 | 0x138 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F38");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x39 | 0x139 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F39");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x3A | 0x13A => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F3A");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x3B | 0x13B => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F3B");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x3C | 0x13C => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F3C");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x3D | 0x13D => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F3D");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x3E | 0x13E => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F3E");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x3F | 0x13F => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F3F");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x40 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F40_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F40_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x140 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F40_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F40_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x41 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F41_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F41_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x141 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F41_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F41_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x42 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F42_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F42_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x142 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F42_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F42_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x43 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F43_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F43_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x143 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F43_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F43_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x44 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F44_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F44_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x144 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F44_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F44_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x45 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F45_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F45_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x145 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F45_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F45_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x46 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F46_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F46_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x146 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F46_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F46_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x47 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F47_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F47_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x147 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F47_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F47_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x48 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F48_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F48_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x148 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F48_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F48_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x49 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F49_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F49_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x149 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F49_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F49_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x4A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F4A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F4A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x14A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F4A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F4A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x4B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F4B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F4B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x14B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F4B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F4B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x4C => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F4C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F4C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x14C => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F4C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F4C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x4D => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F4D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F4D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x14D => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F4D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F4D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x4E => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F4E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F4E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x14E => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F4E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F4E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x4F => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0F4F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0F4F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x14F => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0F4F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0F4F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x50 | 0x150 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_660F50_mem", (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_660F50_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0F50_mem", (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0F50_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0x51 | 0x151 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F51_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F51_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F51_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F51_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F51_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F51_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F51_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F51_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x52 | 0x152 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F52_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F52_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F52_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F52_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x53 | 0x153 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F53_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F53_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F53_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F53_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x54 | 0x154 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F54_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F54_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F54_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F54_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x55 | 0x155 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F55_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F55_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F55_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F55_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x56 | 0x156 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F56_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F56_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F56_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F56_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x57 | 0x157 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F57_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F57_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F57_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F57_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x58 | 0x158 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F58_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F58_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F58_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F58_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F58_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F58_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F58_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F58_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x59 | 0x159 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F59_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F59_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F59_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F59_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F59_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F59_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F59_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F59_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x5A | 0x15A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F5A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F5A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F5A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F5A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F5A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F5A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F5A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F5A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x5B | 0x15B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F5B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F5B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F5B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F5B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F5B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F5B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x5C | 0x15C => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F5C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F5C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F5C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F5C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F5C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F5C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F5C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F5C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x5D | 0x15D => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F5D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F5D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F5D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F5D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F5D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F5D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F5D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F5D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x5E | 0x15E => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F5E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F5E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F5E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F5E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F5E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F5E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F5E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F5E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x5F | 0x15F => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F5F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F5F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20F5F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20F5F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F5F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F5F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F5F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F5F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x60 | 0x160 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F60_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F60_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F60_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F60_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x61 | 0x161 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F61_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F61_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F61_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F61_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x62 | 0x162 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F62_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F62_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F62_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F62_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x63 | 0x163 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F63_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F63_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F63_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F63_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x64 | 0x164 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F64_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F64_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F64_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F64_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x65 | 0x165 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F65_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F65_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F65_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F65_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x66 | 0x166 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F66_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F66_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F66_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F66_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x67 | 0x167 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F67_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F67_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F67_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F67_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x68 | 0x168 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F68_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F68_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F68_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F68_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x69 | 0x169 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F69_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F69_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F69_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F69_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x6A | 0x16A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F6A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F6A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F6A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F6A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x6B | 0x16B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F6B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F6B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F6B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F6B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x6C | 0x16C => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F6C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F6C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0F6C_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0F6C_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0x6D | 0x16D => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F6D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F6D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0F6D_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0F6D_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0x6E | 0x16E => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F6E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F6E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F6E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F6E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x6F | 0x16F => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F6F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F6F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F6F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F6F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F6F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F6F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x70 | 0x170 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_660F70_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_660F70_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_F20F70_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_F20F70_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_F30F70_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_F30F70_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_0F70_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_0F70_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
    },
    0x71 | 0x171 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            match modrm_byte >> 3 & 7 {
                2 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F71_2_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F71_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                4 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F71_4_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F71_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                6 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F71_6_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F71_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                _ => {
                    ::codegen::gen_trigger_ud(ctx);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
            }
        }
        else {
            match modrm_byte >> 3 & 7 {
                2 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F71_2_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F71_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                4 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F71_4_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F71_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                6 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F71_6_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F71_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                _ => {
                    ::codegen::gen_trigger_ud(ctx);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
            }
        }
    },
    0x72 | 0x172 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            match modrm_byte >> 3 & 7 {
                2 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F72_2_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F72_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                4 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F72_4_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F72_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                6 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F72_6_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F72_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                _ => {
                    ::codegen::gen_trigger_ud(ctx);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
            }
        }
        else {
            match modrm_byte >> 3 & 7 {
                2 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F72_2_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F72_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                4 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F72_4_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F72_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                6 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F72_6_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F72_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                _ => {
                    ::codegen::gen_trigger_ud(ctx);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
            }
        }
    },
    0x73 | 0x173 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            match modrm_byte >> 3 & 7 {
                2 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F73_2_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F73_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                3 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F73_3_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F73_3_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                6 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F73_6_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F73_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                7 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F73_7_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_660F73_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                _ => {
                    ::codegen::gen_trigger_ud(ctx);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
            }
        }
        else {
            match modrm_byte >> 3 & 7 {
                2 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F73_2_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F73_2_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                6 => {
                    ::codegen::gen_task_switch_test_mmx(ctx);
                    if modrm_byte < 0xC0 {
                        let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F73_6_mem_jit(ctx, addr, imm);
                        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                    }
                    else {
                        let imm = ctx.cpu.read_imm8() as u32;
                        ::jit_instructions::instr_0F73_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                    }
                },
                _ => {
                    ::codegen::gen_trigger_ud(ctx);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
            }
        }
    },
    0x74 | 0x174 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F74_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F74_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F74_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F74_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x75 | 0x175 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F75_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F75_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F75_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F75_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x76 | 0x176 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F76_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F76_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F76_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F76_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x77 | 0x177 => {
        ::codegen::gen_task_switch_test_mmx(ctx);
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F77");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x78 | 0x178 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F78");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x79 | 0x179 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F79");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x7A | 0x17A => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F7A");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x7B | 0x17B => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F7B");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x7C | 0x17C => {
        ::codegen::gen_task_switch_test_mmx(ctx);
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F7C");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x7D | 0x17D => {
        ::codegen::gen_task_switch_test_mmx(ctx);
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0F7D");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x7E | 0x17E => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F7E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F7E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F7E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F7E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F7E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F7E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x7F | 0x17F => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660F7F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660F7F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30F7F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30F7F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0F7F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0F7F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0x80 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F80_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x180 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F80_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x81 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F81_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x181 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F81_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x82 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F82_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x182 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F82_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x83 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F83_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x183 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F83_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x84 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F84_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x184 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F84_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x85 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F85_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x185 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F85_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x86 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F86_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x186 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F86_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x87 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F87_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x187 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F87_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x88 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F88_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x188 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F88_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x89 => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F89_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x189 => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F89_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x8A => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F8A_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x18A => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F8A_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x8B => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F8B_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x18B => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F8B_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x8C => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F8C_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x18C => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F8C_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x8D => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F8D_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x18D => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F8D_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x8E => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F8E_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x18E => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F8E_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x8F => {
        let imm = ctx.cpu.read_imm16() as u32;
        ::jit_instructions::instr16_0F8F_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x18F => {
        let imm = ctx.cpu.read_imm32() as u32;
        ::jit_instructions::instr32_0F8F_jit(ctx, imm);
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
    },
    0x90 | 0x190 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F90_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F90_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x91 | 0x191 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F91_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F91_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x92 | 0x192 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F92_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F92_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x93 | 0x193 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F93_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F93_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x94 | 0x194 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F94_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F94_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x95 | 0x195 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F95_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F95_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x96 | 0x196 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F96_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F96_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x97 | 0x197 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F97_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F97_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x98 | 0x198 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F98_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F98_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x99 | 0x199 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F99_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F99_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x9A | 0x19A => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F9A_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F9A_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x9B | 0x19B => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F9B_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F9B_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x9C | 0x19C => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F9C_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F9C_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x9D | 0x19D => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F9D_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F9D_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x9E | 0x19E => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F9E_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F9E_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x9F | 0x19F => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0F9F_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0F9F_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xA0 => {
        ::jit_instructions::instr16_0FA0_jit(ctx);
    },
    0x1A0 => {
        ::jit_instructions::instr32_0FA0_jit(ctx);
    },
    0xA1 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_0FA1");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1A1 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_0FA1");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xA2 | 0x1A2 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0FA2");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xA3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FA3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FA3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1A3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FA3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FA3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xA4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            let imm = ctx.cpu.read_imm8() as u32;
            ::jit_instructions::instr16_0FA4_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
        }
        else {
            let imm = ctx.cpu.read_imm8() as u32;
            ::jit_instructions::instr16_0FA4_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
        }
    },
    0x1A4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            let imm = ctx.cpu.read_imm8() as u32;
            ::jit_instructions::instr32_0FA4_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
        }
        else {
            let imm = ctx.cpu.read_imm8() as u32;
            ::jit_instructions::instr32_0FA4_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
        }
    },
    0xA5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FA5_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FA5_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1A5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FA5_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FA5_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xA6 | 0x1A6 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0FA6");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xA7 | 0x1A7 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0FA7");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xA8 => {
        ::jit_instructions::instr16_0FA8_jit(ctx);
    },
    0x1A8 => {
        ::jit_instructions::instr32_0FA8_jit(ctx);
    },
    0xA9 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr16_0FA9");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1A9 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr32_0FA9");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xAA | 0x1AA => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0FAA");
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xAB => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FAB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FAB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1AB => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FAB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FAB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xAC => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            let imm = ctx.cpu.read_imm8() as u32;
            ::jit_instructions::instr16_0FAC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
        }
        else {
            let imm = ctx.cpu.read_imm8() as u32;
            ::jit_instructions::instr16_0FAC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
        }
    },
    0x1AC => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            let imm = ctx.cpu.read_imm8() as u32;
            ::jit_instructions::instr32_0FAC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
        }
        else {
            let imm = ctx.cpu.read_imm8() as u32;
            ::jit_instructions::instr32_0FAC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
        }
    },
    0xAD => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FAD_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FAD_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1AD => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FAD_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FAD_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xAE | 0x1AE => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                ::codegen::gen_task_switch_test(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_0FAE_0_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_0FAE_0_reg", (modrm_byte & 7) as u32);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
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
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_0FAE_1_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_0FAE_1_reg", (modrm_byte & 7) as u32);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            2 => {
                ::codegen::gen_task_switch_test_mmx(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_0FAE_2_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_0FAE_2_reg", (modrm_byte & 7) as u32);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            3 => {
                ::codegen::gen_task_switch_test_mmx(ctx);
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_0FAE_3_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_0FAE_3_reg", (modrm_byte & 7) as u32);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            4 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_0FAE_4_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_0FAE_4_reg", (modrm_byte & 7) as u32);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr_0FAE_5_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr_0FAE_5_reg_jit(ctx, (modrm_byte & 7) as u32);
                }
            },
            6 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_0FAE_6_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_0FAE_6_reg", (modrm_byte & 7) as u32);
                }
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                ::codegen::gen_move_registers_from_memory_to_locals(ctx);
            },
            7 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr_0FAE_7_mem");
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr_0FAE_7_reg", (modrm_byte & 7) as u32);
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
    0xAF => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FAF_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FAF_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1AF => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FAF_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FAF_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xB0 | 0x1B0 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr_0FB0_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr_0FB0_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xB1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FB1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FB1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1B1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FB1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FB1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xB2 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr16_0FB2_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr16_0FB2_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1B2 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr32_0FB2_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr32_0FB2_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xB3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FB3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FB3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1B3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FB3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FB3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xB4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr16_0FB4_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr16_0FB4_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1B4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr32_0FB4_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr32_0FB4_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xB5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr16_0FB5_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr16_0FB5_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0x1B5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr32_0FB5_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr32_0FB5_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xB6 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FB6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FB6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1B6 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FB6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FB6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xB7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FB7_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FB7_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1B7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FB7_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FB7_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xB8 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr16_F30FB8_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr16_F30FB8_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr16_0FB8_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr16_0FB8_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0x1B8 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr32_F30FB8_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr32_F30FB8_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr32_0FB8_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr32_0FB8_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0xB9 | 0x1B9 => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0FB9");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xBA => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_0FBA_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_0FBA_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_0FBA_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_0FBA_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_0FBA_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_0FBA_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_0FBA_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr16_0FBA_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0x1BA => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            4 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_0FBA_4_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_0FBA_4_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            5 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_0FBA_5_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_0FBA_5_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_0FBA_6_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_0FBA_6_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            7 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_0FBA_7_mem_jit(ctx, addr, imm);
                }
                else {
                    let imm = ctx.cpu.read_imm8() as u32;
                    ::jit_instructions::instr32_0FBA_7_reg_jit(ctx, (modrm_byte & 7) as u32, imm);
                }
            },
            _ => {
                ::codegen::gen_trigger_ud(ctx);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
    },
    0xBB => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FBB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FBB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1BB => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FBB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FBB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xBC => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FBC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FBC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1BC => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FBC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FBC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xBD => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FBD_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FBD_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1BD => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FBD_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FBD_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xBE => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FBE_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FBE_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1BE => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FBE_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FBE_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xBF => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FBF_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FBF_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1BF => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FBF_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FBF_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xC0 | 0x1C0 => {
        let modrm_byte = ctx.cpu.read_imm8();
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::codegen::gen_modrm_resolve(ctx, addr);
            ::codegen::gen_modrm_fn1(ctx.builder, "instr_0FC0_mem", (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::codegen::gen_fn2_const(ctx.builder, "instr_0FC0_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xC1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr16_0FC1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr16_0FC1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0x1C1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr32_0FC1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr32_0FC1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
        }
    },
    0xC2 | 0x1C2 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_660FC2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_660FC2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_F20FC2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_F20FC2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_F30FC2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_F30FC2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_0FC2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_0FC2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
    },
    0xC3 | 0x1C3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if modrm_byte < 0xC0 {
            let addr = ::modrm::decode(ctx.cpu, modrm_byte);
            ::jit_instructions::instr_0FC3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
        }
        else {
            ::jit_instructions::instr_0FC3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        }
    },
    0xC4 | 0x1C4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                let imm = ctx.cpu.read_imm8() as u32;
                ::codegen::gen_modrm_fn2(ctx.builder, "instr_660FC4_mem", (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::codegen::gen_fn3_const(ctx.builder, "instr_660FC4_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                let imm = ctx.cpu.read_imm8() as u32;
                ::codegen::gen_modrm_fn2(ctx.builder, "instr_0FC4_mem", (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::codegen::gen_fn3_const(ctx.builder, "instr_0FC4_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0xC5 | 0x1C5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                let imm = ctx.cpu.read_imm8() as u32;
                ::codegen::gen_modrm_fn2(ctx.builder, "instr_660FC5_mem", (modrm_byte >> 3 & 7) as u32, imm);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::codegen::gen_fn3_const(ctx.builder, "instr_660FC5_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                let imm = ctx.cpu.read_imm8() as u32;
                ::codegen::gen_modrm_fn2(ctx.builder, "instr_0FC5_mem", (modrm_byte >> 3 & 7) as u32, imm);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::codegen::gen_fn3_const(ctx.builder, "instr_0FC5_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0xC6 | 0x1C6 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_660FC6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_660FC6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_0FC6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32, imm);
            }
            else {
                let imm = ctx.cpu.read_imm8() as u32;
                ::jit_instructions::instr_0FC6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32, imm);
            }
        }
    },
    0xC7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr16_0FC7_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr16_0FC7_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
            },
            6 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr16_0FC7_6_mem");
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr16_0FC7_6_reg", (modrm_byte & 7) as u32);
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
    0x1C7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            1 => {
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::jit_instructions::instr32_0FC7_1_mem_jit(ctx, addr);
                }
                else {
                    ::jit_instructions::instr32_0FC7_1_reg_jit(ctx, (modrm_byte & 7) as u32);
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
            },
            6 => {
                ::codegen::gen_move_registers_from_locals_to_memory(ctx);
                if modrm_byte < 0xC0 {
                    let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                    ::codegen::gen_modrm_resolve(ctx, addr);
                    ::codegen::gen_modrm_fn0(ctx.builder, "instr32_0FC7_6_mem");
                    *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
                }
                else {
                    ::codegen::gen_fn1_const(ctx.builder, "instr32_0FC7_6_reg", (modrm_byte & 7) as u32);
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
    0xC8 | 0x1C8 => {
        ::jit_instructions::instr_0FC8_jit(ctx);
    },
    0xC9 | 0x1C9 => {
        ::jit_instructions::instr_0FC9_jit(ctx);
    },
    0xCA | 0x1CA => {
        ::jit_instructions::instr_0FCA_jit(ctx);
    },
    0xCB | 0x1CB => {
        ::jit_instructions::instr_0FCB_jit(ctx);
    },
    0xCC | 0x1CC => {
        ::jit_instructions::instr_0FCC_jit(ctx);
    },
    0xCD | 0x1CD => {
        ::jit_instructions::instr_0FCD_jit(ctx);
    },
    0xCE | 0x1CE => {
        ::jit_instructions::instr_0FCE_jit(ctx);
    },
    0xCF | 0x1CF => {
        ::jit_instructions::instr_0FCF_jit(ctx);
    },
    0xD0 | 0x1D0 => {
        ::codegen::gen_task_switch_test_mmx(ctx);
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0FD0");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xD1 | 0x1D1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FD1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FD1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FD1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FD1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xD2 | 0x1D2 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FD2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FD2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FD2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FD2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xD3 | 0x1D3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FD3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FD3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FD3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FD3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xD4 | 0x1D4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FD4_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FD4_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FD4_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FD4_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xD5 | 0x1D5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FD5_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FD5_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FD5_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FD5_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xD6 | 0x1D6 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FD6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FD6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_F20FD6_mem", (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_F20FD6_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_F30FD6_mem", (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_F30FD6_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0FD6_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0FD6_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0xD7 | 0x1D7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FD7_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                ::jit_instructions::instr_660FD7_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FD7_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                ::jit_instructions::instr_0FD7_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xD8 | 0x1D8 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FD8_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FD8_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FD8_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FD8_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xD9 | 0x1D9 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FD9_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FD9_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FD9_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FD9_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xDA | 0x1DA => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FDA_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FDA_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FDA_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FDA_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xDB | 0x1DB => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FDB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FDB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FDB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FDB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xDC | 0x1DC => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FDC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FDC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FDC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FDC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xDD | 0x1DD => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FDD_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FDD_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FDD_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FDD_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xDE | 0x1DE => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FDE_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FDE_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FDE_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FDE_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xDF | 0x1DF => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FDF_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FDF_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FDF_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FDF_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xE0 | 0x1E0 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE0_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE0_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FE0_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FE0_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xE1 | 0x1E1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FE1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FE1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xE2 | 0x1E2 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FE2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FE2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xE3 | 0x1E3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FE3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FE3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xE4 | 0x1E4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE4_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE4_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FE4_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FE4_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xE5 | 0x1E5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE5_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE5_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FE5_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FE5_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xE6 | 0x1E6 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F20FE6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F20FE6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else if ctx.cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_F30FE6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_F30FE6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0FE6_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0FE6_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0xE7 | 0x1E7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE7_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE7_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            ::codegen::gen_move_registers_from_locals_to_memory(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::codegen::gen_modrm_resolve(ctx, addr);
                ::codegen::gen_modrm_fn1(ctx.builder, "instr_0FE7_mem", (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::codegen::gen_fn2_const(ctx.builder, "instr_0FE7_reg", (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            ::codegen::gen_move_registers_from_memory_to_locals(ctx);
        }
    },
    0xE8 | 0x1E8 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE8_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE8_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FE8_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FE8_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xE9 | 0x1E9 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FE9_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FE9_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FE9_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FE9_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xEA | 0x1EA => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FEA_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FEA_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FEA_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FEA_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xEB | 0x1EB => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FEB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FEB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FEB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FEB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xEC | 0x1EC => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FEC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FEC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FEC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FEC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xED | 0x1ED => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FED_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FED_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FED_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FED_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xEE | 0x1EE => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FEE_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FEE_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FEE_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FEE_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xEF | 0x1EF => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FEF_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FEF_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FEF_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FEF_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xF0 | 0x1F0 => {
        ::codegen::gen_task_switch_test_mmx(ctx);
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0FF0");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    0xF1 | 0x1F1 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FF1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FF1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FF1_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FF1_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xF2 | 0x1F2 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FF2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FF2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FF2_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FF2_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xF3 | 0x1F3 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FF3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FF3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FF3_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FF3_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xF4 | 0x1F4 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FF4_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FF4_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FF4_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FF4_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xF5 | 0x1F5 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FF5_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FF5_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FF5_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FF5_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xF6 | 0x1F6 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FF6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FF6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FF6_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FF6_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xF7 | 0x1F7 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FF7_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                ::jit_instructions::instr_660FF7_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FF7_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
                *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
            }
            else {
                ::jit_instructions::instr_0FF7_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xF8 | 0x1F8 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FF8_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FF8_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FF8_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FF8_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xF9 | 0x1F9 => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FF9_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FF9_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FF9_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FF9_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xFA | 0x1FA => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FFA_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FFA_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FFA_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FFA_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xFB | 0x1FB => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FFB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FFB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FFB_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FFB_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xFC | 0x1FC => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FFC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FFC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FFC_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FFC_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xFD | 0x1FD => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FFD_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FFD_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FFD_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FFD_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xFE | 0x1FE => {
        let modrm_byte = ctx.cpu.read_imm8();
        if ctx.cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_660FFE_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_660FFE_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
        else {
            ::codegen::gen_task_switch_test_mmx(ctx);
            if modrm_byte < 0xC0 {
                let addr = ::modrm::decode(ctx.cpu, modrm_byte);
                ::jit_instructions::instr_0FFE_mem_jit(ctx, addr, (modrm_byte >> 3 & 7) as u32);
            }
            else {
                ::jit_instructions::instr_0FFE_reg_jit(ctx, (modrm_byte & 7) as u32, (modrm_byte >> 3 & 7) as u32);
            }
        }
    },
    0xFF | 0x1FF => {
        ::codegen::gen_move_registers_from_locals_to_memory(ctx);
        ::codegen::gen_fn0_const(ctx.builder, "instr_0FFF");
        *instr_flags |= ::jit::JIT_INSTR_BLOCK_BOUNDARY_FLAG;
        ::codegen::gen_move_registers_from_memory_to_locals(ctx);
    },
    _ => {
        assert!(false);
    }
}
}
