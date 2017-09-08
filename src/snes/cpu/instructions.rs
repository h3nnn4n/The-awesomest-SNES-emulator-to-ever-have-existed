use std::borrow::Cow;
use snes::cpu::registers::Register;

#[derive(Clone, Copy)]
enum AddressMode {
    Implied,
    ImmMemoryFlag,
    ImmIndexFlag,
    Imm8bit,
    Relative,
    RelativeLong,
    Direct,
    DirectIndexX,
    DirectIndexY,
    DirectIndirect,
    DirectIndexedIndirect,
    DirectIndirectIndexed,
    DirectIndirectLong,
    DirectIndirectIndexedLong,
    Absolute,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    AbsoluteLong,
    AbsoluteIndexedLong,
    StackRelative,
    StackRelativeIndirectIndexed,
    AbsoluteIndirect,
    AbsoluteIndirectLong,
    AbsoluteIndexedIndirect,
    ImpliedAccumulator,
    BlockMove,
}

#[derive(Clone, Copy)]
enum Operand {
    Reg(Register),
    Imm8(u8),
    Imm16(u16),
    Imm24(u32),
}

pub enum Instruction {
    Adc1(Operand, AddressMode),
    Adc2(Operand, Operand, AddressMode),
    Adc3(Operand, Operand, Operand, AddressMode),
    And1(Operand, AddressMode),
    And2(Operand, Operand, AddressMode),
    Asl,
    // Bcc,
    // Bcs,
    // Beq,
    // Bit,
    // Bmi,
    // Bne,
    // Bpl,
    // Bra,
    // Brk,
    // Brl,
    // Bvc,
    // Bvs,
    // Clc,
    // Cli,
    // Clv,
    // Cmp,
    // Cop,
    // Cpx,
    // Cpy,
    // Dec,
    // Dex,
    // Dey,
    // Eor,
    // Inc,
    // Inx,
    // Iny,
    // Jml,
    // Jmp,
    // Jsl,
    // Jsr,
    // Lda,
    // Ldx,
    // Ldy,
    // Lsr,
    // Mvn,
    // Mvp,
    // Nop,
    // Ora,
    // Pea,
    // Pei,
    // Per,
    // Pha,
    // Phb,
    // Phd,
    // Phk,
    // Php,
    // Phx,
    // Phy,
    // Pla,
    // Plb,
    // Pld,
    // Plp,
    // Plx,
    // Ply,
    // Rep,
    // Rol,
    // Ror,
    // Rti,
    // Rtl,
    // Rts,
    // Sbc,
    // Sep,
    // Sec,
    // Sed,
    // Sei,
    // Sta,
    // Stp,
    // Stx,
    // Sty,
    // Stz,
    // Tax,
    // Tay,
    // Tcd,
    // Tcs,
    // Tdc,
    // Trb,
    // Tsb,
    // Tsc,
    // Tsx,
    // Txa,
    // Txs,
    // Txy,
    // Tya,
    // Tyx,
    // Wai,
    // Wdm,
    // Xba,
    // Xce,
}

pub type AsmStr = Cow<'static, str>;

pub trait ToAsmStr {
    fn to_asm_str(&self) -> AsmStr;
}

impl ToAsmStr for Register {
    fn to_asm_str(&self) -> AsmStr {
        (format!("{:?}", *self)).into()
    }
}


impl ToAsmStr for u8 {
    fn to_asm_str(&self) -> AsmStr {
        (format!("${:02x}", *self)).into()
    }
}

impl ToAsmStr for u16 {
    fn to_asm_str(&self) -> AsmStr {
        (format!("${:04x}", *self)).into()
    }
}

impl ToAsmStr for u32 {
    fn to_asm_str(&self) -> AsmStr {
        (format!("${:06x}", *self)).into()
    }
}

impl ToAsmStr for Operand {
    fn to_asm_str(&self) -> AsmStr {
        match *self {
            Operand::Reg(out) => out.to_asm_str(),
            Operand::Imm8(out) => out.to_asm_str(),
            Operand::Imm16(out) => out.to_asm_str(),
            Operand::Imm24(out) => out.to_asm_str(),
            _ => panic!("Operand not supported"),
        }
    }
}

fn none_op(op: &'static str) -> AsmStr {
    op.into()
}


