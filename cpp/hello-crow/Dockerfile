FROM gcc:14 AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    git \
    libssl-dev \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY CMakeLists.txt main.cpp ./

RUN cmake -B build -DCMAKE_BUILD_TYPE=Release \
    && cmake --build build --parallel $(nproc)

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    zlib1g \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/build/hello-crow ./hello-crow

RUN useradd -u 1001 -r -s /sbin/nologin appuser
USER 1001

EXPOSE 10000

ENTRYPOINT ["./hello-crow"]
