FROM rust:1.33.0

RUN rustup default nightly-2019-01-29

WORKDIR /usr/src/app

EXPOSE 8000

RUN cargo install diesel_cli --no-default-features --features mysql

COPY Cargo.toml /usr/src/app/
RUN cargo fetch

COPY . /usr/src/app/
ARG DATABASE_URL
RUN diesel setup; \
    diesel migration run
RUN cargo build --release

CMD ROCKET_ENV=prod ./target/release/hash-storage
