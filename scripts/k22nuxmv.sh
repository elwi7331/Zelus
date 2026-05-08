#!/usr/bin/env zsh

self_dir=${0:h}
source "$self_dir/config"

if (( $# < 2 )); then
    # not enough arguments
    print -u2 "Usage: $0 <kratos-options> <program.k2> <destination.xmv>"
    exit 1
fi

if [[ ! -f $@[-2] ]]; then
    # input file not found
    print -u2 "Error: file '$@[-2]' not found"
    exit 1
fi

kratos_options="-trans_output_format=nuxmv -apply_slicing=false -verbosity=1000 -trans_encoding=basic"

if (( $# > 2 )); then
    for arg in "${@[1,-3]}"; do
        kratos_options+=" $arg"
    done
fi

"$kratos" ${=kratos_options} -output_file="$@[-1]" "$@[-2]"
