FROM rustlang/rust:nightly
WORKDIR /usr/src/app
EXPOSE 8000
COPY . /usr/src/app/
ARG DATABASE_URL
RUN diesel setup
RUN diesel migration run
RUN cargo build --release
CMD ROCKET_ENV=prod ./target/release/hash-storage
