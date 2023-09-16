use std::env;

struct Settings {
    debug: bool,
    input_file: Option<String>,
}

fn main() {
    display_details();

    let mut settings = Settings { debug: false, input_file: None };

    process_command_line(env::args().collect(), &mut settings);

    println!("Debug is set to {}", settings.debug);
    println!("Input is set to {:?}", settings.input_file);
}

fn display_details() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("{name} {version}");
}

fn process_command_line(args: Vec<String>, settings: &mut Settings) {
    println!("Args received: {:?}", args);

    let mut index = 1;
    while index < args.len() {
        println!("  {}", args[index]);

        if args[index] == "-debug" {
            settings.debug = true;
        }
        else if args[index] == "-input" {
            if index + 1 < args.len() {
                index += 1;
                settings.input_file = Some(args[index].to_string());
            }
        }

        index += 1;
    }
}
