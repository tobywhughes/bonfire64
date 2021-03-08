use crate::configuration::init::InitializationConfiguration;
use crate::file::endian::{parse_endianness, Endianness};
use crate::file::parse::parse_extension;
use std::fs::{File, Metadata};

#[derive(Debug)]
pub struct BonfireFile {
  file: File,
  metadata: Metadata,
  extension: String,
  endianness: Endianness,
}

impl BonfireFile {
  pub fn new(initialization_configuration: &InitializationConfiguration) -> BonfireFile {
    let mut file = File::open(&initialization_configuration.filename).unwrap();
    let metadata = file.metadata().unwrap();
    let extension = parse_extension(&initialization_configuration.filename);

    let endianness: Endianness = parse_endianness(&mut file);

    let bonfire_file: BonfireFile = BonfireFile {
      file,
      metadata,
      endianness,
      extension: extension.to_string(),
    };

    debug!("{:?}", bonfire_file);

    bonfire_file
  }
}
