load("//:rules_hybrid/defs.bzl", "hybrid_step", "hybrid_test")
load("//:cargo.bzl", "cargo_build_env", "cargo_with_tree_sitter_features")

package(default_visibility = ["//visibility:public"])

exports_files([
    "Cargo.toml",
])

hybrid_step(
    name = "build",
    srcs = glob(
        [
            "src/**/*",
        ],
    ),
    outs = [
        ".",
        "$CARGO_TARGET_DIR",
    ],
    cmd = cargo_with_tree_sitter_features("build"),
    env = cargo_build_env,
    deps = [
        "//:cargo_workspace",
        "//tree-sitter-puddlejumper:build",
    ],
)

hybrid_step(
    name = "test_lib",
    outs = [
        ".",
        "$CARGO_TARGET_DIR",
    ],
    cmd = cargo_with_tree_sitter_features("test --no-run"),
    env = cargo_build_env,
    deps = [":build"],
)

hybrid_test(
    name = "test",
    cmd = cargo_with_tree_sitter_features("test"),
    env = cargo_build_env,
    deps = [":test_lib"],
)
