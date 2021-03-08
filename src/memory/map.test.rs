use super::*;

#[test]
fn test_physical_memory_map_from_u32_gets_valid_segment() {
  let address: u32 = 0x1000_0000;
  let expected_map: PhysicalMap = PhysicalMap::CartridgeDomain1_2;

  let result_map: PhysicalMap = PhysicalMap::from_u32(address);

  assert_eq!(expected_map, result_map);
}
