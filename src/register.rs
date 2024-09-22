use crate::cpsr::CPSR;


#[derive(Debug, Clone, Default)]
pub struct GeneralPurposeRegisters {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
}

#[derive(Debug, Clone, Default)]
pub struct FIQRegisters {
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    r13: u32,
    r14: u32,
    spsr: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SVCRegisters {
    r13: u32,
    r14: u32,
    spsr: u32,
}

#[derive(Debug, Clone, Default)]
pub struct ABTRegisters {
    r13: u32,
    r14: u32,
    spsr: u32,
}

#[derive(Debug, Clone, Default)]
pub struct IRQRegisters {
    r13: u32,
    r14: u32,
    spsr: u32,
}

#[derive(Debug, Clone, Default)]
pub struct UNDRegisters {
    r13: u32,
    r14: u32,
    spsr: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Registers {

    // r0-r12 (General Purpose Registers)
    gp: GeneralPurposeRegisters,

    // In THUMP mode, r13 is used as SP
    r13: u32, // Stack Pointer (SP)

    r14: u32, // Link Register (LR)

    r15: u32, // Program Counter (PC)

    // CPSR (Current Program Status Register)
    cpsr: CPSR,

    // SPSR (Saved Program Status Register)
    spsr: u32,

    // Banked Registers
    // FIQ (Fast Interrupt Request)
    fiq: FIQRegisters,

    // SVC (Supervisor)
    svc: SVCRegisters,

    // ABT (Abort)
    abt: ABTRegisters,

    // IRQ (Interrupt Request)
    irq: IRQRegisters,

    // UND (Undefined)
    und: UNDRegisters,
}
