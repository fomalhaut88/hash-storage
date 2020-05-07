FROM rustlang/rust:nightly
WORKDIR /usr/src/app
EXPOSE 8000
COPY . /usr/src/app/
ARG DATABASE_URL
RUN cargo install diesel_cli --no-default-features --features postgres; \
    diesel setup; \
    diesel migration run
RUN cargo build --release
CMD ROCKET_ENV=prod ./target/release/hash-storage
