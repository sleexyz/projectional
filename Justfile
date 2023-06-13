# NOTE: setting positional args will cause script to go in infinite loop
# set positional-arguments

graph-deps:
    #!/usr/bin/env bash
    QUERY='deps(//...)'
    bazel query --noimplicit_deps $QUERY --output graph > graph.dot

list:
    #!/usr/bin/env just _recurse bash
    YELLOW='\033[1;33m'
    CLEAR='\033[0m'
    echo -e "${YELLOW}$DIR${CLEAR}"
    just --summary | pr -to 4

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
    BASE_DIR="${BASE_DIR:-.}"
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
