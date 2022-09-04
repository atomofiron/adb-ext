#!/bin/sh

err() {
    print "$1"
    exit 1
}

need_cmd() {
    if ! check_cmd "$1"; then
        err "need '$1' (command not found)"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
}

ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

need_cmd mkdir
need_cmd cd
need_cmd curl
need_cmd chmod
need_cmd printf

if ! [ -d $HOME ]; then
	err 'Home directory not found'
fi

dir=$HOME'/.local/bin/'

if [ -f $dir'/green-pain' ]; then
	action='updating'
else
	action='installation'
fi

mkdir -p $dir
ensure cd $dir
printf "downloading...\n"
ensure curl -X GET -sSfL https://github.com/Atomofiron/green-pain/releases/download/release-0.1.0/green-pain -o green-pain
ensure chmod u+x green-pain
printf "%s succeed, run \33[1msudo green-pain\33[0m\n" $action
