#[macro_use]
extern crate log;

use env_logger::Env;

mod configuration;
mod cpu;
mod environment;
mod file;
mod logger;
mod memory;
mod system;

use configuration::init::InitializationConfiguration;
use environment::cli::parse_command_line_arguments;
use file::read::BonfireFile;
use system::system::UltraSystem;

fn main() {
    initialize_logger();
    let mut initialization_configuration: InitializationConfiguration = Default::default();

    parse_command_line_arguments(&mut initialization_configuration);

    let mut ultra_system: UltraSystem = UltraSystem::new(&initialization_configuration);

    ultra_system.simulate_boot();

    info!("{:#?}", ultra_system);
}

fn initialize_logger() -> () {
    let log_env = Env::default().filter_or("BONFIRE_LOG_LEVEL", "debug");
    env_logger::init_from_env(log_env);
    trace!("LOGGER INITIALIZED")
}
