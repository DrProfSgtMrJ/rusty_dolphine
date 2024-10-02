use crate::register::{CPSRCell, Mode, RegisterCell, RegisterError, RegisterMap, RegisterSet, CPSR};
use super::{REGISTER_0, REGISTER_1, REGISTER_10, REGISTER_11, REGISTER_12, REGISTER_13, REGISTER_14,  REGISTER_15, REGISTER_2, REGISTER_3, REGISTER_4, REGISTER_5, REGISTER_6, REGISTER_7, REGISTER_8, REGISTER_9};

pub fn init_gba_registers() -> Result<RegisterMap, RegisterError> {
    let mut register_map_builder = RegisterMap::builder();

    let cpsr = CPSRCell::new(CPSR::default());
    let spsr = CPSRCell::new(CPSR::default());

    // General purpose registers: r0 - r14
    let mut system_register_builder = RegisterSet::builder();
    let r0 = RegisterCell::new(0);
    let r1 = RegisterCell::new(0);
    let r2 = RegisterCell::new(0);
    let r3 = RegisterCell::new(0);
    let r4 = RegisterCell::new(0);
    let r5 = RegisterCell::new(0);
    let r6 = RegisterCell::new(0);
    let r7 = RegisterCell::new(0);
    let r8 = RegisterCell::new(0);
    let r9 = RegisterCell::new(0);
    let r10 = RegisterCell::new(0);
    let r11 = RegisterCell::new(0);
    let r12 = RegisterCell::new(0);
    let r13 = RegisterCell::new(0);
    let r14 = RegisterCell::new(0);

    // r15 = PC
    let r15 = RegisterCell::new(0);

    let system_registers = system_register_builder
        .with_register(REGISTER_0, r0.clone())?
        .with_register(REGISTER_1, r1.clone())?
        .with_register(REGISTER_2, r2.clone())?
        .with_register(REGISTER_3, r3.clone())?
        .with_register(REGISTER_4, r4.clone())?
        .with_register(REGISTER_5, r5.clone())?
        .with_register(REGISTER_6, r6.clone())?
        .with_register(REGISTER_7, r7.clone())?
        .with_register(REGISTER_8, r8.clone())?
        .with_register(REGISTER_9, r9.clone())?
        .with_register(REGISTER_10, r10.clone())?
        .with_register(REGISTER_11, r11.clone())?
        .with_register(REGISTER_12, r12.clone())?
        .with_register(REGISTER_13, r13)?
        .with_register(REGISTER_14, r14)?
        .with_register(REGISTER_15, r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr)?
        .build();

    // FIQ registers: r8 - r14
    let mut fiq_register_builder = RegisterSet::builder();
    let spsr_fiq = CPSRCell::new(CPSR::default());
    let r8_fiq = RegisterCell::new(0);
    let r9_fiq = RegisterCell::new(0);
    let r10_fiq = RegisterCell::new(0);
    let r11_fiq = RegisterCell::new(0);
    let r12_fiq = RegisterCell::new(0);
    let r13_fiq = RegisterCell::new(0);
    let r14_fiq = RegisterCell::new(0);

    let fiq_registers = fiq_register_builder
        .with_register(REGISTER_0, r0.clone())?
        .with_register(REGISTER_1, r1.clone())?
        .with_register(REGISTER_2, r2.clone())?
        .with_register(REGISTER_3, r3.clone())?
        .with_register(REGISTER_4, r4.clone())?
        .with_register(REGISTER_5, r5.clone())?
        .with_register(REGISTER_6, r6.clone())?
        .with_register(REGISTER_7, r7.clone())?
        .with_register(REGISTER_8, r8_fiq)?
        .with_register(REGISTER_9, r9_fiq)?
        .with_register(REGISTER_10, r10_fiq)?
        .with_register(REGISTER_11, r11_fiq)?
        .with_register(REGISTER_12, r12_fiq)?
        .with_register(REGISTER_13, r13_fiq)?
        .with_register(REGISTER_14, r14_fiq)?
        .with_register(REGISTER_15, r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr_fiq)?
        .build();

    // Supervisor registers: r13-r14
    let mut supervisor_register_builder = RegisterSet::builder();
    let spsr_svc = CPSRCell::new(CPSR::default());
    let r13_svc = RegisterCell::new(0);
    let r14_svc = RegisterCell::new(0);
    let supervisor_registers = supervisor_register_builder
        .with_register(REGISTER_0, r0.clone())?
        .with_register(REGISTER_1,r1.clone())?
        .with_register(REGISTER_2, r2.clone())?
        .with_register(REGISTER_3, r3.clone())?
        .with_register(REGISTER_4, r4.clone())?
        .with_register(REGISTER_5, r5.clone())?
        .with_register(REGISTER_6, r6.clone())?
        .with_register(REGISTER_7, r7.clone())?
        .with_register(REGISTER_8, r8.clone())?
        .with_register(REGISTER_9, r9.clone())?
        .with_register(REGISTER_10, r10.clone())?
        .with_register(REGISTER_11, r11.clone())?
        .with_register(REGISTER_12, r12.clone())?
        .with_register(REGISTER_13, r13_svc)?
        .with_register(REGISTER_14, r14_svc)?
        .with_register(REGISTER_15, r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr_svc)?
        .build();

    // Abort registers: r13-r14
    let mut abort_register_builder = RegisterSet::builder();
    let spsr_abt = CPSRCell::new(CPSR::default());
    let r13_abt = RegisterCell::new(0);
    let r14_abt = RegisterCell::new(0);
    let abort_registers = abort_register_builder
        .with_register(REGISTER_0, r0.clone())?
        .with_register(REGISTER_1, r1.clone())?
        .with_register(REGISTER_2, r2.clone())?
        .with_register(REGISTER_3, r3.clone())?
        .with_register(REGISTER_4, r4.clone())?
        .with_register(REGISTER_5, r5.clone())?
        .with_register(REGISTER_6, r6.clone())?
        .with_register(REGISTER_7, r7.clone())?
        .with_register(REGISTER_8, r8.clone())?
        .with_register(REGISTER_9, r9.clone())?
        .with_register(REGISTER_10, r10.clone())?
        .with_register(REGISTER_11, r11.clone())?
        .with_register(REGISTER_12, r12.clone())?
        .with_register(REGISTER_13, r13_abt)?
        .with_register(REGISTER_14, r14_abt)?
        .with_register(REGISTER_15, r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr_abt)?
        .build();

    let mut irq_register_builder = RegisterSet::builder();
    let spsr_irq = CPSRCell::new(CPSR::default());
    let r13_irq = RegisterCell::new(0);
    let r14_irq = RegisterCell::new(0);
    let irq_registers = irq_register_builder
        .with_register(REGISTER_0, r0.clone())?
        .with_register(REGISTER_1, r1.clone())?
        .with_register(REGISTER_2, r2.clone())?
        .with_register(REGISTER_3, r3.clone())?
        .with_register(REGISTER_4, r4.clone())?
        .with_register(REGISTER_5, r5.clone())?
        .with_register(REGISTER_6, r6.clone())?
        .with_register(REGISTER_7, r7.clone())?
        .with_register(REGISTER_8, r8.clone())?
        .with_register(REGISTER_9, r9.clone())?
        .with_register(REGISTER_10, r10.clone())?
        .with_register(REGISTER_11, r11.clone())?
        .with_register(REGISTER_12, r12.clone())?
        .with_register(REGISTER_13, r13_irq)?
        .with_register(REGISTER_14, r14_irq)?
        .with_register(REGISTER_15, r15.clone())?
        .with_cpsr(cpsr.clone())?
        .with_spsr(spsr_irq)?
        .build();


    let mut und_register_builder = RegisterSet::builder();
    let spsr_und = CPSRCell::new(CPSR::default());
    let r13_und = RegisterCell::new(0);
    let r14_und = RegisterCell::new(0);
    let und_registers = und_register_builder
        .with_register(REGISTER_0, r0.clone())?
        .with_register(REGISTER_1, r1.clone())?
        .with_register(REGISTER_2, r2.clone())?
        .with_register(REGISTER_3, r3.clone())?
        .with_register(REGISTER_4, r4.clone())?
        .with_register(REGISTER_5, r5.clone())?
        .with_register(REGISTER_6, r6.clone())?
        .with_register(REGISTER_7, r7.clone())?
        .with_register(REGISTER_8, r8.clone())?
        .with_register(REGISTER_9, r9.clone())?
        .with_register(REGISTER_10, r10.clone())?
        .with_register(REGISTER_11, r11.clone())?
        .with_register(REGISTER_12, r12.clone())?
        .with_register(REGISTER_13, r13_und)?
        .with_register(REGISTER_14, r14_und)?
        .with_register(REGISTER_15, r15.clone())?
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

    use crate::register::{read_register_map, write_register_map};

    use super::*;

    #[test]
    fn test_gba_register_map() {

        match init_gba_registers() {
            Ok(mut register_map) => {
                let initial_system_value = read_register_map(&register_map, Mode::SYSTEM, REGISTER_15).unwrap();
                assert_eq!(initial_system_value, 0);

                let expected_value = 500;
                write_register_map(&mut register_map, Mode::SYSTEM, REGISTER_15, expected_value).unwrap();

                let fiq_value = read_register_map(&register_map, Mode::FIQ, REGISTER_15).unwrap();
                assert_eq!(fiq_value, expected_value);

                let other_value = 1000;
                write_register_map(&mut register_map, Mode::ABORT, REGISTER_15, other_value).unwrap();

                let system_value = read_register_map(&register_map, Mode::SYSTEM, REGISTER_15).unwrap();
                assert_eq!(system_value, other_value);

                let svc_value = read_register_map(&register_map, Mode::SUPERVISOR, REGISTER_15).unwrap();
                assert_eq!(svc_value, other_value);

                let abt_value = read_register_map(&register_map, Mode::ABORT, REGISTER_15).unwrap();
                assert_eq!(abt_value, other_value);

                let irq_value = read_register_map(&register_map, Mode::IRQ, REGISTER_15).unwrap();
                assert_eq!(irq_value, other_value);

                let und_value = read_register_map(&register_map, Mode::UNDEFINED, REGISTER_15).unwrap();
                assert_eq!(und_value, other_value);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false);}
        }

    }
}