use core::fmt;
use std::ops::BitAnd;

use bitflags::{bitflags, Flags};

use crate::cpu::CpuState;

use super::{NormalRegister, Register, RegisterError};

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

    pub fn set_N(&mut self, value: bool) {
        self.set(CPSR::N, value);
    }

    pub fn is_zero(&self) -> bool {
        self.contains(CPSR::Z)
    }

    pub fn set_Z(&mut self, value: bool) {
        self.set(CPSR::Z, value);
    }

    pub fn is_carry(&self) -> bool {
        self.contains(CPSR::C)
    }

    pub fn set_C(&mut self, value: bool) {
        self.set(CPSR::C, value);
    }

    pub fn is_overflow(&self) -> bool {
        self.contains(CPSR::V)
    }

    pub fn set_V(&mut self, value: bool) {
        self.set(CPSR::V, value);
    }

    pub fn is_sticky_overflow(&self) -> bool {
        self.contains(CPSR::Q)
    }

    pub fn set_Q(&mut self, value: bool) {
        self.set(CPSR::Q, value);
    }

    pub fn is_jazelle(&self) -> bool {
        self.contains(CPSR::J)
    }

    pub fn set_J(&mut self, value: bool) {
        self.set(CPSR::J, value);
    }

    pub fn is_big_endian(&self) -> bool {
        self.contains(CPSR::E)
    }

    pub fn set_E(&mut self, value: bool) {
        self.set(CPSR::E, value);
    }

    pub fn is_abort_disable(&self) -> bool {
        self.contains(CPSR::A)
    }

    pub fn set_A(&mut self, value: bool) {
        self.set(CPSR::A, value);
    }

    pub fn is_irq_disable(&self) -> bool {
        self.contains(CPSR::I)
    }

    pub fn set_I(&mut self, value: bool) {
        self.set(CPSR::I, value);
    }

    pub fn is_fiq_disable(&self) -> bool {
        self.contains(CPSR::F)
    }

    pub fn set_F(&mut self, value: bool) {
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

impl Register for CPSR {
    fn read(&self, buf: &mut u32) -> Result<(), RegisterError> {
        *buf = self.bits();
        Ok(())
    }

    fn write(&mut self, buf: &u32) -> Result<(), RegisterError> {
        match CPSR::from_bits(*buf) {
            Some(cpsr) => {
                *self = cpsr; 
                return Ok(());
            },
            None => {
                return Err(RegisterError::InvalidCPSR(*buf))
            }
        }
    }

    fn move_to(&self, other: &mut dyn Register) -> Result<(), RegisterError> {
        let mut buf = 0;
        self.read(&mut buf)?;
        other.write(&buf)
    }

    fn clear(&mut self) -> Result<(), RegisterError> {
        self.write(&0)
    }
}
