use log::{debug, error, info};
use std::env;

struct Settings {
    debug: bool,
    input_file: Option<String>,
}

fn main() {
    display_details();

    let result = process_command_line(env::args().collect());
    match result {
        Ok(settings) => {
            if settings.debug {
                initialize_logging(log::Level::Debug);
            } else {
                initialize_logging(log::Level::Info);
            }

            info!("Settings parsed successfully");
            debug!("Debug is set to {}", settings.debug);
            debug!("Input is set to {:?}", settings.input_file);
        }
        Err(message) => {
            initialize_logging(log::Level::Error);
            error!("Error parsing settings: {}", message)
        }
    }
}

fn initialize_logging(level: log::Level) {
    stderrlog::new()
        .verbosity(level)
        .module(module_path!())
        .init()
        .unwrap();
}

fn display_details() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("{name} {version}");
}

fn process_command_line(args: Vec<String>) -> Result<Settings, &'static str> {
    let mut index = 1;

    let mut settings = Settings {
        debug: false,
        input_file: None,
    };

    debug!("Command line arguments:");
    while index < args.len() {
        debug!("  {}", args[index]);

        if args[index] == "--debug" || args[index] == "-d" {
            settings.debug = true;
        } else if args[index] == "--input" || args[index] == "-i" {
            if index + 1 < args.len() {
                index += 1;
                settings.input_file = Some(args[index].to_string());
            } else {
                return Err("Missing input filename");
            }
        }

        index += 1;
    }

    Ok(settings)
}
