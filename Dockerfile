# Copied from Lighthouse's Dockerfile at https://github.com/sigp/lighthouse/blob/stable/Dockerfile

FROM rust:1.69.0-bullseye AS builder
RUN apt-get update && apt-get -y upgrade && apt-get install -y cmake libclang-dev
COPY . contower
ARG FEATURES
ARG PROFILE=release
ENV FEATURES $FEATURES
ENV PROFILE $PROFILE
RUN cd contower && make

FROM ubuntu:22.04
RUN apt-get update && apt-get -y upgrade && apt-get install -y --no-install-recommends \
  libssl-dev \
  ca-certificates \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/contower /usr/local/bin/contower