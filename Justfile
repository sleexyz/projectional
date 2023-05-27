
list:
    #!/usr/bin/env just _recurse bash
    echo "$DIR":
    just -l | tail -n +2

test: (_recurse_subcommand "test")

_recurse_subcommand command:
    #!/usr/bin/env just _recurse bash
    set -Eeuo pipefail
    if [[ "$DIR" == "." ]]; then
        exit 0
    fi
    if ! just -l | grep -q '{{command}}$'; then
        exit 0
    fi
    echo "$DIR":
    just {{command}} |& pr -to 4

_recurse shell script_file:
    #!/bin/bash
    set -Eeuo pipefail
    file_paths() {
        ag -l -G 'Justfile$' . | sort
    }
    while IFS= read -r file_path; do
        export DIR=$(dirname $file_path)
        (cd $DIR && {{shell}} {{script_file}})
    done <<< "$(file_paths)"
