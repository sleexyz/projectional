"""
Macros for building with cargo for this project.
"""

def select_with_common(select_dict, common):
    """
    Allows defaults for select

    Args:
        select_dict: dict of select values
        common: dict of common values for each exelect branch
    """
    return select(
        {key: dict(common, **value) for key, value in select_dict.items()},
    )

cargo_build_env = select_with_common({
    "@platforms//cpu:wasm32": {
        "CARGO_BUILD_TARGET": "wasm32-unknown-unknown",
    },
    "//conditions:default": {
    },
}, {
    "CARGO_TARGET_DIR": "$DIR_ROOT/target",
    "CARGO_TERM_VERBOSE": "false",
    "CARGO_INCREMENTAL": "1",
    "PROFILE": "debug",
})

def cargo_with_tree_sitter_features(cmd):
    return select({
        "@platforms//cpu:wasm32": "cargo %s --features wasm --no-default-features $@" % cmd,
        "//conditions:default": "cargo %s $@" % cmd,
    })

def cargo(cmd):
    return select({
        "@platforms//cpu:wasm32": "cargo %s $@" % cmd,
        "//conditions:default": "cargo %s $@" % cmd,
    })
