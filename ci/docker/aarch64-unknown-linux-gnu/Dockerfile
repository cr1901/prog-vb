FROM rustembedded/cross:aarch64-unknown-linux-gnu-0.2.1
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu/pkgconfig
RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y libusb-1.0-0-dev:arm64
