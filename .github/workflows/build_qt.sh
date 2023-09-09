#!/usr/bin/env bash

set -e

QT_VERSION=6.5.1

sudo apt update
sudo apt -y install \
  git \
  libclang-dev \
  libgl1-mesa-dev \
  libfontconfig1-dev \
  libfreetype6-dev \
  libx11-dev \
  libx11-xcb-dev \
  libxext-dev \
  libxfixes-dev \
  libxi-dev \
  libxrender-dev \
  libxcb1-dev \
  libxcb-glx0-dev \
  libxcb-keysyms1-dev \
  libxcb-image0-dev \
  libxcb-shm0-dev \
  libxcb-icccm4-dev \
  libxcb-sync-dev \
  libxcb-xfixes0-dev \
  libxcb-shape0-dev \
  libxcb-randr0-dev \
  libxcb-render-util0-dev \
  libxcb-util-dev \
  libxcb-xinerama0-dev \
  libxcb-xkb-dev \
  libxkbcommon-dev \
  libxkbcommon-x11-dev \
  libatspi2.0-dev \
  dbus-x11 \
  libpcre2-dev \
  cmake \
  ninja-build \
  clang

git clone https://code.qt.io/qt/qt5.git ${QT_SRC_PATH}
cd ${QT_SRC_PATH}
git checkout v${QT_VERSION}

perl ./init-repository --module-subset=essential,qtsvg,qtimageformats,qtcharts,qtshadertools

mkdir ${QT_DIR_PATH}
mkdir build
cd build
../configure \
  -prefix ${QT_DIR_PATH} \
  -release \
  -static \
  -static-runtime \
  -opensource \
  -nomake tools \
  -nomake examples \
  -nomake tests \
  -nomake benchmarks \
  -nomake manual-tests \
  -opengl desktop \
  -qt-zlib \
  -qt-freetype \
  -qt-harfbuzz \
  -qt-libpng \
  -qt-libjpeg \
  -qt-sqlite \
  -qt-pcre \
  -ltcg \
  -no-pch \
  -optimize-size \
  -no-qml-debug

cmake --build . --parallel 2
cmake --install .
