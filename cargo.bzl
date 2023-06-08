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
#     "RUSTC_WRAPPER": "sccache",
    "PROFILE": "release",
    "CARGO_INCREMENTAL": "0",
    "CARGO_TARGET_DIR": "$DIR_ROOT/__cargo-target__",
    # "CARGO_TARGET_DIR": "/tmp/__cargo-target__",
})

def cargo_with_tree_sitter_features(cmd):
    return select({
        "@platforms//cpu:wasm32": "cargo --verbose " + cmd + " --features wasm --no-default-features",
        "//conditions:default": "cargo --verbose " + cmd,
    })
