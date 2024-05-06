FROM rust:1.78.0-bullseye AS staging
# Install dependencies and remove the cache to reduce the layer size
RUN apt-get update && apt-get install -y cmake libclang-dev && rm -rf /var/lib/apt/lists/*
COPY . Contower
ARG FEATURES
ARG PROFILE=release
ARG CARGO_USE_GIT_CLI=true
ENV FEATURES $FEATURES
ENV PROFILE $PROFILE
ENV CARGO_NET_GIT_FETCH_WITH_CLI=$CARGO_USE_GIT_CLI
RUN cd Contower && make

FROM ubuntu:22.04
# Install dependencies and remove the cache to reduce the layer size
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=staging /usr/local/cargo/bin/contower /usr/local/bin/contower
# Create a non-root user for security purposes
RUN useradd -m contower
USER contower
