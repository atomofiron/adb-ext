#!/bin/sh

err() {
    printf "ERROR: $1"
    exit 1
}

ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

tryDequarantine=false
system=$(ensure uname -sm)
if [ "$system" = "Darwin arm64" ]; then
  variant="adb-ext-macos-arm"
  tryDequarantine=true
elif [ "$system" = "Darwin x86_64" ]; then
  variant="adb-ext-macos-x86_64"
  tryDequarantine=true
elif [ "$system" = "Linux x86_64" ]; then
  variant="adb-ext-linux-x86_64"
elif [ "$system" = "Linux aarch64" ]; then
  variant="adb-ext-linux-arm"
else
  err "Unsupported system or arch: $system"
fi

ensure curl -X GET -fL --progress-bar https://github.com/atomofiron/adb-ext/releases/latest/download/$variant -o adb-ext
ensure chmod u+x adb-ext
if [ tryDequarantine ]; then
  xattr -d com.apple.quarantine ./adb-ext 2&>/dev/null
fi
ensure ./adb-ext deploy
