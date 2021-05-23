FROM rustembedded/cross:armv7-unknown-linux-gnueabihf
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y libusb-1.0-0-dev:armhf
