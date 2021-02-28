mod configuration;
mod environment;

use configuration::init::InitializationConfiguration;
use environment::cli::parse_command_line_arguments;

fn main() {
    let mut initialization_configuration: InitializationConfiguration = Default::default();

    parse_command_line_arguments(&mut initialization_configuration);

    println!("{:?}", initialization_configuration)
}
