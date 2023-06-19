#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

if [ "$OS" = "windows-2022" ]; then
  7z a -tzip "nai-$RELEASE_VERSION-$TARGET.zip" nai-"$RELEASE_VERSION"/
else
  tar -czvf nai-"$RELEASE_VERSION"-"$TARGET".tar.gz nai-"$RELEASE_VERSION"/
  shasum -a 512 nai-"$RELEASE_VERSION"-"$TARGET".tar.gz >nai-"$RELEASE_VERSION"-"$TARGET".tar.gz.sha512
fi
