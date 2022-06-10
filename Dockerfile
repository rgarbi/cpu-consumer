FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=cpu-consumer
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /cpu-consumer

COPY src .

RUN cargo build --release

######################
FROM ubuntu:latest as cpu-consumer

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /cpu-consumer

# Copy our build
COPY --from=builder /cpu-consumer/target/release/cpu-consumer ./
COPY --from=builder /cpu-consumer/configuration ./configuration

# Use an unprivileged user.
USER cpu-consumer:cpu-consumer

CMD ["/cpu-consumer/cpu-consumer"]
