use log::{debug, error, info};
use std::env;
use std::fs::read_to_string;
use std::io;

enum InputStatus {
    Continue,
    Quit
}

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

    loop {
        let mut input = String::new();
        let result = io::stdin().read_line(&mut input);
        match result {
            Ok(_n) => {
                // trim newline
                if input.ends_with("\n") {
                    input.pop();
                    if input.ends_with("\r") {
                        input.pop();
                    }
                }
                if input.len() > 0 {
                    let outcome = handle_input(&input);
                    match outcome {
                        InputStatus::Quit => break,
                        InputStatus::Continue => ()
                    }
                }
            },
            Err(_err) => return Err("Failed to read from input")
        }
    }
    Ok(())
}

fn run_from_file(filename: String) -> Result<(), String> {
    info!("Running from file: {}", filename);

    let result = read_to_string(filename);
    match result {
        Ok(data) => {
            let lines: Vec<String> = data.lines().map(String::from).collect();
            for line in &lines {
                let outcome = handle_input(&line);
                match outcome {
                    InputStatus::Quit => break,
                    InputStatus::Continue => (),
                } 
            }
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}

fn handle_input(input: &String) -> InputStatus {
    debug!("Processing: {input}");

    if input == "quit" {
        return InputStatus::Quit;
    }

    return InputStatus::Continue;
}
