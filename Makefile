# Define the target binary name
TARGET := my_lambda_function

# Define the Rust source file
SRC := main.rs

# Define the Cargo command
CARGO := cargo

# Define the AWS CLI command
AWS := aws

# Define the AWS Lambda function name
FUNCTION_NAME := my-lambda-function

# Define the AWS region
REGION := us-east-1

# Build the Rust project
build:
	$(CARGO) build --release --target

# Deploy the Lambda function
deploy: package
	$(AWS) lambda update-function-code \
		--function-name $(FUNCTION_NAME) \
		--zip-file fileb://target/x86_64-unknown-linux-musl/release/$(TARGET).zip \
		--region $(REGION)

# Clean the build artifacts
clean:
	$(CARGO) clean

.PHONY: build package deploy clean