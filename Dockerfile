FROM rust:1.60 as builder

WORKDIR /root/

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install wasm-bindgen-cli

COPY . .

RUN cargo build --release --bin mdla-server
RUN trunk build --release ./mdla-front/index.html


FROM ubuntu:rolling

WORKDIR /root/

COPY --from=builder /root/target/release/mdla-server /root/
COPY --from=builder /root/mdla-front/dist/ /root/resources/web-app/

COPY --from=builder /root/word_list_all.db /root/
COPY --from=builder /root/word_list_playable.db /root/

ENTRYPOINT ["./mdla-server"]
