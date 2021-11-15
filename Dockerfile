FROM debian:bullseye-slim


RUN \
    apt update && \
    apt install iverilog -y

COPY . .    