# Use the right version correponding to your rust version
FROM rust:latest AS builder

# set up work directory
WORKDIR /myapp
USER root

# copy the entire project into the working dicrectory
COPY . .

# compile rust app in the working directory
RUN cargo build --release

# use the right image according to different versions of glibc
FROM debian:bookworm-slim

# set up working directory
WORKDIR /myapp

# copy the executable file to the working directory for easily launching
COPY --from=builder /myapp/target/release/microservice /myapp

# expose port
EXPOSE 8080

# run the app
CMD ["./microservice"]

