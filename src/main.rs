use clap::Parser;
use std::io::Error;
use tracing::info;
use tracing_subscriber;
mod aws;
mod commands;
mod models;
mod tests;
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    info!("Starting Infrastructure Management CLI Tool");
    let args = Config::parse();
    match args {
        Config::Validate(validate_config) => {
            info!("Validate command called with config: {:?}", validate_config);
            commands::validate::execute(&validate_config);
        }
        Config::Plan(plan_config) => {
            info!("Plan command called with config: {:?}", plan_config);
            commands::plan::execute(&plan_config);
        }
        Config::Apply(apply_config) => {
            info!("Apply command called with config: {:?}", apply_config);
            commands::apply::execute(&apply_config).await;
        }
        Config::Destroy(destroy_config) => {
            info!("Destroy command called with config: {:?}", destroy_config);
            commands::destroy::execute(&destroy_config);
        }
    }

    Ok(())
}
