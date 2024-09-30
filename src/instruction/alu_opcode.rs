use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Default)]
    pub struct DataProcessingALUOpCode: u32 {
        // 31-28 condition code
        const condition_code = 0b1111 << 28;
        // 27-26 Must be 00b for this instruction
        const must_be_00 = 0b00 << 26;
        // 25 I - Immediate 2nd Operand Flag (0=Register, 1=Immediate)
        const I = 1 << 25;
        // 24-21 Opcode (0 - fh)
        // 0 = AND{cond}{S} Rd, Rn, Operand2 : Rd = Rn AND Operand2
        // 1 = EOR{cond}{S} Rd, Rn, Operand2 : Rd = Rn XOR Operand2
        // 2 = SUB{cond}{S} Rd, Rn, Operand2 : Rd = Rn - Operand2
        // 3 = RSB{cond}{S} Rd, Rn, Operand2 : Rd = Operand2 - Rn
        // 4 = ADD{cond}{S} Rd, Rn, Operand2 : Rd = Rn + Operand2
        // 5 = ADC{cond}{S} Rd, Rn, Operand2 : Rd = Rn + Operand2 + C
        // 6 = SBC{cond}{S} Rd, Rn, Operand2 : Rd = Rn - Operand2 + C - 1
        // 7 = RSC{cond}{S} Rd, Rn, Operand2 : Rd = Operand2 - Rn + C - 1
        // 8 = TST{cond} Rn, Operand2 : Void = Rn AND Operand2
        // 9 = TEQ{cond} Rn, Operand2 : Void = Rn XOR Operand2
        // A = CMP{cond} Rn, Operand2 : Void = Rn - Operand2
        // B = CMN{cond} Rn, Operand2 : Void = Rn + Operand2
        // C = ORR{cond}{S} Rd, Rn, Operand2 : Rd = Rn OR Operand2
        // D = MOV{cond}{S} Rd, Operand2 : Rd = Operand2
        // E = BIC{cond}{S} Rd, Rn, Operand2 : Rd = Rn AND NOT Operand2
        // F = MVN{cond}{S} Rd, Operand2 : Rd = NOT Operand2
        const opcode = 0b1111 << 21;
        // 20 S - Set Condition Codes (0=No, 1=Yes) (Must be 1 for opcode 8-B)
        const S = 1 << 20;
        // 19-16 Rn - First Operand Register (R0-R15) (including PC=R15) (Must be 0000b or 1111b for CMP/CMN/TST/TEQ{P})
        const Rn = 0b1111 << 16;
        // 15-12 Rd - Destination Register (R0-R15) (including PC=R15) (Must be 0000b or 1111b for CMP/CMN/TST/TEQ{P})
        const Rd = 0b1111 << 12;
        // When above Bit 25 I=0 (Register as 2nd Operand)
        // When below Bit 4 R=0 - Shift by Immediate
        // 11-7 Is - Shift amount (1-31, 0=Special)
        const Is = 0b11111 << 7;
        // When below Bit 4 R=1 - Shift by Register
        // 11-8 Rs - Shift Register (R0-R14) - only lower 8bit 0-255 used 
        const Rs = 0b1111 << 8;
        // 7 - Reserved, must be 0 (otherwise multiply or LDREX or undefined)
        const reserved = 1 << 7;
        // 6-5 Shift Type (0=LSL, 1=LSR, 2=ASR, 3=ROR)
        const shift_type = 0b11 << 5;
        // 4 R - Shift by Register (0=Immediate, 1=Register)
        const R = 1 << 4;
        // 3-0 Rm - Second Operand Register (R0-R15) (including PC=R15)
        const Rm = 0b1111;
        // When above Bit 25 I=1 (Immediate as 2nd Operand)
        // 11-8 Is - ROR-Shift applied to nn (0-30, in steps of 2)
        const Is_ROR = 0b1111 << 8;
        // 7-0 nn - 2nd Operand Unsigned 8bit Immediate
        const nn = 0b11111111;
    }
}
