#!/usr/bin/env bash

set -e

QT_VERSION=6.5.1

sudo apt update
sudo apt -y install \
  git \
  libclang-dev \
  libgl1-mesa-dev \
  cmake \
  clang

mkdir -p ${QT_DIR_PATH}

git clone https://code.qt.io/qt/qt5.git ${QT_SRC_PATH}
cd ${QT_SRC_PATH}
git checkout v${QT_VERSION}

perl ./init-repository --module-subset=essential,qtsvg,qtimageformats,qtcharts,qtshadertools

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
