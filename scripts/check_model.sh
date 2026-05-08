#!/usr/bin/env zsh

self_dir=${0:h}
source "$self_dir/config"
# source ./config

if (( $# < 1 )); then
    # not enough arguments
    print -u2 "Usage: $0 <model.xmv> [options]"
    exit 1
fi

if [[ ! -f $1 ]]; then
    # input file not found
    print -u2 "Error: file '$1' not found"
    exit 1
fi

if [[ $1 =~ _timed ]]; then
    # run timed nuXmv
    if [[ $2 = "-v" ]]; then
        "$nuxmv" -source "$timed_cmd" -time "$1"
    else
        ("$nuxmv" -source "$timed_cmd" -time "$1") | grep -v -e "\*\*\*" -e "no proof or counterexample" -e "^$"
    fi
elif [[ $1 =~ _infinite ]]; then
    # run nuXmv with infinite-domain compatible commands
    if [[ $2 = "-v" ]]; then
        "$nuxmv" -source "$infinite_cmd" "$1"
    else
        ("$nuxmv" -source "$infinite_cmd" "$1") | grep -v -e "\*\*\*" -e "no proof or counterexample" -e "^$"
    fi
else
    # the regular batch mode does what we need
    "$nuxmv" "$1"
fi
