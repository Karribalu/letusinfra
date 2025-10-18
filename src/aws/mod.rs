pub mod credentials;
pub mod ec2;
pub mod internal;

#[derive(Debug, Clone)]
pub enum AWSClient {
    EC2Client(aws_sdk_ec2::Client),
}
