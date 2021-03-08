use crate::cpu::cop1::Coprocessor0Registers;
use crate::cpu::gpr::GeneralPurposeRegisters;

#[derive(Default, Debug)]
pub struct CPU {
  pub general_purpose_registers: GeneralPurposeRegisters,
  pub coprocessor_0_registers: Coprocessor0Registers,
}
