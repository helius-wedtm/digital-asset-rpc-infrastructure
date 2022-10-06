FROM rust:1.63-bullseye AS chef
RUN cargo install cargo-chef
FROM chef AS planner
COPY load_generation_candy_machine /rust/load_generation_candy_machine/
WORKDIR /rust/load_generation_candy_machine
RUN cargo chef prepare --recipe-path recipe.json
FROM chef AS builder
RUN apt-get update -y && \
    apt-get install -y build-essential make git libudev-dev
COPY load_generation_candy_machine /rust/load_generation_candy_machine
RUN mkdir -p /rust/load_generation_candy_machine
WORKDIR /rust/load_generation_candy_machine
COPY --from=planner /rust/load_generation_candy_machine/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
COPY load_generation_candy_machine/Cargo.toml .
RUN cargo chef cook --release --recipe-path recipe.json
COPY load_generation_candy_machine .
# Build application
RUN cargo build --release
FROM rust:1.63-slim-bullseye
ARG APP=/usr/src/app
RUN apt update \
    && apt install -y curl ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*
ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}
COPY --from=builder /rust/load_generation_candy_machine/target/release/load_generation_candy_machine ${APP}
RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}
CMD /usr/src/app/load_generation_candy_machine
