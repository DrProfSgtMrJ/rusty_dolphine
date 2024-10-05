use core::fmt;

use bitflags::bitflags;

use crate::cpu::CpuState;


bitflags! {
    #[derive(Debug, Clone, Default)]
    pub struct CPSR: u32 {

        // Sign flag (0 = positive, 1 = negative)
        // 31 bit
        const N = 1 << 31;

        // Zero flag (0 = Not zero, 1 = zero)
        // 30 bit
        const Z = 1 << 30;

        // Carry flag (0 = No carry, 1 = carry)
        // 29 bit
        const C = 1 << 29;

        // Overflow flag (0 = No overflow, 1 = overflow)
        // 28 bit
        const V = 1 << 28;

        // Sticky overflow flag : ARMv5TE and above
        // 27 bit
        const Q = 1 << 27;

        // Reserved (DO NOT CHANGE MANUALLY)
        // 26-25 bits
        const RESERVED = 0b11 << 25;

        // Jazelle Bytecode instructions (if supported)
        // 24 bit
        const J = 1 << 24;

        // Reserved (DO NOT CHANGE MANUALLY)
        // 23-10 bits
        const RESERVED2 = 0b11111111111 << 10;

        // Big Endian mode (0 = Little Endian, 1 = Big Endian)
        // 9 bit
        const E = 1 << 9; 

        // Abort disable (1 = disable imprecise data aborts) (ARM11 only)
        // 8 bit
        const A = 1 << 8;

        // IRQ disable (1 = disable IRQs)
        // 7 bit
        const I = 1 << 7;

        // FIQ disable (1 = disable FIQs)
        // 6 bit
        const F = 1 << 6;

        // State bit (0 = ARM, 1 = THUMB)
        // 5 bit
        const T = 1 << 5;

        // Mode bits
        // 4-0 bits
        const M = 0b11111;
    }
}

impl CPSR {

    pub fn is_negative(&self) -> bool {
        self.contains(CPSR::N)
    }

    pub fn setn(&mut self, value: bool) {
        self.set(CPSR::N, value);
    }

    pub fn is_zero(&self) -> bool {
        self.contains(CPSR::Z)
    }

    pub fn setz(&mut self, value: bool) {
        self.set(CPSR::Z, value);
    }

    pub fn carry(&self) -> u32 {
        if self.contains(CPSR::C) {
            1
        } else {
            0
        }
    }

    pub fn setc(&mut self, value: bool) {
        self.set(CPSR::C, value);
    }

    pub fn is_overflow(&self) -> bool {
        self.contains(CPSR::V)
    }

    pub fn setv(&mut self, value: bool) {
        self.set(CPSR::V, value);
    }

    pub fn is_sticky_overflow(&self) -> bool {
        self.contains(CPSR::Q)
    }

    pub fn setq(&mut self, value: bool) {
        self.set(CPSR::Q, value);
    }

    pub fn is_jazelle(&self) -> bool {
        self.contains(CPSR::J)
    }

    pub fn setj(&mut self, value: bool) {
        self.set(CPSR::J, value);
    }

    pub fn is_big_endian(&self) -> bool {
        self.contains(CPSR::E)
    }

    pub fn sete(&mut self, value: bool) {
        self.set(CPSR::E, value);
    }

    pub fn is_abort_disable(&self) -> bool {
        self.contains(CPSR::A)
    }

    pub fn seta(&mut self, value: bool) {
        self.set(CPSR::A, value);
    }

    pub fn is_irq_disable(&self) -> bool {
        self.contains(CPSR::I)
    }

    pub fn seti(&mut self, value: bool) {
        self.set(CPSR::I, value);
    }

    pub fn is_fiq_disable(&self) -> bool {
        self.contains(CPSR::F)
    }

    pub fn setf(&mut self, value: bool) {
        self.set(CPSR::F, value);
    }

    pub fn state(&self) -> CpuState {
        if self.contains(CPSR::T) {
            CpuState::THUMB
        } else {
            CpuState::ARM
        }
    }

    pub fn set_state(&mut self, state: CpuState) {
        match state {
            CpuState::ARM => self.set(CPSR::T, false),
            CpuState::THUMB => self.set(CPSR::T, true),
            _ => {}
        }
    }
}

