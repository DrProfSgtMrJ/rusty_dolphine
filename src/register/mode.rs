#[derive(Debug, strum_macros::Display, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    SYSTEM,
    USER,
    FIQ,
    SUPERVISOR,
    ABORT,
    IRQ,
    UNDEFINED
}
