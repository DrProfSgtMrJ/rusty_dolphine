use std::{cell::RefCell, rc::Rc};

use super::{Mode, RegisterError, RegisterMap, RegisterSet, CPSR, REGISTER_0, REGISTER_1, REGISTER_10, REGISTER_10_FIQ, REGISTER_11, REGISTER_11_FIQ, REGISTER_12, REGISTER_12_FIQ, REGISTER_13, REGISTER_13_ABT, REGISTER_13_FIQ, REGISTER_13_IRQ, REGISTER_13_SVC, REGISTER_13_UND, REGISTER_14, REGISTER_14_ABT, REGISTER_14_FIQ, REGISTER_14_IRQ, REGISTER_14_SVC, REGISTER_14_UND, REGISTER_15, REGISTER_2, REGISTER_3, REGISTER_4, REGISTER_5, REGISTER_6, REGISTER_7, REGISTER_8, REGISTER_8_FIQ, REGISTER_9, REGISTER_9_FIQ};

pub fn init_gba_registers() -> Result<RegisterMap, RegisterError> {
    let mut register_map_builder = RegisterMap::builder();

    let cpsr = Rc::new(RefCell::new(CPSR::default()));
    let spsr = Rc::new(RefCell::new(CPSR::default()));
    // General purpose registers: r0 - r14
    let mut system_register_builder = RegisterSet::builder();
    let r0 = Rc::new(RefCell::new(0));
    let r1 = Rc::new(RefCell::new(0));
    let r2 = Rc::new(RefCell::new(0));
    let r3 = Rc::new(RefCell::new(0));
    let r4 = Rc::new(RefCell::new(0));
    let r5 = Rc::new(RefCell::new(0));
    let r6 = Rc::new(RefCell::new(0));
    let r7 = Rc::new(RefCell::new(0));
    let r8 = Rc::new(RefCell::new(0));
    let r9 = Rc::new(RefCell::new(0));
    let r10 = Rc::new(RefCell::new(0));
    let r11 = Rc::new(RefCell::new(0));
    let r12 = Rc::new(RefCell::new(0));
    let r13 = Rc::new(RefCell::new(0));
    let r14 = Rc::new(RefCell::new(0));

    // r15 = PC
    let r15 = Rc::new(RefCell::new(0));

    let system_registers = system_register_builder
        .with_register(REGISTER_0.to_string(), r0.clone())?
        .with_register(REGISTER_1.to_string(), r1.clone())?
        .with_register(REGISTER_2.to_string(), r2.clone())?
        .with_register(REGISTER_3.to_string(), r3.clone())?
        .with_register(REGISTER_4.to_string(), r4.clone())?
        .with_register(REGISTER_5.to_string(), r5.clone())?
        .with_register(REGISTER_6.to_string(), r6.clone())?
        .with_register(REGISTER_7.to_string(), r7.clone())?
        .with_register(REGISTER_8.to_string(), r8.clone())?
        .with_register(REGISTER_9.to_string(), r9.clone())?
        .with_register(REGISTER_10.to_string(), r10.clone())?
        .with_register(REGISTER_11.to_string(), r11.clone())?
        .with_register(REGISTER_12.to_string(), r12.clone())?
        .with_register(REGISTER_13.to_string(), r13)?
        .with_register(REGISTER_14.to_string(), r14)?
        .with_register(REGISTER_15.to_string(), r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr)?
        .build();

    // FIQ registers: r8 - r14
    let mut fiq_register_builder = RegisterSet::builder();
    let spsr_fiq = Rc::new(RefCell::new(CPSR::default()));
    let r8_fiq = Rc::new(RefCell::new(0));
    let r9_fiq = Rc::new(RefCell::new(0));
    let r10_fiq = Rc::new(RefCell::new(0));
    let r11_fiq = Rc::new(RefCell::new(0));
    let r12_fiq = Rc::new(RefCell::new(0));
    let r13_fiq = Rc::new(RefCell::new(0));
    let r14_fiq = Rc::new(RefCell::new(0));

    let fiq_registers = fiq_register_builder
        .with_register(REGISTER_0.to_string(), r0.clone())?
        .with_register(REGISTER_1.to_string(), r1.clone())?
        .with_register(REGISTER_2.to_string(), r2.clone())?
        .with_register(REGISTER_3.to_string(), r3.clone())?
        .with_register(REGISTER_4.to_string(), r4.clone())?
        .with_register(REGISTER_5.to_string(), r5.clone())?
        .with_register(REGISTER_6.to_string(), r6.clone())?
        .with_register(REGISTER_7.to_string(), r7.clone())?
        .with_register(REGISTER_8_FIQ.to_string(), r8_fiq)?
        .with_register(REGISTER_9_FIQ.to_string(), r9_fiq)?
        .with_register(REGISTER_10_FIQ.to_string(), r10_fiq)?
        .with_register(REGISTER_11_FIQ.to_string(), r11_fiq)?
        .with_register(REGISTER_12_FIQ.to_string(), r12_fiq)?
        .with_register(REGISTER_13_FIQ.to_string(), r13_fiq)?
        .with_register(REGISTER_14_FIQ.to_string(), r14_fiq)?
        .with_register(REGISTER_15.to_string(), r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr_fiq)?
        .build();

    // Supervisor registers: r13-r14
    let mut supervisor_register_builder = RegisterSet::builder();
    let spsr_svc = Rc::new(RefCell::new(CPSR::default()));
    let r13_svc = Rc::new(RefCell::new(0));
    let r14_svc = Rc::new(RefCell::new(0));
    let supervisor_registers = supervisor_register_builder
        .with_register(REGISTER_0.to_string(), r0.clone())?
        .with_register(REGISTER_1.to_string(), r1.clone())?
        .with_register(REGISTER_2.to_string(), r2.clone())?
        .with_register(REGISTER_3.to_string(), r3.clone())?
        .with_register(REGISTER_4.to_string(), r4.clone())?
        .with_register(REGISTER_5.to_string(), r5.clone())?
        .with_register(REGISTER_6.to_string(), r6.clone())?
        .with_register(REGISTER_7.to_string(), r7.clone())?
        .with_register(REGISTER_8.to_string(), r8.clone())?
        .with_register(REGISTER_9.to_string(), r9.clone())?
        .with_register(REGISTER_10.to_string(), r10.clone())?
        .with_register(REGISTER_11.to_string(), r11.clone())?
        .with_register(REGISTER_12.to_string(), r12.clone())?
        .with_register(REGISTER_13_SVC.to_string(), r13_svc)?
        .with_register(REGISTER_14_SVC.to_string(), r14_svc)?
        .with_register(REGISTER_15.to_string(), r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr_svc)?
        .build();

    // Abort registers: r13-r14
    let mut abort_register_builder = RegisterSet::builder();
    let spsr_abt = Rc::new(RefCell::new(CPSR::default()));
    let r13_abt = Rc::new(RefCell::new(0));
    let r14_abt = Rc::new(RefCell::new(0));
    let abort_registers = abort_register_builder
        .with_register(REGISTER_0.to_string(), r0.clone())?
        .with_register(REGISTER_1.to_string(), r1.clone())?
        .with_register(REGISTER_2.to_string(), r2.clone())?
        .with_register(REGISTER_3.to_string(), r3.clone())?
        .with_register(REGISTER_4.to_string(), r4.clone())?
        .with_register(REGISTER_5.to_string(), r5.clone())?
        .with_register(REGISTER_6.to_string(), r6.clone())?
        .with_register(REGISTER_7.to_string(), r7.clone())?
        .with_register(REGISTER_8.to_string(), r8.clone())?
        .with_register(REGISTER_9.to_string(), r9.clone())?
        .with_register(REGISTER_10.to_string(), r10.clone())?
        .with_register(REGISTER_11.to_string(), r11.clone())?
        .with_register(REGISTER_12.to_string(), r12.clone())?
        .with_register(REGISTER_13_ABT.to_string(), r13_abt)?
        .with_register(REGISTER_14_ABT.to_string(), r14_abt)?
        .with_register(REGISTER_15.to_string(), r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr_abt)?
        .build();

    let mut irq_register_builder = RegisterSet::builder();
    let spsr_irq = Rc::new(RefCell::new(CPSR::default()));
    let r13_irq = Rc::new(RefCell::new(0));
    let r14_irq = Rc::new(RefCell::new(0));
    let irq_registers = irq_register_builder
        .with_register(REGISTER_0.to_string(), r0.clone())?
        .with_register(REGISTER_1.to_string(), r1.clone())?
        .with_register(REGISTER_2.to_string(), r2.clone())?
        .with_register(REGISTER_3.to_string(), r3.clone())?
        .with_register(REGISTER_4.to_string(), r4.clone())?
        .with_register(REGISTER_5.to_string(), r5.clone())?
        .with_register(REGISTER_6.to_string(), r6.clone())?
        .with_register(REGISTER_7.to_string(), r7.clone())?
        .with_register(REGISTER_8.to_string(), r8.clone())?
        .with_register(REGISTER_9.to_string(), r9.clone())?
        .with_register(REGISTER_10.to_string(), r10.clone())?
        .with_register(REGISTER_11.to_string(), r11.clone())?
        .with_register(REGISTER_12.to_string(), r12.clone())?
        .with_register(REGISTER_13_IRQ.to_string(), r13_irq)?
        .with_register(REGISTER_14_IRQ.to_string(), r14_irq)?
        .with_register(REGISTER_15.to_string(), r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr_irq)?
        .build();


    let mut und_register_builder = RegisterSet::builder();
    let spsr_und = Rc::new(RefCell::new(CPSR::default()));
    let r13_und = Rc::new(RefCell::new(0));
    let r14_und = Rc::new(RefCell::new(0));
    let und_registers = und_register_builder
        .with_register(REGISTER_0.to_string(), r0.clone())?
        .with_register(REGISTER_1.to_string(), r1.clone())?
        .with_register(REGISTER_2.to_string(), r2.clone())?
        .with_register(REGISTER_3.to_string(), r3.clone())?
        .with_register(REGISTER_4.to_string(), r4.clone())?
        .with_register(REGISTER_5.to_string(), r5.clone())?
        .with_register(REGISTER_6.to_string(), r6.clone())?
        .with_register(REGISTER_7.to_string(), r7.clone())?
        .with_register(REGISTER_8.to_string(), r8.clone())?
        .with_register(REGISTER_9.to_string(), r9.clone())?
        .with_register(REGISTER_10.to_string(), r10.clone())?
        .with_register(REGISTER_11.to_string(), r11.clone())?
        .with_register(REGISTER_12.to_string(), r12.clone())?
        .with_register(REGISTER_13_UND.to_string(), r13_und)?
        .with_register(REGISTER_14_UND.to_string(), r14_und)?
        .with_register(REGISTER_15.to_string(), r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr_und)?
        .build();


    let register_map: RegisterMap = register_map_builder
        .with_registers(Mode::SYSTEM, system_registers)?
        .with_registers(Mode::FIQ, fiq_registers)?
        .with_registers(Mode::SUPERVISOR, supervisor_registers)?
        .with_registers(Mode::ABORT, abort_registers)?
        .with_registers(Mode::IRQ, irq_registers)?
        .with_registers(Mode::UNDEFINED, und_registers)?
        .build();

    Ok(register_map)
}

