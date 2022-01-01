FROM debian:bullseye-slim


RUN \
    apt update && \
    apt install iverilog ca-certificates openssl -y

#COPY . .    
COPY target/release/voj_judger /usr/local/bin




WORKDIR /data
CMD /usr/local/bin/voj_judger