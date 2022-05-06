FROM rust:1.60 as rust-builder

WORKDIR /root/

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install wasm-bindgen-cli


FROM rust-builder as app-builder
COPY . .


FROM app-builder as mdla-server-builder
RUN cargo build --release --bin mdla-server


FROM app-builder as mdla-front-builder
RUN trunk build --release ./mdla-front/index.html


FROM ubuntu:rolling

WORKDIR /root/

COPY --from=mdla-server-builder /root/target/release/mdla-server /root/
COPY --from=mdla-front-builder /root/mdla-front/dist/ /root/resources/web-app/

COPY --from=app-builder /root/word_list_all.db /root/
COPY --from=app-builder /root/word_list_playable.db /root/

ENTRYPOINT ["./mdla-server", "-p", "80"]
