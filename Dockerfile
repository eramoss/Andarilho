FROM rust:latest

# Create a directory to store the CA certificates
RUN mkdir /usr/local/certs
COPY ./certs /usr/local/certs

# Set the SSL_CERT_FILE environment variable to point to the CA certificates
ENV SSL_CERT_FILE=/usr/local/certs/certificate.crt

# Set the working directory in the Docker image
WORKDIR /usr/src/app

# Copy the Rust application source code to the working directory
COPY . .

# Build and run your Rust application
RUN cargo build --release

CMD ["cargo", "run", "--release"]
