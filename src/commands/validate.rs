use serde_core::de::Deserialize;
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
    let content = std::fs::read_to_string(file_path).ok().unwrap();
    let deserializer = serde_yaml::Deserializer::from_str(&content);

    for document in deserializer {
        let value: serde_yaml::Value = match serde_yaml::Value::deserialize(document) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Failed to deserialize YAML document: {}", err);
                return false;
            }
        };
        println!("Parsed YAML document: {:?}", value);
    }
    true
}
