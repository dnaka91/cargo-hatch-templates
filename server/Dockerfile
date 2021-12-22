FROM rust:1.57-alpine as builder

WORKDIR /volume

RUN apk add --no-cache musl-dev=~1.2

COPY assets/ assets/
COPY src/ src/
COPY templates/ templates/
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release && \
    strip --strip-all target/release/{{ project_name }}

FROM alpine:3.15 as newuser

RUN echo "{{ project_name }}:x:1000:" > /tmp/group && \
    echo "{{ project_name }}:x:1000:1000::/dev/null:/sbin/nologin" > /tmp/passwd

FROM scratch

COPY --from=builder /volume/target/release/{{ project_name }} /bin/
COPY --from=newuser /tmp/group /tmp/passwd /etc/

EXPOSE 8080
STOPSIGNAL SIGINT
USER {{ project_name }}

ENTRYPOINT ["/bin/{{ project_name }}"]
