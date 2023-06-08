def select_with_common(select_dict, common):
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
    # "CARGO_TERM_VERBOSE": "true",
})

def cargo_with_tree_sitter_features(cmd):
    return select({
        "@platforms//cpu:wasm32": "cargo %s --release --features wasm --no-default-features" % cmd,
        "//conditions:default": "cargo %s --release" % cmd,
    })

def cargo(cmd):
    return select({
        "@platforms//cpu:wasm32": "cargo %s --release" % cmd,
        "//conditions:default": "cargo %s --release" % cmd,
    })
