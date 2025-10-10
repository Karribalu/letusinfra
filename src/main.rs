use clap::Parser;

pub mod commands {
    pub mod validate;
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Config {
    Validate(commands::validate::Config),
}

fn main() {
    let args = Config::parse();

    match args {
        Config::Validate(validate_config) => {
            println!("Validate command called with config: {:?}", validate_config);
            commands::validate::execute(&validate_config);
        }
    }
}
