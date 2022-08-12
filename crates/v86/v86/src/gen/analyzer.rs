#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn analyzer(opcode: u32, cpu: &mut ::cpu_context::CpuContext, analysis: &mut ::analysis::Analysis) {
match opcode {
    0x00 | 0x100 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x01 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x101 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x02 | 0x102 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x03 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x103 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x04 | 0x104 => {
        cpu.read_imm8();
    },
    0x05 => {
        cpu.read_imm16();
    },
    0x105 => {
        cpu.read_imm32();
    },
    0x06 => {
    },
    0x106 => {
    },
    0x07 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x107 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x08 | 0x108 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x09 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x109 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x0A | 0x10A => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x0B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x10B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x0C | 0x10C => {
        cpu.read_imm8();
    },
    0x0D => {
        cpu.read_imm16();
    },
    0x10D => {
        cpu.read_imm32();
    },
    0x0E => {
    },
    0x10E => {
    },
    0x0F => {
        ::analysis::instr16_0F_analyze(cpu, analysis);
    },
    0x10F => {
        ::analysis::instr32_0F_analyze(cpu, analysis);
    },
    0x10 | 0x110 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x11 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x111 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x12 | 0x112 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x13 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x113 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x14 | 0x114 => {
        cpu.read_imm8();
    },
    0x15 => {
        cpu.read_imm16();
    },
    0x115 => {
        cpu.read_imm32();
    },
    0x16 => {
    },
    0x116 => {
    },
    0x17 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x117 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x18 | 0x118 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x19 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x119 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1A | 0x11A => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x11B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x1C | 0x11C => {
        cpu.read_imm8();
    },
    0x1D => {
        cpu.read_imm16();
    },
    0x11D => {
        cpu.read_imm32();
    },
    0x1E => {
    },
    0x11E => {
    },
    0x1F => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x11F => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x20 | 0x120 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x21 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x121 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x22 | 0x122 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x23 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x123 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x24 | 0x124 => {
        cpu.read_imm8();
    },
    0x25 => {
        cpu.read_imm16();
    },
    0x125 => {
        cpu.read_imm32();
    },
    0x26 | 0x126 => {
        ::analysis::instr_26_analyze(cpu, analysis);
    },
    0x27 | 0x127 => {
    },
    0x28 | 0x128 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x29 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x129 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x2A | 0x12A => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x2B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x12B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x2C | 0x12C => {
        cpu.read_imm8();
    },
    0x2D => {
        cpu.read_imm16();
    },
    0x12D => {
        cpu.read_imm32();
    },
    0x2E | 0x12E => {
        ::analysis::instr_2E_analyze(cpu, analysis);
    },
    0x2F | 0x12F => {
    },
    0x30 | 0x130 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x31 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x131 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x32 | 0x132 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x33 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x133 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x34 | 0x134 => {
        cpu.read_imm8();
    },
    0x35 => {
        cpu.read_imm16();
    },
    0x135 => {
        cpu.read_imm32();
    },
    0x36 | 0x136 => {
        ::analysis::instr_36_analyze(cpu, analysis);
    },
    0x37 | 0x137 => {
    },
    0x38 | 0x138 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x39 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x139 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x3A | 0x13A => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x3B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x13B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x3C | 0x13C => {
        cpu.read_imm8();
    },
    0x3D => {
        cpu.read_imm16();
    },
    0x13D => {
        cpu.read_imm32();
    },
    0x3E | 0x13E => {
        ::analysis::instr_3E_analyze(cpu, analysis);
    },
    0x3F | 0x13F => {
    },
    0x40 => {
    },
    0x140 => {
    },
    0x41 => {
    },
    0x141 => {
    },
    0x42 => {
    },
    0x142 => {
    },
    0x43 => {
    },
    0x143 => {
    },
    0x44 => {
    },
    0x144 => {
    },
    0x45 => {
    },
    0x145 => {
    },
    0x46 => {
    },
    0x146 => {
    },
    0x47 => {
    },
    0x147 => {
    },
    0x48 => {
    },
    0x148 => {
    },
    0x49 => {
    },
    0x149 => {
    },
    0x4A => {
    },
    0x14A => {
    },
    0x4B => {
    },
    0x14B => {
    },
    0x4C => {
    },
    0x14C => {
    },
    0x4D => {
    },
    0x14D => {
    },
    0x4E => {
    },
    0x14E => {
    },
    0x4F => {
    },
    0x14F => {
    },
    0x50 => {
    },
    0x150 => {
    },
    0x51 => {
    },
    0x151 => {
    },
    0x52 => {
    },
    0x152 => {
    },
    0x53 => {
    },
    0x153 => {
    },
    0x54 => {
    },
    0x154 => {
    },
    0x55 => {
    },
    0x155 => {
    },
    0x56 => {
    },
    0x156 => {
    },
    0x57 => {
    },
    0x157 => {
    },
    0x58 => {
    },
    0x158 => {
    },
    0x59 => {
    },
    0x159 => {
    },
    0x5A => {
    },
    0x15A => {
    },
    0x5B => {
    },
    0x15B => {
    },
    0x5C => {
    },
    0x15C => {
    },
    0x5D => {
    },
    0x15D => {
    },
    0x5E => {
    },
    0x15E => {
    },
    0x5F => {
    },
    0x15F => {
    },
    0x60 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x160 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x61 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x161 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x62 | 0x162 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x63 | 0x163 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x64 | 0x164 => {
        ::analysis::instr_64_analyze(cpu, analysis);
    },
    0x65 | 0x165 => {
        ::analysis::instr_65_analyze(cpu, analysis);
    },
    0x66 | 0x166 => {
        ::analysis::instr_66_analyze(cpu, analysis);
    },
    0x67 | 0x167 => {
        ::analysis::instr_67_analyze(cpu, analysis);
    },
    0x68 => {
        cpu.read_imm16();
    },
    0x168 => {
        cpu.read_imm32();
    },
    0x69 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        cpu.read_imm16();
    },
    0x169 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        cpu.read_imm32();
    },
    0x6A => {
        cpu.read_imm8s();
    },
    0x16A => {
        cpu.read_imm8s();
    },
    0x6B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        cpu.read_imm8s();
    },
    0x16B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        cpu.read_imm8s();
    },
    0x6C | 0x16C => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x6D => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x16D => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x6E | 0x16E => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x6F => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x16F => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x70 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x70), is_32: cpu.osize_32() };
    },
    0x170 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x70), is_32: cpu.osize_32() };
    },
    0x71 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x71), is_32: cpu.osize_32() };
    },
    0x171 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x71), is_32: cpu.osize_32() };
    },
    0x72 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x72), is_32: cpu.osize_32() };
    },
    0x172 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x72), is_32: cpu.osize_32() };
    },
    0x73 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x73), is_32: cpu.osize_32() };
    },
    0x173 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x73), is_32: cpu.osize_32() };
    },
    0x74 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x74), is_32: cpu.osize_32() };
    },
    0x174 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x74), is_32: cpu.osize_32() };
    },
    0x75 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x75), is_32: cpu.osize_32() };
    },
    0x175 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x75), is_32: cpu.osize_32() };
    },
    0x76 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x76), is_32: cpu.osize_32() };
    },
    0x176 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x76), is_32: cpu.osize_32() };
    },
    0x77 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x77), is_32: cpu.osize_32() };
    },
    0x177 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x77), is_32: cpu.osize_32() };
    },
    0x78 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x78), is_32: cpu.osize_32() };
    },
    0x178 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x78), is_32: cpu.osize_32() };
    },
    0x79 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x79), is_32: cpu.osize_32() };
    },
    0x179 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x79), is_32: cpu.osize_32() };
    },
    0x7A => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7A), is_32: cpu.osize_32() };
    },
    0x17A => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7A), is_32: cpu.osize_32() };
    },
    0x7B => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7B), is_32: cpu.osize_32() };
    },
    0x17B => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7B), is_32: cpu.osize_32() };
    },
    0x7C => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7C), is_32: cpu.osize_32() };
    },
    0x17C => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7C), is_32: cpu.osize_32() };
    },
    0x7D => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7D), is_32: cpu.osize_32() };
    },
    0x17D => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7D), is_32: cpu.osize_32() };
    },
    0x7E => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7E), is_32: cpu.osize_32() };
    },
    0x17E => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7E), is_32: cpu.osize_32() };
    },
    0x7F => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7F), is_32: cpu.osize_32() };
    },
    0x17F => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0x7F), is_32: cpu.osize_32() };
    },
    0x80 | 0x180 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
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
    0x81 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x181 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x82 | 0x182 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
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
    0x83 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x183 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8s();
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x84 | 0x184 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x85 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x185 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x86 | 0x186 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x87 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x187 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x88 | 0x188 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x89 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x189 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x8A | 0x18A => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x8B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x18B => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x8C => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x18C => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
    },
    0x8D => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x18D => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x8E | 0x18E => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x8F => {
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
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x18F => {
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
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x90 | 0x190 => {
    },
    0x91 => {
    },
    0x191 => {
    },
    0x92 => {
    },
    0x192 => {
    },
    0x93 => {
    },
    0x193 => {
    },
    0x94 => {
    },
    0x194 => {
    },
    0x95 => {
    },
    0x195 => {
    },
    0x96 => {
    },
    0x196 => {
    },
    0x97 => {
    },
    0x197 => {
    },
    0x98 => {
    },
    0x198 => {
    },
    0x99 => {
    },
    0x199 => {
    },
    0x9A => {
        cpu.read_imm16();
        cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x19A => {
        cpu.read_imm32();
        cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x9B | 0x19B => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x9C => {
    },
    0x19C => {
    },
    0x9D => {
    },
    0x19D => {
    },
    0x9E | 0x19E => {
    },
    0x9F | 0x19F => {
    },
    0xA0 | 0x1A0 => {
        cpu.read_moffs();
    },
    0xA1 => {
        cpu.read_moffs();
    },
    0x1A1 => {
        cpu.read_moffs();
    },
    0xA2 | 0x1A2 => {
        cpu.read_moffs();
    },
    0xA3 => {
        cpu.read_moffs();
    },
    0x1A3 => {
        cpu.read_moffs();
    },
    0xA4 | 0x1A4 => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0xA5 => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0x1A5 => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0xA6 | 0x1A6 => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0xA7 => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0x1A7 => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
    },
    0xA8 | 0x1A8 => {
        cpu.read_imm8();
    },
    0xA9 => {
        cpu.read_imm16();
    },
    0x1A9 => {
        cpu.read_imm32();
    },
    0xAA | 0x1AA => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0xAB => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0x1AB => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0xAC | 0x1AC => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0xAD => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0x1AD => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0xAE | 0x1AE => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0xAF => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0x1AF => {
        if cpu.prefixes & ::prefix::PREFIX_F2 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else if cpu.prefixes & ::prefix::PREFIX_F3 != 0 {
            analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        }
        else {
        }
    },
    0xB0 | 0x1B0 => {
        cpu.read_imm8();
    },
    0xB1 | 0x1B1 => {
        cpu.read_imm8();
    },
    0xB2 | 0x1B2 => {
        cpu.read_imm8();
    },
    0xB3 | 0x1B3 => {
        cpu.read_imm8();
    },
    0xB4 | 0x1B4 => {
        cpu.read_imm8();
    },
    0xB5 | 0x1B5 => {
        cpu.read_imm8();
    },
    0xB6 | 0x1B6 => {
        cpu.read_imm8();
    },
    0xB7 | 0x1B7 => {
        cpu.read_imm8();
    },
    0xB8 => {
        cpu.read_imm16();
    },
    0x1B8 => {
        cpu.read_imm32();
    },
    0xB9 => {
        cpu.read_imm16();
    },
    0x1B9 => {
        cpu.read_imm32();
    },
    0xBA => {
        cpu.read_imm16();
    },
    0x1BA => {
        cpu.read_imm32();
    },
    0xBB => {
        cpu.read_imm16();
    },
    0x1BB => {
        cpu.read_imm32();
    },
    0xBC => {
        cpu.read_imm16();
    },
    0x1BC => {
        cpu.read_imm32();
    },
    0xBD => {
        cpu.read_imm16();
    },
    0x1BD => {
        cpu.read_imm32();
    },
    0xBE => {
        cpu.read_imm16();
    },
    0x1BE => {
        cpu.read_imm32();
    },
    0xBF => {
        cpu.read_imm16();
    },
    0x1BF => {
        cpu.read_imm32();
    },
    0xC0 | 0x1C0 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
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
    0xC1 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
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
    0x1C1 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
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
    0xC2 => {
        cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
        analysis.absolute_jump = true;
    },
    0x1C2 => {
        cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
        analysis.absolute_jump = true;
    },
    0xC3 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
        analysis.absolute_jump = true;
    },
    0x1C3 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
        analysis.absolute_jump = true;
    },
    0xC4 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1C4 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xC5 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1C5 => {
        let modrm_byte = cpu.read_imm8();
        if modrm_byte < 0xC0 {
            ::analysis::modrm_analyze(cpu, modrm_byte);
        }
        else {
        }
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xC6 | 0x1C6 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
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
    0xC7 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
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
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xC8 => {
        cpu.read_imm16();
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1C8 => {
        cpu.read_imm16();
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xC9 => {
    },
    0x1C9 => {
    },
    0xCA => {
        cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0x1CA => {
        cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0xCB => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0x1CB => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0xCC | 0x1CC => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xCD | 0x1CD => {
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xCE | 0x1CE => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xCF => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0x1CF => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0xD0 | 0x1D0 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xD1 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x1D1 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xD2 | 0x1D2 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xD3 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x1D3 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xD4 | 0x1D4 => {
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xD5 | 0x1D5 => {
        cpu.read_imm8();
    },
    0xD6 | 0x1D6 => {
    },
    0xD7 | 0x1D7 => {
    },
    0xD8 | 0x1D8 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xD9 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x1D9 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xDA | 0x1DA => {
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
    0xDB | 0x1DB => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
    0xDC | 0x1DC => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xDD => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
    0x1DD => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
    0xDE | 0x1DE => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xDF | 0x1DF => {
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
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xE0 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0xE0), is_32: cpu.osize_32() };
    },
    0x1E0 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0xE0), is_32: cpu.osize_32() };
    },
    0xE1 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0xE1), is_32: cpu.osize_32() };
    },
    0x1E1 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0xE1), is_32: cpu.osize_32() };
    },
    0xE2 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0xE2), is_32: cpu.osize_32() };
    },
    0x1E2 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0xE2), is_32: cpu.osize_32() };
    },
    0xE3 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0xE3), is_32: cpu.osize_32() };
    },
    0x1E3 => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: Some(0xE3), is_32: cpu.osize_32() };
    },
    0xE4 | 0x1E4 => {
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xE5 => {
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1E5 => {
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xE6 | 0x1E6 => {
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xE7 => {
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1E7 => {
        cpu.read_imm8();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xE8 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: None, is_32: cpu.osize_32() };
    },
    0x1E8 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: None, is_32: cpu.osize_32() };
    },
    0xE9 => {
        let jump_offset = cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: None, is_32: cpu.osize_32() };
        analysis.no_next_instruction = true;
    },
    0x1E9 => {
        let jump_offset = cpu.read_imm32();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: None, is_32: cpu.osize_32() };
        analysis.no_next_instruction = true;
    },
    0xEA => {
        cpu.read_imm16();
        cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0x1EA => {
        cpu.read_imm32();
        cpu.read_imm16();
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0xEB => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: None, is_32: cpu.osize_32() };
        analysis.no_next_instruction = true;
    },
    0x1EB => {
        let jump_offset = cpu.read_imm8s();
        analysis.ty = ::analysis::AnalysisType::Jump { offset: jump_offset as i32, condition: None, is_32: cpu.osize_32() };
        analysis.no_next_instruction = true;
    },
    0xEC | 0x1EC => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xED => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1ED => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xEE | 0x1EE => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xEF => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0x1EF => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
    },
    0xF0 | 0x1F0 => {
        ::analysis::instr_F0_analyze(cpu, analysis);
    },
    0xF1 | 0x1F1 => {
    },
    0xF2 | 0x1F2 => {
        ::analysis::instr_F2_analyze(cpu, analysis);
    },
    0xF3 | 0x1F3 => {
        ::analysis::instr_F3_analyze(cpu, analysis);
    },
    0xF4 | 0x1F4 => {
        analysis.ty = ::analysis::AnalysisType::BlockBoundary;
        analysis.no_next_instruction = true;
    },
    0xF5 | 0x1F5 => {
    },
    0xF6 | 0x1F6 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm8();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
    0xF7 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm16();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x1F7 => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                cpu.read_imm32();
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            3 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            4 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
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
            },
            7 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xF8 | 0x1F8 => {
    },
    0xF9 | 0x1F9 => {
    },
    0xFA | 0x1FA => {
    },
    0xFB | 0x1FB => {
        analysis.ty = ::analysis::AnalysisType::STI;
    },
    0xFC | 0x1FC => {
    },
    0xFD | 0x1FD => {
    },
    0xFE | 0x1FE => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0xFF => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.absolute_jump = true;
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
                analysis.no_next_instruction = true;
                analysis.absolute_jump = true;
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    0x1FF => {
        let modrm_byte = cpu.read_imm8();
        match modrm_byte >> 3 & 7 {
            0 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            1 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            2 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.absolute_jump = true;
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
                analysis.no_next_instruction = true;
                analysis.absolute_jump = true;
            },
            5 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            },
            6 => {
                if modrm_byte < 0xC0 {
                    ::analysis::modrm_analyze(cpu, modrm_byte);
                }
                else {
                }
            },
            _ => {
                analysis.ty = ::analysis::AnalysisType::BlockBoundary;
                analysis.no_next_instruction = true;
            }
        }
    },
    _ => {
        dbg_assert!(false);
    }
}
}