#[cfg(test)]
mod tests {

    use crate::register::{read_register, write_register};

    use super::*;

    #[test]
    fn test_gba_register_map() {

        match init_gba_registers() {
            Ok(mut register_map) => {
                let initial_system_value = read_register(&register_map, Mode::SYSTEM, REGISTER_15).unwrap();
                assert_eq!(initial_system_value, 0);

                let expected_value = 500;
                write_register(&mut register_map, Mode::SYSTEM, REGISTER_15, expected_value).unwrap();

                let fiq_value = read_register(&register_map, Mode::FIQ, REGISTER_15).unwrap();
                assert_eq!(fiq_value, expected_value);

                let other_value = 1000;
                write_register(&mut register_map, Mode::ABORT, REGISTER_15, other_value).unwrap();

                let system_value = read_register(&register_map, Mode::SYSTEM, REGISTER_15).unwrap();
                assert_eq!(system_value, other_value);

                let svc_value = read_register(&register_map, Mode::SUPERVISOR, REGISTER_15).unwrap();
                assert_eq!(svc_value, other_value);

                let abt_value = read_register(&register_map, Mode::ABORT, REGISTER_15).unwrap();
                assert_eq!(abt_value, other_value);

                let irq_value = read_register(&register_map, Mode::IRQ, REGISTER_15).unwrap();
                assert_eq!(irq_value, other_value);

                let und_value = read_register(&register_map, Mode::UNDEFINED, REGISTER_15).unwrap();
                assert_eq!(und_value, other_value);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false);}
        }

    }
}