FROM rust:1-alpine AS build

WORKDIR /app
COPY . .
RUN apk add --no-cache bash musl-dev wget
RUN cargo install --root . --path .

FROM alpine

WORKDIR /app
COPY --from=build /app/bin/pluto .

EXPOSE 46963
CMD ["pluto"]

