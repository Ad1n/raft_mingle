FROM ubuntu:focal-20201106

ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_TARGET_DIR=binaries

COPY . .

RUN chmod +x ./Dockerfile.sh
RUN chmod +x ./build.sh

RUN DEBIAN_FRONTEND="noninteractive" ./Dockerfile.sh
RUN ./build.sh

CMD ["server"]
