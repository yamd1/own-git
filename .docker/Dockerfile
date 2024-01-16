FROM rust:1.75-bullseye AS base

ENV TZ=UTC \
  LANG=en_US.UTF-8 \
  LANGUAGE=en_US:en \
  LC_ALL=en_US.UTF-8

RUN apt-get update \
  && apt-get -y install --no-install-recommends \
    git \
    unzip=* \
    libzip-dev=* \
    libicu-dev=* \
    libonig-dev=* \
    locales=* \
    zsh \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* \
  && locale-gen en_US.UTF-8 \
  && localedef -f UTF-8 -i en_US en_US.UTF-8

FROM base as workspace

ARG USER_UID=1000
ARG USER_GID=1000

RUN \
  apt-get update && \
  apt-get install -y --no-install-recommends sudo=* && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

RUN \
  useradd --shell /bin/bash --create-home --user-group appuser && \
  usermod --non-unique --uid ${USER_UID} appuser && \
  groupmod --non-unique --gid ${USER_UID} appuser && \
  echo "appuser ALL=(root) NOPASSWD:ALL" > /etc/sudoers.d/appuser && \
  chmod 0440 /etc/sudoers.d/appuser

RUN install -o appuser -g appuser -d /opt/app

USER appuser
WORKDIR /opt/app
