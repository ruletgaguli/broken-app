FROM rust:1.96-bookworm
RUN apt-get update && apt-get install -y --no-install-recommends valgrind gdb linux-perf python3 librsvg2-bin && rm -rf /var/lib/apt/lists/*
WORKDIR /work
