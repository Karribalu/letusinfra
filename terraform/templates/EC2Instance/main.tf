provider "aws" {
  region = var.aws_region
}

resource "aws_default_vpc" "default" {
  tags = {
    Name = "Default VPC"
  }
}

resource "aws_instance" "app_server" {
  ami           = var.ami_id
  instance_type = var.instance_type

  tags = {
    Environment = "let-us-infra"
  }
}

resource "aws_instance" "app_server_2" {
  ami           = var.ami_id
  instance_type = var.instance_type

  tags = {
    Environment = "let-us-infra"
  }
}

resource "aws_security_group" "allow_tls" {
  name        = "allow_tls"
  description = "Allow TLS inbound traffic and all outbound traffic"
  vpc_id      = aws_default_vpc.default.id

  tags = {
    Name = "allow_tls"
  }
}


output "ec2_output" {
  value = aws_instance.app_server
}

output "ec2_output_1" {
  value = aws_instance.app_server_2
}

output "security_group" {
  value = aws_security_group.allow_tls
}
