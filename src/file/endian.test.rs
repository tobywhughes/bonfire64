use super::*;

#[test]
fn test_endianness_validity_is_known_doesnt_panic() {
  let endianness: Endianness = Endianness::BigEndian;
  let initial_byte = 0x80;

  check_endianness_validity(&endianness, initial_byte);
}

#[test]
#[should_panic]
fn test_endianness_validity_not_know_panics() {
  let endianness: Endianness = Endianness::Unknown;
  let initial_byte = 0x80;

  check_endianness_validity(&endianness, initial_byte);
}

#[test]
fn test_get_endianness_returns_endianness_with_valid_initial_byte() {
  let initial_byte = 0x80;
  let endianness: Endianness = get_endianness_by_byte(initial_byte);

  assert_eq!(endianness, Endianness::BigEndian);
}

#[test]
fn test_get_endianness_returns_unknown_for_invalid_initial_byte() {
  let initial_byte = 0x00;
  let endianness: Endianness = get_endianness_by_byte(initial_byte);

  assert_eq!(endianness, Endianness::Unknown);
}
