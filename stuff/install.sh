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
  variant="green-pain-apple-arm"
elif [ "$system" = "Darwin x86_64" ]; then
  variant="green-pain-apple-x86_64"
elif [ "$system" = "Linux x86_64" ]; then
  variant="green-pain-linux-x86_64"
else
  err "Unsupported system or arch: $system"
fi

local_bin='.local/bin'
home_local_bin=$HOME'/'$local_bin
env_file='$HOME/.local/env'
env_file_path=$HOME'/.local/env'
version_code=3

if [ -f $home_local_bin'/green-pain' ]; then
	action='updating'
else
	action='installation'
fi

mkdir -p $home_local_bin
ensure cd $home_local_bin
println "downloading..."
ensure curl -X GET -sSfL https://github.com/Atomofiron/green-pain/releases/latest/download/$variant -o green-pain
ensure chmod u+x green-pain

env_script="
case \":\${PATH}:\" in
    *:\"\$HOME/$local_bin\":*)
        ;;
    *)
		export PATH=\$PATH:\$HOME/$local_bin
        ;;
esac
alias adb='\$HOME/$local_bin/green-pain adb'
alias lss='\$HOME/$local_bin/green-pain lss'
alias lsc='\$HOME/$local_bin/green-pain lsc'
alias mss='\$HOME/$local_bin/green-pain mss'
alias shot='\$HOME/$local_bin/green-pain shot'
export ADB_EXT_VERSION_CODE=$version_code
"
printf "$env_script" > $env_file_path
if [ "$ADB_EXT_VERSION_CODE" = "" ]; then # the first installation
    added=false
    for file in ~/.bashrc ~/.zshrc; do
        if [ -f $file ]; then
          printf ". $env_file\n" >> $file
          println "$file done"
          added=true
        fi
    done
    if ! $added; then
        err 'no any ~/.*rc file found'
    fi
fi

printf "%s succeed, run \33[1mgreen-pain\33[0m or \33[1mlss 1\33[0m\n" $action
if [ "$ADB_EXT_VERSION_CODE" != "$version_code" ]; then
	println '... however, first of all to configure your current shell, run:'
	println "\33[1msource "$env_file"\33[0m"
fi
