FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
# Only copy files needed for dependency resolution
COPY Cargo.toml Cargo.lock ./
COPY advanced_markdown_parser ./advanced_markdown_parser
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Install `dx` early so it can be cached
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

# Configure git to use less memory and be more patient
RUN git config --global http.postBuffer 524288000 && \
    git config --global http.lowSpeedLimit 0 && \
    git config --global http.lowSpeedTime 999999

COPY --from=planner /app/recipe.json recipe.json
# Copy path dependencies before cook (needed by cargo-chef)
COPY advanced_markdown_parser ./advanced_markdown_parser
RUN cargo chef cook --release --recipe-path recipe.json
# Copy remaining files needed for build (not articles - they'll be mounted at runtime)
COPY Cargo.toml Cargo.lock Dioxus.toml tailwind.css ./
COPY src ./src
COPY assets ./assets

# Create the final bundle folder. Bundle always executes in release mode with optimizations enabled
RUN dx bundle --web --release

# Debug: List the output to verify build location
RUN ls -la /app/target/dx/ && \
    ls -la /app/target/dx/blogger/ || true && \
    find /app/target/dx -type f -name "blogger" || true

FROM chef AS runtime
COPY --from=builder /app/target/dx/blogger/release/web/ /usr/local/app

# Debug: Verify files are present
RUN ls -a /usr/local/app

# Articles will be mounted as a volume at runtime

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/blogger" ]

