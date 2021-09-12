# syntax = docker/dockerfile:1.2
FROM clux/muslrust:stable as builder

WORKDIR /volume

COPY assets/ assets/
COPY src/ src/
COPY templates/ templates/
COPY Cargo.lock Cargo.toml ./

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/volume/target \
    cargo install --locked --path .

RUN strip --strip-all /root/.cargo/bin/{{ project_name }}

FROM scratch

COPY --from=builder /root/.cargo/bin/{{ project_name }} /bin/

EXPOSE 8080
STOPSIGNAL SIGINT

ENTRYPOINT ["/bin/{{ project_name }}"]
