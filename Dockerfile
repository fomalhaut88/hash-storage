FROM rustlang/rust:nightly

WORKDIR /usr/src/app

EXPOSE 8000

RUN cargo install diesel_cli --no-default-features --features mysql

COPY . /usr/src/app/
ARG DATABASE_URL
RUN diesel setup; \
    diesel migration run
RUN BIGI_BITS=512 cargo build --release

CMD ROCKET_ENV=prod ./target/release/hash-storage
