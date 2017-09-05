
struct Instruction {
    operands: (Operand, Operand, Operand),
}

enum Instruction {
    
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
