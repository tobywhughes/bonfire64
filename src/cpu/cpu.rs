use crate::cpu::cop1::Coprocessor0Registers;
use crate::cpu::gpr::GeneralPurposeRegisters;

#[derive(Default, Debug)]
pub struct CPU {
  pub general_purpose_registers: GeneralPurposeRegisters,
  pub coprocessor_0_registers: Coprocessor0Registers,
}

impl CPU {
  pub fn simulate_boot(&mut self) {
    self.general_purpose_registers.simulate_boot();
    self.coprocessor_0_registers.simulate_boot();
  }
}
