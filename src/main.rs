use log::{debug, error, info};
use std::env;
use std::fs::read_to_string;

struct Settings {
    log_level: log::Level,
    input_file: Option<String>,
}

fn main() {
    display_details();

    let result = process_command_line(env::args().collect());
    match result {
        Ok(settings) => {
            initialize_logging(settings.log_level);

            info!("Settings parsed successfully");
            debug!("Logging is set to {:?}", settings.log_level);
            debug!("Input is set to {:?}", settings.input_file);

            if settings.input_file != None {
                let filename = settings.input_file.unwrap();
                let result = run_from_file(filename.clone());
                match result {
                    Ok(_) => info!("Completed"),
                    Err(message) => {
                        error!("Failed to process input file: {:?}: {}", filename, message)
                    }
                }
            } else {
                let result = run_from_stdin();
                match result {
                    Ok(_) => info!("Complete"),
                    Err(message) => error!("Error running manual input: {}", message),
                }
            }
        }
        Err(message) => {
            initialize_logging(log::Level::Info);
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
        log_level: log::Level::Info,
        input_file: None,
    };

    debug!("Command line arguments:");
    while index < args.len() {
        debug!("  {}", args[index]);

        if args[index] == "--debug" || args[index] == "-d" {
            settings.log_level = log::Level::Debug;
        } else if args[index] == "--quiet" || args[index] == "-q" {
            settings.log_level = log::Level::Warn;
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

fn run_from_stdin() -> Result<(), &'static str> {
    info!("Running from standard input");

    Ok(())
}

fn run_from_file(filename: String) -> Result<(), String> {
    info!("Running from file: {}", filename);

    let result = read_to_string(filename);
    match result {
        Ok(data) => {
            let lines: Vec<String> = data.lines().map(String::from).collect();
            for line in &lines {
                handle_input(&line);
            }
            Ok(())
        },
        Err(err) => {
            Err( format!("Failed to read input file: {}", err.to_string().as_str()))
        }
    }
}

fn handle_input(input: &String) {
    debug!("Processing: {input}");
    
}