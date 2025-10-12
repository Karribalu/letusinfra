use crate::models::InfraConfig;
use crate::utils::constants;

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[clap(flatten)]
    pub options: Options,
}

#[derive(clap::Args, Debug)]
pub struct Options {
    #[clap(short = 'f', long = "filepath")]
    pub file_path: String,
}

pub fn execute(config: &Config) {
    println!("Executing validate command with config: {:?}", config);

    let file_path = &config.options.file_path;
    println!("File path is: {}", file_path);
    let is_valid = validate_file(file_path);
    if is_valid {
        println!("YAML file is valid.");
    } else {
        println!("YAML file is invalid.");
    }
}
pub fn validate_file(file_path: &str) -> bool {
    let content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read file: {}", err);
            return false;
        }
    };

    // Try to parse using the structured model
    match InfraConfig::from_yaml(&content) {
        Ok(config) => {
            println!("Successfully parsed YAML using InfraConfig model");
            return validate_infra_config(&config);
        }
        Err(err) => {
            eprintln!("Failed to parse YAML into InfraConfig: {}", err);
        }
    }

    true
}

pub fn validate_infra_config(config: &InfraConfig) -> bool {
    // Validate kind
    if !constants::SupportKind::is_valid(&config.kind) {
        eprintln!("Invalid kind: {}", config.kind);
        return false;
    }

    if !constants::SupportCloud::is_valid(&config.cloud) {
        eprintln!("Invalid cloud: {}", config.cloud);
        return false;
    }

    // Validate metadata
    if config.metadata.name.is_empty() {
        eprintln!("Metadata name cannot be empty");
        return false;
    }

    // Validate components
    if config.components.is_empty() {
        eprintln!("At least one component is required");
        return false;
    }

    for (idx, component) in config.components.iter().enumerate() {
        if component.component_type.is_empty() {
            eprintln!("Component {} has empty type", idx);
            return false;
        }
        if component.name.is_empty() {
            eprintln!("Component {} has empty name", idx);
            return false;
        }
    }

    println!("InfraConfig validation passed");
    true
}
