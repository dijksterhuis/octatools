FROM rust:latest AS build

RUN mkdir -p /build
WORKDIR /build
COPY Cargo.toml Cargo.lock /build/
COPY ./ot-tools-cli/ /build/ot-tools-cli/
COPY ./ot-tools-io/ /build/ot-tools-io/
COPY ./ot-tools-derive/ /build/ot-tools-derive/
COPY ./ot-tools-ops/ /build/ot-tools-ops/
COPY ./ot-tools-py/ /build/ot-tools-py/
RUN cargo build --release --bin ot-tools

FROM rust:latest

MAINTAINER "Mike Robeson [dijksterhuis <ot-tools@dijksterhuis.co.uk>"
LABEL org.ot-tools.image.author="Mike Robeson [dijksterhuis]"
LABEL org.ot-tools.image.email="ot-tools@dijksterhuis.co.uk"
LABEL org.ot-tools.image.homepage="https://gitlab.com/ot-tools/ot-tools"
LABEL org.ot-tools.image.license="GPL-3.0-or-later"

COPY --from=build /build/target/release/ot-tools /ot-tools

COPY <<-EOT /docker-entrypoint.sh
#!/usr/bin/env bash
/ot-tools \${@}
EOT

RUN chmod +x /docker-entrypoint.sh

ENTRYPOINT ["/docker-entrypoint.sh"]
