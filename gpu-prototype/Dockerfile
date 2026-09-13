FROM rocm/dev-ubuntu-24.04:7.2.4

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends rocprim-dev \
    && rm -rf /var/lib/apt/lists/*
