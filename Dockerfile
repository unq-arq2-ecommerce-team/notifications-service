FROM rust:1.68.2

ENV ROCKET_ADDRESS=0.0.0.0

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build
EXPOSE 8000
CMD ["cargo", "run"]