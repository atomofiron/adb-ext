#!/bin/sh

println() {
	printf "$1\n"
}

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

local_bin='.local/bin'
home_local_bin=$HOME'/'$local_bin
env_file='$HOME/.local/env'
env_file_path=$HOME'/.local/env'

if [ -f $home_local_bin'/green-pain' ]; then
	action='updating'
else
	action='installation'
fi

mkdir -p $home_local_bin
ensure cd $home_local_bin
println "downloading..."
ensure curl -X GET -sSfL https://github.com/Atomofiron/green-pain/releases/latest/download/green-pain -o green-pain
ensure chmod u+x green-pain

notice_user=false

case ":${PATH}:" in
    *:"$home_local_bin":*)
        ;;
    *)
		env_case="
case \":\${PATH}:\" in
    *:\"\$HOME/$local_bin\":*)
        ;;
    *)
		export PATH=\$PATH:\$HOME/$local_bin
        ;;
esac
alias adb='\$HOME/$local_bin adb'
alias lss='\$HOME/$local_bin lss'
"
		printf "$env_case" > $env_file_path
		if check_cmd bash; then
			printf ". $env_file\n" >> ~/.bashrc
			println '.bashrc done'
		fi
		if check_cmd zsh; then
			printf ". $env_file\n" >> ~/.zshrc
			println '.zshrc done'
		fi
		notice_user=true
        ;;
esac

printf "%s succeed, run \33[1msudo green-pain\33[0m\n" $action
if $notice_user; then
	println '... however, first of all to configure your current shell, run:'
	println "\33[1msource "$env_file"\33[0m"
fi
