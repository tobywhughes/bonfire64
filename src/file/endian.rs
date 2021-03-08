use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum Endianness {
  BigEndian,
  LittleEndian,
  BigEndianByteSwapped,
  Unknown,
}

fn get_initial_byte_from_file(file: &mut File) -> u8 {
  let mut initial_byte_buffer = [0; 1];
  file.read(&mut initial_byte_buffer).unwrap();

  trace!("ENDIAN BYTE: 0x{:x}", initial_byte_buffer[0]);

  initial_byte_buffer[0]
}

fn get_endianness_by_byte(initial_byte: u8) -> Endianness {
  let endianness: Endianness = match initial_byte {
    0x80 => Endianness::BigEndian,
    0x37 => Endianness::BigEndianByteSwapped,
    0x40 => Endianness::LittleEndian,
    _ => Endianness::Unknown,
  };

  trace!("ENDIANNESS: {:?}", endianness);

  endianness
}

pub fn parse_endianness(file: &mut File) -> Endianness {
  let initial_byte = get_initial_byte_from_file(file);
  let endianness: Endianness = get_endianness_by_byte(initial_byte);

  check_endianness_validity(&endianness, initial_byte);

  endianness
}

fn check_endianness_validity(endianness: &Endianness, initial_byte: u8) {
  if *endianness == Endianness::Unknown {
    error!(
      "Failed to parse endianness of file. Initial Byte: 0x{:x}",
      initial_byte
    );
    panic!()
  } else {
    trace!("ENDIANNESS CHECK SUCCESS")
  }
}
