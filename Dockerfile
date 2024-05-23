FROM rust:1-alpine AS build

WORKDIR /app
COPY . .
RUN apk add --no-cache bash musl-dev pkgconfig openssl-dev openssl-libs-static
RUN cargo install --root . --path .

FROM alpine

RUN apk add --no-cache curl

WORKDIR /app
COPY --from=build /app/bin/pluto .

HEALTHCHECK CMD curl -sSf http://127.0.0.1:46963/health

EXPOSE 46963
CMD ["/app/pluto"]