#[cfg_attr(rustfmt, rustfmt_skip)]
fn unary_op<A: ToAsmStr>(op: &'static str, arg1: A, mode: AddressMode) -> AsmStr {
    use self::AddressMode as Mode;
    
    let str1 = arg1.to_asm_str(); 
    match mode {
        Mode::ImmMemoryFlag        => (format!("{} #{}", op, str1)).into(),
        Mode::ImmIndexFlag         => (format!("{} #{}", op, str1)).into(),
        Mode::Imm8bit              => (format!("{} #{}", op, str1)).into(),
        Mode::Direct               => (format!("{} {}" , op, str1)).into(),
        Mode::DirectIndirect       => (format!("{} ({})", op, str1)).into(),
        Mode::DirectIndirectLong   => (format!("{} [_{}_]", op, str1)).into(),
        Mode::Absolute             => (format!("{} {}", op, str1)).into(),
        Mode::AbsoluteLong         => (format!("{} {}", op, str1)).into(),
        Mode::AbsoluteIndirect     => (format!("{} ({})", op, str1)).into(),
        Mode::AbsoluteIndirectLong => (format!("{} ({})", op, str1)).into(),
        _ => panic!("Addressing mode disassembly not implemented"),
    }
    
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn binary_op<A: ToAsmStr>(op: &'static str, arg1: A, arg2: A, mode: AddressMode) -> AsmStr {
    use self::AddressMode as Mode;
    
    let str1 = arg1.to_asm_str();
    let str2 = arg2.to_asm_str();
    match mode {
        Mode::DirectIndexX              => (format!("{} {}, {}", op, str1, str2)).into(),
        Mode::DirectIndexY              => (format!("{} {}, {}", op, str1, str2)).into(),
        Mode::DirectIndexedIndirect     => (format!("{} ({}, {})", op, str1, str2)).into(),
        Mode::DirectIndirectIndexed     => (format!("{} ({}), {}", op, str1, str2)).into(),
        Mode::DirectIndirectIndexedLong => (format!("{} [{}], {}", op, str1, str2)).into(),
        Mode::AbsoluteIndexedX          => (format!("{} {}, {}", op, str1, str2)).into(),
        Mode::AbsoluteIndexedY          => (format!("{} {}, {}", op, str1, str2)).into(),
        Mode::AbsoluteIndexedLong       => (format!("{} {}, {}", op, str1, str2)).into(),
        Mode::StackRelative             => (format!("{} {}, {}", op, str1, str2)).into(),
        Mode::AbsoluteIndexedIndirect   => (format!("{} {}, {}", op, str1, str2)).into(),
        Mode::BlockMove                 => (format!("{} {}, {}", op, str1, str2)).into(),
        _ => panic!("Addressing mode disassembly not implemented"),
    }
}


#[cfg_attr(rustfmt, rustfmt_skip)]
fn ternary_op<A: ToAsmStr>(op: &'static str, arg1: A, arg2: A, arg3: A, mode: AddressMode) -> AsmStr {
    use self::AddressMode as Mode;
    
    let str1 = arg1.to_asm_str();
    let str2 = arg2.to_asm_str();
    let str3 = arg3.to_asm_str();
    
    match mode {
        Mode::StackRelativeIndirectIndexed => (format!("{} {}, {}", op, str1, str2)).into(), 
        _ => panic!("Addressing mode disassembly not implemented"),
    }
}

impl ToAsmStr for Instruction {
    fn to_asm_str(&self) -> AsmStr {
        use self::Instruction::*;
        match *self {
            Adc1(op0, mode) => unary_op("ADC", op0, mode),
            Adc2(op0, op1, mode) => binary_op("ADC", op0, op1, mode),
            And1(op0, mode) => unary_op("AND", op0, mode),
            And2(op0, op1, mode) => binary_op("AND", op0, op1, mode),
            _ => panic!("¯\\_(ツ)_/¯"),
        }
    }
}


pub fn disassemble(code: &[u8]) -> AsmStr {
    use self::Operand::*;
    use self::AddressMode::*;
    use self::Register::*; // WARNING: many single character symbols are imported

    let as8: u8 = code[1];
    let as16: u16 = ((code[2] as u16) << 8) | as8 as u16;
    let as24: u32 = ((code[3] as u32) << 16) | as16 as u32;

    let opcode = code[0];
    let instr = match opcode {
        // ADC Instructions
        0x61 => Instruction::Adc2(Imm8(as8), Reg(X), DirectIndirectIndexed),
        0x63 => Instruction::Adc2(Imm8(as8), Reg(S), StackRelative),
        0x65 => Instruction::Adc1(Imm8(as8), Direct),
        0x67 => Instruction::Adc1(Imm8(as8), DirectIndirectLong),
        0x69 => Instruction::Adc1(Imm8(as8), Imm8bit),
        0x6D => Instruction::Adc1(Imm16(as16), Absolute),
        0x6F => Instruction::Adc1(Imm24(as24), AbsoluteLong),
        0x71 => Instruction::Adc2(Imm8(as8), Reg(Y), DirectIndirectIndexed),
        0x72 => Instruction::Adc1(Imm8(as8), DirectIndirect),
        0x73 => Instruction::Adc3(Imm8(as8), Reg(S), Reg(Y), StackRelativeIndirectIndexed),
        _ => panic!("Unknown opcode {}", opcode),
    };

    instr.to_asm_str()
}
