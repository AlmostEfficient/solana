FROM rust:latest

RUN apt-get update && apt-get install -y mscgen
RUN cargo install svgbob_cli --version 0.7.0

WORKDIR /solana/docs

COPY ./convert-ascii-to-svg.sh .

RUN chmod +x convert-ascii-to-svg.sh

CMD ["./convert-ascii-to-svg.sh"]
