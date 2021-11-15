FROM debian:bullseye-slim

ENV PATH="/usr/local/bin:${PATH}"

RUN \
    apt update && \
    apt install iverilog -y

#COPY . .    
COPY /release/voj_judger /usr/local/bin




WORKDIR /data
CMD /usr/local/bin/voj_judger