impl fmt::Display for CPSR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPSR {{\n")?;
        write!(f, "    N (Sign flag): {}\n", if self.contains(CPSR::N) { 1 } else { 0 })?;
        write!(f, "    Z (Zero flag): {}\n", if self.contains(CPSR::Z) { 1 } else { 0 })?;
        write!(f, "    C (Carry flag): {}\n", if self.contains(CPSR::C) { 1 } else { 0 })?;
        write!(f, "    V (Overflow flag): {}\n", if self.contains(CPSR::V) { 1 } else { 0 })?;
        write!(f, "    Q (Sticky overflow flag): {}\n", if self.contains(CPSR::Q) { 1 } else { 0 })?;
        write!(f, "    J (Jazelle Bytecode): {}\n", if self.contains(CPSR::J) { 1 } else { 0 })?;
        write!(f, "    E (Endian mode): {}\n", if self.contains(CPSR::E) { 1 } else { 0 })?;
        write!(f, "    A (Abort disable): {}\n", if self.contains(CPSR::A) { 1 } else { 0 })?;
        write!(f, "    I (IRQ disable): {}\n", if self.contains(CPSR::I) { 1 } else { 0 })?;
        write!(f, "    F (FIQ disable): {}\n", if self.contains(CPSR::F) { 1 } else { 0 })?;
        write!(f, "    T (State bit): {}\n", if self.contains(CPSR::T) { 1 } else { 0 })?;
        write!(f, "    M (Mode bits): {:05b}\n", self.bits() & CPSR::M.bits())?;
        write!(f, "    Full CPSR Value: {:032b}\n", self.bits())?;
        write!(f, "}}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpsr_display() {
        let cpsr = CPSR::N | CPSR::Z | CPSR::C | CPSR::V | CPSR::Q | CPSR::J | CPSR::E | CPSR::I | CPSR::F | CPSR::T | CPSR::M;
        let expected = "CPSR {\n    N (Sign flag): 1\n    Z (Zero flag): 1\n    C (Carry flag): 1\n    V (Overflow flag): 1\n    Q (Sticky overflow flag): 1\n    J (Jazelle Bytecode): 1\n    E (Endian mode): 1\n    A (Abort disable): 0\n    I (IRQ disable): 1\n    F (FIQ disable): 1\n    T (State bit): 1\n    M (Mode bits): 11111\n    Full CPSR Value: ";

        assert!(format!("{}", cpsr).starts_with(expected));
    }

    #[test]
    fn test_default_cpsr() {
        let cpsr = CPSR::default();
        assert_eq!(cpsr.bits(), 0);

        assert!(!cpsr.is_negative());
        assert!(!cpsr.is_zero());
        assert!(cpsr.carry() == 0);
        assert!(!cpsr.is_overflow());
        assert!(!cpsr.is_sticky_overflow());
        assert!(!cpsr.is_jazelle());
        assert!(!cpsr.is_big_endian());
        assert!(!cpsr.is_abort_disable());
        assert!(!cpsr.is_irq_disable());
        assert!(!cpsr.is_fiq_disable());
        assert!(cpsr.state() == CpuState::ARM);
    }

    #[test]
    fn test_setn() {
        let mut cpsr = CPSR::default();
        assert!(!cpsr.is_negative());
        cpsr.setn(true);
        assert!(cpsr.is_negative());
    }

    #[test]
    fn test_setz() {
        let mut cpsr = CPSR::default();
        assert!(!cpsr.is_zero());
        cpsr.setz(true);
        assert!(cpsr.is_zero());
    }

    #[test]
    fn test_setc() {
        let mut cpsr = CPSR::default();
        assert!(cpsr.carry() == 0);
        cpsr.setc(true);
        assert!(cpsr.carry() == 1);
    }

    #[test]
    fn test_setv() {
        let mut cpsr = CPSR::default();
        assert!(!cpsr.is_overflow());
        cpsr.setv(true);
        assert!(cpsr.is_overflow());
    }

    #[test]
    fn test_setq() {
        let mut cpsr = CPSR::default();
        assert!(!cpsr.is_sticky_overflow());
        cpsr.setq(true);
        assert!(cpsr.is_sticky_overflow());
    }

    #[test]
    fn test_setj() {
        let mut cpsr = CPSR::default();
        assert!(!cpsr.is_jazelle());
        cpsr.setj(true);
        assert!(cpsr.is_jazelle());
    }

    #[test]
    fn test_sete() {
        let mut cpsr = CPSR::default();
        assert!(!cpsr.is_big_endian());
        cpsr.sete(true);
        assert!(cpsr.is_big_endian());
    }

    #[test]
    fn test_seta() {
        let mut cpsr = CPSR::default();
        assert!(!cpsr.is_abort_disable());
        cpsr.seta(true);
        assert!(cpsr.is_abort_disable());
    }

    #[test]
    fn test_seti() {
        let mut cpsr = CPSR::default();
        assert!(!cpsr.is_irq_disable());
        cpsr.seti(true);
        assert!(cpsr.is_irq_disable());
    }

    #[test]
    fn test_setf() {
        let mut cpsr = CPSR::default();
        assert!(!cpsr.is_fiq_disable());
        cpsr.setf(true);
        assert!(cpsr.is_fiq_disable());
    }

    #[test]
    fn test_set_state() {
        let mut cpsr = CPSR::default();
        assert!(cpsr.state() == CpuState::ARM);
        cpsr.set_state(CpuState::THUMB);
        assert!(cpsr.state() == CpuState::THUMB);
    }
}
