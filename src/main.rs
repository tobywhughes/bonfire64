mod configuration;
mod environment;
mod file;

use configuration::init::InitializationConfiguration;
use environment::cli::parse_command_line_arguments;
use file::parse::FileMetadata;

fn main() {
    let mut initialization_configuration: InitializationConfiguration = Default::default();

    parse_command_line_arguments(&mut initialization_configuration);

    let file_metadata = FileMetadata::new(&initialization_configuration.filename);

    println!("{:?}", file_metadata)
}
