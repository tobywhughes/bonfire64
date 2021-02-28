use crate::configuration::init::InitializationConfiguration;
use crate::file::parse::parse_extension;
use std::fs::{File, Metadata};

#[derive(Debug)]
pub struct BonfireFile {
  file: File,
  metadata: Metadata,
  extension: String,
}

impl BonfireFile {
  pub fn new(initialization_configuration: &InitializationConfiguration) -> BonfireFile {
    let file = File::open(&initialization_configuration.filename).unwrap();
    let metadata = file.metadata().unwrap();
    let extension = parse_extension(&initialization_configuration.filename);

    BonfireFile {
      file,
      metadata,
      extension: extension.to_string(),
    }
  }
}
