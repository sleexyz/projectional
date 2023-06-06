cargo_build_env = select({
    "@platforms//cpu:wasm32": {
        "CC": "clang-wasi32",
        "CARGO_BUILD_TARGET": "wasm32-unknown-unknown",
    },
    "//conditions:default": {
    },
})

def cargo_with_tree_sitter_features(cmd):
    return select({
        "@platforms//cpu:wasm32": "cargo " + cmd + " --features wasm --no-default-features",
        "//conditions:default": "cargo " + cmd,
    })
