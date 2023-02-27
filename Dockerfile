
FROM rust:alpine

ENV LANG C.UTF-8

RUN set -ex && apk add --no-cache --update \
    cmake g++ git linux-headers libpthread-stubs build-base

