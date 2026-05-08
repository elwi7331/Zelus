#!/usr/bin/env zsh

self_dir=${0:h}
source "$self_dir/config"

if (( $# < 2 )); then
    # not enough arguments
    print -u2 "Usage: $0 <c2kratos-options> <program.c> <destination.k2>"
    exit 1
fi

if [[ ! -f $@[-2] ]]; then
    # input file not found
    print -u2 "Error: file '$@[-2]' not found"
    exit 1
fi

c2k2_options="--builtin-assert assert --builtin-havoc havoc --builtin-assume assume --detect-output-params --verbose --initialize-globals --init-function init --floats-as-reals"

if (( $# > 2 )); then
    for arg in "${@[1,-3]}"; do
        c2k2_options+=" $arg"
    done
fi

"$python" "$c2kratos" ${=c2k2_options} -o "$@[-1]" "$@[-2]"
