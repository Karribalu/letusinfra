# AWS Credentials Testing Guide

This document provides comprehensive information about the AWS credentials testing implementation in the `letusinfra` project.

## Overview

The project includes extensive tests for reading and validating AWS credentials from environment variables. This ensures proper authentication with AWS services.

## Environment Variables

The following environment variables are used for AWS authentication:

### Required Variables

- **`AWS_ACCESS_KEY_ID`**: Your AWS access key identifier
- **`AWS_SECRET_ACCESS_KEY`**: Your AWS secret access key

### Optional Variables

- **`AWS_REGION`**: The AWS region for your resources (e.g., `us-west-2`)
- **`AWS_DEFAULT_REGION`**: Fallback region if `AWS_REGION` is not set
- **`AWS_SESSION_TOKEN`**: Temporary session token for AWS STS credentials

## Setting Up Credentials

### For Development

```bash
export AWS_ACCESS_KEY_ID="your-access-key-id"
export AWS_SECRET_ACCESS_KEY="your-secret-access-key"
export AWS_REGION="us-west-2"
```

### For Testing

```bash
# Set test credentials
export AWS_ACCESS_KEY_ID="test-access-key"
export AWS_SECRET_ACCESS_KEY="test-secret-key"
export AWS_REGION="us-west-2"
```

### Using AWS CLI Configuration

Alternatively, you can configure AWS CLI, and the SDK will automatically read from:

- `~/.aws/credentials`
- `~/.aws/config`

```bash
aws configure
```

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Specific Test Modules

```bash
# Test AWS credentials module
cargo test --lib aws::credentials

# Test EC2 instance with credentials
cargo test --lib aws::ec2::ec2_instance::tests

# Run integration tests
cargo test --test aws_credentials_test
```

### Run With Output

```bash
cargo test -- --nocapture
```

### Run Specific Test

```bash
cargo test test_read_aws_access_key_id_from_env
```

## Test Coverage

### Unit Tests (src/aws/credentials.rs)

The credentials module includes comprehensive tests for:

1. **Loading Credentials from Environment**

   - `test_from_env_with_all_credentials` - All credentials present
   - `test_from_env_with_minimal_credentials` - Only required credentials
   - `test_from_env_missing_access_key` - Missing access key error
   - `test_from_env_missing_secret_key` - Missing secret key error
   - `test_from_env_missing_both_keys` - Missing both keys error

2. **Environment Variable Detection**

   - `test_are_env_vars_set_true` - Both credentials set
   - `test_are_env_vars_set_false` - No credentials set
   - `test_are_env_vars_set_partial` - Only one credential set

3. **Region Handling**

   - `test_get_region_or_default_with_aws_region` - AWS_REGION set
   - `test_get_region_or_default_with_aws_default_region` - AWS_DEFAULT_REGION fallback
   - `test_get_region_or_default_fallback` - Default region
   - `test_get_region_or_default_prefers_aws_region` - Region precedence

4. **Validation**

   - `test_validate_valid_credentials` - Valid credentials pass
   - `test_validate_empty_access_key` - Empty access key fails
   - `test_validate_empty_secret_key` - Empty secret key fails

5. **Edge Cases**
   - `test_credentials_with_spaces` - Whitespace handling
   - `test_credentials_clone` - Clone trait
   - `test_session_token_handling` - Session token support
   - `test_multiple_sequential_loads` - Multiple credential loads

### Integration Tests (tests/aws_credentials_test.rs)

Integration tests cover:

1. **Basic Environment Variable Reading**

   - Reading AWS_ACCESS_KEY_ID
   - Reading AWS_SECRET_ACCESS_KEY
   - Reading AWS_REGION

2. **Credential Combinations**

   - All credentials present
   - Missing credentials
   - Partial credentials

3. **Region Fallback Behavior**

   - AWS_DEFAULT_REGION fallback
   - AWS_REGION precedence

