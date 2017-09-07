use std::borrow::Cow;
use snes::cpu::registers::Register;

// struct Instruction {
//     operands: (Operand, Operand, Operand),
// }

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

enum Operand8 {
    Reg(Register),
    Imm8(u8),
}

enum Operand16 {
    Reg(Register),
    Imm16(u16),
}

// enum OperandSet {
//     Adc(u8),
//     Adc16(u16),
// }

pub enum Instruction {
    Adc(u8),
    Adc16(u16),
    Adc24(u8, u16),
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

fn null_op(op: &'static str) -> AsmStr {
    op.into()
}

fn unary_op<A: ToAsmStr>(op: &'static str, arg: A) -> AsmStr {
    (format!("{} {}", op, arg.to_asm_str())).into()
}

fn binary_op<A: ToAsmStr, B: ToAsmStr>(op: &'static str, arg1: A, arg2: B) -> AsmStr {
    (format!("{} {}, {}", op, arg1.to_asm_str(), arg2.to_asm_str())).into()
}

impl ToAsmStr for Instruction {
    fn to_asm_str(&self) -> AsmStr {
        use self::Instruction::*;
        match *self {
            Adc(out8) => unary_op("ADC", out8),
            Adc16(out16) => unary_op("ADC", out16),
            _ => panic!("¯\\_(ツ)_/¯"),
        }
    }
}
