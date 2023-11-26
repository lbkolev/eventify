#----
FROM lukemathwalker/cargo-chef:latest-rust-1.73.0 as chef
WORKDIR /app

LABEL org.opencontainers.image.source=https://github.com/lbkolev/chainthru
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
#----

#----
# Buiild a cargo-chef plan
FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json
#----

#----
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

# Set the build profile to release by default
ARG BUILD_PROFILE=release
ENV BUILD_PROFILE $BUILD_PROFILE
ENV SQLX_OFFLINE true

# Install system dependencies
RUN apt update && apt install lld clang -y

# Build dependencies
RUN cargo chef cook --profile=$BUILD_PROFILE --recipe-path recipe.json

# Build our project
COPY . .
RUN cargo build --profile=$BUILD_PROFILE --locked --bin chainthru

# Determine the correct target directory
RUN if [ "$BUILD_PROFILE" = "dev" ]; then \
        cp /app/target/debug/chainthru /app/chainthru; \
    else \
        cp /app/target/$BUILD_PROFILE/chainthru /app/chainthru; \
    fi
#----

#----
FROM ubuntu AS runtime
WORKDIR /app

# Copy the binary from the build stage
COPY --from=builder /app/chainthru /app
COPY ./migrations/ /app/migrations

# Copy licenses
COPY LICENSE-* ./

EXPOSE 6969
ENTRYPOINT ["/app/chainthru"]
#----
