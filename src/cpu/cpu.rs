use crate::cpu::gpr::GeneralPurposeRegisters;

#[derive(Default, Debug)]
pub struct CPU {
  general_purpose_registers: GeneralPurposeRegisters,
}
