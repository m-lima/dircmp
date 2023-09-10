FROM ubuntu

WORKDIR /src

RUN \
  apt update && \
  apt -y install \
    git \
    libclang-dev \
    libgl1-mesa-dev \
    # libfontconfig1-dev \
    # libfreetype6-dev \
    # libx11-dev \
    # libx11-xcb-dev \
    # libxext-dev \
    # libxfixes-dev \
    # libxi-dev \
    # libxrender-dev \
    # libxcb1-dev \
    # libxcb-glx0-dev \
    # libxcb-keysyms1-dev \
    # libxcb-image0-dev \
    # libxcb-shm0-dev \
    # libxcb-icccm4-dev \
    # libxcb-sync-dev \
    # libxcb-xfixes0-dev \
    # libxcb-shape0-dev \
    # libxcb-randr0-dev \
    # libxcb-render-util0-dev \
    # libxcb-util-dev \
    # libxcb-xinerama0-dev \
    # libxcb-xkb-dev \
    # libxkbcommon-dev \
    # libxkbcommon-x11-dev \
    # libatspi2.0-dev \
    # dbus-x11 \
    # libpcre2-dev \
    cmake \
    # ninja-build \
    clang && \
  mkdir /qt && \
  git clone https://code.qt.io/qt/qt5.git /qt_src

# RUN \
#   cd /qt_src && \
#   git checkout v6.5.1 && \
#   ./init-repository --module-subset=essential,qtsvg,qtimageformats,qtcharts,qtshadertools && \
#   mkdir build && \
#   cd build && \
#   ../configure \
#     -prefix /qt \
#     -release \
#     -static \
#     -static-runtime \
#     -opensource \
#     -nomake tools \
#     -nomake examples \
#     -nomake tests \
#     -nomake benchmarks \
#     -nomake manual-tests \
#     -opengl desktop \
#     -qt-zlib \
#     -qt-freetype \
#     -qt-harfbuzz \
#     -qt-libpng \
#     -qt-libjpeg \
#     -qt-sqlite \
#     -qt-pcre \
#     -ltcg \
#     -no-pch \
#     -optimize-size \
#     -no-qml-debug
