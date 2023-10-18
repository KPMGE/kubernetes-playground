# build stage
FROM rust:1.73 AS builder 

WORKDIR /api-build

COPY . . 
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

CMD [ "./target/release/echo-api" ]

FROM debian:buster-slim

WORKDIR /api

RUN apt update 

COPY --from=builder /api-build/target/x86_64-unknown-linux-gnu/release/echo-api /api/

CMD [ "./echo-api" ]
