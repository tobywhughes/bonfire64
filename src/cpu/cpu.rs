use crate::cpu::gpr::GeneralPurposeRegisters;

#[derive(Default, Debug)]
pub struct CPU {
  pub general_purpose_registers: GeneralPurposeRegisters,
}
