use clap::Parser;
mod commands;
mod models;
mod utils;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
enum Config {
    Validate(commands::validate::Config),
    Plan(commands::plan::Config),
    Apply(commands::apply::Config),
    Destroy(commands::destroy::Config),
}

fn main() {
    let args = Config::parse();

    match args {
        Config::Validate(validate_config) => {
            println!("Validate command called with config: {:?}", validate_config);
            commands::validate::execute(&validate_config);
        }
        Config::Plan(plan_config) => {
            println!("Plan command called with config: {:?}", plan_config);
            commands::plan::execute(&plan_config);
        }
        Config::Apply(apply_config) => {
            println!("Apply command called with config: {:?}", apply_config);
            commands::apply::execute(&apply_config);
        }
        Config::Destroy(destroy_config) => {
            println!("Destroy command called with config: {:?}", destroy_config);
            commands::destroy::execute(&destroy_config);
        }
    }
}
