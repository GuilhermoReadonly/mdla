FROM rust:1.60 as rust-builder

WORKDIR /root/

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install wasm-bindgen-cli

FROM rust-builder as app-builder

COPY . .

RUN cargo build --release --bin mdla-server
RUN trunk build --release ./mdla-front/index.html


FROM ubuntu:rolling

WORKDIR /root/

COPY --from=app-builder /root/target/release/mdla-server /root/
COPY --from=app-builder /root/mdla-front/dist/ /root/resources/web-app/

COPY --from=app-builder /root/word_list_all.db /root/
COPY --from=app-builder /root/word_list_playable.db /root/

ENTRYPOINT ["./mdla-server"]
