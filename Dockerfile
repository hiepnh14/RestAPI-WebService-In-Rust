# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app
USER root

# Copy the source code to the container
COPY . .
# Build the microservice
RUN cargo build --release

# Create a new image with only the compiled microservice
FROM debian:bookworm-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled microservice from the builder stage
COPY --from=builder /app/target/release/microservice .

# Expose the port that the microservice listens on
EXPOSE 8080

# Set the command to run the microservice
CMD ["./microservice"]