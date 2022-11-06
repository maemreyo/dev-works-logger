# STAGE 1: Generate a recipe file for dependencies
FROM rust:1.65 as planner

WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# STAGE 2: Build our dependencies
FROM rust:1.65 as cacher

WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# STAGE 3: Use the main official rust docker image as our build
FROM rust:1.65 as builder

# Copy the app into the docker image
COPY . /app
# Set the work directory
WORKDIR /app

# Copy dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Build the app
RUN cargo build --release

# Use google distroless as runtime image
FROM gcr.io/distroless/cc-debian11

# Copy app from builder
COPY --from=builder /app/target/release/dev-works-logger /app/dev-works-logger
WORKDIR /app

# Start the application
ENTRYPOINT [ "./dev-works-logger" ]