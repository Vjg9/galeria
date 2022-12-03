FROM rust:latest AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:latest

COPY --from=build /app/target/release/ app/
WORKDIR /app
ENV PORT=8080
ENV MODE="production"
CMD ["./server"]
