FROM sptrakesh/cppbase AS base

RUN apk --update --no-cache add curl libunwind \
  && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY Cargo.* /opt/s3-signed-put/
COPY src /opt/s3-signed-put/src

WORKDIR /opt/s3-signed-put
RUN . $HOME/.cargo/env \
  && cargo build --release

FROM alpine
WORKDIR /opt/s3-signed-put/bin
COPY --from=base /opt/s3-signed-put/target/release/s3-signed-put .