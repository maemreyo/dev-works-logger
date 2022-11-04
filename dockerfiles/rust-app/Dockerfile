# Rust as the base image
FROM rust:1.65

# 1. Create a new empty shell project
RUN USER=root cargo new --bin git-stats-bot
WORKDIR /git-stats-bot

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/git-stats-bot*
RUN cargo install --path .

CMD ["git-stats-bot"]