FROM japaric/aarch64-unknown-linux-gnu:v0.1.14
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y libusb-1.0-0-dev:arm64
