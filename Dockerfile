FROM lukemathwalker/cargo-chef:latest-rust-1.72.0 as chef
ARG APP_NAME
WORKDIR /var/www/app
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /var/www/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin $APP_NAME
COPY . .
RUN cargo build --release --bin $APP_NAME

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /var/www/app/target/release/$APP_NAME /
WORKDIR /
EXPOSE 8080
ENV APP_NAME = $APP_NAME
CMD ["./${APP_NAME}"]
