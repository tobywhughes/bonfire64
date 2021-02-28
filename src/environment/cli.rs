use crate::configuration::init::InitializationConfiguration;
use std::env;

pub fn parse_command_line_arguments(
  initialization_configuration: &mut InitializationConfiguration,
) {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  initialization_configuration.filename = filename.to_string();
}
