#!/bin/sh

err() {
    printf "ERROR: $1"
    exit 1
}

ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

system=$(ensure uname -sm)
if [ "$system" = "Darwin arm64" ]; then
  variant="adb-ext-apple-arm"
elif [ "$system" = "Darwin x86_64" ]; then
  variant="adb-ext-apple-x86_64"
elif [ "$system" = "Linux x86_64" ]; then
  variant="adb-ext-linux-x86_64"
else
  err "Unsupported system or arch: $system"
fi

ensure curl -X GET -sSfL https://github.com/Atomofiron/adb-ext/releases/latest/download/$variant -o adb-ext
ensure chmod u+x adb-ext
./adb-ext deploy
