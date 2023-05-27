# NOTE: setting positional args will cause script to go in infinite loop

list:
    #!/usr/bin/env just _recurse bash
    echo "$DIR":
    just -l | tail -n +2

test directory=".": 
    BASE_DIR="{{directory}}" just _recurse_subcommand "test"

_recurse_subcommand command:
    #!/usr/bin/env just _recurse bash
    set -Eeuo pipefail
    if [[ "$DIR" == "." ]]; then
        exit 0
    fi
    if ! just -l | grep -q '{{command}}$'; then
        exit 0
    fi
    echo "command: $DIR {{command}}"
    just {{command}} |& pr -to 4

_recurse shell script_file:
    #!/bin/bash
    set -Eeuo pipefail
    BASE_DIR=$(realpath "${BASE_DIR:-.}")
    file_paths() {
        ag -l -g 'Justfile$' "$BASE_DIR" | sort
    }
    while IFS= read -r file_path; do
        export DIR=$(dirname $file_path)
        (cd $DIR && {{shell}} {{script_file}})
    done <<< "$(file_paths)"
