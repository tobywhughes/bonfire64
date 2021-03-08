use crate::cpu::cpu::CPU;
use crate::BonfireFile;
use crate::InitializationConfiguration;

#[derive(Debug)]
pub struct UltraSystem {
  file: BonfireFile,
  cpu: CPU,
}

impl UltraSystem {
  pub fn new(initialization_configuration: &InitializationConfiguration) -> UltraSystem {
    let file: BonfireFile = BonfireFile::new(&initialization_configuration);
    let cpu: CPU = Default::default();

    UltraSystem { file, cpu }
  }
}
