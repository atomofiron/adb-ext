#!/bin/sh

println() {
	printf "$1\n"
}

err() {
    printf "ERROR: $1"
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

local_bin='.local/bin'
home_local_bin=$HOME'/'$local_bin
env_file='$HOME/.local/env'
env_file_path=$HOME'/.local/env'
version_code=3

if [ -f $home_local_bin'/adb-ext' ]; then
	action='updating'
else
	action='installation'
fi

mkdir -p $home_local_bin
ensure cd $home_local_bin
println "downloading..."
ensure curl -X GET -sSfL https://github.com/Atomofiron/adb-ext/releases/latest/download/$variant -o adb-ext
ensure chmod u+x adb-ext
ensure ln -sf adb-ext adb
ensure ln -sf adb-ext lss
ensure ln -sf adb-ext lsc
ensure ln -sf adb-ext mss
ensure ln -sf adb-ext shot

env_script="
case \":\${PATH}:\" in
    *:\"\$HOME/$local_bin\":*)
        ;;
    *)
		export PATH=\$HOME/$local_bin:\$PATH
        ;;
esac
alias adb=adb-ext
unalias lss 2>/dev/null
unalias lsc 2>/dev/null
unalias mss 2>/dev/null
unalias shot 2>/dev/null
export ADB_EXT_VERSION_CODE=$version_code
"
printf "$env_script" > $env_file_path
if [ "$ADB_EXT_VERSION_CODE" = "" ]; then # the first installation
    added=false
    for file in ~/.bashrc ~/.zshrc ~/.profile; do
        if [ -f $file ]; then
          printf ". $env_file\n" >> $file
          println "$file done"
          added=true
        fi
    done
    if ! $added; then
        err 'no any ~/.*rc or ~/.profile file found'
    fi
fi

printf "%s succeed, run \33[1madb fix\33[0m or \33[1mlss\33[0m\n" $action
if [ "$ADB_EXT_VERSION_CODE" != "$version_code" ]; then
	println '... however, first of all to configure your current shell, run:'
	println "\33[1msource "$env_file"\33[0m"
fi
