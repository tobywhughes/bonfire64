use crate::cpu::cpu::CPU;
use crate::mips::mi::MipsInterfaceRegisters;
use crate::BonfireFile;
use crate::InitializationConfiguration;

#[derive(Debug)]
pub struct UltraSystem {
  file: BonfireFile,
  cpu: CPU,
  mips_interface: MipsInterfaceRegisters,
}

impl UltraSystem {
  pub fn new(initialization_configuration: &InitializationConfiguration) -> UltraSystem {
    let file: BonfireFile = BonfireFile::new(&initialization_configuration);
    let cpu: CPU = Default::default();
    let mips_interface: MipsInterfaceRegisters = Default::default();

    UltraSystem {
      file,
      cpu,
      mips_interface,
    }
  }

  pub fn simulate_boot(&mut self) {
    self.cpu.simulate_boot();
    self.mips_interface.simulate_boot();
  }
}
