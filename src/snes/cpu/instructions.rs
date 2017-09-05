
struct Instruction {
    operands: (Operand, Operand, Operand),
}


#[cfg_attr(rustfmt, rustfmt_skip)]
enum Instruction {
    Adc,
    And, Asl,
    Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Bra, Brk, Brl, Bvc, Bvs,
    Clc, Cli, Clv, Cmp, Cop, Cpx, Cpy,
    Dec, Dex, Dey, Eor,
    Inc, Inx, Iny,
    Jml, Jmp, Jsl, Jsr,
    Lda, Ldx, Ldy, Lsr,
    Mvn, Mvp, Nop, Ora,
    Pea, Pei, Per, Pha, Phb, Phd, Phk, Php, Phx, Phy, Pla, Plb, Pld, Plp, Plx, Ply,
    Rep, Rol, Ror, Rti, Rtl, Rts,
    Sbc, Sep, Sec, Sed, Sei, Sta, Stp, Stx, Sty, Stz,
    Tax, Tay, Tcd, Tcs, Tdc, Trb, Tsb, Tsc, Tsx, Txa, Txs, Txy, Tya, Tyx,
    Wai, Wdm,
    Xba, Xce    
}

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

enum Operand {

}
