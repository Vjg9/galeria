FROM rust:latest AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:latest

COPY --from=build /app/target/release/ app/
WORKDIR /app
CMD ["./server"]
