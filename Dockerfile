FROM rust:1.60 as builder

WORKDIR /root/

COPY . .

RUN cargo build --release --bin mdla-server


FROM ubuntu:rolling

WORKDIR /root/

COPY --from=builder /root/target/release/mdla-server /root/
COPY --from=builder /root/word_list_all.db /root/
COPY --from=builder /root/word_list_playable.db /root/

ENTRYPOINT ["./mdla-server"]
