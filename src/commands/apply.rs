use aws_config::{BehaviorVersion, Region};

use crate::{
    aws::ec2::ec2_instance::EC2Instance, commands::validate::validate_file, models::InfraConfig,
    utils::constants::TEMPLATES_DIR,
};

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

pub async fn execute(config: &Config) {
    println!("Executing plan command with config: {:?}", config);

    let file_path = &config.options.file_path;
    println!("File path is: {}", file_path);
    let is_valid = validate_file(file_path);

    let content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read file: {}", err);
            return;
        }
    };

    // Try to parse using the structured model
    match InfraConfig::from_yaml(&content) {
        Ok(config) => {
            println!("Successfully parsed YAML using InfraConfig model");
            create_components(&config.metadata.name, &config.region, &config.components).await;
        }
        Err(err) => {
            eprintln!("Failed to parse YAML into InfraConfig: {}", err);
        }
    }
}

async fn create_components(name: &str, region: &str, components: &[crate::models::Component]) {
    for component in components {
        match component.component_type.as_str() {
            "EC2Instance" => {
                // Create EC2 instance Terraform code
                match create_ec2_instance(name, region, component).await {
                    Ok(instance) => {
                        println!("Successfully created EC2 instance: {:?}", instance);
                    }
                    Err(err) => {
                        eprintln!("Failed to create EC2 instance: {}", err);
                    }
                }
            }
            _ => {
                eprintln!("Unsupported component type: {}", component.component_type);
            }
        }
    }
}

async fn create_ec2_instance(
    deployment_name: &str,
    region: &str,
    component: &crate::models::Component,
) -> Result<aws_sdk_ec2::types::Instance, crate::aws::ec2::ec2_instance::EC2Error> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .profile_name("default")
        .region(Region::new(region.to_string()))
        .load()
        .await;
    let ec2_instance = EC2Instance::from_config(&config);
    let instance_opts = EC2Instance::opts_from_yaml(&component.properties)?;
    ec2_instance.create_instance(&instance_opts).await
}
