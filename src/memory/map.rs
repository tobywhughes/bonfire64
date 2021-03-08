#[derive(Debug, PartialEq)]
pub enum PhysicalMap {
  RSPDataMemory,      // Signal Processing Data Memory (4KB)
  CartridgeDomain1_2, //Rom Mapping
  NotMapped,          // Development Resource - Have not yet defined mapping
}

impl PhysicalMap {
  pub fn from_u32(address: u32) -> PhysicalMap {
    match address {
      0x1000_0000..=0x1FBF_FFFF => PhysicalMap::CartridgeDomain1_2,
      _ => PhysicalMap::NotMapped,
    }
  }
}

#[cfg(test)]
#[path = "./map.test.rs"]
mod map_test;
