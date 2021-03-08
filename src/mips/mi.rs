#[derive(Default, Debug)]
pub struct MipsInterfaceRegisters {
  init_mode: u32,
  version: u32,
  interrupt: u32,
  interrupt_mask: u32,
}

impl MipsInterfaceRegisters {
  pub fn simulate_boot(&mut self) {
    self.version = 0x0101_0101;
  }

  pub fn get_by_address(&mut self, address: u32) -> u32 {
    match address {
      _ => panic!(
        "Mips Interface - Unimplemented Read Functionality - Address: 0x{:x}",
        address
      ),
    }
  }

  pub fn set_by_address(&mut self, address: u32, value: u32) -> () {
    match address {
      _ => panic!(
        "Mips Interface - Unimplemented Set Functionality - Address: 0x{:x} - Value: 0x{:x}",
        address, value
      ),
    };
  }
}
