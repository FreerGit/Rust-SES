FROM rust:1.63.0
# switch working directory to `app`
WORKDIR /app
# Install system dependencies for linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
# Make sure that sqlx does not require the DB running (compiled offline)
ENV SQLX_OFFLINE true
# Build prod
RUN cargo build --release
# Prod mode for configuration
ENV APP_ENVIRONMENT production
# When `docker run` is executed
ENTRYPOINT ["./target/release/zero2prod"]