use crate::BonfireFile;
use crate::InitializationConfiguration;

pub struct UltraSystem {
  file: BonfireFile,
}

impl UltraSystem {
  pub fn new(initialization_configuration: &InitializationConfiguration) -> UltraSystem {
    let bonfire_file: BonfireFile = BonfireFile::new(&initialization_configuration);

    UltraSystem { file: bonfire_file }
  }
}
