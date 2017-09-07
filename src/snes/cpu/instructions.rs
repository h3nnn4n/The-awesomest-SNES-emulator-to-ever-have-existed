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
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Bra,
    Brk,
    Brl,
    Bvc,
    Bvs,
    Clc,
    Cli,
    Clv,
    Cmp,
    Cop,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jml,
    Jmp,
    Jsl,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Mvn,
    Mvp,
    Nop,
    Ora,
    Pea,
    Pei,
    Per,
    Pha,
    Phb,
    Phd,
    Phk,
    Php,
    Phx,
    Phy,
    Pla,
    Plb,
    Pld,
    Plp,
    Plx,
    Ply,
    Rep,
    Rol,
    Ror,
    Rti,
    Rtl,
    Rts,
    Sbc,
    Sep,
    Sec,
    Sed,
    Sei,
    Sta,
    Stp,
    Stx,
    Sty,
    Stz,
    Tax,
    Tay,
    Tcd,
    Tcs,
    Tdc,
    Trb,
    Tsb,
    Tsc,
    Tsx,
    Txa,
    Txs,
    Txy,
    Tya,
    Tyx,
    Wai,
    Wdm,
    Xba,
    Xce,
}

pub type AsmStr = Cow<'static, str>;

pub trait ToAsmStr {
    fn to_asm_str(&self) -> AsmStr;
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
            // Operand::Reg(out)   => out.to_asm_str(),
            Operand::Imm8(out)  => out.to_asm_str(),
            Operand::Imm16(out) => out.to_asm_str(),
            Operand::Imm24(out) => out.to_asm_str(),
            _ => panic!("Operand not supported"),
        }
    }
}

fn none_op(op: &'static str) -> AsmStr {
    op.into()
}

fn unary_op<A: ToAsmStr>(op: &'static str, arg: A, mode: AddressMode) -> AsmStr {
    use self::AddressMode::*;
    
    let arg_str = arg.to_asm_str(); 
    match mode {
        Imm8 => (format!("{} #{}", op, arg_str)).into(),
        _ => (format!("{} {}", op, arg_str)).into(),
    }
    
}

fn binary_op<A: ToAsmStr, B: ToAsmStr>(op: &'static str, arg1: A, arg2: B, mode: AddressMode) -> AsmStr {
    let arg1_str = arg1.to_asm_str();
    let arg2_str = arg2.to_asm_str();
    match mode {
        _ => (format!("{} {}, {}", op, arg1_str, arg2_str)).into(),
    }
    
    
}

impl ToAsmStr for Instruction {
    fn to_asm_str(&self) -> AsmStr {
        use self::Instruction::*;
        match *self {
            Adc1(op0, mode)      => unary_op ("ADC", op0, mode),
            Adc2(op0, op1, mode) => binary_op ("ADC", op0, op1, mode), 
            _ => panic!("¯\\_(ツ)_/¯"),
        }
    }
}


pub fn disassemble (code: &[u8]) -> AsmStr {
    use self::Operand::*;
    use self::AddressMode::*;
    let my16: u16 = ((code[2] as u16) << 8) & code[1] as u16;
    let my8: u8 = code[1];
    let opcode = code[0];
    let instr = match opcode {
        0x69 => Instruction::Adc1(Imm8(my8), Imm8bit),
        _ => panic!("Unknown opcode {}", opcode),
    };

    instr.to_asm_str()
}