4. **Special Cases**

   - Special characters in credentials
   - Case sensitivity
   - Empty string values
   - Whitespace handling
   - Multiple updates

5. **SDK Integration**
   - SDK config creation
   - Different regions
   - Credential validation logic

### EC2 Instance Tests (src/aws/ec2/ec2_instance.rs)

Tests for EC2 instance initialization:

1. **Environment Variable Tests**

   - `test_aws_access_key_id_environment_variable`
   - `test_aws_secret_access_key_environment_variable`
   - `test_aws_region_environment_variable`
   - `test_missing_aws_access_key_id`
   - `test_missing_aws_secret_access_key`
   - `test_both_credentials_set`

2. **EC2 Instance Creation**

   - `test_ec2_instance_new_with_client`
   - `test_ec2_instance_from_config`

3. **Credential Handling**

   - `test_credentials_override`
   - `test_credentials_with_special_characters`

4. **Configuration Parsing**
   - `test_opts_from_yaml_required_fields`
   - `test_opts_from_yaml_missing_required_fields`
   - `test_opts_from_yaml_with_ami_field`

## Code Examples

### Using the Credentials Module

```rust
use letusinfra::aws::credentials::AwsCredentials;

// Load credentials from environment
match AwsCredentials::from_env() {
    Ok(creds) => {
        println!("Access Key: {}", creds.access_key_id);
        println!("Region: {:?}", creds.region);

        // Validate credentials
        if let Err(e) = creds.validate() {
            eprintln!("Invalid credentials: {}", e);
        }
    }
    Err(e) => {
        eprintln!("Failed to load credentials: {}", e);
    }
}

// Check if credentials are set
if AwsCredentials::are_env_vars_set() {
    println!("Credentials are configured");
}

// Get region with fallback
let region = AwsCredentials::get_region_or_default("us-east-1");
println!("Using region: {}", region);
```

### Creating EC2 Instance with Credentials

```rust
use aws_types::SdkConfig;
use aws_types::region::Region;
use letusinfra::aws::ec2::ec2_instance::EC2Instance;

// Create SDK config from environment
let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string());
let config = SdkConfig::builder()
    .region(Region::new(region))
    .build();

// Create EC2 instance
let ec2_instance = EC2Instance::from_config(&config);
```

## Security Best Practices

1. **Never hardcode credentials** in your source code
2. **Use environment variables** for local development
3. **Use IAM roles** for production AWS environments
4. **Rotate credentials** regularly
5. **Use AWS Secrets Manager** or Parameter Store for sensitive data
6. **Never commit credentials** to version control
7. **Use `.env` files** with `.gitignore` for local development

## Troubleshooting

### Tests Failing

```bash
# Clear environment variables
unset AWS_ACCESS_KEY_ID
unset AWS_SECRET_ACCESS_KEY
unset AWS_REGION

# Run tests
cargo test
```

### Credential Errors

If you see credential-related errors:

1. Check if environment variables are set:

   ```bash
   echo $AWS_ACCESS_KEY_ID
   echo $AWS_SECRET_ACCESS_KEY
   ```

2. Verify AWS CLI configuration:

   ```bash
   aws configure list
   ```

3. Check IAM permissions for your credentials

### Region Errors

If you encounter region-related issues:

1. Set the region explicitly:

   ```bash
   export AWS_REGION=us-west-2
   ```

2. Verify region is valid:
   ```bash
   aws ec2 describe-regions
   ```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run tests
        run: cargo test
        env:
          AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
          AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          AWS_REGION: us-west-2
```

## Additional Resources

- [AWS SDK for Rust Documentation](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html)
- [AWS Credentials Configuration](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)
- [AWS Security Best Practices](https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html)

## Contributing

When adding new credential-related features:

1. Add comprehensive unit tests
2. Add integration tests if needed
3. Update this documentation
4. Follow existing patterns for error handling
5. Ensure backward compatibility
