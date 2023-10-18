# build stage
FROM rust:1.73 AS builder 

WORKDIR /api-build

COPY . . 

RUN cargo build --release

CMD [ "./target/release/echo-api" ]
