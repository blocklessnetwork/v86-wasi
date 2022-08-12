#![allow(unused)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn analyzer(opcode: u32, cpu: &mut ::cpu_context::CpuContext, analysis: &mut ::analysis::Analysis) {
match opcode {
    0x00 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x100 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x01 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x101 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x02 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x102 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x03 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x103 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x04 | 0x104 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x05 | 0x105 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x06 | 0x106 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x07 | 0x107 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x08 | 0x108 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x09 | 0x109 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x0A | 0x10A => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x0B | 0x10B => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0x0C | 0x10C => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x0D | 0x10D => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x0E | 0x10E => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x0F | 0x10F => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x10 | 0x110 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x11 | 0x111 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x12 | 0x112 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x13 | 0x113 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
        }
    },
    0x14 | 0x114 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x15 | 0x115 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x16 | 0x116 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x17 | 0x117 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x18 | 0x118 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x19 | 0x119 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1A | 0x11A => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1B | 0x11B => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1C | 0x11C => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1D | 0x11D => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1E | 0x11E => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1F | 0x11F => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x20 | 0x120 => {
        let modrm_byte = cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x21 | 0x121 => {
        let modrm_byte = cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x22 | 0x122 => {
        let modrm_byte = cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x23 | 0x123 => {
        let modrm_byte = cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x24 | 0x124 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x25 | 0x125 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x26 | 0x126 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x27 | 0x127 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x28 | 0x128 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x29 | 0x129 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x2A | 0x12A => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x2B | 0x12B => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
        }
    },
    0x2C | 0x12C => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x2D | 0x12D => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x2E | 0x12E => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x2F | 0x12F => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x30 | 0x130 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x31 | 0x131 => {
    },
    0x32 | 0x132 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x33 | 0x133 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x34 | 0x134 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0x35 | 0x135 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0x36 | 0x136 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x37 | 0x137 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x38 | 0x138 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x39 | 0x139 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x3A | 0x13A => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x3B | 0x13B => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x3C | 0x13C => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x3D | 0x13D => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x3E | 0x13E => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x3F | 0x13F => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x40 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x140 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x41 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x141 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x42 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x142 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x43 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x143 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x44 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x144 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x45 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x145 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x46 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x146 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x47 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x147 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x48 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x148 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x49 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x149 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x4A => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x14A => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x4B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x14B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x4C => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x14C => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x4D => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x14D => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x4E => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x14E => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x4F => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x14F => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x50 | 0x150 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x51 | 0x151 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x52 | 0x152 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x53 | 0x153 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x54 | 0x154 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x55 | 0x155 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x56 | 0x156 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x57 | 0x157 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x58 | 0x158 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x59 | 0x159 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x5A | 0x15A => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x5B | 0x15B => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x5C | 0x15C => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x5D | 0x15D => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x5E | 0x15E => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x5F | 0x15F => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x60 | 0x160 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x61 | 0x161 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x62 | 0x162 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x63 | 0x163 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x64 | 0x164 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x65 | 0x165 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x66 | 0x166 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x67 | 0x167 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x68 | 0x168 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x69 | 0x169 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x6A | 0x16A => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x6B | 0x16B => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x6C | 0x16C => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x6D | 0x16D => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x6E | 0x16E => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x6F | 0x16F => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x70 | 0x170 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
    },
    0x71 | 0x171 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            match modrm_byte >> 3 & 7 {
                2 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                4 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                6 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                _ => {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    analysis.no_next_instruction = true;
                }
            }
        }
        else {
            match modrm_byte >> 3 & 7 {
                2 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                4 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                6 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                _ => {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    analysis.no_next_instruction = true;
                }
            }
        }
    },
    0x72 | 0x172 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            match modrm_byte >> 3 & 7 {
                2 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                4 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                6 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                _ => {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    analysis.no_next_instruction = true;
                }
            }
        }
        else {
            match modrm_byte >> 3 & 7 {
                2 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                4 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                6 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                _ => {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    analysis.no_next_instruction = true;
                }
            }
        }
    },
    0x73 | 0x173 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            match modrm_byte >> 3 & 7 {
                2 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                3 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                6 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                7 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                _ => {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    analysis.no_next_instruction = true;
                }
            }
        }
        else {
            match modrm_byte >> 3 & 7 {
                2 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                6 => {
                    if modrm_byte < 0xC0 {
                        ::analysis::modrm_analyze(cpu, modrm_byte);
                        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    }
                    else {
                    }
                    cpu.read_imm8();
                },
                _ => {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                    analysis.no_next_instruction = true;
                }
            }
        }
    },
    0x74 | 0x174 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x75 | 0x175 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x76 | 0x176 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x77 | 0x177 => {
    },
    0x78 | 0x178 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x79 | 0x179 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x7A | 0x17A => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x7B | 0x17B => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x7C | 0x17C => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x7D | 0x17D => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x7E | 0x17E => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x7F | 0x17F => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0x80 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x80), is_32: cpu.osize_32() };
    },
    0x180 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x80), is_32: cpu.osize_32() };
    },
    0x81 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x81), is_32: cpu.osize_32() };
    },
    0x181 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x81), is_32: cpu.osize_32() };
    },
    0x82 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x82), is_32: cpu.osize_32() };
    },
    0x182 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x82), is_32: cpu.osize_32() };
    },
    0x83 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x83), is_32: cpu.osize_32() };
    },
    0x183 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x83), is_32: cpu.osize_32() };
    },
    0x84 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x84), is_32: cpu.osize_32() };
    },
    0x184 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x84), is_32: cpu.osize_32() };
    },
    0x85 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x85), is_32: cpu.osize_32() };
    },
    0x185 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x85), is_32: cpu.osize_32() };
    },
    0x86 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x86), is_32: cpu.osize_32() };
    },
    0x186 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x86), is_32: cpu.osize_32() };
    },
    0x87 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x87), is_32: cpu.osize_32() };
    },
    0x187 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x87), is_32: cpu.osize_32() };
    },
    0x88 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x88), is_32: cpu.osize_32() };
    },
    0x188 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x88), is_32: cpu.osize_32() };
    },
    0x89 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x89), is_32: cpu.osize_32() };
    },
    0x189 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x89), is_32: cpu.osize_32() };
    },
    0x8A => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8A), is_32: cpu.osize_32() };
    },
    0x18A => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8A), is_32: cpu.osize_32() };
    },
    0x8B => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8B), is_32: cpu.osize_32() };
    },
    0x18B => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8B), is_32: cpu.osize_32() };
    },
    0x8C => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8C), is_32: cpu.osize_32() };
    },
    0x18C => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8C), is_32: cpu.osize_32() };
    },
    0x8D => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8D), is_32: cpu.osize_32() };
    },
    0x18D => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8D), is_32: cpu.osize_32() };
    },
    0x8E => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8E), is_32: cpu.osize_32() };
    },
    0x18E => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8E), is_32: cpu.osize_32() };
    },
    0x8F => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8F), is_32: cpu.osize_32() };
    },
    0x18F => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x8F), is_32: cpu.osize_32() };
    },
    0x90 | 0x190 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x91 | 0x191 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x92 | 0x192 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x93 | 0x193 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x94 | 0x194 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x95 | 0x195 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x96 | 0x196 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x97 | 0x197 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x98 | 0x198 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x99 | 0x199 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x9A | 0x19A => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x9B | 0x19B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x9C | 0x19C => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x9D | 0x19D => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x9E | 0x19E => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x9F | 0x19F => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xA0 => {
    },
    0x1A0 => {
    },
    0xA1 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1A1 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xA2 | 0x1A2 => {
    },
    0xA3 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1A3 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xA4 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        cpu.read_imm8();
    },
    0x1A4 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        cpu.read_imm8();
    },
    0xA5 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1A5 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xA6 | 0x1A6 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xA7 | 0x1A7 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xA8 => {
    },
    0x1A8 => {
    },
    0xA9 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1A9 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xAA | 0x1AA => {
    },
    0xAB => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1AB => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xAC => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        cpu.read_imm8();
    },
    0x1AC => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        cpu.read_imm8();
    },
    0xAD => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1AD => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xAE | 0x1AE => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xAF => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1AF => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xB0 | 0x1B0 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xB1 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1B1 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xB2 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1B2 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xB3 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1B3 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xB4 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1B4 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xB5 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1B5 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xB6 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1B6 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xB7 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1B7 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xB8 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x1B8 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0xB9 | 0x1B9 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xBA => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x1BA => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xBB => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1BB => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xBC => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1BC => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xBD => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1BD => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xBE => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1BE => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xBF => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1BF => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xC0 | 0x1C0 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xC1 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1C1 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0xC2 | 0x1C2 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
    },
    0xC3 | 0x1C3 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0xC4 | 0x1C4 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0xC5 | 0x1C5 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
            cpu.read_imm8();
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
            cpu.read_imm8();
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0xC6 | 0x1C6 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            cpu.read_imm8();
        }
    },
    0xC7 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x1C7 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                }
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                    analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xC8 | 0x1C8 => {
    },
    0xC9 | 0x1C9 => {
    },
    0xCA | 0x1CA => {
    },
    0xCB | 0x1CB => {
    },
    0xCC | 0x1CC => {
    },
    0xCD | 0x1CD => {
    },
    0xCE | 0x1CE => {
    },
    0xCF | 0x1CF => {
    },
    0xD0 | 0x1D0 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xD1 | 0x1D1 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xD2 | 0x1D2 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xD3 | 0x1D3 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xD4 | 0x1D4 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xD5 | 0x1D5 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xD6 | 0x1D6 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0xD7 | 0x1D7 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
        }
    },
    0xD8 | 0x1D8 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xD9 | 0x1D9 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xDA | 0x1DA => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xDB | 0x1DB => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xDC | 0x1DC => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xDD | 0x1DD => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xDE | 0x1DE => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xDF | 0x1DF => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xE0 | 0x1E0 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xE1 | 0x1E1 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xE2 | 0x1E2 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xE3 | 0x1E3 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xE4 | 0x1E4 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xE5 | 0x1E5 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xE6 | 0x1E6 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0xE7 | 0x1E7 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0xE8 | 0x1E8 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xE9 | 0x1E9 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xEA | 0x1EA => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xEB | 0x1EB => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xEC | 0x1EC => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xED | 0x1ED => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xEE | 0x1EE => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xEF | 0x1EF => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xF0 | 0x1F0 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xF1 | 0x1F1 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xF2 | 0x1F2 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xF3 | 0x1F3 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xF4 | 0x1F4 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xF5 | 0x1F5 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xF6 | 0x1F6 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xF7 | 0x1F7 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
            }
            else {
            }
        }
    },
    0xF8 | 0x1F8 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xF9 | 0x1F9 => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xFA | 0x1FA => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xFB | 0x1FB => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xFC | 0x1FC => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xFD | 0x1FD => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xFE | 0x1FE => {
        let modrm_byte = cpu.read_imm8();
        if cpu.prefixes & ::prefix::PREFIX_66 != 0 {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
        else {
            if modrm_byte < 0xC0 {
                ::analysis::modrm_analyze(cpu, modrm_byte);
            }
            else {
            }
        }
    },
    0xFF | 0x1FF => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    _ => {
        dbg_assert!(false);
    }
}
}
