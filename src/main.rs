mod configuration;
mod environment;
mod file;

use configuration::init::InitializationConfiguration;
use environment::cli::parse_command_line_arguments;
use file::read::BonfireFile;

fn main() {
    let mut initialization_configuration: InitializationConfiguration = Default::default();

    parse_command_line_arguments(&mut initialization_configuration);

    let bonfire_file: BonfireFile = BonfireFile::new(&initialization_configuration);

    println!("{:?}", bonfire_file)
}
