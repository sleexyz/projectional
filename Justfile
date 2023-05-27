# NOTE: setting positional args will cause script to go in infinite loop
# set positional-arguments

list:
    #!/usr/bin/env just _recurse bash
    echo "$DIR":
    just -l | tail -n +2

run directory="." *args="":
    BASE_DIR="{{directory}}" just _recurse_subcommand "run" {{args}}

build directory="." *args="":
    BASE_DIR="{{directory}}" just _recurse_subcommand "build" {{args}}

test directory="." *args="":
    BASE_DIR="{{directory}}" just _recurse_subcommand "test" {{args}}

[no-exit-message]
_recurse_subcommand command *args='':
    #!/usr/bin/env just _recurse bash
    set -Eeuo pipefail

    if [[ $(realpath "$DIR") == $(realpath {{justfile_directory()}}) ]]; then
        exit 0
    fi
    if ! just -l | grep -q -E '{{command}}$|^\s+{{command}}'; then
        exit 0
    fi
    just {{command}} {{args}} |& pr -to 4

[no-exit-message]
_recurse shell script_file:
    #!/bin/bash
    set -Eeuo pipefail
    ORIG_BASE_DIR=$BASE_DIR
    BASE_DIR=$(realpath "${BASE_DIR:-.}")
    file_paths=$(ag -l -g 'Justfile$' "$BASE_DIR" | sort)
    matches=0
    while IFS= read -r file_path; do
        export DIR=$(dirname $file_path)
        output=$(cd $DIR && {{shell}} {{script_file}} | tee /dev/stderr)
        if [[ -n "$output" ]]; then
            matches=$((matches + 1))
        fi
    done <<< "$file_paths"
    if [[ $matches == 0 ]]; then
        echo "no matching recipes found"
        exit 1
    fi
