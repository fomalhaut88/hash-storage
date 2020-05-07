FROM rustlang/rust:nightly

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
